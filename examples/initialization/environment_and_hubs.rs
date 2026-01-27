//! Environment and Hubs Example
//!
//! This example demonstrates how to initialize the Senzing environment
//! and obtain handles to the various SDK components.
//!
//! Run with: cargo run --example environment_and_hubs

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("Initializing Senzing SDK...");

    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("environment-and-hubs")?;
    println!("✅ Environment initialized successfully!");

    // Try to get handles to SDK components
    println!("\nTesting SDK components:");

    match env.get_product() {
        Ok(product) => {
            println!("✅ Product component available");
            if let Ok(version) = product.get_version() {
                println!("   Version: {}", version);
            }
        }
        Err(e) => println!("⚠️  Product component: {}", e),
    }

    match env.get_config_manager() {
        Ok(_) => println!("✅ Config Manager component available"),
        Err(e) => println!("⚠️  Config Manager component: {}", e),
    }

    match env.get_diagnostic() {
        Ok(_) => println!("✅ Diagnostic component available"),
        Err(e) => println!("⚠️  Diagnostic component: {}", e),
    }

    match ExampleEnvironment::get_engine_with_setup(&env) {
        Ok(_) => println!("✅ Engine component available"),
        Err(e) => println!("⚠️  Engine component: {}", e),
    }

    // Clean up the test database
    ExampleEnvironment::cleanup(env)?;

    Ok(())
}
