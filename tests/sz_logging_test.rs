//! SzLogging integration tests against the real native libSzLogging (no mocks).
//!
//! Written for the Senzing 4.5.0 API and its usage rule: registrations and default-output changes
//! happen only while nothing else logs (no engine is initialized, or before init / after destroy).
//! Logging is process-global native state, so every test is `#[serial]` and resets the default
//! output, format and level on exit ([`Reset`]).
//!
//! Deterministic native message without an engine: native `SzLogging_setDefaultOutput` with a file
//! path that cannot be opened logs `Failed to create file log sink for '<path>': ...` at ERROR, on
//! the calling thread, and keeps the previous output. The SDK's `set_default_output` now rejects
//! such a path before the native call, so [`emit_error`] calls the native function directly. The
//! path embeds a marker so each test can tell its messages apart.
//!
//! Tests marked `#[ignore = "... GDEV-4960 ..."]` need Senzing >= 4.5.1 / 4.6.0; run them with
//! `cargo test --test sz_logging_test -- --ignored` once that native ships (and the SZ-FUTURE stubs
//! are wired up).

use serial_test::serial;
use std::ffi::CString;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

const WAIT: Duration = Duration::from_secs(5);
const NEWER: &str = "requires Senzing >= 4.5.1/4.6.0";

fn logging() -> Box<dyn SzLogging> {
    // get_logging performs no native init, so a plain (non-singleton) environment is enough.
    SzEnvironmentCore::new("sz-logging-test", "{}", false)
        .and_then(|env| env.get_logging())
        .expect("get_logging")
}

/// Silences the default output for the test and resets output, format and level on drop.
struct Reset;

impl Reset {
    fn quiet() -> Self {
        logging()
            .set_default_output(SzLogOutput::Null)
            .expect("set Null");
        Self
    }
}

impl Drop for Reset {
    fn drop(&mut self) {
        let logging = logging();
        let _ = logging.set_default_output(SzLogOutput::Stderr);
        let _ = logging.set_default_format(SzLogFormat::Legacy);
        let _ = logging.set_default_log_level(SzLogLevel::Error);
    }
}

/// Asserts the "requires a newer Senzing" stub error, naming `function`.
fn assert_requires_newer<T: std::fmt::Debug>(result: SzResult<T>, function: &str) {
    let err = result.expect_err("stubbed until Senzing >= 4.5.1/4.6.0");
    assert!(matches!(err, SzError::Configuration(_)), "{err:?}");
    let text = err.to_string();
    assert!(text.contains(NEWER), "{text}");
    assert!(text.contains(function), "{text}");
}

fn scratch_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sz-logging-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

/// A file path native cannot open: its parent is a regular file (spdlog creates missing parent
/// directories, so a missing directory would be created).
fn unopenable_path(marker: &str) -> PathBuf {
    let blocker = scratch_dir().join("not-a-directory");
    std::fs::write(&blocker, b"").expect("create blocker file");
    blocker.join(format!("{marker}.log"))
}

/// Makes native log one ERROR containing `marker`, on the calling thread; the previous default
/// output is kept. Calls native directly: the SDK rejects an unopenable path before native sees it.
fn emit_error(marker: &str) {
    let path = unopenable_path(marker);
    let path = CString::new(path.to_str().expect("UTF-8 temp path")).expect("no NUL");
    let file = i32::try_from(sz_rust_sdk_ffi::logging::SzLogOutputType_SZ_LOG_OUTPUT_FILE).unwrap();
    // SAFETY: a NUL-terminated path that outlives the call; 4.5.0 returns void.
    unsafe { sz_rust_sdk_ffi::logging::SzLogging_setDefaultOutput(file, path.as_ptr()) };
}

/// Sets a flag when dropped: observes when a closure is freed.
struct DropFlag(Arc<AtomicBool>);
impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn abs_diff(a: SystemTime, b: SystemTime) -> Duration {
    a.duration_since(b).unwrap_or_else(|e| e.duration())
}

fn collect(rx: &mpsc::Receiver<String>) -> Vec<String> {
    rx.try_iter().collect()
}

