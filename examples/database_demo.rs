//! Database Demo
//!
//! This example demonstrates Senzing SDK operations using the configured database.
//! Works with any database backend (SQLite, PostgreSQL, etc.)
//!
//! Run with: cargo run --example database_demo

use sz_rust_sdk::helpers::EnvironmentGuard;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Senzing Database Demo ===\n");

    println!("Initializing Senzing environment...");

    // Initialize Senzing environment using the helper
    let env = EnvironmentGuard::new("database-demo")?;
    let engine = env.get_engine()?;
    println!("✅ Connected to Senzing database backend\n");

    // Demonstrate working operations
    demo_search_operations(&*engine)?;
    demo_entity_operations(&*engine)?;

    Ok(())
}

fn demo_search_operations(engine: &dyn SzEngine) -> SzResult<()> {
    println!("🔍 Search Operations Demo:");

    // Search by person name
    println!("1. Searching by person name...");
    let search_attrs = r#"{"NAME_LAST": "Smith", "NAME_FIRST": "John"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("   ✅ Search completed");
    println!("   Results: {}\n", results);

    // Search by organization name
    println!("2. Searching by organization...");
    let search_attrs = r#"{"ENTITY_TYPE": "ORGANIZATION", "ORG_NAME": "Acme Corp"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("   ✅ Organization search completed");
    println!("   Results: {}\n", results);

    Ok(())
}

fn demo_entity_operations(engine: &dyn SzEngine) -> SzResult<()> {
    println!("👤 Entity Operations Demo:");

    // Try to add a record
    println!("1. Adding a sample record...");
    let record_data = r#"{
        "NAMES": [{"NAME_TYPE": "PRIMARY", "NAME_LAST": "Johnson", "NAME_FIRST": "Alice"}],
        "EMAIL_ADDRESSES": [{"EMAIL_ADDRESS": "alice.johnson@example.com"}],
        "PHONE_NUMBERS": [{"PHONE_NUMBER": "555-0123"}]
    }"#;

    match engine.add_record("TEST", "DEMO001", record_data, None) {
        Ok(result) => {
            println!("   ✅ Record added successfully!");
            println!("   Result: {}", result);
        }
        Err(e) => println!("   ⚠️  Add record: {}", e),
    }

    // Search for the record we just added
    println!("\n2. Searching for the added record...");
    let search_attrs = r#"{"NAME_LAST": "Johnson", "NAME_FIRST": "Alice"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("   Search results: {}", results);

    // Try getting entity by ID
    println!("\n3. Attempting to retrieve entity by ID...");
    match engine.get_entity(1, None) {
        Ok(entity) => println!("   ✅ Entity 1: {}", entity),
        Err(e) => println!("   ⚠️  Entity 1 not found: {}", e),
    }

    Ok(())
}
