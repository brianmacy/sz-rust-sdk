//! Force Unresolve Example
//!
//! This example demonstrates how to force records to be unresolved from an entity
//! when they were automatically resolved but shouldn't have been. It shows
//! stewardship operations where a data steward manually separates records.
//!
//! Rust equivalent of: stewardship/ForceUnresolve/Program.cs

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

    println!("Force Unresolve Example");
    println!("=======================");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Load records that are likely to automatically resolve together
    println!("\n1. Loading records that will likely resolve together...");

    let record1 = r#"{
        "RECORD_ID": "FORCE_UNRESOLVE_1",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "Robert",
        "NAME_LAST": "Johnson",
        "ADDR_FULL": "789 Pine St, Anytown, NY 10001",
        "EMAIL_ADDRESS": "robert.johnson@example.com",
        "PHONE_NUMBER": "555-987-6543"
    }"#;

    let record2 = r#"{
        "RECORD_ID": "FORCE_UNRESOLVE_2",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "Bob",
        "NAME_LAST": "Johnson",
        "ADDR_FULL": "789 Pine Street, Anytown, NY 10001",
        "EMAIL_ADDRESS": "robert.johnson@example.com"
    }"#;

    let record3 = r#"{
        "RECORD_ID": "FORCE_UNRESOLVE_3",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "Robert",
        "NAME_LAST": "Johnson",
        "PHONE_NUMBER": "555-987-6543",
        "DATE_OF_BIRTH": "1985-03-15"
    }"#;

    // Load the records with info to track resolution
    let load_flags = SzFlags::ADD_RECORD_DEFAULT;
    let info1 = engine.add_record("TEST", "FORCE_UNRESOLVE_1", record1, Some(load_flags))?;
    let info2 = engine.add_record("TEST", "FORCE_UNRESOLVE_2", record2, Some(load_flags))?;
    let info3 = engine.add_record("TEST", "FORCE_UNRESOLVE_3", record3, Some(load_flags))?;

    println!("Loaded record 1 with info: {}", info1);
    println!("Loaded record 2 with info: {}", info2);
    println!("Loaded record 3 with info: {}", info3);

    // Parse the load info to get entity IDs
    let entity_id1 = extract_entity_id(&info1, "record 1")?;
    let entity_id2 = extract_entity_id(&info2, "record 2")?;
    let entity_id3 = extract_entity_id(&info3, "record 3")?;

    println!("\n2. Entity IDs after loading:");
    println!("Record 1 resolved to entity: {}", entity_id1);
    println!("Record 2 resolved to entity: {}", entity_id2);
    println!("Record 3 resolved to entity: {}", entity_id3);

    // Find the entity with the most records (likely all resolved together)
    let primary_entity_id = if entity_id1 == entity_id2 && entity_id2 == entity_id3 {
        println!(
            "\n✅ All records resolved to the same entity ({})",
            entity_id1
        );
        entity_id1
    } else if entity_id1 == entity_id2 {
        println!(
            "\n✅ Records 1 and 2 resolved to the same entity ({})",
            entity_id1
        );
        entity_id1
    } else if entity_id1 == entity_id3 {
        println!(
            "\n✅ Records 1 and 3 resolved to the same entity ({})",
            entity_id1
        );
        entity_id1
    } else if entity_id2 == entity_id3 {
        println!(
            "\n✅ Records 2 and 3 resolved to the same entity ({})",
            entity_id2
        );
        entity_id2
    } else {
        println!("\n⚠️  All records resolved to different entities");
        println!("Force unresolve works best when records are resolved together");
        entity_id1 // Use first entity for demonstration
    };

    // Show the entity before unresolving
    println!("\n3. Entity details before force unresolve:");
    let entity_json = engine.get_entity(primary_entity_id.into(), None)?;
    println!("Entity: {}", get_entity_summary(&entity_json)?);

    // Demonstrate force unresolve by deleting and reloading a record
    // with different characteristics to discourage resolution
    println!("\n4. Performing force unresolve operation...");

    // Method 1: Delete a record and reload with modified data to discourage resolution
    println!("Deleting record 2 to separate it from the entity...");
    let delete_flags = SzFlags::DELETE_RECORD_DEFAULT;
    let delete_info = engine.delete_record("TEST", "FORCE_UNRESOLVE_2", Some(delete_flags))?;
    println!("Delete info: {}", delete_info);

    // Reload the record with slightly different data to discourage automatic resolution
    let modified_record2 = r#"{
        "RECORD_ID": "FORCE_UNRESOLVE_2",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "Bobby",
        "NAME_LAST": "Johnson",
        "ADDR_FULL": "456 Different St, Otherville, NY 10002",
        "EMAIL_ADDRESS": "bobby.johnson@different.com"
    }"#;

    println!("Reloading record 2 with modified data to encourage separation...");
    let reload_info = engine.add_record(
        "TEST",
        "FORCE_UNRESOLVE_2",
        modified_record2,
        Some(load_flags),
    )?;
    println!("Reload info: {}", reload_info);

    let new_entity_id2 = extract_entity_id(&reload_info, "reloaded record 2")?;

    // Show the results
    println!("\n5. Results after force unresolve:");
    println!(
        "Original entity {} (should have fewer records now)",
        primary_entity_id
    );
    println!(
        "New/separate entity {} for the unresolved record",
        new_entity_id2
    );

    // Get updated entity details
    let updated_entity1 = engine.get_entity(primary_entity_id.into(), None)?;
    let new_entity2 = engine.get_entity(new_entity_id2.into(), None)?;

    println!(
        "Updated original entity: {}",
        get_entity_summary(&updated_entity1)?
    );
    println!("New separate entity: {}", get_entity_summary(&new_entity2)?);

    // Show why analysis to understand the separation
    println!("\n6. Why analysis for the unresolve operation:");
    if primary_entity_id != new_entity_id2 {
        let why_flags = SzFlags::WHY_ENTITY_DEFAULT;
        let why_result = engine.why_entities(primary_entity_id, new_entity_id2, Some(why_flags))?;
        println!("Why entities result: {}", why_result);
    } else {
        println!("Records are still resolved together - separation didn't occur");
    }

    // Alternative method: Use reevaluate to force reconsideration
    println!("\n7. Alternative: Using reevaluate to force reconsideration...");
    let reevaluate_flags = SzFlags::REEVALUATE_ENTITY_DEFAULT;
    let reevaluate_info = engine.reevaluate_entity(primary_entity_id, Some(reevaluate_flags))?;
    println!("Reevaluate info: {}", reevaluate_info);

    // Final entity state
    println!("\n8. Final entity states:");
    let final_entity1 = engine.get_entity(primary_entity_id.into(), None)?;
    let final_entity2 = engine.get_entity(new_entity_id2.into(), None)?;

    println!(
        "Final original entity: {}",
        get_entity_summary(&final_entity1)?
    );
    println!(
        "Final separate entity: {}",
        get_entity_summary(&final_entity2)?
    );

    // Clean up test records
    println!("\n9. Cleaning up test records...");
    let _cleanup1 = engine.delete_record("TEST", "FORCE_UNRESOLVE_1", None);
    let _cleanup2 = engine.delete_record("TEST", "FORCE_UNRESOLVE_2", None);
    let _cleanup3 = engine.delete_record("TEST", "FORCE_UNRESOLVE_3", None);
    println!("Test records deleted.");

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    println!("\n✅ Force Unresolve example completed successfully!");

    Ok(())
}

