//! Senzing native logging (libSzLogging): public types.
//!
//! The entry point is [`SzEnvironment::get_logging`](crate::traits::SzEnvironment::get_logging),
//! which returns a [`SzLogging`](crate::traits::SzLogging); the closure conveniences are on
//! [`SzLoggingExt`](crate::traits::SzLoggingExt).
//!
//! # All state is native
//!
//! Registrations, levels, formats and the default output live in native libSzLogging. The Rust side
//! keeps one opaque handle per registration (plus the boxed closure native calls) and nothing
//! else: every Rust registration is its own native registration, and dropping its
//! [`SzLogRegistration`] calls `SzLogging_unregisterCallback`. There is no Rust registry,
//! dispatcher, cache or formatter.
//!
//! # Senzing versions
//!
//! The crate binds the libSzLogging API first released in Senzing 4.5.0 (the minimum). The
//! final API also covers functionality that needs a newer native release (Senzing **>= 4.5.1 /
//! 4.6.0**): per-callback levels ([`SzLogRegistration::set_level`] / [`level`](SzLogRegistration::level)),
//! the default-state getters, append mode ([`SzLogOutput::append_file`]), thread-local
//! registrations ([`register_thread_local`](crate::traits::SzLoggingExt::register_thread_local),
//! [`capture_thread_logs`](crate::traits::SzLoggingExt::capture_thread_logs); 4.5.0 never delivers
//! to them, GDEV-4960), status-code error reporting, the record's
//! [`timestamp`](SzLogRecord::timestamp) and [`thread_id`](SzLogRecord::thread_id), changing
//! registrations from inside a callback, and native-owned closure lifetime. Until then those calls
//! return `SzError::Configuration` naming the native function and "Senzing >= 4.5.1/4.6.0" (or
//! `None` for the record fields). There is no runtime version detection.
//!
//! On 4.5.0, [`SzLogRecord::thread_hash`] is the per-thread handle (it is in every record's module).
//!
//! # The Senzing 4.5.0 usage rule
//!
//! **Change registrations, the default callback, the default output or the default format only
//! while no engine threads are running: before the first engine initialization or after
//! [`destroy`](crate::SzEnvironmentCore::destroy).**
//! [`set_default_log_level`](crate::traits::SzLogging::set_default_log_level) and
//! [`get_effective_log_level`](crate::traits::SzLogging::get_effective_log_level) are safe at any
//! time.
//!
//! - "Before the first engine initialization" means before the first `get_engine()` or
//!   `get_diagnostic()` (they run `Sz_init`).
//! - Joining your own worker threads is not enough: engine initialization starts Senzing's
//!   lock-watcher thread, which logs on its own until destroy.
//! - Why: in 4.5.0 these calls rebuild native's sink list without synchronizing with threads that
//!   are logging. The SDK does not enforce the rule. The full concurrency guarantees (any call, any
//!   thread, any time) need the GDEV-4960 native fix.
//!
//! # `verbose_logging` and engine initialization
//!
//! Each component initialization (`Sz_init` and the other `*_init` calls) re-applies the
//! environment's `verbose_logging` to the default output (level TRACE or ERROR, format
//! [`Legacy`](SzLogFormat::Legacy)) **unless the application configured logging first**.
//! [`set_default_log_level`](crate::traits::SzLogging::set_default_log_level),
//! [`set_default_format`](crate::traits::SzLogging::set_default_format) and
//! [`set_default_callback`](crate::traits::SzLogging::set_default_callback) count as
//! configuration (native keeps their settings from then on);
//! [`set_default_output`](crate::traits::SzLogging::set_default_output) and the `register*` calls do
//! not. Registered callbacks keep their own level and format either way.
//!
//! # Flushing the default file output
//!
//! A file default output is flushed when the output is replaced (any
//! [`set_default_output`](crate::traits::SzLogging::set_default_output) call) or at process exit.
//! On Senzing 4.5.0 `destroy()` does **not** flush it, so call `set_default_output(..)` after
//! `destroy()` before reading the file. Senzing >= 4.5.1 flushes at destroy.
//!
//! # Callback rules
//!
//! - Callbacks run synchronously on the thread that logs, concurrently across threads. Keep them
//!   fast; hand expensive work to another thread (for example through a channel).
//! - Do not call engine, config, config-manager, product or diagnostic methods from a callback:
//!   some messages are logged while the engine holds internal locks, so re-entering it can
//!   deadlock.
//! - Inside a callback, registering, setting a default callback and changing the default output or
//!   format return an error on 4.5.0 (native would rebuild the sink list it is iterating).
//!   [`set_default_log_level`](crate::traits::SzLogging::set_default_log_level) and
//!   [`get_effective_log_level`](crate::traits::SzLogging::get_effective_log_level) are allowed.
//! - A panic in a callback (or in a closure's `Drop`) is caught at the FFI boundary and discarded;
//!   it never unwinds into native code. The panic hook still runs, so the message is printed. With
//!   `panic = "abort"` nothing can be caught: a panicking callback aborts the process.
//!
//! # Closure lifetime
//!
//! - Dropping a registration **outside any callback** unregisters it and then drops the closure
//!   (and everything it captured, such as a channel sender) before `drop` returns. With the
//!   GDEV-4960 native fix, unregistering waits for in-flight invocations; on 4.5.0 the usage rule
//!   above guarantees there are none.
//! - Dropping a registration **inside a callback** (its own or another's) deactivates it at once,
//!   so its closure is not called again, but leaves the native registration in place and **leaks**
//!   the closure: on 4.5.0 unregistering there would rebuild the sink list native is iterating,
//!   and the closure may still be running on this or another thread. Once the native release hook
//!   ships, native frees it exactly after its last invocation instead.
//! - "Inside a callback" means inside a callback registered through **this SDK**. Do not drop a
//!   registration from inside a log callback registered some other way (C/C++, another language
//!   binding in the same process): the SDK cannot tell, frees the closure, and an invocation may
//!   still be running. Once the native release hook ships, native owns the free and this is safe.
//! - A **default callback** ([`set_default_callback`](crate::traits::SzLogging::set_default_callback))
//!   is leaked when it is replaced: 4.5.0 has no release hook, and freeing it would require the SDK
//!   to track which closure is the current default.

