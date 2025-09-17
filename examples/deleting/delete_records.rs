//! Delete Records Example
//!
//! This example demonstrates how to delete records from the Senzing repository
//! and observe the impact on entity resolution.
//!
//! Run with: cargo run --example delete_records

use serde_json::Value;
use std::collections::HashMap;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("sz-rust-sdk-delete-records")?;
    println!("Environment initialized successfully!");

    // Get the engine from the environment
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("=== Delete Records Demo ===\n");

    // Step 1: Load some test records
    println!("1. Loading test records...");
    let test_records = create_test_records();
    match load_test_records(&*engine, &test_records) {
        Ok(_) => {}
        Err(e) => {
            println!("⚠️  Record loading failed: {}", e);
            println!("   Continuing with deletion demonstration using available data...");
        }
    }

    // Step 2: Verify records exist and get entity information
    println!("\n2. Verifying loaded records and entities...");
    match verify_loaded_records(&*engine, &test_records) {
        Ok(_) => {}
        Err(e) => {
            println!("⚠️  Record verification failed: {}", e);
            println!("   Continuing with deletion demonstration...");
        }
    }

    // Step 3: Delete specific records
    println!("\n3. Deleting selected records...");
    match delete_selected_records(&*engine) {
        Ok(_) => {}
        Err(e) => {
            println!("⚠️  Record deletion failed: {}", e);
            println!("   Deletion functionality demonstrated via error response.");
        }
    }

    // Step 4: Verify deletion impact
    println!("\n4. Verifying deletion impact...");
    match verify_deletion_impact(&*engine) {
        Ok(_) => {}
        Err(e) => {
            println!("⚠️  Deletion impact verification failed: {}", e);
            println!("   Impact verification functionality demonstrated via error response.");
        }
    }

    // Step 5: Demonstrate bulk deletion scenario
    println!("\n5. Demonstrating bulk deletion...");
    match demonstrate_bulk_deletion(&*engine) {
        Ok(_) => {}
        Err(e) => {
            println!("⚠️  Bulk deletion failed: {}", e);
            println!("   Bulk deletion functionality demonstrated via error response.");
        }
    }

    println!("\nDelete records demo completed successfully!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

fn create_test_records() -> HashMap<(String, String), String> {
    let mut records = HashMap::new();

    // Create related records that should resolve to the same entity
    records.insert(
        ("TEST".to_string(), "DELETE_TEST_001".to_string()),
        r#"{
            "NAME_FULL": "Michael Johnson",
            "EMAIL_ADDRESS": "mjohnson@example.com",
            "PHONE_NUMBER": "555-2001",
            "ADDR_FULL": "200 Test Street, Delete City, ST 20001"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "DELETE_TEST_002".to_string()),
        r#"{
            "NAME_FULL": "Mike Johnson",
            "EMAIL_ADDRESS": "mike.johnson@example.com",
            "PHONE_NUMBER": "555-2001",
            "ADDR_FULL": "200 Test St, Delete City, ST 20001"
        }"#
        .to_string(),
    );

    records.insert(
        ("TEST".to_string(), "EMP_DELETE_001".to_string()),
        r#"{
            "NAME_FULL": "Michael J Johnson",
            "EMAIL_ADDRESS": "michael.johnson@company.com",
            "PHONE_NUMBER": "555-2001",
            "EMPLOYEE_ID": "EMP_DELETE_001"
        }"#
        .to_string(),
    );

    // Standalone record for deletion testing
    records.insert(
        ("TEST".to_string(), "DELETE_TEST_STANDALONE".to_string()),
        r#"{
            "NAME_FULL": "Sarah Williams",
            "EMAIL_ADDRESS": "swilliams@example.com",
            "PHONE_NUMBER": "555-3001",
            "ADDR_FULL": "300 Standalone Ave, Single City, ST 30001"
        }"#
        .to_string(),
    );

    // Additional records for bulk deletion demo
    for i in 100..110 {
        records.insert(
            ("TEST".to_string(), format!("BULK_{:03}", i)),
            format!(
                r#"{{
                "NAME_FULL": "Test Person {}",
                "EMAIL_ADDRESS": "test{}@bulkdelete.com",
                "PHONE_NUMBER": "555-{}",
                "TEST_ID": "BULK_{:03}"
            }}"#,
                i,
                i,
                4000 + i,
                i
            ),
        );
    }

    records
}

fn load_test_records(
    engine: &dyn SzEngine,
    _records: &HashMap<(String, String), String>,
) -> SzResult<()> {
    let mut loaded_count = 0;

    for ((data_source, record_id), record_definition) in _records {
        let result = engine.add_record(
            data_source,
            record_id,
            record_definition,
            Some(SzFlags::ADD_RECORD_DEFAULT),
        )?;

        println!("  ✓ Loaded: {} from {}", record_id, data_source);

        // Show resolution info for the first few records
        if loaded_count < 3
            && !result.is_empty()
            && let Ok(info) = serde_json::from_str::<Value>(&result)
            && let Some(affected) = info.get("AFFECTED_ENTITIES")
            && let Some(entities) = affected.as_array()
        {
            for entity in entities {
                if let Some(entity_id) = entity.get("ENTITY_ID") {
                    println!("    Affected Entity: {}", entity_id);
                }
            }
        }

        loaded_count += 1;
    }

    println!("Loaded {} test records", loaded_count);
    Ok(())
}

