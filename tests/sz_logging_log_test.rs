//! `SzLoggingExt::forward_to_log` (cargo feature `log`) against the real native libSzLogging, with a
//! real `log::Log` implementation that collects what it receives (no mocks).
//!
//! Its own test binary: `log::set_logger` can be called once per process. Run with
//! `cargo test --features log --test sz_logging_log_test`.
#![cfg(feature = "log")]

use std::ffi::CString;
use std::sync::Mutex;
use sz_rust_sdk::prelude::*;

/// A `log::Log` that keeps every record as `(level, target, message)`.
struct Collector(Mutex<Vec<(log::Level, String, String)>>);

impl log::Log for Collector {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        self.0.lock().unwrap().push((
            record.level(),
            record.target().to_owned(),
            record.args().to_string(),
        ));
    }

    fn flush(&self) {}
}

static COLLECTOR: Collector = Collector(Mutex::new(Vec::new()));

/// Makes native log one ERROR containing `marker` on the calling thread (see sz_logging_test.rs).
fn emit_error(marker: &str) {
    let dir = std::env::temp_dir().join(format!("sz-logging-log-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let blocker = dir.join("not-a-directory");
    std::fs::write(&blocker, b"").unwrap();
    let path = blocker.join(format!("{marker}.log"));
    let path = CString::new(path.to_str().unwrap()).unwrap();
    let file = i32::try_from(sz_rust_sdk_ffi::logging::SzLogOutputType_SZ_LOG_OUTPUT_FILE).unwrap();
    // SAFETY: a NUL-terminated path that outlives the call; 4.5.0 returns void.
    unsafe { sz_rust_sdk_ffi::logging::SzLogging_setDefaultOutput(file, path.as_ptr()) };
}

fn forwarded(marker: &str) -> Vec<(log::Level, String, String)> {
    COLLECTOR
        .0
        .lock()
        .unwrap()
        .iter()
        .filter(|(_, _, message)| message.contains(marker))
        .cloned()
        .collect()
}

#[test]
fn test_forward_to_log() -> SzResult<()> {
    log::set_logger(&COLLECTOR).expect("first logger in this process");
    log::set_max_level(log::LevelFilter::Trace);

    let logging = SzEnvironmentCore::new("sz-logging-log-test", "{}", false)?.get_logging()?;
    logging.set_default_output(SzLogOutput::Null)?;
    let forwarding = logging.forward_to_log(SzLogLevel::Error)?;

    emit_error("forward-1");
    let records = forwarded("forward-1");
    assert_eq!(records.len(), 1, "{records:?}");
    let (level, target, message) = &records[0];
    assert_eq!(*level, log::Level::Error);
    assert!(target.starts_with("senzing::"), "{target}");
    assert!(
        target.len() > "senzing::".len(),
        "names the module: {target}"
    );
    assert!(
        message.starts_with("Failed to create file log sink"),
        "BARE message: {message}"
    );

    // Records above log::max_level() are skipped.
    log::set_max_level(log::LevelFilter::Off);
    emit_error("forward-filtered");
    assert!(forwarded("forward-filtered").is_empty());
    log::set_max_level(log::LevelFilter::Trace);

    drop(forwarding);
    emit_error("forward-2");
    assert!(
        forwarded("forward-2").is_empty(),
        "dropping the registration stops forwarding"
    );

    logging.set_default_output(SzLogOutput::Stderr)?;
    Ok(())
}
