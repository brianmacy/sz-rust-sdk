//! Test Singleton Pattern
//!
//! This example tests the singleton pattern for SzEnvironmentCore
//! to ensure only one instance exists per process.

use std::sync::Arc;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Test Singleton Pattern ===\n");

    // Test 1: Create first instance
    println!("1. Creating first environment instance...");
    let env1 = ExampleEnvironment::initialize("singleton-test")?;
    println!("✅ First instance created: {:p}", Arc::as_ptr(&env1));

    // Test 2: Try to create second instance with same parameters - should return the same one
    println!("\n2. Attempting to create second instance with same parameters...");
    let env2 = ExampleEnvironment::initialize("singleton-test")?;
    println!("✅ Second instance obtained: {:p}", Arc::as_ptr(&env2));

    // Test 3: Verify they are the same instance
    let same_instance = Arc::ptr_eq(&env1, &env2);
    if same_instance {
        println!("✅ Both references point to the same singleton instance");
    } else {
        println!("❌ ERROR: Different instances created (singleton pattern broken)!");
        return Err(SzError::unknown(
            "Singleton pattern not working correctly".to_string(),
        ));
    }

    // Test 4: Test that we can use both references
    println!("\n3. Testing operations with both references...");

    // Test with first reference
    let engine1 = ExampleEnvironment::get_engine_with_setup(&env1)?;
    println!("✅ Engine obtained from first reference");

    // Test with second reference
    let engine2 = ExampleEnvironment::get_engine_with_setup(&env2)?;
    println!("✅ Engine obtained from second reference");

    // Test search operation with first engine
    println!("\n4. Testing search with first engine...");
    match engine1.search_by_attributes(r#"{"NAME_LAST": "Test"}"#, None, None) {
        Ok(results) => {
            println!("✅ Search successful with first engine");
            println!("   Results: {}", results);
        }
        Err(e) => println!("⚠️  Search failed: {}", e),
    }

    // Test search operation with second engine
    println!("\n5. Testing search with second engine...");
    match engine2.search_by_attributes(r#"{"NAME_FIRST": "Demo"}"#, None, None) {
        Ok(results) => {
            println!("✅ Search successful with second engine");
            println!("   Results: {}", results);
        }
        Err(e) => println!("⚠️  Search failed: {}", e),
    }

    // Test 6: Check if we can get existing instance
    println!("\n6. Testing try_get_instance()...");
    match SzEnvironmentCore::try_get_instance() {
        Some(existing_env) => {
            let same_as_existing = Arc::ptr_eq(&env1, &existing_env);
            if same_as_existing {
                println!("✅ try_get_instance() returned the same singleton instance");
            } else {
                println!("❌ try_get_instance() returned a different instance");
            }
        }
        None => {
            println!("❌ try_get_instance() returned None (unexpected)");
        }
    }

    println!("\n🎯 Singleton pattern test complete!");
    println!("Reference count: {}", Arc::strong_count(&env1));

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
