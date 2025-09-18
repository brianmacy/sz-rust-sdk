//! Delete Records Example
//!
//! This example demonstrates how to delete records from Senzing.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Environment initialization
//! - Adding records with add_record()
//! - Deleting records with delete_record()
//! - Verifying deletion with get_record()
//! - Error handling for "not found" cases

use serde_json::json;
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment
    let env = get_environment()?;

    // Step 2: Get the engine for data operations
    let engine = env.get_engine()?;

    println!("Demonstrating record deletion...");

    // Step 3: First load a test record
    // Using "TEST" data source which is always available
    engine.add_record(
        "TEST",
        "DELETE_ME",
        &json!({
            "NAME_FIRST": "Test",
            "NAME_LAST": "Record",
            "PHONE_NUMBER": "555-0000"
        })
        .to_string(),
        None,
    )?;
    println!("✓ Added test record DELETE_ME");

    // Step 4: Verify the record exists
    let record = engine.get_record("TEST", "DELETE_ME", None)?;
    println!("✓ Confirmed record exists: {} chars", record.len());

    // Step 5: Delete the record
    // delete_record(data_source, record_id, flags)
    engine.delete_record("TEST", "DELETE_ME", None)?;
    println!("✓ Deleted record DELETE_ME");

    // Step 6: Verify the record was deleted
    match engine.get_record("TEST", "DELETE_ME", None) {
        Ok(_) => println!("⚠ WARNING: Record still exists!"),
        Err(_) => println!("✓ Confirmed record was deleted"),
    }

    println!("✅ Record deletion demonstration complete");

    Ok(())
}

/// Simple helper to get a configured Senzing environment
/// Handles database setup and configuration automatically
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("delete_records_example")
}
