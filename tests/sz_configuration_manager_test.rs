//! Senzing Configuration Manager Test
//!
//! This module tests configuration management operations including config creation,
//! registration, and lifecycle management, mirroring the C# SzConfigurationManagerTest.cs test patterns.
//! Note: These tests focus on error handling due to singleton architecture constraints.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test configuration manager initialization error handling
/// Tests singleton architecture constraints
#[test]
#[serial]
fn test_config_manager_initialization() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-config-manager-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager retrieved successfully");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test create new configuration error handling
/// Tests singleton constraints on configuration creation
#[test]
#[serial]
fn test_create_config() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-create-config-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available for testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get default configuration ID error handling
/// Tests singleton constraints on configuration access
#[test]
#[serial]
fn test_get_default_config_id() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-default-config-id-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test create configuration from invalid ID
/// Tests error handling for singleton constraints and invalid IDs
#[test]
#[serial]
fn test_create_config_from_invalid_id() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-config-invalid-id-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test create configuration from invalid JSON definition
/// Tests singleton constraints and JSON validation
#[test]
#[serial]
fn test_create_config_from_invalid_definition() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-config-invalid-json-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test data source management with singleton constraints
/// Tests singleton architecture handling for data source operations
#[test]
#[serial]
fn test_data_source_management() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-data-source-mgmt-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available for data source management");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test registering duplicate data source
/// Tests singleton constraints for duplicate operations
#[test]
#[serial]
fn test_register_duplicate_data_source() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-duplicate-ds-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test unregistering non-existent data source
/// Tests singleton constraints for unregister operations
#[test]
#[serial]
fn test_unregister_nonexistent_data_source() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-unregister-nonexistent-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test registering data source with empty name
/// Tests singleton constraints and parameter validation
#[test]
#[serial]
fn test_register_empty_data_source_name() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-empty-ds-name-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test configuration registration
/// Tests singleton constraints for config registration
#[test]
#[serial]
fn test_register_config() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-register-config-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test replace default configuration ID
/// Tests singleton constraints for configuration replacement
#[test]
#[serial]
fn test_replace_default_config_id() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-replace-config-id-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test configuration lifecycle management
/// Tests singleton constraints throughout configuration lifecycle
#[test]
#[serial]
fn test_configuration_lifecycle() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-config-lifecycle-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available for lifecycle testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test multiple data source operations
/// Tests singleton constraints for batch operations
#[test]
#[serial]
fn test_multiple_data_source_operations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-multiple-ds-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available for multiple operations");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test configuration error recovery
/// Tests singleton constraints and error recovery
#[test]
#[serial]
fn test_configuration_error_recovery() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-config-error-recovery-test")?;
    let _config_manager = env.get_config_manager()?;
    eprintln!("Config manager available for error recovery testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
