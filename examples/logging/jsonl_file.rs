//! Write Senzing's default log output to a JSONL file (one JSON object per line).
//!
//! Set the output first, then the format: on Senzing 4.5.0 changing the output resets the format
//! to LEGACY. JSONL timestamps are UTC (`...Z`) from Senzing 4.5.1 / 4.6.0; 4.5.0 writes local time.

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let path = std::env::temp_dir().join("senzing-default-output.jsonl");
    let env = ExampleEnvironment::initialize("logging-jsonl-file")?;
    let logging = env.get_logging()?;

    // Senzing 4.5.0: change the default output only before the engine is initialized.
    logging.set_default_output(&SzLogOutput::file(&path))?;
    logging.set_default_formatter(SzLogFormat::Jsonl)?;
    logging.set_log_level(SzLogLevel::Debug)?;

    let engine = env.get_engine()?;
    engine.add_record("TEST", "JSONL_1", r#"{"NAME_FULL": "Robert Smith"}"#, None)?;
    drop(engine);
    ExampleEnvironment::cleanup(env)?;

    // After destroy: restore stderr (closes the file) and the LEGACY format.
    logging.set_default_output(&SzLogOutput::Stderr)?;
    logging.set_default_formatter(SzLogFormat::Legacy)?;
    logging.set_log_level(SzLogLevel::Error)?;

    let text = std::fs::read_to_string(&path).map_err(|e| SzError::bad_input(e.to_string()))?;
    let mut lines = 0;
    for line in text.lines() {
        let json: serde_json::Value = serde_json::from_str(line)?;
        if lines == 0 {
            println!(
                "first line: timestamp={} severity={} module={}",
                json["timestamp"], json["severity"], json["module"]
            );
        }
        lines += 1;
    }
    println!("{lines} JSONL lines in {}", path.display());
    Ok(())
}
