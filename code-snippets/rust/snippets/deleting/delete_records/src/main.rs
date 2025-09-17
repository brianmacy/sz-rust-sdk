//! Delete Records Example
//!
//! This example demonstrates how to delete records from the Senzing repository
//! with proper error handling and verification of deletions.
//!
//! Rust equivalent of: deleting/DeleteRecords/Program.cs

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

    println!("Demonstrating record deletion functionality...");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // First, load some test records to delete later
    load_test_records(&engine)?;

    // Then demonstrate deleting them
    delete_test_records(&engine)?;

    println!("✅ Record deletion demonstration completed successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Load test records that we will delete later
fn load_test_records(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    println!("\n--- Loading Test Records for Deletion ---");

    let test_records = get_test_records_for_deletion();

    for ((data_source_code, record_id), record_definition) in &test_records {
        engine.add_record(
            data_source_code,
            record_id,
            record_definition,
            None,
        )?;

        println!("Loaded record {} from {}", record_id, data_source_code);
    }

    println!("Loaded {} test records for deletion", test_records.len());
    Ok(())
}

/// Delete the test records and verify deletion
fn delete_test_records(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    println!("\n--- Deleting Test Records ---");

    let test_records = get_test_records_for_deletion();

    for ((data_source_code, record_id), _) in &test_records {
        println!("Deleting record {} from {}...", record_id, data_source_code);

        // Delete the record with no flags
        match engine.delete_record(
            data_source_code,
            record_id,
            None,
        ) {
            Ok(_delete_info) => {
                println!("  ✓ Successfully deleted record {}", record_id);

                // Verify the record was deleted by trying to get it
                verify_record_deletion(engine, data_source_code, record_id)?;
            },
            Err(e) => {
                eprintln!("  ✗ Failed to delete record {}: {}", record_id, e);
                return Err(e);
            }
        }
    }

    println!("All test records deleted successfully");
    Ok(())
}

/// Verify that a record was actually deleted
fn verify_record_deletion(
    engine: &Box<dyn SzEngine>,
    data_source_code: &str,
    record_id: &str,
) -> SzResult<()> {
    // Try to get the record - this should fail if it was properly deleted
    match engine.get_record(data_source_code, record_id, None) {
        Ok(_record_json) => {
            eprintln!("  ⚠ WARNING: Record {} still exists after deletion!", record_id);
        },
        Err(e) => {
            // Check if this is the expected "record not found" error
            if e.to_string().contains("record not found") || e.to_string().contains("not found") {
                println!("  ✓ Verified: Record {} no longer exists", record_id);
            } else {
                eprintln!("  ⚠ Unexpected error when verifying deletion: {}", e);
            }
        }
    }

    Ok(())
}

/// Get test records specifically for deletion demonstration
fn get_test_records_for_deletion() -> HashMap<(String, String), String> {
    let mut records = HashMap::new();

    // Add records that will be deleted
    records.insert(
        ("TEST".to_string(), "DEL_001".to_string()),
        json!({
            "RECORD_ID": "DEL_001",
            "NAME_FIRST": "Delete",
            "NAME_LAST": "TestRecord",
            "PHONE_NUMBER": "555-0001",
            "EMAIL_ADDRESS": "delete.test1@example.com",
            "ADDR_FULL": "123 Delete Street, Test City, TX 12345"
        }).to_string(),
    );

    records.insert(
        ("TEST".to_string(), "DEL_002".to_string()),
        json!({
            "RECORD_ID": "DEL_002",
            "NAME_FIRST": "Remove",
            "NAME_LAST": "TestRecord",
            "PHONE_NUMBER": "555-0002",
            "EMAIL_ADDRESS": "remove.test2@example.com",
            "ADDR_FULL": "456 Remove Avenue, Test City, TX 12346"
        }).to_string(),
    );

    records.insert(
        ("TEST".to_string(), "DEL_003".to_string()),
        json!({
            "RECORD_ID": "DEL_003",
            "NAME_FIRST": "Temporary",
            "NAME_LAST": "TestRecord",
            "PHONE_NUMBER": "555-0003",
            "EMAIL_ADDRESS": "temp.test3@example.com",
            "ADDR_FULL": "789 Temporary Lane, Test City, TX 12347"
        }).to_string(),
    );

    records
}
