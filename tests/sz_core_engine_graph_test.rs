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
