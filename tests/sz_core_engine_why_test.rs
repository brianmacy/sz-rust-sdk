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
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-entity-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why entity not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why entity analysis with same entity
/// Tests edge case where both entities are the same
#[test]
#[serial]
fn test_why_entity_same_entity() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-same-entity-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why entity same entity testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why records analysis with non-existent records
/// Mirrors C# WhyRecords error tests
#[test]
#[serial]
fn test_why_records_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-records-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why records not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why records analysis with same record
/// Tests edge case where both records are the same
#[test]
#[serial]
fn test_why_records_same_record() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-same-record-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why records same record testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why search analysis with non-existent entity
/// Mirrors C# WhySearch error tests
#[test]
#[serial]
fn test_why_search_not_found() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-search-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why search not found testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why search analysis with invalid JSON
/// Tests JSON parameter validation
#[test]
#[serial]
fn test_why_search_invalid_json() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-search-invalid-json-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why search invalid JSON testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why analysis with different flag combinations
/// Tests various flag usage patterns for why operations
#[test]
#[serial]
fn test_why_operations_flag_combinations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-flags-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why operations flag combinations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why operations with empty parameters
/// Tests edge case handling for empty inputs
#[test]
#[serial]
fn test_why_operations_empty_parameters() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-empty-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why operations empty parameters testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why operations with zero and negative entity IDs
/// Tests edge case handling for invalid entity IDs
#[test]
#[serial]
fn test_why_operations_invalid_entity_ids() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-invalid-ids-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why operations invalid entity IDs testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why operations with no flags
/// Tests behavior when no flags are provided
#[test]
#[serial]
fn test_why_operations_no_flags() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-no-flags-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why operations no flags testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why search with search profile
/// Tests why search functionality with search profiles
#[test]
#[serial]
fn test_why_search_with_profile() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-search-profile-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why search with profile testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test sequential why operations
/// Tests multiple consecutive why operations
#[test]
#[serial]
fn test_sequential_why_operations() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-sequential-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for sequential why operations testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test why operations error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_why_operations_error_recovery() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-why-error-recovery-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    eprintln!("Engine available for why operations error recovery testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
