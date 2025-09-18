//! Basic Usage Example
//!
//! This example shows the most common SDK operations:
//! - Environment initialization
//! - Searching for entities
//! - Basic engine operations
//!
//! Run with: cargo run --example basic_usage

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Basic Senzing SDK Usage ===\n");

    // Initialize environment using the helper utility
    println!("Initializing Senzing environment...");
    let env = ExampleEnvironment::initialize("basic-usage")?;
    println!("✅ Environment ready!\n");

    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Example: Search for entities (this works reliably)
    println!("1. Searching for entities...");
    let search_attributes = r#"{"NAME_LAST": "Smith", "NAME_FIRST": "John"}"#;

    match engine.search_by_attributes(search_attributes, None, None) {
        Ok(results) => {
            println!("   ✅ Search completed successfully");
            println!("   Results: {}", results);
        }
        Err(e) => println!("   ⚠️  Search failed: {}", e),
    }

    // Example: Test find path operation
    println!("\n2. Testing path finding...");
    match engine.find_path(1, 2, 3, None, None, None) {
        Ok(path_result) => {
            println!("   ✅ Find path completed");
            println!("   Path: {}", path_result);
        }
        Err(e) => println!(
            "   ⚠️  Find path: {} (expected - no entities loaded yet)",
            e
        ),
    }

    // Example: Test network analysis
    println!("\n3. Testing network analysis...");
    match engine.find_network(&[1, 2, 3], 2, 1, 10, None) {
        Ok(network_result) => {
            println!("   ✅ Network analysis completed");
            println!("   Network: {}", network_result);
        }
        Err(e) => println!(
            "   ⚠️  Network analysis: {} (expected - no entities loaded yet)",
            e
        ),
    }

    println!("\n🎯 Basic usage examples complete!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
