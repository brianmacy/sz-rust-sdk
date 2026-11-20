//! Core implementation of the `SzLogging` trait over the Senzing 4.5.0 libSzLogging API.
//!
//! Every method is at most one native call per native concept. There is no Rust-side registry,
//! dispatch, cache or formatting: each registration passes its own [`CallbackSlot`] to native as
//! `user_data`, with [`on_log`] as the entry point.
//!
//! The only Rust-side state is a per-thread callback depth ([`inside_callback`]), the minimum needed
//! for soundness on 4.5.0: native neither drains nor tolerates sink changes made from inside a
//! callback, so the SDK must know when a drop or a sink change happens there.
//!
//! Functionality that needs Senzing >= 4.5.1 / 4.6.0 is stubbed; every stub is marked
//! `SZ-FUTURE(<C function>)`.

use crate::error::{SzError, SzResult};
use crate::ffi::logging as native;
use crate::logging::{
    SzLogCallback, SzLogCallbackOptions, SzLogFormat, SzLogLevel, SzLogOutput, SzLogRecord,
    SzLogRegistration, level_from_native, requires_newer_senzing,
};
use crate::traits::SzLogging;
use std::cell::Cell;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

/// Zero-sized: native holds every piece of logging state.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SzLoggingCore;

/// What native holds as one registration's `user_data`: the closure and whether it may still be
/// called. `active` is cleared when the registration is dropped inside a callback, where the native
/// registration has to stay in place (see [`unregister`]).
pub(crate) struct CallbackSlot {
    active: AtomicBool,
    callback: SzLogCallback,
}

impl std::fmt::Debug for CallbackSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackSlot")
            .field("active", &self.active)
            .finish_non_exhaustive()
    }
}

impl CallbackSlot {
    fn new_raw(callback: SzLogCallback) -> NonNull<Self> {
        let slot = Box::new(Self {
            active: AtomicBool::new(true),
            callback,
        });
        NonNull::from(Box::leak(slot))
    }
}

thread_local! {
    /// How many [`on_log`] invocations are running on this thread. No destructor, so it is usable
    /// during thread teardown.
    static CALLBACK_DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// True while this thread runs a callback registered through this SDK.
///
/// Callbacks registered outside this SDK (C/C++, another binding) are not seen; see the module
/// docs' closure-lifetime rules.
pub(crate) fn inside_callback() -> bool {
    CALLBACK_DEPTH.with(Cell::get) > 0
}

struct DepthGuard;

impl DepthGuard {
    fn enter() -> Self {
        CALLBACK_DEPTH.with(|d| d.set(d.get() + 1));
        Self
    }
}

impl Drop for DepthGuard {
    fn drop(&mut self) {
        CALLBACK_DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
    }
}

/// Refuses a sink-changing call made from inside a callback (4.5.0 would rebuild the sink list
/// native is iterating on this thread).
fn refuse_inside_callback(operation: &str) -> SzResult<()> {
    if inside_callback() {
        // SZ-FUTURE(GDEV-4960 sink changes from inside a callback)
        return Err(requires_newer_senzing(&format!(
            "{operation} from inside a log callback"
        )));
    }
    Ok(())
}

/// Native entry point for every Rust registration (`SzLogCallback`). `user_data` is that
/// registration's own [`CallbackSlot`]; nothing here is shared between registrations.
unsafe extern "C" fn on_log(
    level: c_int,
    module: *const c_char,
    message: *const c_char,
    user_data: *mut c_void,
) {
    if module.is_null() || message.is_null() || user_data.is_null() {
        return;
    }
    // Out of range, or OFF (a filter value, never a message level): drop.
    let Some(level) = SzLogLevel::from_native(level).filter(|l| *l != SzLogLevel::Off) else {
        return;
    };
    // SAFETY: user_data is a CallbackSlot from `CallbackSlot::new_raw`. It is freed only after
    // `SzLogging_unregisterCallback` returned outside any callback (see `unregister`), and is
    // otherwise leaked, so it is live whenever native can still call this registration.
    let slot = unsafe { &*user_data.cast::<CallbackSlot>() };
    if !slot.active.load(Ordering::Acquire) {
        return;
    }
    // SAFETY: native passes NUL-terminated strings valid for the duration of this call.
    let (module, message) = unsafe { (CStr::from_ptr(module), CStr::from_ptr(message)) };
    // SZ-FUTURE(SzLogging_registerCallbackEx SzLogRecord.timestamp_ns / thread_id)
    let record = SzLogRecord::new(level, module, message, None, None);
    let _depth = DepthGuard::enter();
    // AssertUnwindSafe: the closure is user-owned and only called through `&dyn Fn`; state it left
    // half-updated by a panic is its own (mutexes poison), as with `std::thread::spawn`.
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| (slot.callback)(&record))) {
        discard_panic(payload);
    }
}

