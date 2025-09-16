//! Verbose Debug Example
//!
//! This example enables verbose logging to help identify configuration issues.
//!
//! Run with: cargo run --example verbose_debug

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Senzing SDK Verbose Debug ===\n");

    println!("Initializing Senzing environment with verbose logging...");

    // Initialize using ExampleEnvironment helper (verbose logging is enabled by default)
    let env = ExampleEnvironment::initialize_verbose("verbose-debug")?;
    println!("✅ Environment initialized successfully!\n");

    // Try to get each component with verbose logging
    println!("Testing Product component:");
    match env.get_product() {
        Ok(product) => {
            println!("✅ Product component available");
            match product.get_version() {
                Ok(version) => println!("   Version: {}", version),
                Err(e) => println!("   ⚠️  Version error: {}", e),
            }
        }
        Err(e) => println!("⚠️  Product component failed: {}", e),
    }

    println!("\nTesting Config Manager component:");
    match env.get_config_manager() {
        Ok(_) => println!("✅ Config Manager component available"),
        Err(e) => println!("⚠️  Config Manager failed: {}", e),
    }

    println!("\nTesting Diagnostic component:");
    match env.get_diagnostic() {
        Ok(_) => println!("✅ Diagnostic component available"),
        Err(e) => println!("⚠️  Diagnostic failed: {}", e),
    }

    println!("\nTesting Engine component:");
    match ExampleEnvironment::get_engine_with_setup(&env) {
        Ok(_) => println!("✅ Engine component available"),
        Err(e) => println!("⚠️  Engine failed: {}", e),
    }

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
