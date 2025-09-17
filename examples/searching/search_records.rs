//! Search Records Example
//!
//! This example demonstrates how to search for entities by attributes
//! using the Senzing search functionality.
//!
//! Run with: cargo run --example search_records

use serde_json::Value;
use sz_rust_sdk::prelude::*;

/// Get sample search criteria for testing different search scenarios
fn get_search_criteria() -> Vec<String> {
    vec![
        // Search by name
        r#"{
            "NAME_FULL": "Robert Smith"
        }"#
        .to_string(),
        // Search by name and phone
        r#"{
            "NAME_FULL": "John Doe",
            "PHONE_NUMBER": "555-9876"
        }"#
        .to_string(),
        // Search by email
        r#"{
            "EMAIL_ADDRESS": "alice.johnson@email.com"
        }"#
        .to_string(),
        // Search by address
        r#"{
            "ADDR_FULL": "123 Main Street, Anytown, ST 12345"
        }"#
        .to_string(),
        // Search by partial name and birth date
        r#"{
            "NAME_FULL": "Alice",
            "DATE_OF_BIRTH": "1985-03-15"
        }"#
        .to_string(),
        // Search by phone number only
        r#"{
            "PHONE_NUMBER": "555-1212"
        }"#
        .to_string(),
        // Search that should return no results
        r#"{
            "NAME_FULL": "Nonexistent Person",
            "EMAIL_ADDRESS": "nobody@nowhere.com"
        }"#
        .to_string(),
    ]
}

fn print_search_results(criteria: &str, result_json: &str) -> SzResult<()> {
    let result: Value = serde_json::from_str(result_json)?;

    println!("Search criteria: {}", criteria);

    if let Some(resolved_entities) = result.get("RESOLVED_ENTITIES")
        && let Some(entities_array) = resolved_entities.as_array()
    {
        if entities_array.is_empty() {
            println!("  No results found");
        } else {
            println!("  Found {} result(s):", entities_array.len());

            for (index, entity_obj) in entities_array.iter().enumerate() {
                if let Some(entity) = entity_obj.get("ENTITY") {
                    let entity_id = entity
                        .get("RESOLVED_ENTITY")
                        .and_then(|re| re.get("ENTITY_ID"))
                        .and_then(|id| id.as_i64())
                        .unwrap_or(0);

                    let entity_name = entity
                        .get("RESOLVED_ENTITY")
                        .and_then(|re| re.get("ENTITY_NAME"))
                        .and_then(|name| name.as_str())
                        .unwrap_or("Unknown");

                    println!(
                        "    {}. Entity ID: {}, Name: {}",
                        index + 1,
                        entity_id,
                        entity_name
                    );

                    // Print match score if available
                    if let Some(match_info) = entity_obj.get("MATCH_INFO")
                        && let Some(match_score) = match_info.get("MATCH_SCORE")
                    {
                        println!("       Match Score: {}", match_score);
                    }

                    // Print some record details
                    if let Some(records) = entity
                        .get("RESOLVED_ENTITY")
                        .and_then(|re| re.get("RECORDS"))
                        && let Some(records_array) = records.as_array()
                    {
                        println!("       Records: {} record(s)", records_array.len());
                        for record in records_array.iter().take(3) {
                            // Show first 3 records
                            let data_source = record
                                .get("DATA_SOURCE")
                                .and_then(|ds| ds.as_str())
                                .unwrap_or("Unknown");
                            let record_id = record
                                .get("RECORD_ID")
                                .and_then(|id| id.as_str())
                                .unwrap_or("Unknown");
                            println!("         - {}: {}", data_source, record_id);
                        }
                    }
                }
            }
        }
    }

    println!();
    Ok(())
}

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-search-records")?;

    // Get the engine from the environment
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("Searching for entities by attributes...\n");

    let search_criteria = get_search_criteria();
    let mut searches_performed = 0;

    // Loop through the search criteria and perform searches
    for criteria in &search_criteria {
        // Call the search_by_attributes() function with default flags
        let result = engine.search_by_attributes(
            criteria,
            None, // No specific search profile
            Some(SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT),
        )?;

        // Print the search results
        if let Err(e) = print_search_results(criteria, &result) {
            println!("Warning: Could not parse search results: {}", e);
            println!("Raw result: {}", result);
        }

        searches_performed += 1;
    }

    println!("Completed {} searches", searches_performed);

    // Demonstrate search with specific search profile (if supported)
    println!("\n--- Advanced Search Example ---");
    let advanced_criteria = r#"{
        "NAME_FULL": "Smith",
        "ADDR_STATE": "ST"
    }"#;

    let advanced_result = engine.search_by_attributes(
        advanced_criteria,
        Some("SEARCH"), // Use a specific search profile
        Some(SzFlags::SEARCH_BY_ATTRIBUTES_ALL),
    )?;

    print_search_results(advanced_criteria, &advanced_result)?;

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_records() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Search records should succeed");
        }
    }

    #[test]
    fn test_get_search_criteria() {
        let criteria = get_search_criteria();
        assert!(!criteria.is_empty(), "Should have search criteria");
        assert!(criteria.len() >= 5, "Should have multiple search scenarios");

        // Verify all criteria are valid JSON
        for criterion in &criteria {
            let parsed: Result<Value, _> = serde_json::from_str(criterion);
            assert!(
                parsed.is_ok(),
                "Search criteria should be valid JSON: {}",
                criterion
            );
        }
    }

    #[test]
    fn test_print_search_results() {
        let sample_result = r#"{
            "RESOLVED_ENTITIES": [
                {
                    "ENTITY": {
                        "RESOLVED_ENTITY": {
                            "ENTITY_ID": 123,
                            "ENTITY_NAME": "Test Entity"
                        }
                    }
                }
            ]
        }"#;

        let result = print_search_results("test criteria", sample_result);
        assert!(result.is_ok(), "Should parse sample search results");
    }
}
