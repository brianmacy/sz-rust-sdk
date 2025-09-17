//! Integration tests for the Senzing Rust SDK
//!
//! These tests require the native Senzing library to be installed at /opt/senzing/er/
//!
//! Note: These tests validate that the SDK correctly interfaces with Senzing
//! and handles expected error conditions appropriately.

use serial_test::serial;
use sz_rust_sdk::prelude::*;

#[test]
#[serial]
fn test_sdk_compilation_and_imports() {
    // Test that all major SDK components can be imported and referenced
    // This validates the public API is correctly exposed

    // Verify trait availability
    fn _check_traits() {
        fn _sz_environment(_: &dyn SzEnvironment) {}
        fn _sz_engine(_: &dyn SzEngine) {}
        fn _sz_config(_: &dyn SzConfig) {}
        fn _sz_config_manager(_: &dyn SzConfigManager) {}
        fn _sz_diagnostic(_: &dyn SzDiagnostic) {}
        fn _sz_product(_: &dyn SzProduct) {}
    }

    // Verify core types are available
    fn _check_types() {
        let _entity_id: EntityId = 1;
        let _config_id: ConfigId = 1;
        let _json: JsonString = String::new();
        let _flags = SzFlags::EXPORT_DEFAULT_FLAGS;
    }

    println!("✅ SDK API compilation test passed");
}

#[test]
#[serial]
fn test_environment_initialization_with_expected_error() {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test that environment initialization behaves as expected with current setup
    // This validates FFI integration is working correctly

    let settings = r#"{
        "PIPELINE": {
            "CONFIGPATH": "/etc/opt/senzing",
            "RESOURCEPATH": "/opt/senzing/er/resources",
            "SUPPORTPATH": "/opt/senzing/data"
        },
        "SQL": {
            "CONNECTION": "sqlite3://na:na@/tmp/G2C.db"
        }
    }"#;

    let env_result = SzEnvironmentCore::new("sz-rust-sdk-test", settings, false);

    // We expect this to fail with a specific Senzing error, which proves FFI is working
    match env_result {
        Ok(_) => {
            println!("✅ Environment initialized successfully (datastore was pre-configured)");
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);

            // Verify we get proper Senzing error codes, not FFI/linking errors
            if error_msg.contains("SENZ7220") || error_msg.contains("No engine configuration") {
                println!(
                    "✅ Environment initialization failed with expected Senzing error: {}",
                    error_msg
                );
                println!("✅ This confirms FFI integration is working correctly");
            } else if error_msg.contains("SENZ") {
                println!(
                    "✅ Environment initialization failed with Senzing error: {}",
                    error_msg
                );
                println!("✅ This confirms FFI integration is working correctly");
            } else {
                panic!(
                    "❌ Unexpected error type (suggests FFI issues): {}",
                    error_msg
                );
            }
        }
    }
}

#[test]
#[serial]
fn test_error_handling_system() {
    // Test that our error handling system works correctly

    // Test error creation
    let config_error = SzError::configuration("Test configuration error");
    assert!(matches!(config_error, SzError::Configuration { .. }));

    let unknown_error = SzError::unknown("Test unknown error");
    assert!(matches!(unknown_error, SzError::Unknown { .. }));

    // Test error display
    let error_string = format!("{}", config_error);
    assert!(error_string.contains("Test configuration error"));

    println!("✅ Error handling system test passed");
}

#[test]
#[serial]
fn test_flags_system() {
    // Test that bitflags work correctly

    let default_flags = SzFlags::EXPORT_DEFAULT_FLAGS;
    let include_related = SzFlags::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES;
    let combined = default_flags | include_related;

    assert!(combined.contains(default_flags));
    assert!(combined.contains(include_related));

    println!("✅ Flags system test passed");
}

#[test]
#[serial]
fn test_types_and_aliases() {
    // Test that type aliases work correctly

    let entity_id: EntityId = 123;
    let config_id: ConfigId = 456;
    let json_data: JsonString = r#"{"test": "data"}"#.to_string();
    let ds_code: DataSourceCode = "TEST_DS".to_string();
    let record_id: RecordId = "REC123".to_string();

    // Verify they're the expected underlying types
    fn _check_entity_id(_: i64) {}
    fn _check_config_id(_: i64) {}
    fn _check_json(_: String) {}
    fn _check_ds_code(_: String) {}
    fn _check_record_id(_: String) {}

    _check_entity_id(entity_id);
    _check_config_id(config_id);
    _check_json(json_data);
    _check_ds_code(ds_code);
    _check_record_id(record_id);

    println!("✅ Types and aliases test passed");
}