#[test]
#[serial]
fn test_register_delivers_and_drop_frees_closure() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (tx, rx) = mpsc::channel();
    let registration = logging
        .register(SzLogLevel::Error, move |r| {
            let _ = tx.send(r.to_entry());
        })
        .unwrap();

    emit_error("register-drop-1");
    let entry = rx.recv_timeout(WAIT).expect("callback received the ERROR");
    assert_eq!(entry.level, SzLogLevel::Error);
    assert!(
        entry.message.contains("Failed to create file log sink"),
        "{entry:?}"
    );
    assert!(entry.message.contains("register-drop-1"), "{entry:?}");
    assert!(entry.thread_hash.is_some(), "{entry:?}");

    // Outside a callback, drop unregisters and then frees the closure, so the sender is gone.
    drop(registration);
    assert_eq!(rx.try_recv().unwrap_err(), TryRecvError::Disconnected);
    emit_error("register-drop-2");
    assert_eq!(rx.try_recv().unwrap_err(), TryRecvError::Disconnected);
}

#[test]
#[serial]
fn test_formats_for_registered_callbacks() {
    let _reset = Reset::quiet();
    let logging = logging();
    let mut channels = Vec::new();
    let mut registrations = Vec::new();
    for &format in SzLogFormat::ALL {
        let (tx, rx) = mpsc::channel();
        let options = SzLogCallbackOptions::new(SzLogLevel::Error).format(format);
        registrations.push(
            logging
                .register_callback(
                    options,
                    Box::new(move |r| {
                        let _ = tx.send(r.message().into_owned());
                    }),
                )
                .unwrap(),
        );
        channels.push((format, rx));
    }
    let (tx, default_rx) = mpsc::channel();
    registrations.push(
        logging
            .register(SzLogLevel::Error, move |r| {
                let _ = tx.send(r.message().into_owned());
            })
            .unwrap(),
    );
    emit_error("formats");
    drop(registrations);

    let message = collect(&default_rx)
        .into_iter()
        .find(|m| m.contains("formats"))
        .expect("delivered");
    assert!(
        message.starts_with("Failed to create file log sink"),
        "default options are BARE: {message}"
    );

    for (format, rx) in channels {
        let message = collect(&rx)
            .into_iter()
            .find(|m| m.contains("formats"))
            .expect("delivered");
        match format {
            SzLogFormat::Legacy => {
                assert!(
                    message.contains("] ERR: Failed to create file log sink"),
                    "LEGACY is `<time> [module:hash] SEV: msg`: {message}"
                )
            }
            SzLogFormat::Jsonl => {
                let json: serde_json::Value = serde_json::from_str(&message).expect("JSONL");
                assert!(json["timestamp"].is_string(), "{message}");
                assert!(
                    json["message"].as_str().unwrap().contains("formats"),
                    "{message}"
                );
            }
            // DEFAULT delivers the message only to a registered callback, like BARE.
            _ => assert!(
                message.starts_with("Failed to create file log sink"),
                "{message}"
            ),
        }
    }
}