/// Extract entity ID from load/delete info JSON
fn extract_entity_id(info_json: &str, record_description: &str) -> SzResult<i64> {
    let info: Value = serde_json::from_str(info_json).map_err(|e| {
        SzError::bad_input(format!(
            "Failed to parse info for {}: {}",
            record_description, e
        ))
    })?;

    info["RESOLVED_ENTITY"]["ENTITY_ID"]
        .as_i64()
        .or_else(|| {
            info["AFFECTED_ENTITIES"]
                .as_array()?
                .first()?
                .get("ENTITY_ID")?
                .as_i64()
        })
        .ok_or_else(|| {
            SzError::bad_input(format!(
                "Could not extract entity ID from {}",
                record_description
            ))
        })
}

/// Extract a summary of an entity from the JSON response
fn get_entity_summary(entity_json: &str) -> SzResult<String> {
    let entity: Value = serde_json::from_str(entity_json)
        .map_err(|e| SzError::bad_input(format!("Failed to parse entity JSON: {}", e)))?;

    let entity_id = entity["RESOLVED_ENTITY"]["ENTITY_ID"].as_i64().unwrap_or(0);

    let records = entity["RESOLVED_ENTITY"]["RECORDS"]
        .as_array()
        .map(|arr| arr.len())
        .unwrap_or(0);

    let record_ids: Vec<String> = entity["RESOLVED_ENTITY"]["RECORDS"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|r| r["RECORD_ID"].as_str())
        .map(|s| s.to_string())
        .collect();

    Ok(format!(
        "Entity {} with {} record(s): {:?}",
        entity_id, records, record_ids
    ))
}