use crate::error::{SzError, SzResult};
use std::borrow::Cow;
use std::ffi::{CStr, c_void};
use std::fmt;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::str::FromStr;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, SystemTime};

/// Log severity. The discriminants are the native `SZ_LOG_LEVEL_*` values.
///
/// Ordered from most to least verbose, so `level >= SzLogLevel::Warn` reads naturally. This is
/// the native order and the **opposite** of `log::Level` / `tracing::Level`, where `Error` is the
/// smallest.
///
/// # Level names in native output
///
/// Native writes its own spellings, which differ from [`as_str`](Self::as_str). When parsing a log
/// file, compare against these (or parse case-insensitively with [`FromStr`], which accepts
/// `"warning"`), not against `as_str()`:
///
/// | Level | `as_str()` | JSONL `severity` | Legacy tag |
/// |---|---|---|---|
/// | `Trace` | `TRACE` | `trace` | `TRCE` |
/// | `Debug` | `DEBUG` | `debug` | `DBUG` |
/// | `Info` | `INFO` | `info` | `INFO` |
/// | `Warn` | `WARN` | `warning` | `WARN` |
/// | `Error` | `ERROR` | `error` | `ERR` |
/// | `Critical` | `CRITICAL` | `critical` | `CRIT` |
///
/// With the `log` feature, `log::Level::try_from(level)` converts a record level
/// ([`Critical`](Self::Critical) maps to `log::Level::Error`; [`Off`](Self::Off) is an error).
///
/// # Examples
///
/// ```
/// use sz_rust_sdk::prelude::*;
///
/// let level: SzLogLevel = "warning".parse()?;
/// assert_eq!(level, SzLogLevel::Warn);
/// assert_eq!(level.to_string(), "WARN");
/// assert!(SzLogLevel::Trace < SzLogLevel::Error);
/// # Ok::<(), SzError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum SzLogLevel {
    /// Most verbose: execution flow (`SZ_LOG_LEVEL_TRACE`).
    Trace = 0,
    /// Debug information (`SZ_LOG_LEVEL_DEBUG`).
    Debug = 1,
    /// Informational messages (`SZ_LOG_LEVEL_INFO`).
    Info = 2,
    /// Warnings (`SZ_LOG_LEVEL_WARN`).
    Warn = 3,
    /// Errors (`SZ_LOG_LEVEL_ERROR`).
    Error = 4,
    /// Critical errors (`SZ_LOG_LEVEL_CRITICAL`).
    Critical = 5,
    /// Nothing is delivered (`SZ_LOG_LEVEL_OFF`). A filter value only; never a record's level
    /// (a native record claiming `OFF` is dropped).
    Off = 6,
}