#[test]
#[serial]
fn test_level_filtering_and_effective_level() {
    let _reset = Reset::quiet();
    let logging = logging();
    logging.set_default_log_level(SzLogLevel::Error).unwrap();
    assert_eq!(
        logging.get_effective_log_level().unwrap(),
        SzLogLevel::Error
    );

    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let critical = logging
        .register(SzLogLevel::Critical, move |_r| {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    emit_error("filtered");
    assert_eq!(calls.load(Ordering::SeqCst), 0, "ERROR is below CRITICAL");

    let debug = logging.register(SzLogLevel::Debug, |_r| {}).unwrap();
    assert_eq!(
        logging.get_effective_log_level().unwrap(),
        SzLogLevel::Debug,
        "min over sinks"
    );
    drop(debug);
    assert_eq!(
        logging.get_effective_log_level().unwrap(),
        SzLogLevel::Error
    );
    drop(critical);
}

#[test]
#[serial]
fn test_set_level_and_level_are_stubbed() {
    let _reset = Reset::quiet();
    let logging = logging();
    let registration = logging.register(SzLogLevel::Error, |_r| {}).unwrap();
    assert_requires_newer(
        registration.set_level(SzLogLevel::Trace),
        "SzLogging_setCallbackLevel",
    );
    assert_requires_newer(registration.level(), "SzLogging_getCallbackLevel");
}

#[test]
#[serial]
fn test_default_getters_are_stubbed() {
    let logging = logging();
    assert_requires_newer(logging.get_default_output(), "SzLogging_getDefaultOutput");
    assert_requires_newer(
        logging.get_default_format(),
        "SzLogging_getDefaultFormatter",
    );
    assert_requires_newer(
        logging.get_default_log_level(),
        "SzLogging_getDefaultLogLevel",
    );
}

#[test]
#[serial]
fn test_record_timestamp_and_thread_id_are_none() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (tx, rx) = mpsc::channel();
    let registration = logging
        .register(SzLogLevel::Error, move |r| {
            let _ = tx.send((r.timestamp(), r.timestamp_ns(), r.thread_id()));
        })
        .unwrap();
    emit_error("record-fields");
    drop(registration);
    assert_eq!(rx.recv_timeout(WAIT).unwrap(), (None, None, None));
}

#[test]
#[serial]
fn test_default_output_argument_errors() {
    let _reset = Reset::quiet();
    let logging = logging();
    let err = logging
        .set_default_output(SzLogOutput::Callback)
        .unwrap_err();
    assert!(err.is_bad_input(), "Callback is get-only: {err}");
    let err = logging
        .set_default_output(SzLogOutput::file(""))
        .unwrap_err();
    assert!(err.is_bad_input(), "empty path: {err}");
    let path = scratch_dir().join("append.log");
    assert_requires_newer(
        logging.set_default_output(SzLogOutput::append_file(&path)),
        "SZ_LOG_OUTPUT_FILE_APPEND",
    );
    assert!(!path.exists(), "the stub does not touch the file");
}

#[test]
#[serial]
fn test_default_output_unopenable_file_is_bad_input() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (tx, rx) = mpsc::channel();
    let registration = logging
        .register(SzLogLevel::Error, move |r| {
            let _ = tx.send(r.message().into_owned());
        })
        .unwrap();
    let path = unopenable_path("rejected");
    let err = logging
        .set_default_output(SzLogOutput::file(&path))
        .unwrap_err();
    drop(registration);
    assert!(err.is_bad_input(), "{err:?}");
    assert!(
        err.to_string().contains("rejected.log"),
        "names the path: {err}"
    );
    assert!(
        collect(&rx).is_empty(),
        "rejected before native: no ERROR is logged"
    );
}

