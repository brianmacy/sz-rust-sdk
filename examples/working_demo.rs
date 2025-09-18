//! Working Senzing SDK Demo
//!
//! This example demonstrates SDK functionality that works with basic configuration.
//! It focuses on operations that succeed with the current setup.
//!
//! Run with: cargo run --example working_demo

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    println!("=== Working Senzing SDK Demo ===\n");

    // Initialize environment using the helper utility
    println!("Initializing Senzing environment...");
    let env = ExampleEnvironment::initialize("working-demo")?;
    println!("✅ Environment ready!\n");

    // Use the engine component (this one works)
    println!("Testing engine operations that work:");
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    println!("✅ Engine component ready");

    // Test search (this typically works even without full setup)
    let search_attributes = r#"{"NAME_LAST": "Smith"}"#;
    match engine.search_by_attributes(search_attributes, None, None) {
        Ok(results) => {
            println!("✅ Search operation successful");
            println!("   Results: {}", results);
        }
        Err(e) => println!("⚠️  Search failed: {}", e),
    }

    // Test find path (basic operation)
    match engine.find_path(1, 2, 3, None, None, None) {
        Ok(result) => println!("✅ Find path successful: {}", result),
        Err(e) => println!("⚠️  Find path failed (expected): {}", e),
    }

    println!("\n🎯 Demo complete! This shows working SDK operations.");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
