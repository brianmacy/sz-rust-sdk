//! Engine Priming Example
//!
//! This example demonstrates how to prime the Senzing engine for optimal performance.
//! Priming loads commonly used data structures into memory to improve response times.
//!
//! Run with: cargo run --example engine_priming

use std::time::Instant;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-priming-example")?;

    // Get the engine instance
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("Priming the Senzing engine...");
    let start_time = Instant::now();

    // Prime the engine for optimal performance
    engine.prime_engine()?;

    let elapsed = start_time.elapsed();
    println!(
        "Engine priming completed in {:.2} seconds",
        elapsed.as_secs_f64()
    );

    // Get engine statistics after priming
    let stats = engine.get_stats()?;
    println!("\nEngine statistics after priming:");
    println!("{}", stats);

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_priming() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Engine priming should succeed");
        }
    }
}