#[test]
#[serial]
fn test_default_output_file_creates_missing_parent_directories() {
    let _reset = Reset::quiet();
    let logging = logging();
    let dir = scratch_dir().join(format!("missing-{:?}", Instant::now()));
    let path = dir.join("nested").join("sz.log");
    logging
        .set_default_output(SzLogOutput::file(&path))
        .unwrap();
    logging.set_default_output(SzLogOutput::Null).unwrap();
    assert!(
        path.is_file(),
        "created like native does: {}",
        path.display()
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
#[serial]
fn test_default_output_file_truncates_and_keeps_previous_on_failure() {
    let _reset = Reset::quiet();
    let logging = logging();
    logging.set_default_log_level(SzLogLevel::Error).unwrap();
    let path = scratch_dir().join("truncate.log");
    std::fs::write(&path, "PREEXISTING\n").unwrap();

    logging
        .set_default_output(SzLogOutput::file(&path))
        .unwrap();
    // The failed open is logged to the previous output, which is still this file.
    emit_error("truncate-run");
    logging.set_default_output(SzLogOutput::Null).unwrap(); // flush + close
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains("PREEXISTING"), "FILE truncates: {text}");
    assert!(text.contains("truncate-run"), "{text}");
}

#[test]
#[serial]
fn test_default_output_jsonl_file() {
    let _reset = Reset::quiet();
    let logging = logging();
    let path = scratch_dir().join("default.jsonl");
    logging.set_default_log_level(SzLogLevel::Error).unwrap();
    // 4.5.0: setting the output resets the format to LEGACY, so set the format afterwards.
    logging
        .set_default_output(SzLogOutput::file(&path))
        .unwrap();
    logging.set_default_format(SzLogFormat::Jsonl).unwrap();
    emit_error("jsonl-file");
    logging.set_default_output(SzLogOutput::Null).unwrap();

    let text = std::fs::read_to_string(&path).unwrap();
    let line = text
        .lines()
        .find(|l| l.contains("jsonl-file"))
        .expect("line written");
    let json: serde_json::Value = serde_json::from_str(line).expect("JSONL line");
    assert!(json["timestamp"].is_string(), "{line}");
    assert_eq!(
        json["severity"], "error",
        "JSONL spells levels in lower case: {line}"
    );
}

#[test]
#[serial]
fn test_default_callback_with_format() {
    let _reset = Reset::quiet();
    let logging = logging();
    let freed = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    let flag = DropFlag(Arc::clone(&freed));
    logging
        .set_default_callback(
            SzLogCallbackOptions::new(SzLogLevel::Error).format(SzLogFormat::Bare),
            Box::new(move |r| {
                let _keep = &flag;
                let _ = tx.send(r.message().into_owned());
            }),
        )
        .unwrap();
    emit_error("default-callback");
    let message = rx.recv_timeout(WAIT).unwrap();
    assert!(
        message.starts_with("Failed to create file log sink"),
        "BARE: {message}"
    );

    logging.set_default_output(SzLogOutput::Null).unwrap();
    emit_error("default-callback-replaced");
    assert!(
        collect(&rx).is_empty(),
        "replaced default callback is not called"
    );
    // 4.5.0 has no release hook: the replaced closure is leaked, never freed.
    assert!(!freed.load(Ordering::SeqCst));
}

#[test]
#[serial]
fn test_drop_inside_own_callback_deactivates_and_leaks() {
    let _reset = Reset::quiet();
    let logging = logging();
    let freed = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let slot: Arc<Mutex<Option<SzLogRegistration>>> = Arc::new(Mutex::new(None));

    let flag = DropFlag(Arc::clone(&freed));
    let counter = Arc::clone(&calls);
    let own = Arc::clone(&slot);
    let registration = logging
        .register(SzLogLevel::Error, move |_r| {
            let _keep = &flag;
            counter.fetch_add(1, Ordering::SeqCst);
            drop(own.lock().unwrap().take());
        })
        .unwrap();
    *slot.lock().unwrap() = Some(registration);

    emit_error("self-drop-1");
    emit_error("self-drop-2");
    assert_eq!(calls.load(Ordering::SeqCst), 1, "not called after its drop");
    assert!(
        !freed.load(Ordering::SeqCst),
        "leaked: it may still be running on 4.5.0"
    );
}

#[test]
#[serial]
fn test_drop_other_registration_inside_callback() {
    let _reset = Reset::quiet();
    let logging = logging();
    let b_calls = Arc::new(AtomicUsize::new(0));
    let b_slot: Arc<Mutex<Option<SzLogRegistration>>> = Arc::new(Mutex::new(None));

    let counter = Arc::clone(&b_calls);
    let b = logging
        .register(SzLogLevel::Error, move |_r| {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    *b_slot.lock().unwrap() = Some(b);
    let other = Arc::clone(&b_slot);
    let a = logging
        .register(SzLogLevel::Error, move |_r| {
            drop(other.lock().unwrap().take())
        })
        .unwrap();

    emit_error("cross-drop-1"); // B may run before A in this delivery
    let after_first = b_calls.load(Ordering::SeqCst);
    assert!(after_first <= 1);
    emit_error("cross-drop-2");
    assert_eq!(
        b_calls.load(Ordering::SeqCst),
        after_first,
        "B deactivated by A"
    );
    drop(a);
}

#[test]
#[serial]
fn test_sink_changes_inside_callback_are_refused() {
    let _reset = Reset::quiet();
    let logging: Arc<dyn SzLogging> = Arc::from(logging());
    let results = Arc::new(Mutex::new(Vec::new()));
    let inner = Arc::clone(&logging);
    let sink = Arc::clone(&results);
    let registration = logging
        .register(SzLogLevel::Error, move |_r| {
            let mut out = sink.lock().unwrap();
            out.push((
                "register",
                inner.register(SzLogLevel::Info, |_r| {}).map(drop),
            ));
            out.push((
                "set_default_output",
                inner.set_default_output(SzLogOutput::Null),
            ));
            out.push((
                "set_default_format",
                inner.set_default_format(SzLogFormat::Bare),
            ));
            out.push((
                "set_default_callback",
                inner.set_default_callback(SzLogLevel::Info.into(), Box::new(|_r| {})),
            ));
            out.push((
                "set_default_log_level",
                inner.set_default_log_level(SzLogLevel::Error),
            ));
            out.push((
                "get_effective_log_level",
                inner.get_effective_log_level().map(drop),
            ));
        })
        .unwrap();
    emit_error("inside");
    drop(registration);

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 6, "delivered once");
    for (name, result) in results.iter() {
        match *name {
            "set_default_log_level" | "get_effective_log_level" => {
                assert!(result.is_ok(), "{name}: {result:?}")
            }
            _ => {
                let err = result.as_ref().unwrap_err();
                assert!(matches!(err, SzError::Configuration(_)), "{name}: {err:?}");
                assert!(
                    err.to_string().contains("from inside a log callback"),
                    "{err}"
                );
            }
        }
    }
}

#[test]
#[serial]
fn test_panics_in_callback_and_drop_do_not_cross_ffi() {
    let _reset = Reset::quiet();
    let logging = logging();
    let calls = Arc::new(AtomicUsize::new(0));

    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("closure drop panic (expected by this test)");
        }
    }
    let guard = PanicOnDrop;
    let counter = Arc::clone(&calls);
    let registration = logging
        .register(SzLogLevel::Error, move |_r| {
            let _keep = &guard;
            counter.fetch_add(1, Ordering::SeqCst);
            panic!("callback panic (expected by this test)");
        })
        .unwrap();

    emit_error("panic-1");
    emit_error("panic-2");
    assert_eq!(
        calls.load(Ordering::SeqCst),
        2,
        "delivery continues after a panic"
    );
    drop(registration); // the closure's Drop panics; it is swallowed
    emit_error("panic-3");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
#[serial]
fn test_thread_local_requires_newer_senzing() {
    let _reset = Reset::quiet();
    let logging = logging();
    logging.set_default_log_level(SzLogLevel::Error).unwrap();
    let tl = "SzLogging_registerThreadLocalCallback, GDEV-4960";
    let freed = Arc::new(AtomicBool::new(false));
    let flag = DropFlag(Arc::clone(&freed));
    assert_requires_newer(
        logging.register_thread_local_callback(
            SzLogLevel::Trace.into(),
            Box::new(move |_r| {
                let _keep = &flag;
            }),
        ),
        tl,
    );
    assert!(
        freed.load(Ordering::SeqCst),
        "the refused closure is dropped"
    );
    let options = SzLogCallbackOptions::new(SzLogLevel::Info).format(SzLogFormat::Legacy);
    assert_requires_newer(logging.register_thread_local(options, |_r| {}), tl);
    assert_requires_newer(logging.capture_thread_logs(SzLogLevel::Error), tl);
    assert_eq!(
        logging.get_effective_log_level().unwrap(),
        SzLogLevel::Error,
        "nothing was registered natively (a TRACE registration would lower it)"
    );
}

#[test]
#[serial]
fn test_set_default_closure() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (tx, rx) = mpsc::channel();
    logging
        .set_default(SzLogLevel::Error, move |r| {
            let _ = tx.send((r.level(), r.message().into_owned()));
        })
        .unwrap();
    emit_error("set-default-closure");
    let (level, message) = rx.recv_timeout(WAIT).expect("default closure called");
    assert_eq!(level, SzLogLevel::Error);
    assert!(
        message.starts_with("Failed to create file log sink"),
        "BARE by default: {message}"
    );
    logging.set_default_output(SzLogOutput::Null).unwrap();
    emit_error("set-default-closure-replaced");
    assert!(rx.try_iter().next().is_none(), "replaced");
}

