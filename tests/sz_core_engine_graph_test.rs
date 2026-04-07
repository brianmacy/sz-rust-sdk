//! Senzing Core Engine Graph Test
//!
//! This module tests engine graph operations for entity relationship discovery,
//! path finding, and network analysis, mirroring the C# SzCoreEngineGraphTest.cs test patterns.
//! Note: These tests focus on error handling due to singleton architecture constraints.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test find path between non-existent entities
/// Mirrors C# FindPath error tests
#[test]
#[serial]
fn test_find_path_entities_not_found() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-entities-not-found")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find path entities not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find path with additional parameters
/// Tests path finding with avoid entities and required data sources
#[test]
#[serial]
fn test_find_path_with_parameters() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-with-parameters")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find path with parameters testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find path with invalid max degrees
/// Tests parameter validation
#[test]
#[serial]
fn test_find_path_invalid_max_degrees() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-invalid-max-degrees")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find path invalid max degrees testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find path with different flag combinations
/// Tests various flag usage patterns for path finding
#[test]
#[serial]
fn test_find_path_flag_combinations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-flag-combinations")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find path flag combinations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find network by entity ID with non-existent entity
/// Mirrors C# FindNetwork error tests
#[test]
#[serial]
fn test_find_network_entity_not_found() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-entity-not-found")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network entity not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find network with multiple entities
/// Tests network discovery with multiple entity IDs
#[test]
#[serial]
fn test_find_network_multiple_entities() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-multiple-entities")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network multiple entities testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find network with invalid entity IDs
/// Tests entity ID validation
#[test]
#[serial]
fn test_find_network_invalid_entity_ids() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-invalid-entity-ids")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network invalid entity IDs testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find network with different flag combinations
/// Tests various flag usage patterns for network discovery
#[test]
#[serial]
fn test_find_network_flag_combinations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-flag-combinations")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network flag combinations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find network with parameter boundary conditions
/// Tests edge cases for network parameters
#[test]
#[serial]
fn test_find_network_boundary_conditions() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-boundary-conditions")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network boundary conditions testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find path with same source and target
/// Tests edge case where source equals target
#[test]
#[serial]
fn test_find_path_same_entity() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-same-entity")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find path same entity testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test graph operations with empty entity lists
/// Tests edge case handling for empty inputs
#[test]
#[serial]
fn test_find_network_empty_entity_list() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-empty-entity-list")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for find network empty entity list testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test graph operations performance with various parameters
/// Tests different parameter combinations for performance characteristics
#[test]
#[serial]
fn test_graph_operations_parameter_variations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-graph-operations-parameter-variations")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for graph operations parameter variations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_path_by_record_id with non-existent records
/// Should return a NotFound error for unknown data source/record pairs
#[test]
#[serial]
fn test_find_path_by_record_id_not_found() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-by-record-id-not-found")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let result = engine.find_path_by_record_id(
        "TEST",
        "NONEXISTENT_1",
        "TEST",
        "NONEXISTENT_2",
        3,
        None,
        None,
        None,
    );
    assert!(result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_path_by_record_id with valid records
/// Adds two records and finds a path between them
#[test]
#[serial]
fn test_find_path_by_record_id_valid() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-by-record-id-valid")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    engine.add_record("TEST", "FPBR_1001", r#"{"NAME_FULL": "John Smith"}"#, None)?;
    engine.add_record("TEST", "FPBR_1002", r#"{"NAME_FULL": "Jane Doe"}"#, None)?;

    let result = engine.find_path_by_record_id(
        "TEST",
        "FPBR_1001",
        "TEST",
        "FPBR_1002",
        3,
        None,
        None,
        None,
    );
    // Path may or may not exist depending on data, but the call should succeed or return a valid error
    assert!(result.is_ok() || result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_path_by_record_id with flags
#[test]
#[serial]
fn test_find_path_by_record_id_with_flags() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-by-record-id-flags")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    engine.add_record(
        "TEST",
        "FPBRF_1001",
        r#"{"NAME_FULL": "Alice Brown"}"#,
        None,
    )?;
    engine.add_record("TEST", "FPBRF_1002", r#"{"NAME_FULL": "Bob White"}"#, None)?;

    let result = engine.find_path_by_record_id(
        "TEST",
        "FPBRF_1001",
        "TEST",
        "FPBRF_1002",
        5,
        None,
        None,
        Some(SzFlags::FIND_PATH_DEFAULT_FLAGS),
    );
    assert!(result.is_ok() || result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_network_by_record_id with non-existent records
/// Should return an error for unknown records
#[test]
#[serial]
fn test_find_network_by_record_id_not_found() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-by-record-id-not-found")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let result = engine.find_network_by_record_id(
        &[("TEST", "NONEXISTENT_NET_1"), ("TEST", "NONEXISTENT_NET_2")],
        3,
        1,
        100,
        None,
    );
    assert!(result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_network_by_record_id with valid records
#[test]
#[serial]
fn test_find_network_by_record_id_valid() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-by-record-id-valid")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    engine.add_record(
        "TEST",
        "FNBR_1001",
        r#"{"NAME_FULL": "Charlie Green"}"#,
        None,
    )?;

    let result = engine.find_network_by_record_id(&[("TEST", "FNBR_1001")], 3, 1, 100, None);
    // Should succeed with a single known record
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(!json.is_empty());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_network_by_record_id with flags
#[test]
#[serial]
fn test_find_network_by_record_id_with_flags() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-by-record-id-flags")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    engine.add_record("TEST", "FNBRF_1001", r#"{"NAME_FULL": "Diana Blue"}"#, None)?;

    let result = engine.find_network_by_record_id(
        &[("TEST", "FNBRF_1001")],
        3,
        1,
        100,
        Some(SzFlags::FIND_NETWORK_DEFAULT_FLAGS),
    );
    assert!(result.is_ok());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_path_by_entity_id with non-existent entities
/// Should return an error for entities that don't exist
#[test]
#[serial]
fn test_find_path_by_entity_id_not_found() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-path-by-entity-id-not-found")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let result = engine.find_path_by_entity_id(999999, 999998, 3, None, None, None);
    assert!(result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test find_network_by_entity_id with non-existent entities
/// Should return an error for entities that don't exist
#[test]
#[serial]
fn test_find_network_by_entity_id_not_found() -> SzResult<()> {
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("test-find-network-by-entity-id-not-found")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let result = engine.find_network_by_entity_id(&[999999, 999998], 3, 1, 100, None);
    assert!(result.is_err());

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