impl SzLogLevel {
    /// All levels, most verbose first.
    pub const ALL: &'static [Self] = &[
        Self::Trace,
        Self::Debug,
        Self::Info,
        Self::Warn,
        Self::Error,
        Self::Critical,
        Self::Off,
    ];

    /// The canonical upper-case name: `"TRACE"`, `"DEBUG"`, `"INFO"`, `"WARN"`, `"ERROR"`,
    /// `"CRITICAL"` or `"OFF"`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
            Self::Off => "OFF",
        }
    }

    /// The level for a native `SZ_LOG_LEVEL_*` value, or `None` when it is out of range.
    pub(crate) fn from_native(value: i32) -> Option<Self> {
        usize::try_from(value)
            .ok()
            .and_then(|i| Self::ALL.get(i).copied())
    }

    /// The native `SZ_LOG_LEVEL_*` value.
    pub(crate) const fn to_native(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for SzLogLevel {
    /// Writes [`as_str`](Self::as_str); width and alignment flags are honored (`{:<8}`).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

impl FromStr for SzLogLevel {
    type Err = SzError;

    /// Case-insensitive; surrounding whitespace is ignored and `"warning"` is accepted for `Warn`.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - the text names no level
    fn from_str(s: &str) -> SzResult<Self> {
        match s.trim().to_ascii_uppercase().as_str() {
            "TRACE" => Ok(Self::Trace),
            "DEBUG" => Ok(Self::Debug),
            "INFO" => Ok(Self::Info),
            "WARN" | "WARNING" => Ok(Self::Warn),
            "ERROR" => Ok(Self::Error),
            "CRITICAL" => Ok(Self::Critical),
            "OFF" => Ok(Self::Off),
            _ => Err(SzError::bad_input(format!("unknown log level {s:?}"))),
        }
    }
}

/// `Critical` and `Error` map to `log::Level::Error`; the rest map by name. `Off` is a filter
/// value, never a record level, so it is an error (`SzError::BadInput`).
///
/// ```
/// use sz_rust_sdk::prelude::*;
///
/// assert_eq!(log::Level::try_from(SzLogLevel::Critical)?, log::Level::Error);
/// assert_eq!(log::Level::try_from(SzLogLevel::Warn)?, log::Level::Warn);
/// assert!(log::Level::try_from(SzLogLevel::Off).is_err());
/// # Ok::<(), SzError>(())
/// ```
#[cfg(feature = "log")]
impl TryFrom<SzLogLevel> for log::Level {
    type Error = SzError;

    fn try_from(level: SzLogLevel) -> SzResult<Self> {
        match level {
            SzLogLevel::Trace => Ok(Self::Trace),
            SzLogLevel::Debug => Ok(Self::Debug),
            SzLogLevel::Info => Ok(Self::Info),
            SzLogLevel::Warn => Ok(Self::Warn),
            SzLogLevel::Error | SzLogLevel::Critical => Ok(Self::Error),
            SzLogLevel::Off => Err(SzError::bad_input(
                "SzLogLevel::Off is a filter value and has no log::Level",
            )),
        }
    }
}

/// Message format, applied by native (`SZ_LOG_FORMAT_*`).
///
/// There is deliberately no `Default` impl: a variant is named `Default`, so
/// `SzLogFormat::default()` would read ambiguously.
///
/// # Examples
///
/// ```
/// use sz_rust_sdk::prelude::*;
///
/// assert_eq!("jsonl".parse::<SzLogFormat>()?, SzLogFormat::Jsonl);
/// assert_eq!(SzLogFormat::Bare.to_string(), "BARE");
/// # Ok::<(), SzError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SzLogFormat {
    /// `YYYY-MM-DD HH:MM:SS.mmm [module:hash] SEV: msg`, in **local** time.
    Legacy,
    /// One JSON object per line: `timestamp`, `module`, `severity`, `message`. The timestamp is
    /// UTC ISO-8601 ending in `Z` from Senzing 4.5.1 / 4.6.0 (GDEV-4960); 4.5.0 writes local time
    /// without a zone.
    Jsonl,
    /// The message text only.
    Bare,
    /// For the default output: spdlog's default pattern
    /// `[YYYY-MM-DD HH:MM:SS.mmm] [module] [level] msg`, in **local** time. For a registered
    /// callback native delivers the message only, as [`Bare`](Self::Bare).
    Default,
}

impl SzLogFormat {
    /// All formats.
    pub const ALL: &'static [Self] = &[Self::Legacy, Self::Jsonl, Self::Bare, Self::Default];

    /// The canonical upper-case name: `"LEGACY"`, `"JSONL"`, `"BARE"` or `"DEFAULT"`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Legacy => "LEGACY",
            Self::Jsonl => "JSONL",
            Self::Bare => "BARE",
            Self::Default => "DEFAULT",
        }
    }

    /// The format for a native `SZ_LOG_FORMAT_*` value, or `None` when it is out of range.
    // SZ-FUTURE(SzLogging_getDefaultFormatter): decodes its return value.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn from_native(value: i32) -> Option<Self> {
        usize::try_from(value)
            .ok()
            .and_then(|i| Self::ALL.get(i).copied())
    }

    /// The native `SZ_LOG_FORMAT_*` value.
    pub(crate) const fn to_native(self) -> i32 {
        match self {
            Self::Legacy => 0,
            Self::Jsonl => 1,
            Self::Bare => 2,
            Self::Default => 3,
        }
    }
}

impl fmt::Display for SzLogFormat {
    /// Writes [`as_str`](Self::as_str); width and alignment flags are honored.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

impl FromStr for SzLogFormat {
    type Err = SzError;