#[test]
#[serial]
#[ignore = "thread-local registration returns an error on 4.5.0 (native never delivered, thread-id mismatch); needs GDEV-4960 (Senzing >= 4.5.1/4.6.0) and the SZ-FUTURE stub removed"]
fn test_thread_local_scope() {
    let _reset = Reset::quiet();
    let logging: Arc<dyn SzLogging> = Arc::from(logging());
    let (tx, rx) = mpsc::channel();
    let registration = logging
        .register_thread_local(SzLogLevel::Error, move |r| {
            let _ = tx.send(r.message().into_owned());
        })
        .unwrap();
    emit_error("tl-mine");
    thread::spawn(move || emit_error("tl-other"))
        .join()
        .unwrap();
    drop(registration);
    let received: Vec<_> = rx.iter().collect();
    assert!(
        received.iter().any(|m| m.contains("tl-mine")),
        "{received:?}"
    );
    assert!(
        !received.iter().any(|m| m.contains("tl-other")),
        "{received:?}"
    );
}

#[test]
#[serial]
#[ignore = "thread-local registration returns an error on 4.5.0 (native never delivered, thread-id mismatch); needs GDEV-4960 (Senzing >= 4.5.1/4.6.0) and the SZ-FUTURE stub removed"]
fn test_capture_thread_logs() {
    let _reset = Reset::quiet();
    let logging: Arc<dyn SzLogging> = Arc::from(logging());
    let capture = logging.capture_thread_logs(SzLogLevel::Error).unwrap();
    emit_error("capture-mine");
    thread::spawn(move || emit_error("capture-other"))
        .join()
        .unwrap();
    let entries = capture.finish();
    assert!(
        entries.iter().any(|e| e.message.contains("capture-mine")),
        "{entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.message.contains("capture-other")),
        "{entries:?}"
    );
}

