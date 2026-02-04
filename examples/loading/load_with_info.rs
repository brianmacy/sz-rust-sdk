//! Load Records with Information Example
//!
//! This example demonstrates how to load records and capture detailed
//! information about the entity resolution process.
//!
//! Run with: cargo run --example load_with_info

use serde_json::Value;
use std::collections::HashMap;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Get sample records for loading with detailed tracking
fn get_records_with_info() -> HashMap<(String, String), String> {
    let mut records = HashMap::new();

    records.insert(
        ("TEST".to_string(), "C001".to_string()),
        r#"{
            "NAME_FULL": "Alice Johnson",
            "EMAIL_ADDRESS": "alice.johnson@email.com",
            "PHONE_NUMBER": "555-0001",
            "ADDR_FULL": "100 First Street, City A, ST 10001",
            "DATE_OF_BIRTH": "1985-03-15"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "C002".to_string()),
        r#"{
            "NAME_FULL": "A. Johnson",
            "EMAIL_ADDRESS": "ajohnson@email.com",
            "PHONE_NUMBER": "555-0001",
            "ADDR_FULL": "100 1st St, City A, ST 10001",
            "DATE_OF_BIRTH": "1985-03-15"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "EMP001".to_string()),
        r#"{
            "NAME_FULL": "Alice M Johnson",
            "EMAIL_ADDRESS": "alice.johnson@company.com",
            "PHONE_NUMBER": "555-0001",
            "ADDR_FULL": "100 First Street, City A, ST 10001",
            "EMPLOYEE_ID": "EMP001"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "W001".to_string()),
        r#"{
            "NAME_FULL": "Robert Thompson",
            "DATE_OF_BIRTH": "1975-12-10",
            "PASSPORT_NUMBER": "X12345678",
            "WATCHLIST_TYPE": "SANCTIONS"
        }"#
        .to_string(),
    );

    records
}

fn print_resolution_info(record_id: &str, data_source: &str, info_json: &str) -> SzResult<()> {
    // Parse the JSON response to extract meaningful information
    let info: Value = serde_json::from_str(info_json)?;

    println!("Resolution details for {record_id} from {data_source}:");

    // Check for affected entities
    if let Some(affected_entities) = info.get("AFFECTED_ENTITIES")
        && let Some(entities_array) = affected_entities.as_array()
    {
        println!("  Affected entities: {}", entities_array.len());
        for (i, entity) in entities_array.iter().enumerate() {
            if let Some(entity_id) = entity.get("ENTITY_ID") {
                println!("    Entity {}: {}", i + 1, entity_id);
            }
        }
    }

    // Check for interesting entities (potential matches)
    if let Some(interesting) = info.get("INTERESTING_ENTITIES")
        && let Some(interesting_array) = interesting.get("ENTITIES")
        && let Some(entities) = interesting_array.as_array()
        && !entities.is_empty()
    {
        println!("  Interesting entities found: {}", entities.len());
        for entity in entities {
            if let Some(entity_id) = entity.get("ENTITY_ID") {
                println!("    Potential match: {entity_id}");
            }
        }
    }

    println!();
    Ok(())
}

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-load-with-info")?;

    // Get the engine from the environment
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("Loading records with detailed resolution information...\n");

    let records = get_records_with_info();
    let mut loaded_count = 0;

    for ((data_source_code, record_id), record_definition) in records.iter() {
        // Use flags to get detailed information about the resolution process
        let flags = SzFlags::ADD_RECORD_DEFAULT_FLAGS;

        let result =
            engine.add_record(data_source_code, record_id, record_definition, Some(flags))?;

        println!("✓ Loaded record {record_id} from {data_source_code}");

        // Print detailed resolution information if available
        if !result.is_empty()
            && let Err(e) = print_resolution_info(record_id, data_source_code, &result)
        {
            println!("  Warning: Could not parse resolution info: {e}");
        }

        loaded_count += 1;
    }

    println!("Successfully loaded {loaded_count} records with detailed tracking");

    // Get final engine statistics
    let stats = engine.get_stats()?;
    println!("\nFinal engine statistics:");
    println!("{stats}");

    // Clean up the test database
    ExampleEnvironment::cleanup(env)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_with_info() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Loading with info should succeed");
        }
    }

    #[test]
    fn test_get_records_with_info() {
        let records = get_records_with_info();
        assert!(!records.is_empty(), "Should have sample records");

        // Verify we have records that should potentially resolve together
        let customer_count = records.keys().filter(|(ds, _)| ds == "TEST").count();
        assert!(
            customer_count >= 2,
            "Should have multiple customer records for resolution testing"
        );
    }
}