    /// Case-insensitive; surrounding whitespace is ignored and `"json"` is accepted for `Jsonl`.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - the text names no format
    fn from_str(s: &str) -> SzResult<Self> {
        match s.trim().to_ascii_uppercase().as_str() {
            "LEGACY" => Ok(Self::Legacy),
            "JSONL" | "JSON" => Ok(Self::Jsonl),
            "BARE" => Ok(Self::Bare),
            "DEFAULT" => Ok(Self::Default),
            _ => Err(SzError::bad_input(format!("unknown log format {s:?}"))),
        }
    }
}

/// Destination of native's default output (`SZ_LOG_OUTPUT_*`).
///
/// # Examples
///
/// ```
/// use sz_rust_sdk::prelude::*;
///
/// let truncate = SzLogOutput::file("/var/log/senzing/sz.log");
/// let append = SzLogOutput::append_file("/var/log/senzing/sz.log");
/// assert_ne!(truncate, append);
/// assert_eq!(SzLogOutput::default(), SzLogOutput::Stderr);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SzLogOutput {
    /// Standard error (the native default).
    #[default]
    Stderr,
    /// Standard output.
    Stdout,
    /// A file opened by native: `append == false` truncates it (`SZ_LOG_OUTPUT_FILE`),
    /// `append == true` keeps its content (`SZ_LOG_OUTPUT_FILE_APPEND`, Senzing >= 4.5.1 / 4.6.0).
    File {
        /// The file path.
        path: PathBuf,
        /// Keep the existing content instead of truncating.
        append: bool,
    },
    /// Discard; only registered callbacks receive messages.
    Null,
    /// A default callback is set (`SZ_LOG_OUTPUT_CALLBACK`). Reported by
    /// [`get_default_output`](crate::traits::SzLogging::get_default_output) only; install one with
    /// [`set_default_callback`](crate::traits::SzLogging::set_default_callback).
    Callback,
}

impl SzLogOutput {
    /// A file output that truncates the file when it is opened.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File {
            path: path.into(),
            append: false,
        }
    }

    /// A file output that appends to the file, keeping what is already there.
    ///
    /// Setting it requires Senzing >= 4.5.1 / 4.6.0 (`SZ_LOG_OUTPUT_FILE_APPEND`).
    pub fn append_file(path: impl Into<PathBuf>) -> Self {
        Self::File {
            path: path.into(),
            append: true,
        }
    }
}

/// Registration parameters: the callback's minimum level and the format native applies to
/// [`SzLogRecord::message`].
///
/// [`SzLogLevel`] converts into it, so `logging.register(SzLogLevel::Info, ..)` works. The default
/// format is [`SzLogFormat::Bare`]: the record already carries the level and module, so the
/// message needs no prefix (Legacy would stamp them a second time when the record is forwarded).
/// Use [`format`](Self::format) to pick another.
///
/// # Examples
///
/// ```
/// use sz_rust_sdk::prelude::*;
///
/// let options = SzLogCallbackOptions::new(SzLogLevel::Debug).format(SzLogFormat::Jsonl);
/// assert_eq!(options.min_level, SzLogLevel::Debug);
/// assert_eq!(SzLogCallbackOptions::from(SzLogLevel::Info).format, SzLogFormat::Bare);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SzLogCallbackOptions {
    /// Minimum level delivered to the callback.
    pub min_level: SzLogLevel,
    /// Format native applies to the message.
    pub format: SzLogFormat,
}

impl SzLogCallbackOptions {
    /// Options with `min_level` and the [`Bare`](SzLogFormat::Bare) format (the message text only;
    /// level and module are on the record).
    pub const fn new(min_level: SzLogLevel) -> Self {
        Self {
            min_level,
            format: SzLogFormat::Bare,
        }
    }

    /// Returns these options with `format` instead.
    #[must_use]
    pub const fn format(mut self, format: SzLogFormat) -> Self {
        self.format = format;
        self
    }
}

impl From<SzLogLevel> for SzLogCallbackOptions {
    fn from(min_level: SzLogLevel) -> Self {
        Self::new(min_level)
    }
}

/// One log message, borrowed from native for the duration of a callback.
///
/// A record cannot outlive its callback (native's strings are valid only for the call); use
/// [`to_entry`](Self::to_entry) to keep an owned copy. [`Display`](fmt::Display) writes
/// [`message`](Self::message) verbatim, already formatted by native per the registration's
/// [`SzLogFormat`].
///
/// ```compile_fail,E0521
/// use sz_rust_sdk::prelude::*;
/// use std::sync::{Arc, Mutex};
/// fn escape(logging: &dyn SzLogging) {
///     let keep: Arc<Mutex<Vec<SzLogRecord<'static>>>> = Default::default();
///     let k = Arc::clone(&keep);
///     let _r = logging.register(SzLogLevel::Info, move |r| k.lock().unwrap().push(*r));
/// }
/// ```
#[derive(Clone, Copy)]
pub struct SzLogRecord<'a> {
    level: SzLogLevel,
    module: &'a CStr,
    message: &'a CStr,
    timestamp_ns: Option<i64>,
    thread_id: Option<u64>,
}

impl<'a> SzLogRecord<'a> {
    /// Builds a record over native's fields.
    pub(crate) fn new(
        level: SzLogLevel,
        module: &'a CStr,
        message: &'a CStr,
        timestamp_ns: Option<i64>,
        thread_id: Option<u64>,
    ) -> Self {
        Self {
            level,
            module,
            message,
            timestamp_ns,
            thread_id,
        }
    }