#[test]
#[serial]
#[ignore = "drops while 8 threads log: needs the GDEV-4960 native fix (Senzing >= 4.5.1/4.6.0)"]
fn test_drop_from_another_thread_while_delivering() {
    let _reset = Reset::quiet();
    let logging: Arc<dyn SzLogging> = Arc::from(logging());
    let freed = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let flag = DropFlag(Arc::clone(&freed));
    let counter = Arc::clone(&calls);
    let registration = logging
        .register(SzLogLevel::Error, move |_r| {
            let _keep = &flag;
            counter.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_micros(200));
        })
        .unwrap();

    let stop = Arc::new(AtomicBool::new(false));
    let emitters: Vec<_> = (0..8)
        .map(|i| {
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                while !stop.load(Ordering::SeqCst) {
                    emit_error(&format!("emitter-{i}"));
                }
            })
        })
        .collect();
    let start = Instant::now();
    while calls.load(Ordering::SeqCst) < 50 && start.elapsed() < WAIT {
        thread::sleep(Duration::from_millis(1));
    }
    thread::spawn(move || drop(registration)).join().unwrap();
    assert!(freed.load(Ordering::SeqCst), "freed when drop returned");
    let after = calls.load(Ordering::SeqCst);
    thread::sleep(Duration::from_millis(50));
    assert_eq!(
        calls.load(Ordering::SeqCst),
        after,
        "no delivery after drop"
    );
    stop.store(true, Ordering::SeqCst);
    for emitter in emitters {
        emitter.join().unwrap();
    }
}

#[test]
#[serial]
#[ignore = "record time and UTC JSONL need Senzing >= 4.5.1/4.6.0 (SzLogRecord, GDEV-4960)"]
fn test_record_timestamp_is_utc_now_and_matches_jsonl() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (tx, rx) = mpsc::channel();
    let options = SzLogCallbackOptions::new(SzLogLevel::Error).format(SzLogFormat::Jsonl);
    let registration = logging
        .register(options, move |r| {
            let _ = tx.send((r.timestamp(), r.thread_id(), r.message().into_owned()));
        })
        .unwrap();
    let before = SystemTime::now();
    emit_error("record-time");
    let (timestamp, thread_id, message) = rx.recv_timeout(WAIT).unwrap();
    drop(registration);
    let timestamp = timestamp.expect("record timestamp");
    assert!(thread_id.is_some());
    assert!(
        abs_diff(timestamp, before) < Duration::from_secs(5),
        "record time is now (UTC)"
    );
    let json: serde_json::Value = serde_json::from_str(&message).unwrap();
    let stamp = json["timestamp"].as_str().unwrap();
    assert!(
        stamp.ends_with('Z'),
        "JSONL is UTC ISO-8601 with Z: {stamp}"
    );
}

