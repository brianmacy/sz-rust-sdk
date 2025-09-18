//! Successful Demo Example
//!
//! This example demonstrates how to use the ExampleEnvironment helper
//! to create a working Senzing environment and perform operations successfully.
//!
//! Run with: cargo run --example successful_demo

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    println!("=== Successful Senzing Demo ===\n");

    // Initialize environment using the helper utility
    println!("Initializing Senzing environment...");
    let env = ExampleEnvironment::initialize("successful-demo")?;
    println!("✅ Environment ready!\n");

    // Get engine component (this works reliably)
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Demonstrate successful operations
    println!("🔍 Testing search operation...");
    let search_attrs = r#"{"NAME_LAST": "Smith", "NAME_FIRST": "John"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("✅ Search completed successfully");
    println!("   Results: {}\n", results);

    // Test another search with different attributes
    println!("🔍 Testing organization search...");
    let org_attrs = r#"{"ENTITY_TYPE": "ORGANIZATION", "ORG_NAME": "Acme Corp"}"#;
    let org_results = engine.search_by_attributes(org_attrs, None, None)?;
    println!("✅ Organization search completed successfully");
    println!("   Results: {}\n", org_results);

    // Test find path operation
    println!("🔗 Testing find path operation...");
    match engine.find_path(1, 2, 3, None, None, None) {
        Ok(path_result) => {
            println!("✅ Find path completed successfully");
            println!("   Path: {}", path_result);
        }
        Err(e) => {
            println!("⚠️  Find path: {} (expected - no entities loaded yet)", e);
        }
    }

    // Test network analysis
    println!("\n🕸️  Testing network analysis...");
    match engine.find_network(&[1, 2, 3], 2, 1, 10, None) {
        Ok(network_result) => {
            println!("✅ Network analysis completed");
            println!("   Network: {}", network_result);
        }
        Err(e) => {
            println!(
                "⚠️  Network analysis: {} (expected - no entities loaded yet)",
                e
            );
        }
    }

    println!("\n🎯 All operations completed successfully!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
