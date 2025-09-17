//! Test Get Existing Instance
//!
//! This example tests the new get_existing_instance() method and parameter validation

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Test Get Existing Instance and Parameter Validation ===\n");

    // Test 1: Try to get existing instance before any instance is created - should fail
    println!("1. Testing get_existing_instance() before any instance is created...");
    match SzEnvironmentCore::get_existing_instance() {
        Ok(_) => {
            println!("❌ ERROR: get_existing_instance() should have failed but didn't!");
            return Err(SzError::unknown(
                "get_existing_instance() should have failed".to_string(),
            ));
        }
        Err(e) => {
            println!("✅ get_existing_instance() correctly failed: {}", e);
        }
    }

    // Test 2: Create first instance with specific parameters
    println!("\n2. Creating first instance with specific parameters...");
    // Remove any existing environment configuration
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    // Use a test settings string since this test is focused on singleton pattern validation
    // not actual database connectivity. The settings don't need to be valid for this test.
    let settings = r#"{"PIPELINE":{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"},"SQL":{"CONNECTION":"sqlite3://na:na@/tmp/test_singleton.db"}}"#;
    let env1 = SzEnvironmentCore::get_instance("test-module-1", settings, true)?;
    println!(
        "✅ First instance created: {:p}",
        std::sync::Arc::as_ptr(&env1)
    );

    // Test 3: Get existing instance - should succeed
    println!("\n3. Testing get_existing_instance() after instance is created...");
    let env_existing = SzEnvironmentCore::get_existing_instance()?;
    println!(
        "✅ get_existing_instance() succeeded: {:p}",
        std::sync::Arc::as_ptr(&env_existing)
    );

    // Verify they are the same instance
    let same_instance = std::sync::Arc::ptr_eq(&env1, &env_existing);
    if same_instance {
        println!("✅ get_existing_instance() returned the same instance");
    } else {
        println!("❌ ERROR: get_existing_instance() returned a different instance");
        return Err(SzError::unknown("Different instances returned".to_string()));
    }

    // Test 4: Try to call get_instance with same parameters - should succeed
    println!("\n4. Testing get_instance() with same parameters...");
    let env2 = SzEnvironmentCore::get_instance("test-module-1", settings, true)?;
    println!(
        "✅ get_instance() with same parameters succeeded: {:p}",
        std::sync::Arc::as_ptr(&env2)
    );

    let same_as_first = std::sync::Arc::ptr_eq(&env1, &env2);
    if same_as_first {
        println!("✅ get_instance() with same parameters returned the same instance");
    } else {
        println!("❌ ERROR: get_instance() with same parameters returned a different instance");
        return Err(SzError::unknown("Different instances returned".to_string()));
    }

    // Test 5: Try to call get_instance with different module name - should succeed (module name can differ)
    println!("\n5. Testing get_instance() with different module name...");
    match SzEnvironmentCore::get_instance("test-module-2", settings, true) {
        Ok(env3) => {
            println!(
                "✅ get_instance() with different module name succeeded (module names can differ)"
            );

            // Should return the same instance since settings and verbose match
            let same_as_first = std::sync::Arc::ptr_eq(&env1, &env3);
            if same_as_first {
                println!("✅ Returned the same singleton instance despite different module name");
            } else {
                println!("❌ ERROR: Should have returned the same singleton instance");
                return Err(SzError::unknown(
                    "Different instances returned for same critical parameters".to_string(),
                ));
            }
        }
        Err(e) => {
            println!(
                "❌ ERROR: get_instance() with different module name should have succeeded: {}",
                e
            );
            return Err(e);
        }
    }

    // Test 6: Try to call get_instance with different ini_params - should fail
    println!("\n6. Testing get_instance() with different ini_params...");
    let different_settings = r#"{"PIPELINE":{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"},"SQL":{"CONNECTION":"sqlite3://na:na@/tmp/G2C_different_test.db"}}"#;
    match SzEnvironmentCore::get_instance("test-module-1", different_settings, true) {
        Ok(_) => {
            println!("❌ ERROR: get_instance() with different ini_params should have failed!");
            return Err(SzError::unknown("Parameter validation failed".to_string()));
        }
        Err(e) => {
            println!(
                "✅ get_instance() with different ini_params correctly failed: {}",
                e
            );
        }
    }

    // Test 7: Try to call get_instance with different verbose_logging - should fail
    println!("\n7. Testing get_instance() with different verbose_logging...");
    match SzEnvironmentCore::get_instance("test-module-1", settings, false) {
        Ok(_) => {
            println!("❌ ERROR: get_instance() with different verbose_logging should have failed!");
            return Err(SzError::unknown("Parameter validation failed".to_string()));
        }
        Err(e) => {
            println!(
                "✅ get_instance() with different verbose_logging correctly failed: {}",
                e
            );
        }
    }

    // Test 8: Test that we can still get the existing instance
    println!("\n8. Testing that existing instance can still be retrieved...");
    let test_env = SzEnvironmentCore::get_existing_instance()?;
    println!(
        "✅ Existing instance retrieved successfully: {:p}",
        std::sync::Arc::as_ptr(&test_env)
    );

    // Verify it's the same instance as env1
    let same_instance = std::sync::Arc::ptr_eq(&env1, &test_env);
    if same_instance {
        println!("✅ Retrieved instance is the same as the original instance");
    } else {
        return Err(SzError::unknown(
            "Retrieved instance is different from original".to_string(),
        ));
    }

    println!("\n🎯 All get_existing_instance() and parameter validation tests completed!");

    Ok(())
}