/// Drops a caught panic payload without letting a panicking `Drop` of the payload unwind into
/// native code. Only a payload whose own drop panics is leaked.
fn discard_panic(payload: Box<dyn std::any::Any + Send>) {
    if let Err(nested) = catch_unwind(AssertUnwindSafe(move || drop(payload))) {
        std::mem::forget(nested);
    }
}

/// Unregisters `handle` and frees `slot`, or, inside a callback, deactivates and leaks it.
///
/// # Safety
///
/// `handle` and `slot` must come from one accepted registration, and this must be called once.
pub(crate) unsafe fn unregister(handle: NonNull<c_void>, slot: NonNull<CallbackSlot>) {
    // SAFETY: the slot is live until freed below.
    unsafe { slot.as_ref() }
        .active
        .store(false, Ordering::Release);
    if inside_callback() {
        // 4.5.0: unregistering here rebuilds the sink list native is iterating on this thread, and
        // the closure may still be running here or on other threads. Leave the (now inactive)
        // native registration in place and leak the slot.
        // SZ-FUTURE(SzLogging_registerCallbackEx release): unregister, and native releases the slot
        // after its last invocation.
        return;
    }
    // SAFETY: an accepted handle, unregistered exactly once.
    unsafe { native::SzLogging_unregisterCallback(handle.as_ptr()) };
    // Nothing can call the closure any more: with the GDEV-4960 fix unregister waited for in-flight
    // invocations; on 4.5.0 the documented usage rule guarantees there are none.
    // SAFETY: native no longer references the slot; it came from Box::leak.
    let slot = unsafe { Box::from_raw(slot.as_ptr()) };
    if let Err(payload) = catch_unwind(AssertUnwindSafe(move || drop(slot))) {
        discard_panic(payload);
    }
}

impl SzLoggingCore {
    fn register(
        options: SzLogCallbackOptions,
        callback: SzLogCallback,
        thread_local: bool,
    ) -> SzResult<SzLogRegistration> {
        refuse_inside_callback("registering a log callback")?;
        if thread_local && options.format != SzLogFormat::Legacy {
            // SZ-FUTURE(SzLogging_registerCallbackEx SZ_LOG_SCOPE_THREAD with a format)
            return Err(requires_newer_senzing(
                "a thread-local callback with a format other than LEGACY (SzLogging_registerCallbackEx)",
            ));
        }
        let slot = CallbackSlot::new_raw(callback);
        let user_data = slot.as_ptr().cast::<c_void>();
        let level = options.min_level.to_native();
        // SAFETY: on_log matches SzLogCallback; the slot stays allocated while native may call it.
        let handle = unsafe {
            if thread_local {
                native::SzLogging_registerThreadLocalCallback(Some(on_log), user_data, level)
            } else {
                native::SzLogging_registerCallbackWithFormat(
                    Some(on_log),
                    user_data,
                    level,
                    options.format.to_native(),
                )
            }
        };
        match NonNull::new(handle) {
            Some(handle) => Ok(SzLogRegistration::new(handle, slot)),
            None => {
                // SAFETY: native refused, so it holds no reference to the slot.
                drop(unsafe { Box::from_raw(slot.as_ptr()) });
                // Level and format come from Rust enums and the callback is non-null, so native
                // refuses only when it is shutting down (process exit) or out of memory.
                Err(SzError::unrecoverable(
                    "SzLogging_registerCallback: registration refused (process exit in progress)",
                ))
            }
        }
    }
}

impl SzLogging for SzLoggingCore {
    fn register_callback(
        &self,
        options: SzLogCallbackOptions,
        callback: SzLogCallback,
    ) -> SzResult<SzLogRegistration> {
        Self::register(options, callback, false)
    }

    fn register_thread_local_callback(
        &self,
        options: SzLogCallbackOptions,
        callback: SzLogCallback,
    ) -> SzResult<SzLogRegistration> {
        Self::register(options, callback, true)
    }

