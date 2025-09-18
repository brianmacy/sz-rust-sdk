#![allow(clippy::borrowed_box)]
//! Why Search Example
//!
//! This example demonstrates the "why" functionality which explains why certain
//! entities were or were not returned in search results. It provides detailed
//! analysis of scoring and matching logic.
//!
//! Rust equivalent of: searching/WhySearch/Program.cs

use serde_json::Value;
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

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

    println!("Demonstrating 'Why Search' functionality...");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Perform search and explain results
    perform_why_search_analysis(&engine)?;

    println!("✅ Why search analysis completed successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Perform search and analyze why entities were returned
fn perform_why_search_analysis(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    println!("\n--- Performing Search with 'Why' Analysis ---");

    // Search criteria
    let search_attributes = r#"{
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith",
        "PHONE_NUMBER": "555-1234"
    }"#;

    println!("Search criteria: {}", search_attributes);

    // First, perform a regular search to get entities
    let search_results = engine.search_by_attributes(
        search_attributes,
        None,
        Some(SzFlags::SEARCH_BY_ATTRIBUTES_ALL),
    )?;

    let results: Value = serde_json::from_str(&search_results)
        .map_err(|e| SzError::unknown(format!("Failed to parse search results: {}", e)))?;

    if let Some(resolved_entities) = results.get("RESOLVED_ENTITIES").and_then(|v| v.as_array()) {
        if resolved_entities.is_empty() {
            println!("No entities found to analyze.");
            return Ok(());
        }

        println!(
            "Found {} entities. Analyzing why they matched...",
            resolved_entities.len()
        );

        // Analyze each entity
        for (index, entity) in resolved_entities.iter().enumerate() {
            if let Some(entity_id) = entity.get("ENTITY_ID").and_then(|v| v.as_i64()) {
                println!(
                    "\n--- Analysis for Entity {} (ID: {}) ---",
                    index + 1,
                    entity_id
                );

                // Get why information for this entity
                analyze_why_entity_matched(engine, entity_id, search_attributes)?;
            }
        }
    } else {
        println!("No entities found in search results.");
    }

    Ok(())
}

/// Analyze why a specific entity matched the search criteria
fn analyze_why_entity_matched(
    engine: &Box<dyn SzEngine>,
    entity_id: i64,
    search_attributes: &str,
) -> SzResult<()> {
    // Get why records information
    let why_results = engine.why_search(
        search_attributes,
        entity_id,
        None,
        Some(SzFlags::WHY_ENTITIES_DEFAULT_FLAGS),
    )?;

    let why_info: Value = serde_json::from_str(&why_results)
        .map_err(|e| SzError::unknown(format!("Failed to parse why results: {}", e)))?;

    // Display why information
    if let Some(why_results_array) = why_info.get("WHY_RESULTS").and_then(|v| v.as_array()) {
        for why_result in why_results_array {
            if let Some(entity_info) = why_result.get("ENTITY") {
                display_entity_match_info(entity_info);
            }

            if let Some(match_info) = why_result.get("MATCH_INFO") {
                display_match_analysis(match_info);
            }
        }
    }

    // Additionally, get why records information to understand record-level matching
    get_why_records_analysis(engine, entity_id)?;

    Ok(())
}

/// Display entity match information
fn display_entity_match_info(entity_info: &Value) {
    println!("Entity Information:");

    if let Some(resolved_entity) = entity_info.get("RESOLVED_ENTITY") {
        if let Some(entity_name) = resolved_entity.get("ENTITY_NAME").and_then(|v| v.as_str()) {
            println!("  Entity Name: {}", entity_name);
        }

        if let Some(records) = resolved_entity.get("RECORDS").and_then(|v| v.as_array()) {
            println!("  Records in Entity:");
            for record in records {
                if let Some(data_source) = record.get("DATA_SOURCE").and_then(|v| v.as_str()) {
                    if let Some(record_id) = record.get("RECORD_ID").and_then(|v| v.as_str()) {
                        println!("    - {}: {}", data_source, record_id);
                    }
                }
            }
        }
    }
}

/// Display detailed match analysis
fn display_match_analysis(match_info: &Value) {
    println!("Match Analysis:");

    if let Some(match_score) = match_info.get("MATCH_SCORE").and_then(|v| v.as_i64()) {
        println!("  Overall Match Score: {}", match_score);
    }

    if let Some(feature_scores) = match_info.get("FEATURE_SCORES").and_then(|v| v.as_object()) {
        println!("  Feature Scores:");
        for (feature_type, score_info) in feature_scores {
            if let Some(score) = score_info.get("SCORE").and_then(|v| v.as_i64()) {
                println!("    {}: {}", feature_type, score);

                // Show individual features that contributed to the score
                if let Some(candidate_features) = score_info
                    .get("CANDIDATE_FEATURES")
                    .and_then(|v| v.as_array())
                {
                    for feature in candidate_features {
                        if let Some(feat_desc) = feature.get("FEAT_DESC").and_then(|v| v.as_str()) {
                            print!("      - {}", feat_desc);

                            if let Some(score) = feature.get("FULL_SCORE").and_then(|v| v.as_i64())
                            {
                                print!(" (score: {})", score);
                            }

                            println!();
                        }
                    }
                }
            }
        }
    }

    // Display disclosure information if available
    if let Some(disclosed_relations) = match_info
        .get("DISCLOSED_RELATIONS")
        .and_then(|v| v.as_array())
    {
        if !disclosed_relations.is_empty() {
            println!("  Disclosed Relations:");
            for relation in disclosed_relations {
                if let Some(rel_type) = relation.get("REL_TYPE").and_then(|v| v.as_str()) {
                    println!("    - Relationship Type: {}", rel_type);
                }
            }
        }
    }
}

/// Get additional why records analysis
fn get_why_records_analysis(engine: &Box<dyn SzEngine>, entity_id: i64) -> SzResult<()> {
    println!("\n--- Record-level Analysis ---");

    // Get entity details to analyze individual records
    let entity_details = engine.get_entity(entity_id, Some(SzFlags::ENTITY_INCLUDE_RECORD_DATA))?;

    let entity_data: Value = serde_json::from_str(&entity_details)
        .map_err(|e| SzError::unknown(format!("Failed to parse entity details: {}", e)))?;

    if let Some(resolved_entity) = entity_data.get("RESOLVED_ENTITY") {
        if let Some(records) = resolved_entity.get("RECORDS").and_then(|v| v.as_array()) {
            println!("Analyzing {} records in entity:", records.len());

            for record in records {
                if let Some(data_source) = record.get("DATA_SOURCE").and_then(|v| v.as_str()) {
                    if let Some(record_id) = record.get("RECORD_ID").and_then(|v| v.as_str()) {
                        println!("  Record {}: {}", data_source, record_id);

                        // Show key features for this record
                        if let Some(features) = record.get("FEATURES") {
                            display_record_features(features);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Display features for a specific record
fn display_record_features(features: &Value) {
    if let Some(features_obj) = features.as_object() {
        for (feature_type, feature_list) in features_obj {
            if let Some(features_array) = feature_list.as_array() {
                for feature in features_array {
                    if let Some(feat_desc) = feature.get("FEAT_DESC").and_then(|v| v.as_str()) {
                        print!("    - {}: {}", feature_type, feat_desc);

                        if let Some(usage_type) = feature.get("USAGE_TYPE").and_then(|v| v.as_str())
                        {
                            print!(" ({})", usage_type);
                        }

                        println!();
                    }
                }
            }
        }
    }
}
