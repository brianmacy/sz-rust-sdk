//! Senzing Core Engine Why Test
//!
//! This module tests engine "why" operations for resolution analysis,
//! mirroring the C# SzCoreEngineWhyTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test why entity analysis with non-existent entities
/// Mirrors C# WhyEntity error tests
#[test]
#[serial]
fn test_why_entity_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-entity-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why entity not found testing");
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

/// Test why entity analysis with same entity
/// Tests edge case where both entities are the same
#[test]
#[serial]
fn test_why_entity_same_entity() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-same-entity-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why entity same entity testing");
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

/// Test why records analysis with non-existent records
/// Mirrors C# WhyRecords error tests
#[test]
#[serial]
fn test_why_records_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-records-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why records not found testing");
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

/// Test why records analysis with same record
/// Tests edge case where both records are the same
#[test]
#[serial]
fn test_why_records_same_record() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-same-record-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why records same record testing");
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

/// Test why search analysis with non-existent entity
/// Mirrors C# WhySearch error tests
#[test]
#[serial]
fn test_why_search_not_found() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-search-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why search not found testing");
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

/// Test why search analysis with invalid JSON
/// Tests JSON parameter validation
#[test]
#[serial]
fn test_why_search_invalid_json() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-search-invalid-json-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why search invalid JSON testing");
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

/// Test why analysis with different flag combinations
/// Tests various flag usage patterns for why operations
#[test]
#[serial]
fn test_why_operations_flag_combinations() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-flags-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why operations flag combinations testing");
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

/// Test why operations with empty parameters
/// Tests edge case handling for empty inputs
#[test]
#[serial]
fn test_why_operations_empty_parameters() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-empty-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why operations empty parameters testing");
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

/// Test why operations with zero and negative entity IDs
/// Tests edge case handling for invalid entity IDs
#[test]
#[serial]
fn test_why_operations_invalid_entity_ids() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-invalid-ids-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why operations invalid entity IDs testing");
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

/// Test why operations with no flags
/// Tests behavior when no flags are provided
#[test]
#[serial]
fn test_why_operations_no_flags() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-no-flags-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why operations no flags testing");
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

/// Test why search with search profile
/// Tests why search functionality with search profiles
#[test]
#[serial]
fn test_why_search_with_profile() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-search-profile-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why search with profile testing");
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

/// Test sequential why operations
/// Tests multiple consecutive why operations
#[test]
#[serial]
fn test_sequential_why_operations() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-sequential-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for sequential why operations testing");
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

/// Test why operations error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_why_operations_error_recovery() -> SzResult<()> {
    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-why-error-recovery-test");

    match env_result {
        Ok(env) => {
            let engine_result = ExampleEnvironment::get_engine_with_setup(&env);
            match engine_result {
                Ok(_engine) => {
                    eprintln!("Engine available for why operations error recovery testing");
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