    /// The message's severity.
    pub fn level(&self) -> SzLogLevel {
        self.level
    }

    /// The module name: native's `module` up to its last `:` (for `"szstatic:82eb15f5faba9bab"`,
    /// `"szstatic"`). Invalid UTF-8 is replaced lossily.
    pub fn module(&self) -> Cow<'a, str> {
        let raw = self.module.to_bytes();
        let end = raw.iter().rposition(|b| *b == b':').unwrap_or(raw.len());
        String::from_utf8_lossy(&raw[..end])
    }

    /// The thread hash after the last `:` of native's `module` (`"82eb15f5faba9bab"`), if present.
    ///
    /// The per-thread handle on Senzing 4.5.0: records from one thread share it.
    /// [`thread_id`](Self::thread_id) needs Senzing >= 4.5.1 / 4.6.0.
    pub fn thread_hash(&self) -> Option<&'a str> {
        let raw = self.module.to_bytes();
        let start = raw.iter().rposition(|b| *b == b':')? + 1;
        std::str::from_utf8(&raw[start..]).ok()
    }

    /// The message exactly as native formatted it for this registration. Invalid UTF-8 is replaced
    /// lossily.
    pub fn message(&self) -> Cow<'a, str> {
        self.message.to_string_lossy()
    }

    /// Native's `module` bytes, unparsed (`b"szstatic:82eb15f5faba9bab"`).
    pub fn raw_module(&self) -> &'a [u8] {
        self.module.to_bytes()
    }

    /// Native's `message` bytes.
    pub fn raw_message(&self) -> &'a [u8] {
        self.message.to_bytes()
    }

    /// When native logged the message (UTC), as a [`SystemTime`].
    ///
    /// Precision is the platform's `SystemTime` resolution (100 ns on Windows), so sub-100 ns
    /// digits are truncated there; [`timestamp_ns`](Self::timestamp_ns) is exact.
    ///
    /// `None` until Senzing >= 4.5.1 / 4.6.0: the 4.5.0 callback does not receive the time.
    pub fn timestamp(&self) -> Option<SystemTime> {
        self.timestamp_ns.map(system_time_from_unix_nanos)
    }

    /// When native logged the message: UTC nanoseconds since the Unix epoch.
    ///
    /// `None` until Senzing >= 4.5.1 / 4.6.0.
    pub fn timestamp_ns(&self) -> Option<i64> {
        self.timestamp_ns
    }

    /// OS id of the thread that logged the message (the id native's thread-local filtering uses).
    ///
    /// `None` until Senzing >= 4.5.1 / 4.6.0; use [`thread_hash`](Self::thread_hash) on 4.5.0.
    pub fn thread_id(&self) -> Option<u64> {
        self.thread_id
    }

    /// An owned copy, for channels and buffers.
    pub fn to_entry(&self) -> SzLogEntry {
        SzLogEntry {
            level: self.level,
            module: self.module().into_owned(),
            thread_hash: self.thread_hash().map(str::to_owned),
            message: self.message().into_owned(),
            timestamp: self.timestamp(),
            thread_id: self.thread_id,
        }
    }
}

impl fmt::Display for SzLogRecord<'_> {
    /// Writes [`message`](Self::message) verbatim; no formatting is added.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

impl fmt::Debug for SzLogRecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SzLogRecord")
            .field("level", &self.level)
            .field("module", &self.module)
            .field("message", &self.message)
            .field("timestamp_ns", &self.timestamp_ns)
            .field("thread_id", &self.thread_id)
            .finish()
    }
}

/// UTC nanoseconds since the Unix epoch as a [`SystemTime`] (saturating at the epoch if the value
/// cannot be represented, which does not happen for a 64-bit nanosecond count on supported
/// platforms).
fn system_time_from_unix_nanos(nanos: i64) -> SystemTime {
    let magnitude = Duration::from_nanos(nanos.unsigned_abs());
    let time = if nanos >= 0 {
        SystemTime::UNIX_EPOCH.checked_add(magnitude)
    } else {
        SystemTime::UNIX_EPOCH.checked_sub(magnitude)
    };
    time.unwrap_or(SystemTime::UNIX_EPOCH)
}

/// An owned copy of a [`SzLogRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SzLogEntry {
    /// The message's severity.
    pub level: SzLogLevel,
    /// The module name ([`SzLogRecord::module`]).
    pub module: String,
    /// The thread hash ([`SzLogRecord::thread_hash`]).
    pub thread_hash: Option<String>,
    /// The message as native formatted it.
    pub message: String,
    /// When native logged the message (UTC); `None` until Senzing >= 4.5.1 / 4.6.0.
    pub timestamp: Option<SystemTime>,
    /// OS id of the logging thread; `None` until Senzing >= 4.5.1 / 4.6.0.
    pub thread_id: Option<u64>,
}