/// The 4.5.0 lifecycle: register before init, run the engine, destroy, then unregister.
#[test]
#[serial]
fn test_engine_messages_across_init_and_destroy() -> SzResult<()> {
    let _reset = Reset::quiet();
    let logging = logging();
    let entries = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&entries);
    let registration = logging.register(SzLogLevel::Trace, move |r| {
        sink.lock().unwrap().push(r.to_entry());
    })?;

    let env = ExampleEnvironment::initialize("sz-rust-sdk-logging-engine-test")?;
    let engine = env.get_engine()?;
    engine.add_record(
        "TEST",
        "LOGGING_1",
        r#"{"NAME_FULL": "Robert Smith", "EMAIL_ADDRESS": "bsmith@example.com"}"#,
        None,
    )?;
    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    drop(registration); // after destroy: nothing else logs

    let entries = entries.lock().unwrap();
    eprintln!("captured {} engine records at TRACE", entries.len());
    assert!(
        !entries.is_empty(),
        "engine init/add_record/destroy logged at TRACE"
    );
    assert!(entries.iter().all(|e| !e.module.is_empty()));
    Ok(())
}

/// Concurrent delivery to a registration that stays in place (valid on 4.5.0): register before
/// init, log from several engine threads, destroy, then unregister.
#[test]
#[serial]
fn test_concurrent_delivery_to_stable_registration() -> SzResult<()> {
    const THREADS: usize = 4;
    const RECORDS: usize = 25;
    let _reset = Reset::quiet();
    let logging = logging();
    let entries = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&entries);
    let registration = logging.register(SzLogLevel::Trace, move |r| {
        sink.lock().unwrap().push(r.to_entry());
    })?;

    let env = ExampleEnvironment::initialize("sz-rust-sdk-logging-concurrent-test")?;
    let engine = env.get_engine()?;
    thread::scope(|s| {
        for t in 0..THREADS {
            let engine = &engine;
            s.spawn(move || {
                for i in 0..RECORDS {
                    let json = format!(r#"{{"NAME_FULL": "Robert Smith {t} {i}"}}"#);
                    engine
                        .add_record("TEST", &format!("LOG_{t}_{i}"), &json, None)
                        .expect("add_record");
                }
            });
        }
    });
    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    drop(registration);

    let entries = entries.lock().unwrap();
    let threads: std::collections::HashSet<_> = entries
        .iter()
        .filter_map(|e| e.thread_hash.clone())
        .collect();
    eprintln!(
        "captured {} records from {} threads",
        entries.len(),
        threads.len()
    );
    assert!(
        threads.len() > 1,
        "records delivered concurrently from several threads"
    );
    Ok(())
}

/// A replaced default callback is no longer called and (4.5.0) is leaked, not freed.
#[test]
#[serial]
fn test_default_callback_replaced_by_default_callback() {
    let _reset = Reset::quiet();
    let logging = logging();
    let (first_freed, second_freed) = (
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicBool::new(false)),
    );
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let flag1 = DropFlag(Arc::clone(&first_freed));
    let flag2 = DropFlag(Arc::clone(&second_freed));
    logging
        .set_default_callback(
            SzLogLevel::Error.into(),
            Box::new(move |r| {
                let _keep = &flag1;
                let _ = tx1.send(r.message().into_owned());
            }),
        )
        .unwrap();
    logging
        .set_default_callback(
            SzLogLevel::Error.into(),
            Box::new(move |r| {
                let _keep = &flag2;
                let _ = tx2.send(r.message().into_owned());
            }),
        )
        .unwrap();
    emit_error("default-callback-twice");
    assert!(
        rx2.recv_timeout(WAIT).is_ok(),
        "the new default callback is called"
    );
    assert!(collect(&rx1).is_empty(), "the replaced one is not");
    assert!(
        !first_freed.load(Ordering::SeqCst),
        "4.5.0: replaced closure is leaked"
    );
    assert!(!second_freed.load(Ordering::SeqCst));
}
