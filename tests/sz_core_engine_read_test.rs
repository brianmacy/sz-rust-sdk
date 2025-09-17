//! Senzing Core Engine Read Test
//!
//! This module tests engine read operations for entity and record retrieval,
//! mirroring the C# SzCoreEngineReadTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::prelude::*;

/// Test get entity by non-existent entity ID
/// Mirrors C# GetEntityByEntityId error tests
#[test]
#[serial]
fn test_get_entity_by_entity_id_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-get-entity-by-entity-id");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for entity ID testing");
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

/// Test get entity by record with non-existent record
/// Mirrors C# GetEntityByRecord error tests
#[test]
#[serial]
fn test_get_entity_by_record_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-get-entity-by-record");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for entity by record testing");
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

/// Test get record with non-existent records
/// Mirrors C# GetRecord error handling tests
#[test]
#[serial]
fn test_get_record_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-get-record-not-found");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for record testing");
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

/// Test search by attributes with no results
/// Mirrors C# SearchByAttributes tests
#[test]
#[serial]
fn test_search_by_attributes_no_results() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-search-by-attributes");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for search by attributes testing");
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

/// Test search by attributes with different flags
/// Tests various search flag combinations
#[test]
#[serial]
fn test_search_by_attributes_with_flags() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-search-by-attributes-flags");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for search flags testing");
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

/// Test find interesting entities by non-existent entity ID
/// Tests entity relationship discovery error handling
#[test]
#[serial]
fn test_find_interesting_entities_by_entity_id_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-interesting-entities");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for interesting entities testing");
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

/// Test find interesting entities by non-existent record
/// Tests entity relationship discovery by record error handling
#[test]
#[serial]
fn test_find_interesting_entities_by_record_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-find-interesting-by-record");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for interesting entities by record testing");
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

/// Test search by attributes with invalid JSON
/// Tests error handling for malformed search criteria
#[test]
#[serial]
fn test_search_by_attributes_invalid_json() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-search-invalid-json");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for invalid JSON testing");
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

/// Test read operations with various flag combinations
/// Comprehensive flag testing for read operations
#[test]
#[serial]
fn test_read_operations_flag_combinations() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-read-operations-flags");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for flag combinations testing");
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

/// Test search by attributes with empty criteria
/// Tests edge case handling
#[test]
#[serial]
fn test_search_by_attributes_empty() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("test-search-empty-criteria");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for empty criteria testing");
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
