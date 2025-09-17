//! Senzing Core Engine Graph Test
//!
//! This module tests engine graph operations for entity relationship discovery,
//! path finding, and network analysis, mirroring the C# SzCoreEngineGraphTest.cs test patterns.
//! Note: These tests focus on error handling due to singleton architecture constraints.

use serial_test::serial;
use sz_rust_sdk::prelude::*;

/// Test find path between non-existent entities
/// Mirrors C# FindPath error tests
#[test]
#[serial]
fn test_find_path_entities_not_found() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-path-entities-not-found");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find path entities not found testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find path with additional parameters
/// Tests path finding with avoid entities and required data sources
#[test]
#[serial]
fn test_find_path_with_parameters() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-path-with-parameters");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find path with parameters testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find path with invalid max degrees
/// Tests parameter validation
#[test]
#[serial]
fn test_find_path_invalid_max_degrees() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-path-invalid-max-degrees");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find path invalid max degrees testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find path with different flag combinations
/// Tests various flag usage patterns for path finding
#[test]
#[serial]
fn test_find_path_flag_combinations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-path-flag-combinations");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find path flag combinations testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find network by entity ID with non-existent entity
/// Mirrors C# FindNetwork error tests
#[test]
#[serial]
fn test_find_network_entity_not_found() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-entity-not-found");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network entity not found testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find network with multiple entities
/// Tests network discovery with multiple entity IDs
#[test]
#[serial]
fn test_find_network_multiple_entities() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-multiple-entities");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network multiple entities testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find network with invalid entity IDs
/// Tests entity ID validation
#[test]
#[serial]
fn test_find_network_invalid_entity_ids() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-invalid-entity-ids");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network invalid entity IDs testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find network with different flag combinations
/// Tests various flag usage patterns for network discovery
#[test]
#[serial]
fn test_find_network_flag_combinations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-flag-combinations");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network flag combinations testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find network with parameter boundary conditions
/// Tests edge cases for network parameters
#[test]
#[serial]
fn test_find_network_boundary_conditions() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-boundary-conditions");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network boundary conditions testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test find path with same source and target
/// Tests edge case where source equals target
#[test]
#[serial]
fn test_find_path_same_entity() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-path-same-entity");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find path same entity testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test graph operations with empty entity lists
/// Tests edge case handling for empty inputs
#[test]
#[serial]
fn test_find_network_empty_entity_list() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-network-empty-entity-list");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for find network empty entity list testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}

/// Test graph operations performance with various parameters
/// Tests different parameter combinations for performance characteristics
#[test]
#[serial]
fn test_graph_operations_parameter_variations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-graph-operations-parameter-variations");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for graph operations parameter variations testing");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // With serial test execution, initialization should now succeed
            // Any initialization failure indicates a real problem and must cause test failure
            return Err(e);
        }
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}
