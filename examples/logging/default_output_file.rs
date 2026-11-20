//! Send Senzing's default log output to a file: truncate on open, or append (Senzing >= 4.5.1 /
//! 4.6.0).
//!
//! Native opens and writes the file. Senzing 4.5.0: change the default output only before the
//! engine is initialized or after it is destroyed.

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let path = std::env::temp_dir().join("senzing-default-output.log");
    let logging = run(&path, SzLogOutput::file(&path), "truncate")?;
    let after_first = line_count(&path);
    println!("truncate: {} has {after_first} lines", path.display());

    // Append keeps the previous run's lines (SZ_LOG_OUTPUT_FILE_APPEND, Senzing >= 4.5.1 / 4.6.0).
    match logging.set_default_output(&SzLogOutput::append_file(&path)) {
        Ok(()) => {
            logging.set_default_output(&SzLogOutput::Stderr)?;
            run(&path, SzLogOutput::append_file(&path), "append")?;
            println!("append: {} has {} lines", path.display(), line_count(&path));
        }
        Err(e) => println!("append: {e}"),
    }
    Ok(())
}

/// Points the default output at `output`, runs the engine, then restores stderr (closing the file).
fn run(path: &std::path::Path, output: SzLogOutput, name: &str) -> SzResult<Box<dyn SzLogging>> {
    let env = ExampleEnvironment::initialize(&format!("logging-default-output-{name}"))?;
    let logging = env.get_logging()?;
    logging.set_default_output(&output)?;
    logging.set_log_level(SzLogLevel::Debug)?;

    let engine = env.get_engine()?;
    engine.add_record("TEST", "FILE_1", r#"{"NAME_FULL": "Robert Smith"}"#, None)?;
    drop(engine);
    ExampleEnvironment::cleanup(env)?;

    logging.set_default_output(&SzLogOutput::Stderr)?; // flushes and closes the file
    logging.set_log_level(SzLogLevel::Error)?;
    println!("{name}: wrote to {}", path.display());
    Ok(logging)
}

fn line_count(path: &std::path::Path) -> usize {
    std::fs::read_to_string(path).map_or(0, |text| text.lines().count())
}
