//! Load Records Example
//!
//! This example shows the basic pattern for loading records into Senzing
//! with simple error handling and tuple-based record organization.
//!
//! Rust equivalent of: loading/LoadRecords/Program.cs

use sz_rust_sdk::prelude::*;
use serde_json::json;
use std::collections::HashMap;

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

    println!("Loading records into Senzing repository...");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Get example records to load
    let records = get_records();

    // Loop through the example records and add them to the repository
    for ((data_source_code, record_id), record_definition) in records {
        // Call the add_record() function with no flags (equivalent to SzNoFlags)
        engine.add_record(
            &data_source_code,
            &record_id,
            &record_definition,
            None,
        )?;

        println!("Record {} added", record_id);

        // Flush stdout for real-time feedback (equivalent to Console.Out.Flush())
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    println!("✅ All records loaded successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Get example records for loading
/// Returns a HashMap with (data_source, record_id) as key and JSON record as value
/// This mimics the C# KeyValuePair<(string, string), string> structure
fn get_records() -> HashMap<(String, String), String> {
    let mut records = HashMap::new();

    // Add sample person records using TEST data source (which is available by default)
    records.insert(
        ("TEST".to_string(), "1001".to_string()),
        json!({
            "RECORD_ID": "1001",
            "NAME_FIRST": "John",
            "NAME_LAST": "Smith",
            "PHONE_NUMBER": "555-1234",
            "EMAIL_ADDRESS": "john.smith@example.com",
            "ADDR_FULL": "123 Main Street, Anytown, TX 12345"
        }).to_string(),
    );

    records.insert(
        ("TEST".to_string(), "1002".to_string()),
        json!({
            "RECORD_ID": "1002",
            "NAME_FIRST": "Jane",
            "NAME_LAST": "Doe",
            "PHONE_NUMBER": "555-5678",
            "EMAIL_ADDRESS": "jane.doe@example.com",
            "ADDR_FULL": "456 Oak Avenue, Somewhere, TX 12346"
        }).to_string(),
    );

    records.insert(
        ("TEST".to_string(), "2001".to_string()),
        json!({
            "RECORD_ID": "2001",
            "NAME_FIRST": "Robert",
            "NAME_LAST": "Johnson",
            "PHONE_NUMBER": "555-9876",
            "EMAIL_ADDRESS": "r.johnson@company.com",
            "ADDR_FULL": "789 Pine Street, Elsewhere, TX 12347"
        }).to_string(),
    );

    records.insert(
        ("TEST".to_string(), "2002".to_string()),
        json!({
            "RECORD_ID": "2002",
            "NAME_FIRST": "Sarah",
            "NAME_LAST": "Williams",
            "PHONE_NUMBER": "555-4321",
            "EMAIL_ADDRESS": "sarah.w@company.com",
            "ADDR_FULL": "321 Cedar Lane, Newtown, TX 12348"
        }).to_string(),
    );

    records
}