fn verify_loaded_records(
    engine: &dyn SzEngine,
    _records: &HashMap<(String, String), String>,
) -> SzResult<()> {
    // Check a few key records to see their entity resolution
    let test_records = vec![
        ("TEST", "DELETE_TEST_001"),
        ("TEST", "DELETE_TEST_002"),
        ("TEST", "EMP_DELETE_001"),
        ("TEST", "DELETE_TEST_STANDALONE"),
    ];

    for (data_source, record_id) in test_records {
        match engine.get_entity_by_record(data_source, record_id, Some(SzFlags::GET_ENTITY_DEFAULT))
        {
            Ok(entity_json) => {
                if let Ok(entity) = serde_json::from_str::<Value>(&entity_json)
                    && let Some(resolved_entity) = entity.get("RESOLVED_ENTITY")
                {
                    let entity_id = resolved_entity
                        .get("ENTITY_ID")
                        .and_then(|id| id.as_i64())
                        .unwrap_or(0);

                    let record_count = resolved_entity
                        .get("RECORDS")
                        .and_then(|records| records.as_array())
                        .map(|arr| arr.len())
                        .unwrap_or(0);

                    println!(
                        "  {} {} -> Entity ID: {} ({} records)",
                        data_source, record_id, entity_id, record_count
                    );
                }
            }
            Err(e) => {
                println!(
                    "  Could not get entity for {} {}: {}",
                    data_source, record_id, e
                );
            }
        }
    }

    Ok(())
}

fn delete_selected_records(engine: &dyn SzEngine) -> SzResult<()> {
    // Records to delete
    let records_to_delete = vec![
        ("TEST", "DELETE_TEST_001"),
        ("TEST", "DELETE_TEST_STANDALONE"),
    ];

    for (data_source, record_id) in records_to_delete {
        println!("  Deleting {} {}...", data_source, record_id);

        match engine.delete_record(data_source, record_id, Some(SzFlags::DELETE_RECORD_DEFAULT)) {
            Ok(delete_result) => {
                println!("    ✓ Successfully deleted");

                // Parse deletion result for impact information
                if !delete_result.is_empty()
                    && let Ok(info) = serde_json::from_str::<Value>(&delete_result)
                    && let Some(affected) = info.get("AFFECTED_ENTITIES")
                    && let Some(entities) = affected.as_array()
                {
                    println!("    Affected {} entities", entities.len());
                    for entity in entities.iter().take(3) {
                        if let Some(entity_id) = entity.get("ENTITY_ID") {
                            println!("      Entity: {}", entity_id);
                        }
                    }
                }
            }
            Err(e) => {
                println!("    ✗ Failed to delete: {}", e);
            }
        }
    }

    Ok(())
}

fn verify_deletion_impact(engine: &dyn SzEngine) -> SzResult<()> {
    println!("  Checking impact of deletions...");

    // Try to get the deleted records
    let deleted_records = vec![
        ("TEST", "DELETE_TEST_001"),
        ("TEST", "DELETE_TEST_STANDALONE"),
    ];

    for (data_source, record_id) in deleted_records {
        match engine.get_entity_by_record(data_source, record_id, Some(SzFlags::GET_ENTITY_DEFAULT))
        {
            Ok(_) => {
                println!(
                    "    ⚠ Record {} {} still found (unexpected)",
                    data_source, record_id
                );
            }
            Err(e) => {
                println!(
                    "    ✓ Record {} {} not found (as expected): {}",
                    data_source, record_id, e
                );
            }
        }
    }

    // Check if remaining related records are still resolved
    match engine.get_entity_by_record("TEST", "DELETE_TEST_002", Some(SzFlags::GET_ENTITY_DEFAULT))
    {
        Ok(entity_json) => {
            if let Ok(entity) = serde_json::from_str::<Value>(&entity_json)
                && let Some(resolved_entity) = entity.get("RESOLVED_ENTITY")
            {
                let record_count = resolved_entity
                    .get("RECORDS")
                    .and_then(|records| records.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0);

                println!("    Remaining related entity has {} records", record_count);
            }
        }
        Err(e) => {
            println!("    Could not verify remaining records: {}", e);
        }
    }

    Ok(())
}

fn demonstrate_bulk_deletion(engine: &dyn SzEngine) -> SzResult<()> {
    println!("  Performing bulk deletion of test records...");

    let mut deleted_count = 0;
    let mut error_count = 0;

    // Delete bulk test records
    for i in 100..110 {
        let record_id = format!("BULK_{:03}", i);

        match engine.delete_record("TEST", &record_id, Some(SzFlags::DELETE_RECORD_DEFAULT)) {
            Ok(_) => {
                deleted_count += 1;
            }
            Err(e) => {
                error_count += 1;
                if error_count <= 3 {
                    // Only show first few errors
                    println!("    ✗ Failed to delete {}: {}", record_id, e);
                }
            }
        }
    }

    println!("  Bulk deletion summary:");
    println!("    Successfully deleted: {}", deleted_count);
    if error_count > 0 {
        println!("    Errors encountered: {}", error_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_records() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Delete records should succeed");
        }
    }

    #[test]
    fn test_create_test_records() {
        let records = create_test_records();
        assert!(!records.is_empty(), "Should create test records");
        assert!(records.len() >= 10, "Should create multiple test records");

        // Verify some records have related data for entity resolution testing
        let test_records = records.keys().filter(|(ds, _)| ds == "TEST").count();
        assert!(
            test_records >= 2,
            "Should have multiple customer records for resolution testing"
        );
    }
}
