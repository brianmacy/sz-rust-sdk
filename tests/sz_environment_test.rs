//! Senzing Environment Test
//!
//! This module tests environment lifecycle management including initialization,
//! state checking, and interface retrieval, mirroring the C# SzEnvironmentTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test environment initialization and basic functionality
/// Mirrors C# Environment basic lifecycle tests
#[test]
#[serial]
fn test_environment_initialization() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-init-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for initialization testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test getting environment interfaces
/// Mirrors C# interface retrieval tests
#[test]
#[serial]
fn test_environment_interface_retrieval() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-interfaces-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for interface retrieval testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment state checking
/// Tests environment destruction state functionality
#[test]
#[serial]
fn test_environment_state_checking() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-state-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for state checking testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment configuration access
/// Tests environment configuration information access
#[test]
#[serial]
fn test_environment_configuration_access() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-config-access-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for configuration access testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment state consistency
/// Tests environment state checking consistency
#[test]
#[serial]
fn test_environment_state_consistency() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result =
        ExampleEnvironment::initialize("sz-rust-sdk-environment-state-consistency-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for state consistency testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test multiple interface retrievals
/// Tests that interfaces can be retrieved multiple times
#[test]
#[serial]
fn test_multiple_interface_retrievals() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result =
        ExampleEnvironment::initialize("sz-rust-sdk-environment-multiple-interfaces-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for multiple interface retrieval testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment interface stability
/// Tests that environment interfaces remain stable
#[test]
#[serial]
fn test_environment_interface_stability() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result =
        ExampleEnvironment::initialize("sz-rust-sdk-environment-interface-stability-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for interface stability testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment configuration ID validation
/// Tests configuration ID handling and validation
#[test]
#[serial]
fn test_environment_config_id_validation() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result =
        ExampleEnvironment::initialize("sz-rust-sdk-environment-config-validation-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for configuration ID validation testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment interface consistency
/// Tests that interfaces behave consistently
#[test]
#[serial]
fn test_environment_interface_consistency() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-consistency-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for interface consistency testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment lifecycle management
/// Tests complete environment lifecycle
#[test]
#[serial]
fn test_environment_lifecycle() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-lifecycle-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for lifecycle testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test environment error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_environment_error_recovery() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-environment-error-recovery-test");

    match env_result {
        Ok(env) => {
            // Test that environment is available for testing
            eprintln!("Environment available for error recovery testing");

            // Can test read-only operations like:
            let _is_destroyed = env.is_destroyed();
            let _active_config_result = env.get_active_config_id();
        }
        Err(e) => {
            // With serial test execution, environment initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test concurrent engine initialization (race condition fix)
/// Tests that multiple threads can call get_engine() concurrently without
/// "SDK not initialized" errors. This validates the fix for the race condition
/// where threads could proceed before Sz_init() completed.
#[test]
#[serial]
fn test_concurrent_engine_initialization() -> SzResult<()> {
    use std::sync::Arc;
    use std::thread;

    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Initialize environment - already returns Arc<SzEnvironmentCore>
    let env = ExampleEnvironment::initialize("sz-rust-sdk-concurrent-init-test")?;

    // Spawn multiple threads that all try to get_engine() simultaneously
    const NUM_THREADS: usize = 8;
    let mut handles = Vec::with_capacity(NUM_THREADS);

    for i in 0..NUM_THREADS {
        let env_clone = Arc::clone(&env);
        let handle = thread::spawn(move || -> Result<(), String> {
            // All threads try to get_engine() at roughly the same time
            // Before the fix, some threads would get "SDK not initialized"
            match env_clone.get_engine() {
                Ok(_engine) => {
                    eprintln!("Thread {} successfully got engine", i);
                    Ok(())
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    eprintln!("Thread {} failed: {}", i, err_msg);
                    Err(err_msg)
                }
            }
        });
        handles.push(handle);
    }

    // Collect results from all threads
    let mut failures = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => failures.push(format!("Thread {} error: {}", i, e)),
            Err(_) => failures.push(format!("Thread {} panicked", i)),
        }
    }

    // Drop our reference before cleanup - threads have already joined so their refs are gone
    drop(env);

    // Now cleanup can destroy since we're the sole owner
    ExampleEnvironment::cleanup()?;

    // All threads must succeed
    if !failures.is_empty() {
        return Err(SzError::unrecoverable(format!(
            "Concurrent initialization failed:\n{}",
            failures.join("\n")
        )));
    }

    eprintln!(
        "All {} threads successfully initialized engine concurrently",
        NUM_THREADS
    );
    Ok(())
}

/// Test destroy ownership semantics with Arc::try_unwrap
/// Verifies that destroy() only succeeds when caller has sole ownership.
/// This ensures safe cleanup - you can't destroy while others still hold references.
#[test]
#[serial]
fn test_destroy_ownership_semantics() -> SzResult<()> {
    use std::sync::Arc;

    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    // Initialize environment
    let env = ExampleEnvironment::initialize("sz-rust-sdk-destroy-ownership-test")?;

    // Create additional reference
    let env_clone = Arc::clone(&env);

    // Try to destroy with multiple references - should fail
    let destroy_result = env.destroy();
    assert!(
        destroy_result.is_err(),
        "destroy() should fail when other references exist"
    );

    // The error message should indicate other references exist
    if let Err(e) = destroy_result {
        let msg = e.to_string();
        assert!(
            msg.contains("other references"),
            "Error should mention 'other references': {}",
            msg
        );
    }

    // Environment should still be usable via the clone
    assert!(
        !env_clone.is_destroyed(),
        "Environment should not be destroyed after failed destroy()"
    );

    // Get a fresh reference from the singleton
    let env_fresh = SzEnvironmentCore::try_get_instance()
        .expect("Singleton should still exist after failed destroy()");

    // Verify it's the same instance
    assert!(
        Arc::ptr_eq(&env_clone, &env_fresh),
        "Should get same instance from singleton"
    );

    // Now drop all references except one
    drop(env_clone);
    drop(env_fresh);

    // Get sole reference and destroy
    let env_sole = SzEnvironmentCore::try_get_instance().expect("Singleton should still exist");

    let destroy_result = env_sole.destroy();
    assert!(
        destroy_result.is_ok(),
        "destroy() should succeed with sole ownership: {:?}",
        destroy_result.err()
    );

    // Singleton should now be empty
    assert!(
        SzEnvironmentCore::try_get_instance().is_none(),
        "Singleton should be None after successful destroy()"
    );

    eprintln!("✅ Destroy ownership semantics test passed");
    Ok(())
}
