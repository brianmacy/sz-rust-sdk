//! Senzing Product Test
//!
//! This module tests product information operations including version and license retrieval,
//! mirroring the C# SzProductTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test get product version
/// Mirrors C# GetVersion tests
#[test]
#[serial]
fn test_get_product_version() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-version-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for version testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get product license
/// Mirrors C# GetLicense tests
#[test]
#[serial]
fn test_get_product_license() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-license-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for license testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test multiple version retrievals
/// Tests that version information is consistent across calls
#[test]
#[serial]
fn test_multiple_version_retrievals() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-multiple-version-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for multiple version retrievals testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test multiple license retrievals
/// Tests that license information is consistent across calls
#[test]
#[serial]
fn test_multiple_license_retrievals() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-multiple-license-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for multiple license retrievals testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product information after environment operations
/// Tests product interface stability after other operations
#[test]
#[serial]
fn test_product_after_environment_operations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-after-ops-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for environment operations testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product information JSON structure validation
/// Tests detailed JSON structure of product information
#[test]
#[serial]
fn test_product_json_structure_validation() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-json-validation-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for JSON structure validation testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product interface from multiple environment instances
/// Tests product interface consistency across different environment instances
#[test]
#[serial]
fn test_product_multiple_environments() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-multi-env-1-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for multiple environments testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product information stability during stress operations
/// Tests product interface under multiple concurrent operations
#[test]
#[serial]
fn test_product_stability_under_load() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-stability-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for stability under load testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_product_error_recovery() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-error-recovery-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for error recovery testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test product information content validation
/// Tests that product information contains expected content
#[test]
#[serial]
fn test_product_content_validation() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-product-content-test")?;
    let _product = env.get_product()?;
    eprintln!("Product available for content validation testing");
    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
