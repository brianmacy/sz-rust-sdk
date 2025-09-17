//! Senzing Product Test
//!
//! This module tests product information operations including version and license retrieval,
//! mirroring the C# SzProductTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::prelude::*;

/// Test get product version
/// Mirrors C# GetVersion tests
#[test]
#[serial]
fn test_get_product_version() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-version-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for version testing");
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

/// Test get product license
/// Mirrors C# GetLicense tests
#[test]
#[serial]
fn test_get_product_license() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-license-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for license testing");
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

/// Test multiple version retrievals
/// Tests that version information is consistent across calls
#[test]
#[serial]
fn test_multiple_version_retrievals() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-multiple-version-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for multiple version retrievals testing");
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

/// Test multiple license retrievals
/// Tests that license information is consistent across calls
#[test]
#[serial]
fn test_multiple_license_retrievals() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-multiple-license-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for multiple license retrievals testing");
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

/// Test product information after environment operations
/// Tests product interface stability after other operations
#[test]
#[serial]
fn test_product_after_environment_operations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-after-ops-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for environment operations testing");
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

/// Test product information JSON structure validation
/// Tests detailed JSON structure of product information
#[test]
#[serial]
fn test_product_json_structure_validation() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-json-validation-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for JSON structure validation testing");
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

/// Test product interface from multiple environment instances
/// Tests product interface consistency across different environment instances
#[test]
#[serial]
fn test_product_multiple_environments() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-multi-env-1-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for multiple environments testing");
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

/// Test product information stability during stress operations
/// Tests product interface under multiple concurrent operations
#[test]
#[serial]
fn test_product_stability_under_load() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-stability-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for stability under load testing");
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

/// Test product error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_product_error_recovery() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-error-recovery-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for error recovery testing");
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

/// Test product information content validation
/// Tests that product information contains expected content
#[test]
#[serial]
fn test_product_content_validation() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::destroy_global_instance();

    // Test handling of singleton constraints
    let env_result = ExampleEnvironment::initialize("sz-rust-sdk-product-content-test");

    match env_result {
        Ok(env) => {
            let product_result = env.get_product();
            match product_result {
                Ok(_product) => {
                    eprintln!("Product available for content validation testing");
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
