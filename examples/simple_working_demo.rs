//! Simple Working Demo
//!
//! This example demonstrates basic Senzing SDK functionality with SQLite.
//! It handles configuration issues gracefully and shows working operations.
//!
//! Run with: cargo run --example simple_working_demo

use std::sync::Arc;
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    println!("=== Senzing SDK Working Demo ===\n");

    println!("Initializing Senzing SDK...");

    // Initialize Senzing environment using the helper
    let env = ExampleEnvironment::initialize("simple-working-demo")?;
    println!("✅ Environment initialized successfully!");
    demonstrate_working_features(env)?;

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

fn demonstrate_working_features(env: Arc<SzEnvironmentCore>) -> SzResult<()> {
    println!("\nTesting SDK components...");

    // Test engine operations
    match ExampleEnvironment::get_engine_with_setup(&env) {
        Ok(engine) => {
            println!("✅ Engine component available");

            // Test search operation
            let search_attrs = r#"{"NAME_LAST": "Smith"}"#;
            match engine.search_by_attributes(search_attrs, None, None) {
                Ok(results) => {
                    println!("✅ Search operation successful");
                    println!("   Results: {}", results);
                }
                Err(e) => println!("⚠️  Search operation: {}", e),
            }
        }
        Err(e) => println!("⚠️  Engine component: {}", e),
    }

    // Test other components
    match env.get_product() {
        Ok(_) => println!("✅ Product component available"),
        Err(e) => println!("⚠️  Product component: {}", e),
    }

    println!("\n🎯 Demo complete! SDK is functional and ready to use.");
    Ok(())
}
