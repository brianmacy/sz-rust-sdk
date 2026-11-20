//! Forward Senzing's log records to the `log` crate facade (cargo feature `log`).
//!
//! Run with `cargo run --example forward_to_log --features log`.
//!
//! Any `log` implementation works (env_logger, tracing-log, ...); this example installs a tiny one
//! that prints `LEVEL target: message`. Records arrive with target `senzing::<module>`, so a
//! logger's filter can select them. The default output is set to `Null` because the `log` backend
//! is the only sink. Senzing 4.5.0: set up logging before the engine is initialized.

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// A minimal `log` implementation: prints Senzing's records to stdout.
struct PrintLogger;

impl log::Log for PrintLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.target().starts_with("senzing")
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            println!(
                "{:<5} {}: {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: PrintLogger = PrintLogger;

fn main() -> SzResult<()> {
    log::set_logger(&LOGGER).map_err(|e| SzError::configuration(e.to_string()))?;
    log::set_max_level(log::LevelFilter::Debug);

    let env = ExampleEnvironment::initialize("logging-forward-to-log")?;
    let logging = env.get_logging()?;
    logging.set_default_output(SzLogOutput::Null)?; // `log` is the only sink
    let forwarding = logging.forward_to_log(SzLogLevel::Debug)?;

    let engine = env.get_engine()?;
    engine.add_record("TEST", "LOG_1", r#"{"NAME_FULL": "Robert Smith"}"#, None)?;
    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    drop(forwarding); // after destroy: nothing else logs

    logging.set_default_output(SzLogOutput::Stderr)?;
    Ok(())
}