    fn set_default_callback(
        &self,
        options: SzLogCallbackOptions,
        callback: SzLogCallback,
    ) -> SzResult<()> {
        refuse_inside_callback("set_default_callback")?;
        // Leaked: 4.5.0 has no release hook, and freeing a replaced default callback would require
        // the SDK to track which closure is the current default.
        // SZ-FUTURE(SzLogging_setDefaultCallbackEx release)
        let slot = CallbackSlot::new_raw(callback);
        // SAFETY: on_log matches SzLogCallback; the slot is never freed. The return is always null.
        unsafe {
            native::SzLogging_setDefaultCallback(
                Some(on_log),
                slot.as_ptr().cast::<c_void>(),
                options.min_level.to_native(),
            );
        }
        if options.format != SzLogFormat::Legacy {
            // setDefaultCallback installs LEGACY; apply the requested format to the new default.
            // SZ-FUTURE(SzLogging_setDefaultCallbackEx): one call sets level and format.
            // SAFETY: plain value call with an in-range format.
            unsafe { native::SzLogging_setDefaultFormatter(options.format.to_native()) };
        }
        Ok(())
    }

    fn set_default_output(&self, output: SzLogOutput) -> SzResult<()> {
        refuse_inside_callback("set_default_output")?;
        let (output_type, path) = match &output {
            SzLogOutput::Stderr => (native::SzLogOutputType_SZ_LOG_OUTPUT_STDERR, None),
            SzLogOutput::Stdout => (native::SzLogOutputType_SZ_LOG_OUTPUT_STDOUT, None),
            SzLogOutput::Null => (native::SzLogOutputType_SZ_LOG_OUTPUT_NULL, None),
            SzLogOutput::File { append: true, .. } => {
                // SZ-FUTURE(SZ_LOG_OUTPUT_FILE_APPEND)
                return Err(requires_newer_senzing(
                    "SzLogging_setDefaultOutput append mode (SZ_LOG_OUTPUT_FILE_APPEND)",
                ));
            }
            SzLogOutput::File {
                path,
                append: false,
            } => (
                native::SzLogOutputType_SZ_LOG_OUTPUT_FILE,
                Some(path_to_c_string(path)?),
            ),
            SzLogOutput::Callback => {
                return Err(SzError::bad_input(
                    "SzLogOutput::Callback is reported by get_default_output only; \
                     use set_default_callback to install a callback",
                ));
            }
        };
        let param = path.as_ref().map_or(ptr::null(), |p| p.as_ptr());
        // SAFETY: param is null or a NUL-terminated path that outlives the call.
        unsafe { native::SzLogging_setDefaultOutput(output_type.cast_signed(), param) };
        // SZ-FUTURE(int SzLogging_setDefaultOutput): 4.5.0 returns void, so a file that cannot be
        // opened is not reported here (native logs an ERROR and keeps the previous output).
        Ok(())
    }

    fn get_default_output(&self) -> SzResult<SzLogOutput> {
        // SZ-FUTURE(SzLogging_getDefaultOutput)
        Err(requires_newer_senzing("SzLogging_getDefaultOutput"))
    }

    fn set_default_format(&self, format: SzLogFormat) -> SzResult<()> {
        refuse_inside_callback("set_default_format")?;
        // SAFETY: plain value call with an in-range format.
        unsafe { native::SzLogging_setDefaultFormatter(format.to_native()) };
        // SZ-FUTURE(int SzLogging_setDefaultFormatter): status return.
        Ok(())
    }

    fn get_default_format(&self) -> SzResult<SzLogFormat> {
        // SZ-FUTURE(SzLogging_getDefaultFormatter)
        Err(requires_newer_senzing("SzLogging_getDefaultFormatter"))
    }

    fn set_default_log_level(&self, level: SzLogLevel) -> SzResult<()> {
        // SAFETY: plain value call with an in-range level. Safe at any time, also in a callback.
        unsafe { native::SzLogging_setLogLevel(level.to_native()) };
        // SZ-FUTURE(int SzLogging_setLogLevel): status return.
        Ok(())
    }

    fn get_default_log_level(&self) -> SzResult<SzLogLevel> {
        // SZ-FUTURE(SzLogging_getDefaultLogLevel)
        Err(requires_newer_senzing("SzLogging_getDefaultLogLevel"))
    }

    fn get_effective_log_level(&self) -> SzResult<SzLogLevel> {
        // SAFETY: plain value call.
        let value = unsafe { native::SzLogging_getLogLevel() };
        level_from_native(value, "SzLogging_getLogLevel")
    }
}

