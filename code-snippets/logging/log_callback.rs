//! Log Callback Example
//!
//! This example demonstrates how to receive Senzing log messages in your own code.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Getting the logging interface with get_logging()
//! - Registering a closure callback with a minimum level and a format
//! - The registration is RAII: dropping it unregisters the callback
//! - Senzing 4.5.0 rule: register before the engine is initialized, unregister after destroy

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment (the engine is not initialized yet)
    let env = sz_rust_sdk::helpers::ExampleEnvironment::initialize("log_callback_example")?;

    // Step 2: Get the logging interface (no native initialization happens here)
    let logging = env.get_logging()?;

    // Step 3: Register a callback for INFO and above, as one JSON object per message.
    // It runs on the thread that logs: keep it fast and never call engine methods from it.
    let options = SzLogCallbackOptions::new(SzLogLevel::Info).format(SzLogFormat::Jsonl);
    let registration = logging.register(options, |record| {
        println!("[{}] {}", record.level(), record);
    })?;

    // Step 4: Use the engine; its initialization and work are logged to the callback
    let engine = env.get_engine()?;
    engine.add_record("TEST", "1", r#"{"NAME_FULL": "Robert Smith"}"#, None)?;

    // Step 5: Destroy the environment, then drop the registration (unregisters the callback)
    drop(engine);
    env.destroy()?;
    drop(registration);

    println!("✅ Log callback example complete");
    Ok(())
}
