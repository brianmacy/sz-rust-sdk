//! Load Records Example
//!
//! This example demonstrates the basic Senzing SDK pattern for loading records.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Environment initialization with database configuration
//! - Getting an engine instance for data operations
//! - Adding records using add_record() method
//! - Proper error handling with SzResult
//! - Resource cleanup

use serde_json::json;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment
    let env = get_environment()?;

    // Step 2: Get the engine for data operations
    let engine = env.get_engine()?;

    println!("Loading records into Senzing...");

    // Step 3: Load some example records
    // add_record(data_source, record_id, json_data, flags)
    // Note: "TEST" is the default data source - always available, no setup required
    engine.add_record(
        "TEST",
        "1001",
        &json!({
            "NAME_FIRST": "John",
            "NAME_LAST": "Smith",
            "PHONE_NUMBER": "555-1234",
            "EMAIL_ADDRESS": "john.smith@example.com"
        })
        .to_string(),
        None,
    )?;
    println!("✓ Added record 1001");

    engine.add_record(
        "TEST",
        "1002",
        &json!({
            "NAME_FIRST": "Jane",
            "NAME_LAST": "Doe",
            "PHONE_NUMBER": "555-5678",
            "EMAIL_ADDRESS": "jane.doe@example.com"
        })
        .to_string(),
        None,
    )?;
    println!("✓ Added record 1002");

    engine.add_record(
        "TEST",
        "1003",
        &json!({
            "NAME_FIRST": "Bob",
            "NAME_LAST": "Johnson",
            "PHONE_NUMBER": "555-9999",
            "EMAIL_ADDRESS": "bob.johnson@example.com"
        })
        .to_string(),
        None,
    )?;
    println!("✓ Added record 1003");

    println!("✅ Successfully loaded 3 records");

    Ok(())
}

/// Simple helper to get a configured Senzing environment
/// Handles database setup and configuration automatically
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("load_records_example")
}
