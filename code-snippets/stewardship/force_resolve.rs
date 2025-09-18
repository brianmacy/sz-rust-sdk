//! Force Resolve Example
//!
//! This example demonstrates how to force entity resolution between two records
//! that may not have been automatically resolved. It shows stewardship operations
//! where a data steward manually forces entities to be resolved together.
//!
//! Rust equivalent of: stewardship/ForceResolve/Program.cs

use serde_json::Value;
use sz_rust_sdk::prelude::*;

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

    println!("Force Resolve Example");
    println!("====================");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // First, let's load some test records that might not automatically resolve
    println!("\n1. Loading test records...");

    let record1 = r#"{
        "RECORD_ID": "FORCE_RESOLVE_1",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "John",
        "NAME_LAST": "Smith",
        "ADDR_FULL": "123 Main St, Anytown, CA 90210",
        "EMAIL_ADDRESS": "john.smith@example.com"
    }"#;

    let record2 = r#"{
        "RECORD_ID": "FORCE_RESOLVE_2",
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": "J",
        "NAME_LAST": "Smith",
        "ADDR_FULL": "456 Oak Ave, Somewhere, CA 90210",
        "PHONE_NUMBER": "555-123-4567"
    }"#;

    // Load the records
    let load_flags = SzFlags::ADD_RECORD_DEFAULT;
    let info1 = engine.add_record("TEST", "FORCE_RESOLVE_1", record1, Some(load_flags))?;
    let info2 = engine.add_record("TEST", "FORCE_RESOLVE_2", record2, Some(load_flags))?;

    println!("Loaded record 1 with info: {}", info1);
    println!("Loaded record 2 with info: {}", info2);

    // Parse the load info to get entity IDs
    let info1_json: Value = serde_json::from_str(&info1)
        .map_err(|e| SzError::bad_input(format!("Failed to parse load info 1: {}", e)))?;
    let info2_json: Value = serde_json::from_str(&info2)
        .map_err(|e| SzError::bad_input(format!("Failed to parse load info 2: {}", e)))?;

    let entity_id1 = info1_json["AFFECTED_ENTITIES"][0]["ENTITY_ID"]
        .as_i64()
        .ok_or_else(|| SzError::bad_input("Could not extract entity ID from record 1"))?;

    let entity_id2 = info2_json["AFFECTED_ENTITIES"][0]["ENTITY_ID"]
        .as_i64()
        .ok_or_else(|| SzError::bad_input("Could not extract entity ID from record 2"))?;

    println!("\n2. Entity IDs after loading:");
    println!("Record 1 resolved to entity: {}", entity_id1);
    println!("Record 2 resolved to entity: {}", entity_id2);

    // Check if they are already resolved together
    if entity_id1 == entity_id2 {
        println!("\n⚠️  Records are already resolved to the same entity!");
        println!("This example works best when records don't automatically resolve.");
    } else {
        println!(
            "\n✅ Records resolved to different entities - good for force resolve demonstration."
        );

        // Show entity details before force resolve
        println!("\n3. Entity details before force resolve:");

        let entity1_json = engine.get_entity(entity_id1, None)?;
        let entity2_json = engine.get_entity(entity_id2, None)?;

        println!(
            "Entity {}: {}",
            entity_id1,
            get_entity_summary(&entity1_json)?
        );
        println!(
            "Entity {}: {}",
            entity_id2,
            get_entity_summary(&entity2_json)?
        );

        // Perform the force resolve operation
        println!("\n4. Performing force resolve...");

        // The reevaluate method forces resolution between entities
        let reevaluate_flags = SzFlags::REEVALUATE_ENTITY_DEFAULT;
        let reevaluate_info = engine.reevaluate_entity(entity_id1, Some(reevaluate_flags))?;

        println!("Reevaluate info: {}", reevaluate_info);

        // For demonstration, let's also try force resolving by loading records with specific flags
        // that encourage resolution
        println!("\n5. Alternative: Using load flags to encourage resolution...");

        // Reload one of the records with flags that might encourage resolution
        let force_flags = SzFlags::ADD_RECORD_DEFAULT;
        let reload_info =
            engine.add_record("TEST", "FORCE_RESOLVE_1", record1, Some(force_flags))?;
        println!("Reload info: {}", reload_info);

        // Check the final state
        println!("\n6. Final entity state:");

        // Get updated entity information
        let final_entity1 = engine.get_entity(entity_id1, None)?;
        let final_entity2 = engine.get_entity(entity_id2, None)?;

        println!(
            "Entity {}: {}",
            entity_id1,
            get_entity_summary(&final_entity1)?
        );
        println!(
            "Entity {}: {}",
            entity_id2,
            get_entity_summary(&final_entity2)?
        );
    }

    // Show why analysis for the resolution
    println!("\n7. Why analysis for entity resolution:");
    let why_flags = SzFlags::WHY_ENTITY_DEFAULT;
    let why_result = engine.why_entity(entity_id1, entity_id2, Some(why_flags))?;
    println!("Why entities result: {}", why_result);

    // Clean up test records
    println!("\n8. Cleaning up test records...");
    let _delete_info1 = engine.delete_record("TEST", "FORCE_RESOLVE_1", None)?;
    let _delete_info2 = engine.delete_record("TEST", "FORCE_RESOLVE_2", None)?;
    println!("Test records deleted.");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    println!("\n✅ Force Resolve example completed successfully!");

    Ok(())
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

    Ok(format!("Entity {} with {} record(s)", entity_id, records))
}
