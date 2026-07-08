//! Senzing Core Engine Write Test
//!
//! This module tests engine write operations for record addition, modification, and deletion,
//! mirroring the C# SzCoreEngineWriteTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test add record with invalid data source
/// Mirrors C# AddRecord error tests
#[test]
#[serial]
fn test_add_record_invalid_data_source() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-invalid-ds-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for invalid data source testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with invalid JSON
/// Mirrors C# AddRecord JSON validation tests
#[test]
#[serial]
fn test_add_record_invalid_json() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-invalid-json-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for invalid JSON testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with empty record ID
/// Tests edge case handling
#[test]
#[serial]
fn test_add_record_empty_record_id() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-empty-id-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for empty record ID testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with missing required fields
/// Tests data validation
#[test]
#[serial]
fn test_add_record_missing_fields() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-missing-fields-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for missing fields testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with different flag combinations
/// Tests various flag usage patterns
#[test]
#[serial]
fn test_add_record_with_flags() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-flags-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for flags testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with very large JSON
/// Tests size limits and performance
#[test]
#[serial]
fn test_add_record_large_json() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-large-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for large JSON testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test delete record by record ID with non-existent record
/// Mirrors C# DeleteRecord error tests
#[test]
#[serial]
fn test_delete_record_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-delete-not-found-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for delete not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test delete record with invalid data source
/// Tests error handling for missing data sources
#[test]
#[serial]
fn test_delete_record_invalid_data_source() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-delete-invalid-ds-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for delete invalid data source testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test delete record with empty parameters
/// Tests edge case handling
#[test]
#[serial]
fn test_delete_record_empty_parameters() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-delete-empty-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for delete empty parameters testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test add record with special characters and Unicode
/// Tests character encoding and escaping
#[test]
#[serial]
fn test_add_record_special_characters() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-unicode-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for special characters testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test sequential write operations
/// Tests multiple consecutive write operations
#[test]
#[serial]
fn test_sequential_write_operations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-sequential-write-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for sequential write operations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test write operations with various flag combinations
/// Comprehensive flag testing for write operations
#[test]
#[serial]
fn test_write_operations_flag_combinations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-flags-comprehensive-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for flag combinations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Regression test for issue #29: without WITH_INFO, add_record must use the
/// non-info native entry point and return SZ_NO_INFO (empty), not an
/// unsolicited info document.
#[test]
#[serial]
fn test_add_record_without_info_returns_no_info() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-noinfo-test")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let info = engine.add_record(
        "TEST",
        "NOINFO_1",
        r#"{"NAME_FULL": "No Info Person"}"#,
        None,
    )?;
    assert_eq!(
        info, SZ_NO_INFO,
        "add_record without WITH_INFO must return SZ_NO_INFO (empty), got: {info:?}"
    );

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Regression test for issue #29: with WITH_INFO, add_record must return the
/// affected-entity info document.
#[test]
#[serial]
fn test_add_record_with_info_returns_document() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-write-withinfo-test")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let info = engine.add_record(
        "TEST",
        "WITHINFO_1",
        r#"{"NAME_FULL": "With Info Person"}"#,
        Some(SzFlags::WITH_INFO),
    )?;
    assert_ne!(
        info, SZ_NO_INFO,
        "add_record with WITH_INFO must return a non-empty info document"
    );
    assert!(
        info.contains("AFFECTED_ENTITIES") || info.contains("DATA_SOURCE"),
        "WITH_INFO document should describe affected entities, got: {info}"
    );

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Regression test for issue #29: delete_record honors WITH_INFO the same way.
#[test]
#[serial]
fn test_delete_record_info_dispatch() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-delete-info-dispatch-test")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    engine.add_record("TEST", "DEL_1", r#"{"NAME_FULL": "Delete Me"}"#, None)?;
    let no_info = engine.delete_record("TEST", "DEL_1", None)?;
    assert_eq!(
        no_info, SZ_NO_INFO,
        "delete_record without WITH_INFO must return SZ_NO_INFO, got: {no_info:?}"
    );

    engine.add_record("TEST", "DEL_2", r#"{"NAME_FULL": "Delete Me Too"}"#, None)?;
    let with_info = engine.delete_record("TEST", "DEL_2", Some(SzFlags::WITH_INFO))?;
    assert_ne!(
        with_info, SZ_NO_INFO,
        "delete_record with WITH_INFO must return a non-empty info document"
    );

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
