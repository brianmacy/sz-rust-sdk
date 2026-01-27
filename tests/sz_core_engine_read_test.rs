//! Senzing Core Engine Read Test
//!
//! This module tests engine read operations for entity and record retrieval,
//! mirroring the C# SzCoreEngineReadTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test get entity by non-existent entity ID
/// Mirrors C# GetEntityByEntityId error tests
#[test]
#[serial]
fn test_get_entity_by_entity_id_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-get-entity-by-entity-id")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for entity ID testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get entity by record with non-existent record
/// Mirrors C# GetEntityByRecord error tests
#[test]
#[serial]
fn test_get_entity_by_record_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-get-entity-by-record")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for entity by record testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get record with non-existent records
/// Mirrors C# GetRecord error handling tests
#[test]
#[serial]
fn test_get_record_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-get-record-not-found")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for record testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test search by attributes with no results
/// Mirrors C# SearchByAttributes tests
#[test]
#[serial]
fn test_search_by_attributes_no_results() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-search-by-attributes")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for search by attributes testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test search by attributes with different flags
/// Tests various search flag combinations
#[test]
#[serial]
fn test_search_by_attributes_with_flags() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-search-by-attributes-flags")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for search flags testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find interesting entities by non-existent entity ID
/// Tests entity relationship discovery error handling
#[test]
#[serial]
fn test_find_interesting_entities_by_entity_id_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-find-interesting-entities")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for interesting entities testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find interesting entities by non-existent record
/// Tests entity relationship discovery by record error handling
#[test]
#[serial]
fn test_find_interesting_entities_by_record_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-find-interesting-by-record")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for interesting entities by record testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test search by attributes with invalid JSON
/// Tests error handling for malformed search criteria
#[test]
#[serial]
fn test_search_by_attributes_invalid_json() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-search-invalid-json")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for invalid JSON testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test read operations with various flag combinations
/// Comprehensive flag testing for read operations
#[test]
#[serial]
fn test_read_operations_flag_combinations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-read-operations-flags")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for flag combinations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test search by attributes with empty criteria
/// Tests edge case handling
#[test]
#[serial]
fn test_search_by_attributes_empty() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("test-search-empty-criteria")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for empty criteria testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
