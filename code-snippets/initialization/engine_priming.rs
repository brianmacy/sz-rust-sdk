//! Engine Priming Example
//!
//! This example demonstrates how to prime the Senzing engine for optimal performance
//! by warming up internal caches and data structures.
//!
//! Rust equivalent of: initialization/EnginePriming/Program.cs

use std::time::Instant;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    // Initialize the Senzing environment using the singleton pattern
    let environment = match ExampleEnvironment::initialize(instance_name) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to initialize environment: {}", e);
            return Err(e);
        }
    };

    // Get the engine from the environment
    let engine = environment.get_engine()?;

    println!("Starting engine priming...");

    // Start timing the priming operation
    let start_time = Instant::now();

    // Prime the engine - this warms up internal caches and data structures
    engine.prime_engine()?;

    let duration = start_time.elapsed();

    println!("Primed Senzing engine. ({} ms)", duration.as_millis());

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}
