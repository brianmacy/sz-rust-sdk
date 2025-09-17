//! Search Records Example
//!
//! This example demonstrates how to search for entities using various search criteria
//! and process the JSON results. It shows both simple searches and searches with
//! detailed entity information.
//!
//! Rust equivalent of: searching/SearchRecords/Program.cs

use sz_rust_sdk::prelude::*;
use serde_json::Value;

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

    println!("Searching for records in Senzing repository...");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Perform various types of searches
    search_by_attributes(&engine)?;
    search_with_entity_details(&engine)?;

    println!("✅ All searches completed successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Search for entities using specific attributes
fn search_by_attributes(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    println!("\n--- Searching by Attributes ---");

    // Search criteria as JSON
    let search_attributes = r#"{
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith"
    }"#;

    println!("Searching for: {}", search_attributes);

    // Perform the search with no flags (basic search)
    let search_results = engine.search_by_attributes(
        search_attributes,
        None,
        None,
    )?;

    // Parse and display results
    let results: Value = serde_json::from_str(&search_results)
        .map_err(|e| SzError::unknown(&format!("Failed to parse search results: {}", e)))?;

    if let Some(resolved_entities) = results.get("RESOLVED_ENTITIES").and_then(|v| v.as_array()) {
        println!("Found {} matching entities:", resolved_entities.len());

        for (index, entity) in resolved_entities.iter().enumerate() {
            if let Some(entity_id) = entity.get("ENTITY_ID").and_then(|v| v.as_i64()) {
                println!("  {}. Entity ID: {}", index + 1, entity_id);

                // Show entity name if available
                if let Some(entity_name) = entity.get("ENTITY_NAME").and_then(|v| v.as_str()) {
                    println!("     Name: {}", entity_name);
                }

                // Show match score if available
                if let Some(match_score) = entity.get("MATCH_SCORE").and_then(|v| v.as_i64()) {
                    println!("     Match Score: {}", match_score);
                }
            }
        }
    } else {
        println!("No entities found matching the search criteria.");
    }

    Ok(())
}

/// Search with detailed entity information
fn search_with_entity_details(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    println!("\n--- Searching with Entity Details ---");

    // Search for entities with email addresses
    let search_attributes = r#"{
        "EMAIL_ADDRESS": "john.smith@example.com"
    }"#;

    println!("Searching for: {}", search_attributes);

    // Perform search with entity details flags
    let search_results = engine.search_by_attributes(
        search_attributes,
        None,
        Some(SzFlags::SEARCH_BY_ATTRIBUTES_ALL | SzFlags::ENTITY_INCLUDE_RECORD_DATA),
    )?;

    // Parse and display detailed results
    let results: Value = serde_json::from_str(&search_results)
        .map_err(|e| SzError::unknown(&format!("Failed to parse detailed search results: {}", e)))?;

    if let Some(resolved_entities) = results.get("RESOLVED_ENTITIES").and_then(|v| v.as_array()) {
        println!("Found {} entities with detailed information:", resolved_entities.len());

        for (index, entity) in resolved_entities.iter().enumerate() {
            println!("  Entity #{}:", index + 1);

            if let Some(entity_id) = entity.get("ENTITY_ID").and_then(|v| v.as_i64()) {
                println!("    Entity ID: {}", entity_id);
            }

            // Display records within the entity
            if let Some(records) = entity.get("RECORDS").and_then(|v| v.as_array()) {
                println!("    Records ({}):", records.len());
                for record in records {
                    if let Some(data_source) = record.get("DATA_SOURCE").and_then(|v| v.as_str()) {
                        if let Some(record_id) = record.get("RECORD_ID").and_then(|v| v.as_str()) {
                            println!("      - {}: {}", data_source, record_id);
                        }
                    }
                }
            }

            // Display features if available
            if let Some(features) = entity.get("FEATURES") {
                display_entity_features(features);
            }
        }
    } else {
        println!("No entities found with the specified email address.");
    }

    Ok(())
}

/// Display entity features in a readable format
fn display_entity_features(features: &Value) {
    if let Some(features_obj) = features.as_object() {
        println!("    Features:");

        for (feature_type, feature_list) in features_obj {
            if let Some(features_array) = feature_list.as_array() {
                println!("      {}:", feature_type);
                for feature in features_array {
                    if let Some(feat_desc) = feature.get("FEAT_DESC").and_then(|v| v.as_str()) {
                        print!("        - {}", feat_desc);

                        // Add usage type if available
                        if let Some(usage_type) = feature.get("USAGE_TYPE").and_then(|v| v.as_str()) {
                            print!(" ({})", usage_type);
                        }

                        println!();
                    }
                }
            }
        }
    }
}
