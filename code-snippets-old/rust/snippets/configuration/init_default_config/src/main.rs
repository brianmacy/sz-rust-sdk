//! Initialize Default Configuration Example
//!
//! This example shows how to initialize and verify the default configuration
//! for a Senzing repository using automatic setup.
//!
//! Rust equivalent of: configuration/InitDefaultConfig/Program.cs

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    println!("Initializing default configuration...");

    // Initialize the Senzing environment with automatic configuration setup
    let environment = match ExampleEnvironment::initialize(instance_name) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to initialize environment: {}", e);
            return Err(e);
        }
    };

    // Get the engine with automatic setup which ensures default configuration exists
    let _engine = ExampleEnvironment::get_engine_with_setup(&environment)?;
    println!("✅ Engine initialized with default configuration");

    // Get the current active configuration ID to verify setup
    let current_config_id = environment.get_active_config_id()?;
    println!("Active configuration ID: {}", current_config_id);

    // Test that configuration is working by performing a simple operation
    println!("Testing configuration with a simple search...");
    let results = _engine.search_by_attributes(r#"{"NAME_LAST": "TestConfiguration"}"#, None, None)?;
    println!("✅ Configuration test successful");
    println!("Search results: {}", results);

    println!("✅ Default configuration has been initialized and verified");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}
