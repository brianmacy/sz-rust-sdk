//! Load records into the Senzing repository for entity resolution

use std::collections::HashMap;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Get sample records for loading into the repository
fn get_records() -> HashMap<(String, String), String> {
    let mut records = HashMap::new();

    // Customer records
    records.insert(
        ("TEST".to_string(), "1001".to_string()),
        r#"{
            "NAME_FULL": "Robert Smith",
            "EMAIL_ADDRESS": "rsmith@example.com",
            "PHONE_NUMBER": "555-1212",
            "ADDR_FULL": "123 Main Street, Anytown, ST 12345"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "1002".to_string()),
        r#"{
            "NAME_FULL": "Bob J Smith",
            "EMAIL_ADDRESS": "bsmith@example.com",
            "PHONE_NUMBER": "555-1212",
            "ADDR_FULL": "123 Main St, Anytown, ST 12345"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "1003".to_string()),
        r#"{
            "NAME_FULL": "John Doe",
            "EMAIL_ADDRESS": "jdoe@example.com",
            "PHONE_NUMBER": "555-9876",
            "ADDR_FULL": "456 Oak Avenue, Another City, ST 67890"
        }"#
        .to_string(),
    );

    // Employee records
    records.insert(
        ("TEST".to_string(), "E1001".to_string()),
        r#"{
            "NAME_FULL": "Robert Smith",
            "EMAIL_ADDRESS": "robert.smith@company.com",
            "PHONE_NUMBER": "555-1212",
            "ADDR_FULL": "123 Main Street, Anytown, ST 12345",
            "EMPLOYEE_ID": "E1001"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "E1002".to_string()),
        r#"{
            "NAME_FULL": "Jane Johnson",
            "EMAIL_ADDRESS": "jane.johnson@company.com",
            "PHONE_NUMBER": "555-5555",
            "ADDR_FULL": "789 Pine Road, Different Town, ST 11111",
            "EMPLOYEE_ID": "E1002"
        }"#
        .to_string(),
    );

    records
}

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-load-records")?;

    // Get the engine from the environment
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("Loading records into Senzing repository...\n");

    // Loop through the example records and add them to the repository
    let records = get_records();
    let mut loaded_count = 0;

    for ((data_source_code, record_id), record_definition) in records.iter() {
        // Call the add_record() function with default flags
        let result = engine.add_record(
            data_source_code,
            record_id,
            record_definition,
            Some(SzFlags::ADD_RECORD_DEFAULT_FLAGS),
        )?;

        println!("Record {} from {} added", record_id, data_source_code);

        // Optionally print the resolution result
        if !result.is_empty() {
            println!("  Resolution info: {}", result);
        }

        loaded_count += 1;
    }

    println!("\nSuccessfully loaded {} records", loaded_count);

    // Get updated engine statistics
    let stats = engine.get_stats()?;
    println!("\nEngine statistics after loading:");
    println!("{}", stats);

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_loading() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Record loading should succeed");
        }
    }

    #[test]
    fn test_get_records() {
        let records = get_records();
        assert!(!records.is_empty(), "Should have sample records");
        assert!(records.len() >= 3, "Should have multiple records");

        // Verify we have records from different data sources
        let has_customers = records.keys().any(|(ds, _)| ds == "TEST");
        let has_employees = records.keys().any(|(ds, _)| ds == "TEST");
        assert!(
            has_customers && has_employees,
            "Should have records from multiple data sources"
        );
    }
}