impl fmt::Display for SzLogEntry {
    /// Writes the message verbatim, like [`SzLogRecord`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// A boxed log callback, as accepted by the object-safe [`SzLogging`](crate::traits::SzLogging)
/// methods. [`SzLoggingExt`](crate::traits::SzLoggingExt) takes plain closures instead.
pub type SzLogCallback = Box<dyn Fn(&SzLogRecord<'_>) + Send + Sync + 'static>;

/// A live native callback registration. Dropping it unregisters the callback.
///
/// - **Outside any callback**, drop calls `SzLogging_unregisterCallback` and then drops the
///   closure (and everything it captured, such as a channel sender) before it returns.
/// - **Inside a callback**, drop deactivates the closure (it is not called again) and leaks it,
///   leaving the native registration in place; see [the module docs](crate::logging#closure-lifetime).
///
/// On Senzing 4.5.0, drop it only while nothing else logs (the
/// [usage rule](crate::logging#the-senzing-450-usage-rule)).
///
/// Bind it to a name (`let registration = ...`): `let _ = ...` drops it, and so unregisters the
/// callback, immediately.
///
/// The handle is `Send + Sync`: share it through an `Arc` or drop it on another thread.
///
/// A thread-local registration (Senzing >= 4.5.1 / 4.6.0) follows the **OS thread** that
/// registered it: it stays registered after that thread exits, and OS thread ids can be reused (a
/// later thread with the same id would receive its messages). Drop it before the registering
/// thread exits.
#[must_use = "the callback is unregistered as soon as the registration is dropped"]
#[derive(Debug)]
pub struct SzLogRegistration {
    handle: NonNull<c_void>,
    slot: NonNull<crate::core::logging::CallbackSlot>,
}

// SAFETY: the handle is an opaque id (native: a counter that is never reused), and the slot holds a
// `Send + Sync` closure plus an atomic flag. Every SzLogging_* function may be called from any
// thread; the 4.5.0 timing rule is documented on the type.
unsafe impl Send for SzLogRegistration {}
// SAFETY: as for `Send`; the methods taking `&self` touch no shared Rust state.
unsafe impl Sync for SzLogRegistration {}

impl SzLogRegistration {
    /// Wraps a native handle and the slot passed to native as `user_data`.
    pub(crate) fn new(
        handle: NonNull<c_void>,
        slot: NonNull<crate::core::logging::CallbackSlot>,
    ) -> Self {
        Self { handle, slot }
    }

    /// Changes this callback's minimum level in place (`SzLogging_setCallbackLevel`).
    ///
    /// Use it to turn verbose logging on and off at runtime, for example from a signal-handling
    /// thread. Until it is available, [`SzLogging::set_default_log_level`](crate::traits::SzLogging::set_default_log_level)
    /// changes the default output's level at any time.
    ///
    /// # Errors
    ///
    /// * `SzError::Configuration` - requires Senzing >= 4.5.1 / 4.6.0
    pub fn set_level(&self, min_level: SzLogLevel) -> SzResult<()> {
        let _ = min_level;
        // SZ-FUTURE(SzLogging_setCallbackLevel)
        Err(requires_newer_senzing("SzLogging_setCallbackLevel"))
    }

    /// This callback's current minimum level (`SzLogging_getCallbackLevel`).
    ///
    /// # Errors
    ///
    /// * `SzError::Configuration` - requires Senzing >= 4.5.1 / 4.6.0
    pub fn level(&self) -> SzResult<SzLogLevel> {
        // SZ-FUTURE(SzLogging_getCallbackLevel)
        Err(requires_newer_senzing("SzLogging_getCallbackLevel"))
    }

    /// Unregisters now. Same as dropping the registration.
    pub fn unregister(self) {}

    /// Keeps the callback registered for the rest of the process, with no handle to remove it.
    pub fn leak(self) {
        std::mem::forget(self);
    }
}

impl Drop for SzLogRegistration {
    fn drop(&mut self) {
        // SAFETY: handle and slot come from one accepted registration, and Drop runs once.
        unsafe { crate::core::logging::unregister(self.handle, self.slot) }
    }
}

/// Collects the records logged on the thread that created it, until [`finish`](Self::finish) (or
/// drop). Created by [`SzLoggingExt::capture_thread_logs`](crate::traits::SzLoggingExt::capture_thread_logs),
/// which needs Senzing >= 4.5.1 / 4.6.0 (thread-local delivery, GDEV-4960).
///
/// It is one thread-local registration whose closure owns the buffer. Work the engine does on its
/// own threads is not captured.
#[must_use = "dropping the capture ends it"]
#[derive(Debug)]
pub struct SzLogCapture {
    registration: SzLogRegistration,
    entries: Arc<Mutex<Vec<SzLogEntry>>>,
}

impl SzLogCapture {
    /// Wraps a thread-local registration whose closure pushes into `entries`.
    pub(crate) fn new(
        registration: SzLogRegistration,
        entries: Arc<Mutex<Vec<SzLogEntry>>>,
    ) -> Self {
        Self {
            registration,
            entries,
        }
    }

    /// Unregisters, then returns everything captured.
    pub fn finish(self) -> Vec<SzLogEntry> {
        let Self {
            registration,
            entries,
        } = self;
        drop(registration);
        std::mem::take(&mut *entries.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

/// The error for functionality the linked Senzing (4.5.0 API) lacks.
///
/// `SzError::Configuration`: the fix is deploying a newer Senzing (an installation/configuration
/// matter); it is not bad input, not transient, and needs no reinitialization.
pub(crate) fn requires_newer_senzing(what: &str) -> SzError {
    SzError::configuration(format!(
        "{what} requires Senzing >= 4.5.1/4.6.0 (not in the Senzing 4.5.0 logging API this SDK binds)"
    ))
}

/// Maps a libSzLogging status return to `SzResult` (for the `int` returns of Senzing >= 4.5.1 /
/// 4.6.0; the 4.5.0 setters return `void`).
///
/// | Return | Meaning (libSzLogging.h) | `SzError` |
/// |---|---|---|
/// | `0` | ok | - |
/// | `-1` | invalid argument | `BadInput` |
/// | `-2` | the output could not be opened; the previous output is kept | `BadInput` |
/// | `-3` | refused: process exit in progress | `Unrecoverable` |
/// | other | not documented | `Unknown` |
// SZ-FUTURE(int SzLogging_setLogLevel / setDefaultFormatter / setDefaultOutput status returns)
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn check_status(rc: i32, operation: &str) -> SzResult<()> {
    match rc {
        0 => Ok(()),
        negative => Err(status_error(negative, operation)),
    }
}

/// The error for a negative libSzLogging status or getter return (see [`check_status`]).
pub(crate) fn status_error(rc: i32, operation: &str) -> SzError {
    match rc {
        -1 => SzError::bad_input(format!("{operation}: invalid argument")),
        -2 => SzError::bad_input(format!(
            "{operation}: the output could not be opened; the previous output is kept"
        )),
        -3 => SzError::unrecoverable(format!("{operation}: refused, process exit in progress")),
        other => SzError::unknown(format!("{operation}: unexpected return {other}")),
    }
}

/// A level getter's return value as `SzLogLevel`, or the status error for a negative value.
pub(crate) fn level_from_native(value: i32, operation: &str) -> SzResult<SzLogLevel> {
    if value < 0 {
        return Err(status_error(value, operation));
    }
    SzLogLevel::from_native(value)
        .ok_or_else(|| SzError::unknown(format!("{operation}: unexpected level {value}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record<'a>(
        module: &'a CStr,
        message: &'a CStr,
        timestamp_ns: Option<i64>,
    ) -> SzLogRecord<'a> {
        SzLogRecord::new(SzLogLevel::Info, module, message, timestamp_ns, Some(42))
    }

    #[test]
    fn test_level_round_trips() {
        for (i, level) in SzLogLevel::ALL.iter().enumerate() {
            let native = i32::try_from(i).unwrap();
            assert_eq!(level.to_native(), native);
            assert_eq!(SzLogLevel::from_native(native), Some(*level));
            assert_eq!(level.as_str().parse::<SzLogLevel>().unwrap(), *level);
            assert_eq!(
                level.as_str().to_lowercase().parse::<SzLogLevel>().unwrap(),
                *level
            );
        }
        assert_eq!(SzLogLevel::from_native(-1), None);
        assert_eq!(SzLogLevel::from_native(7), None);
    }

    #[test]
    fn test_level_parse_and_display() {
        assert_eq!(" Warning ".parse::<SzLogLevel>().unwrap(), SzLogLevel::Warn);
        assert!("verbose".parse::<SzLogLevel>().unwrap_err().is_bad_input());
        assert_eq!(format!("{:<6}|", SzLogLevel::Info), "INFO  |");
        assert!(SzLogLevel::Trace < SzLogLevel::Debug);
        assert!(SzLogLevel::Critical < SzLogLevel::Off);
    }

    #[test]
    fn test_format_round_trips() {
        for (i, format) in SzLogFormat::ALL.iter().enumerate() {
            let native = i32::try_from(i).unwrap();
            assert_eq!(format.to_native(), native);
            assert_eq!(SzLogFormat::from_native(native), Some(*format));
            assert_eq!(format.to_string().parse::<SzLogFormat>().unwrap(), *format);
        }
        assert_eq!("json".parse::<SzLogFormat>().unwrap(), SzLogFormat::Jsonl);
        assert!("xml".parse::<SzLogFormat>().unwrap_err().is_bad_input());
        assert_eq!(SzLogFormat::from_native(4), None);
        assert_eq!(format!("{:>7}", SzLogFormat::Bare), "   BARE");
    }

    #[test]
    fn test_output_constructors() {
        assert_eq!(
            SzLogOutput::file("a.log"),
            SzLogOutput::File {
                path: "a.log".into(),
                append: false
            }
        );
        assert_eq!(
            SzLogOutput::append_file("a.log"),
            SzLogOutput::File {
                path: "a.log".into(),
                append: true
            }
        );
        assert_eq!(SzLogOutput::default(), SzLogOutput::Stderr);
    }

    #[test]
    fn test_options() {
        let options = SzLogCallbackOptions::from(SzLogLevel::Debug);
        assert_eq!(options.format, SzLogFormat::Bare);
        assert_eq!(
            options.format(SzLogFormat::Jsonl).format,
            SzLogFormat::Jsonl
        );
    }

    #[cfg(feature = "log")]
    #[test]
    fn test_log_level_conversion() {
        let expected = [
            (SzLogLevel::Trace, log::Level::Trace),
            (SzLogLevel::Debug, log::Level::Debug),
            (SzLogLevel::Info, log::Level::Info),
            (SzLogLevel::Warn, log::Level::Warn),
            (SzLogLevel::Error, log::Level::Error),
            (SzLogLevel::Critical, log::Level::Error),
        ];
        for (level, log_level) in expected {
            assert_eq!(log::Level::try_from(level).unwrap(), log_level);
        }
        assert!(
            log::Level::try_from(SzLogLevel::Off)
                .unwrap_err()
                .is_bad_input()
        );
    }

    #[test]
    fn test_record_parsing() {
        let rec = record(c"szstatic:82eb15f5faba9bab", c"hello", None);
        assert_eq!(rec.module(), "szstatic");
        assert_eq!(rec.thread_hash(), Some("82eb15f5faba9bab"));
        assert_eq!(rec.message(), "hello");
        assert_eq!(rec.to_string(), "hello");
        assert_eq!(rec.raw_module(), b"szstatic:82eb15f5faba9bab");
        assert_eq!(rec.raw_message(), b"hello");
        assert_eq!(rec.thread_id(), Some(42));
        assert_eq!(rec.level(), SzLogLevel::Info);
        assert!(format!("{rec:?}").contains("szstatic"));

        let rec = record(c"a:b:c", c"m", None);
        assert_eq!(rec.module(), "a:b");
        assert_eq!(rec.thread_hash(), Some("c"));

        let rec = record(c"plain", c"m", None);
        assert_eq!(rec.module(), "plain");
        assert_eq!(rec.thread_hash(), None);
    }

    #[test]
    fn test_record_lossy_utf8() {
        let rec = record(c"m\xff:h", c"bad \xff byte", None);
        assert_eq!(rec.module(), "m\u{fffd}");
        assert_eq!(rec.message(), "bad \u{fffd} byte");
        assert_eq!(rec.raw_message(), b"bad \xff byte");
    }

    #[test]
    fn test_record_timestamp() {
        assert_eq!(record(c"m:h", c"x", None).timestamp(), None);

        // A multiple of 100 ns: Windows `SystemTime` truncates finer digits.
        let ns = 1_763_568_000_123_456_700_i64;
        let rec = record(c"m:h", c"x", Some(ns));
        assert_eq!(rec.timestamp_ns(), Some(ns));
        let since = rec
            .timestamp()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        assert_eq!(since.as_nanos(), u128::try_from(ns).unwrap());

        let before = record(c"m:h", c"x", Some(-1_000_000_000))
            .timestamp()
            .unwrap();
        assert_eq!(
            SystemTime::UNIX_EPOCH.duration_since(before).unwrap(),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn test_entry_copy() {
        let rec = record(c"szdb:abc", c"msg", Some(5));
        let entry = rec.to_entry();
        assert_eq!(entry.module, "szdb");
        assert_eq!(entry.thread_hash.as_deref(), Some("abc"));
        assert_eq!(entry.message, "msg");
        assert_eq!(entry.to_string(), "msg");
        assert_eq!(entry.thread_id, Some(42));
        assert_eq!(entry.timestamp, rec.timestamp());
    }

    #[test]
    fn test_requires_newer_senzing() {
        let err = requires_newer_senzing("SzLogging_getDefaultOutput");
        assert!(matches!(err, SzError::Configuration(_)));
        let text = err.to_string();
        assert!(text.contains("SzLogging_getDefaultOutput"), "{text}");
        assert!(text.contains("Senzing >= 4.5.1/4.6.0"), "{text}");
    }

    #[test]
    fn test_status_mapping() {
        assert!(check_status(0, "op").is_ok());
        assert!(check_status(-1, "op").unwrap_err().is_bad_input());
        let opened = check_status(-2, "op").unwrap_err();
        assert!(opened.is_bad_input());
        assert!(opened.to_string().contains("could not be opened"));
        assert!(check_status(-3, "op").unwrap_err().is_unrecoverable());
        assert!(matches!(
            check_status(-9, "op").unwrap_err(),
            SzError::Unknown(_)
        ));
        assert_eq!(level_from_native(3, "op").unwrap(), SzLogLevel::Warn);
        assert!(level_from_native(-1, "op").unwrap_err().is_bad_input());
        assert!(level_from_native(-3, "op").unwrap_err().is_unrecoverable());
        assert!(matches!(
            level_from_native(9, "op").unwrap_err(),
            SzError::Unknown(_)
        ));
    }

    #[test]
    fn test_auto_traits() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SzLogRegistration>();
        assert_send_sync::<SzLogCapture>();
        assert_send_sync::<SzLogEntry>();
        assert_send_sync::<SzLogCallback>();
    }
}
