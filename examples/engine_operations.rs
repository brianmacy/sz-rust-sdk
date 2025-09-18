//! Engine Operations
//!
//! This example demonstrates successful Senzing engine operations
//! including search and network analysis operations.
//!

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Senzing Engine Operations ===\n");

    // Initialize environment using the helper utility
    println!("Initializing Senzing environment...");
    let env = ExampleEnvironment::initialize("engine-operations")?;
    println!("✅ Environment ready!\n");

    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    println!("✅ Senzing engine ready for operations\n");

    // 1. Search by attributes
    println!("1. Searching by attributes...");
    let search_attrs = r#"{"NAME_LAST": "Johnson", "NAME_FIRST": "John"}"#;
    let search_results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("   Search completed successfully");
    println!("   Results: {}\n", search_results);

    // 2. Add a record
    println!("2. Adding a record...");
    let record_data = r#"{
        "NAMES": [{"NAME_TYPE": "PRIMARY", "NAME_LAST": "Smith", "NAME_FIRST": "Jane"}],
        "EMAIL_ADDRESSES": [{"EMAIL_ADDRESS": "jane.smith@example.com"}],
        "PHONE_NUMBERS": [{"PHONE_NUMBER": "555-1234"}]
    }"#;

    match engine.add_record("TEST", "REC001", record_data, None) {
        Ok(result) => {
            println!("   ✅ Record added successfully");
            println!("   Add result: {}", result);
        }
        Err(e) => println!("   ⚠️  Add record failed: {}", e),
    }

    // 3. Search again to see if record was added
    println!("\n3. Searching for the added record...");
    let search_attrs2 = r#"{"NAME_LAST": "Smith", "NAME_FIRST": "Jane"}"#;
    let search_results2 = engine.search_by_attributes(search_attrs2, None, None)?;
    println!("   Search results: {}", search_results2);

    // 4. Find network relationships
    println!("\n4. Testing network analysis...");
    println!("   Note: Testing with non-existent entity IDs for demonstration");
    match engine.find_network(&[999999, 999998], 2, 1, 10, None) {
        Ok(network) => println!("   Network found: {}", network),
        Err(e) => println!(
            "   Network analysis error (expected for non-existent IDs): {}",
            e
        ),
    }

    println!("\n🎯 Engine operations demo complete!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
