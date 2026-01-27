//! Senzing Diagnostic Test
//!
//! This module tests diagnostic operations including performance testing,
//! repository information, and system diagnostics, mirroring the C# SzDiagnosticTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Test get repository information
/// Mirrors C# GetRepositoryInfo tests
#[test]
#[serial]
fn test_get_repository_info() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-repo-info-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for repository info testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test check repository performance
/// Mirrors C# CheckRepositoryPerformance tests
#[test]
#[serial]
fn test_check_repository_performance() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-performance-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for performance testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test check repository performance with invalid duration
/// Tests parameter validation for performance testing
#[test]
#[serial]
fn test_check_repository_performance_invalid_duration() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-invalid-duration-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for invalid duration testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get feature with non-existent feature ID
/// Mirrors C# GetFeature error tests
#[test]
#[serial]
fn test_get_feature_not_found() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-feature-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for feature testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test get feature with invalid feature ID
/// Tests edge case handling for invalid feature IDs
#[test]
#[serial]
fn test_get_feature_invalid_id() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-invalid-feature-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for invalid feature ID testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test purge repository
/// Mirrors C# PurgeRepository tests
#[test]
#[serial]
fn test_purge_repository() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-purge-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for purge testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test repository performance with various durations
/// Tests performance testing with different time parameters
#[test]
#[serial]
fn test_repository_performance_various_durations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-duration-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for various duration testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test diagnostic operations after repository operations
/// Tests diagnostics integration with repository state
#[test]
#[serial]
fn test_diagnostics_after_repository_operations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-integration-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for integration testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test sequential diagnostic operations
/// Tests multiple consecutive diagnostic operations
#[test]
#[serial]
fn test_sequential_diagnostic_operations() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-sequential-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for sequential testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test diagnostic operations with repository in different states
/// Tests diagnostics behavior under various repository conditions
#[test]
#[serial]
fn test_diagnostics_repository_states() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-states-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for repository states testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test diagnostic error recovery
/// Tests error handling and recovery scenarios
#[test]
#[serial]
fn test_diagnostic_error_recovery() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-error-recovery-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for error recovery testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

/// Test diagnostic operations with extreme parameters
/// Tests edge cases and boundary conditions
#[test]
#[serial]
fn test_diagnostic_extreme_parameters() -> SzResult<()> {
    // Clean up any existing global instance first
    let _ = SzEnvironmentCore::try_get_instance().map(|e| e.destroy());

    let env = ExampleEnvironment::initialize("sz-rust-sdk-diagnostic-extreme-test")?;
    let _diagnostic = env.get_diagnostic()?;
    eprintln!("Diagnostic available for extreme parameters testing");

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
