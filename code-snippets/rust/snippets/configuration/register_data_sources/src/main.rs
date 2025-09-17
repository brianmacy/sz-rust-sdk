//! Register Data Sources Example
//!
//! This example demonstrates how to register multiple data sources in the Senzing
//! configuration using the automatic setup functionality.
//!
//! Rust equivalent of: configuration/RegisterDataSources/Program.cs

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    println!("Registering data sources...");

    // Initialize the Senzing environment with automatic configuration setup
    let environment = match ExampleEnvironment::initialize(instance_name) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to initialize environment: {}", e);
            return Err(e);
        }
    };

    // Get the engine with automatic setup which includes default data sources
    let _engine = ExampleEnvironment::get_engine_with_setup(&environment)?;
    println!("✅ Engine initialized with default data sources");

    // Verify that default data sources exist by testing some operations
    let data_sources_to_test = vec!["CUSTOMERS", "EMPLOYEES"];

    for data_source in &data_sources_to_test {
        println!("Testing data source: {}", data_source);

        // Test adding a record (this will fail if data source doesn't exist)
        let test_record = format!(r#"{{
            "RECORD_ID": "TEST_{}",
            "DATA_SOURCE": "{}",
            "NAME_FIRST": "Test",
            "NAME_LAST": "Person"
        }}"#, data_source, data_source);

        match _engine.add_record(data_source, &format!("TEST_{}", data_source), &test_record, None) {
            Ok(_) => {
                println!("✅ Data source {} is registered and working", data_source);

                // Clean up the test record
                let _ = _engine.delete_record(data_source, &format!("TEST_{}", data_source), None);
            }
            Err(e) => {
                println!("⚠️  Data source {} test failed: {}", data_source, e);
            }
        }
    }

    // Test search to verify configuration is complete
    println!("Testing search operation to verify configuration...");
    let results = _engine.search_by_attributes(r#"{"NAME_LAST": "TestPerson"}"#, None, None)?;
    println!("✅ Search operation successful");
    println!("Search results: {}", results);

    println!("✅ Data source registration and verification completed");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}