//! Why Search Example
//!
//! This example demonstrates how to use the "why" functionality to understand
//! why a particular entity was returned as a search result.
//!
//! Run with: cargo run --example why_search

use serde_json::Value;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-search")?;

    // Get the engine from the environment
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("Demonstrating 'Why Search' functionality...\n");

    // First, perform a search to get some entities
    let search_criteria = r#"{
        "NAME_FULL": "Robert Smith",
        "PHONE_NUMBER": "555-1212"
    }"#;

    println!("1. Performing initial search:");
    println!("   Criteria: {}", search_criteria);

    let search_result = engine.search_by_attributes(
        search_criteria,
        None,
        Some(SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT),
    )?;

    // Parse the search results to get entity IDs
    let search_json: Value = serde_json::from_str(&search_result)?;
    let mut entity_ids = Vec::new();

    if let Some(resolved_entities) = search_json.get("RESOLVED_ENTITIES")
        && let Some(entities_array) = resolved_entities.as_array()
    {
        println!("   Found {} entities", entities_array.len());

        for (index, entity_obj) in entities_array.iter().enumerate() {
            if let Some(entity) = entity_obj.get("ENTITY")
                && let Some(entity_id) = entity
                    .get("RESOLVED_ENTITY")
                    .and_then(|re| re.get("ENTITY_ID"))
                    .and_then(|id| id.as_i64())
            {
                entity_ids.push(entity_id);

                let entity_name = entity
                    .get("RESOLVED_ENTITY")
                    .and_then(|re| re.get("ENTITY_NAME"))
                    .and_then(|name| name.as_str())
                    .unwrap_or("Unknown");

                println!(
                    "     {}. Entity ID: {}, Name: {}",
                    index + 1,
                    entity_id,
                    entity_name
                );
            }
        }
    }

    // Now demonstrate "why search" for each found entity
    if !entity_ids.is_empty() {
        println!("\n2. Analyzing why each entity was returned:\n");

        for entity_id in entity_ids {
            println!("--- Why Analysis for Entity ID: {} ---", entity_id);

            match engine.why_search(
                search_criteria,
                entity_id,
                None, // No specific search profile
                Some(SzFlags::WHY_ENTITY_DEFAULT),
            ) {
                Ok(why_result) => {
                    // Parse and display the why analysis
                    match serde_json::from_str::<Value>(&why_result) {
                        Ok(why_json) => {
                            print_why_analysis(&why_json)?;
                        }
                        Err(e) => {
                            println!("Could not parse why analysis: {}", e);
                            println!("Raw result: {}", why_result);
                        }
                    }
                }
                Err(e) => {
                    println!("Why search analysis failed: {}", e);
                }
            }

            println!();
        }
    } else {
        println!("No entities found to analyze");

        // Demonstrate why search with a hypothetical entity ID
        println!("\n2. Demonstrating why search with example entity ID:");
        let example_entity_id = 1;

        match engine.why_search(
            search_criteria,
            example_entity_id,
            None,
            Some(SzFlags::WHY_ENTITY_DEFAULT),
        ) {
            Ok(why_result) => {
                println!(
                    "Why analysis for entity {}: {}",
                    example_entity_id, why_result
                );
            }
            Err(e) => {
                println!(
                    "Why search failed (expected if entity doesn't exist): {}",
                    e
                );
            }
        }
    }

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

fn print_why_analysis(why_json: &Value) -> SzResult<()> {
    // Extract key information from the why analysis
    if let Some(why_results) = why_json.get("WHY_RESULTS")
        && let Some(results_array) = why_results.as_array()
    {
        for result in results_array {
            if let Some(entity_id) = result.get("ENTITY_ID") {
                println!("Entity ID: {}", entity_id);
            }

            if let Some(match_info) = result.get("MATCH_INFO") {
                println!("Match Information:");

                if let Some(why_key) = match_info.get("WHY_KEY") {
                    println!("  Why Key: {}", why_key);
                }

                if let Some(why_errule_code) = match_info.get("WHY_ERRULE_CODE") {
                    println!("  Rule Code: {}", why_errule_code);
                }

                if let Some(match_level_code) = match_info.get("MATCH_LEVEL_CODE") {
                    println!("  Match Level: {}", match_level_code);
                }

                // Print feature matches
                if let Some(feature_scores) = match_info.get("FEATURE_SCORES")
                    && let Some(scores_obj) = feature_scores.as_object()
                {
                    println!("  Feature Matches:");
                    for (feature_type, score_info) in scores_obj {
                        if let Some(score) = score_info.as_i64() {
                            println!("    {}: {}", feature_type, score);
                        }
                    }
                }
            }
        }
    }

    // Print candidate keys if available
    if let Some(candidate_keys) = why_json.get("CANDIDATE_KEYS")
        && let Some(keys_obj) = candidate_keys.as_object()
        && !keys_obj.is_empty()
    {
        println!("Candidate Keys Used:");
        for (key_type, key_info) in keys_obj {
            println!("  {}: {:?}", key_type, key_info);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_why_search() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Why search should succeed");
        }
    }

    #[test]
    fn test_print_why_analysis() {
        let sample_why = serde_json::json!({
            "WHY_RESULTS": [
                {
                    "ENTITY_ID": 123,
                    "MATCH_INFO": {
                        "WHY_KEY": "test-key",
                        "WHY_ERRULE_CODE": "SF1",
                        "MATCH_LEVEL_CODE": "RESOLVED",
                        "FEATURE_SCORES": {
                            "NAME": 100,
                            "PHONE": 90
                        }
                    }
                }
            ]
        });

        let result = print_why_analysis(&sample_why);
        assert!(result.is_ok(), "Should parse sample why analysis");
    }
}
