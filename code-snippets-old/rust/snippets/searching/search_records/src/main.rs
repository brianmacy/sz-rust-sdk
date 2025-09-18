//! Search Records Example
//!
//! This example demonstrates how to search for entities in Senzing.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Environment initialization
//! - Adding records for searching
//! - Searching with search_by_attributes()
//! - Processing JSON search results
//! - Using search flags for different result formats

use sz_rust_sdk::prelude::*;
use serde_json::{json, Value};

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment
    let env = get_environment()?;

    // Step 2: Get the engine for data operations
    let engine = env.get_engine()?;

    println!("Demonstrating entity search...");

    // Step 3: Load some test records to search for
    // Using "TEST" data source which is always available
    engine.add_record("TEST", "SEARCH_1", &json!({
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith",
        "PHONE_NUMBER": "555-1234"
    }).to_string(), None)?;

    engine.add_record("TEST", "SEARCH_2", &json!({
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith",
        "EMAIL_ADDRESS": "john.smith@company.com"
    }).to_string(), None)?;
    println!("✓ Added test records");

    // Step 4: Search for entities by name
    let search_criteria = json!({
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith"
    }).to_string();

    // search_by_attributes(search_json, flags)
    let search_results = engine.search_by_attributes(&search_criteria, None)?;
    println!("✓ Search completed");

    // Step 5: Process the search results
    if let Ok(json_result) = serde_json::from_str::<Value>(&search_results) {
        if let Some(entities) = json_result["RESOLVED_ENTITIES"].as_array() {
            println!("Found {} matching entities:", entities.len());

            for (i, entity) in entities.iter().enumerate() {
                if let Some(entity_id) = entity["ENTITY"]["RESOLVED_ENTITY"]["ENTITY_ID"].as_i64() {
                    println!("  Entity {}: ID = {}", i + 1, entity_id);
                }
            }
        } else {
            println!("No entities found matching the search criteria");
        }
    }

    println!("✅ Search demonstration complete");

    Ok(())
}

/// Simple helper to get a configured Senzing environment
/// Handles database setup and configuration automatically
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("search_records_example")
}