/// A path as the NUL-terminated byte string native opens. Rejects an empty path (4.5.0 ignores it
/// silently) and interior NUL bytes.
fn path_to_c_string(path: &Path) -> SzResult<CString> {
    if path.as_os_str().is_empty() {
        return Err(SzError::bad_input("log file path is empty"));
    }
    #[cfg(unix)]
    let bytes = {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    };
    #[cfg(not(unix))]
    let bytes = path
        .to_str()
        .ok_or_else(|| SzError::bad_input(format!("log file path is not UTF-8: {path:?}")))?
        .as_bytes()
        .to_vec();
    CString::new(bytes)
        .map_err(|_| SzError::bad_input(format!("log file path contains a NUL byte: {path:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;

    /// Calls the trampoline the way native does.
    fn deliver(slot: NonNull<CallbackSlot>, level: c_int, module: &CStr, message: &CStr) {
        // SAFETY: valid strings and a live slot.
        unsafe {
            on_log(
                level,
                module.as_ptr(),
                message.as_ptr(),
                slot.as_ptr().cast(),
            )
        };
    }

    fn free(slot: NonNull<CallbackSlot>) {
        // SAFETY: test-owned slot, freed once.
        drop(unsafe { Box::from_raw(slot.as_ptr()) });
    }

    #[test]
    fn test_trampoline_delivers_and_filters() {
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        let slot = CallbackSlot::new_raw(Box::new(move |r: &SzLogRecord<'_>| {
            assert_eq!(r.message(), "x");
            assert_eq!(r.module(), "m");
            assert_eq!(r.level(), SzLogLevel::Info);
            assert_eq!(r.timestamp(), None);
            assert_eq!(r.thread_id(), None);
            assert!(inside_callback());
            counter.fetch_add(1, Ordering::SeqCst);
        }));
        deliver(slot, 2, c"m:h", c"x");
        deliver(slot, 99, c"m:h", c"x"); // level out of range: dropped
        deliver(slot, 6, c"m:h", c"x"); // OFF is never a message level: dropped
        // SAFETY: null arguments are rejected before any dereference.
        unsafe {
            on_log(2, ptr::null(), c"x".as_ptr(), slot.as_ptr().cast());
            on_log(2, c"m".as_ptr(), c"x".as_ptr(), ptr::null_mut());
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(!inside_callback(), "depth restored");

        // SAFETY: test-owned slot.
        unsafe { slot.as_ref() }
            .active
            .store(false, Ordering::SeqCst);
        deliver(slot, 2, c"m:h", c"x");
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "inactive slot is not called"
        );
        free(slot);
        assert_eq!(
            Arc::strong_count(&calls),
            1,
            "freeing the slot drops the closure"
        );
    }

    #[test]
    fn test_panic_does_not_cross_the_trampoline() {
        let slot = CallbackSlot::new_raw(Box::new(|_r: &SzLogRecord<'_>| {
            panic!("callback panic (expected by this test)");
        }));
        // An escaping panic would abort the test binary (extern "C").
        deliver(slot, 4, c"m:h", c"x");
        assert!(!inside_callback(), "depth restored after a panic");
        free(slot);
    }

    #[test]
    fn test_sink_changes_refused_inside_callback() {
        let results = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = Arc::clone(&results);
        let slot = CallbackSlot::new_raw(Box::new(move |_r: &SzLogRecord<'_>| {
            sink.lock().unwrap().push(refuse_inside_callback("op"));
        }));
        deliver(slot, 4, c"m:h", c"x");
        free(slot);
        let results = results.lock().unwrap();
        let err = results[0].as_ref().unwrap_err();
        assert!(matches!(err, SzError::Configuration(_)));
        assert!(
            err.to_string().contains("op from inside a log callback"),
            "{err}"
        );
        assert!(
            refuse_inside_callback("op").is_ok(),
            "allowed outside callbacks"
        );
    }

    #[test]
    fn test_path_to_c_string() {
        let c = path_to_c_string(Path::new("/tmp/sz logs/sz.log")).unwrap();
        assert_eq!(c.as_bytes(), b"/tmp/sz logs/sz.log");
        assert!(path_to_c_string(Path::new("")).unwrap_err().is_bad_input());
        assert!(
            path_to_c_string(Path::new("a\0b"))
                .unwrap_err()
                .is_bad_input()
        );
    }
}
