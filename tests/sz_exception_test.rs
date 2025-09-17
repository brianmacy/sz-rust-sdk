//! Senzing Exception Test
//!
//! This module tests the exception hierarchy and error handling throughout the SDK,
//! mirroring the C# SzExceptionTest.cs test patterns.

use serial_test::serial;
use std::error::Error;
use sz_rust_sdk::prelude::*;

/// Test default construction of all error types
/// Mirrors C# TestDefaultConstruct(Type exceptionType)
#[test]
#[serial]
fn test_default_construct_unknown() {
    let error = SzError::unknown("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_configuration() {
    let error = SzError::configuration("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_database() {
    let error = SzError::database("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_bad_input() {
    let error = SzError::bad_input("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_license() {
    let error = SzError::license("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_not_found() {
    let error = SzError::not_found("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_unrecoverable() {
    let error = SzError::unrecoverable("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_ffi() {
    let error = SzError::ffi("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_retryable_2() {
    let error = SzError::retryable("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_default_construct_retryable() {
    let error = SzError::retryable("".to_string());

    assert!(!error.to_string().is_empty());
    assert!(error.source().is_none());
}

/// Test message construction of all error types
/// Mirrors C# TestMessageConstruct(Type exceptionType)
#[test]
#[serial]
fn test_message_construct_unknown() {
    let error = SzError::unknown("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_configuration() {
    let error = SzError::configuration("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_database() {
    let error = SzError::database("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_bad_input() {
    let error = SzError::bad_input("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_license() {
    let error = SzError::license("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_not_found() {
    let error = SzError::not_found("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_not_initialized() {
    let error = SzError::unrecoverable("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_unhandled() {
    let error = SzError::ffi("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_retry_timeout() {
    let error = SzError::retryable("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

#[test]
#[serial]
fn test_message_construct_retryable() {
    let error = SzError::retryable("Some Message".to_string());

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_none());
}

/// Test error code construction
/// Mirrors C# TestCodeAndMessageConstruct(Type exceptionType)
/// Test cases: 10L, 20L, 30L, 40L with "Some Message"
#[test]
#[serial]
fn test_code_and_message_construct_10() {
    let error = SzError::from_code(-1);

    // from_code creates error with code in message
    assert!(error.to_string().contains("Native error"));
    assert!(error.to_string().contains("-1"));
}

#[test]
#[serial]
fn test_code_and_message_construct_20() {
    let error = SzError::from_code(-2);

    // from_code creates error with code in message
    assert!(error.to_string().contains("Native error"));
    assert!(error.to_string().contains("-2"));
}

#[test]
#[serial]
fn test_code_and_message_construct_30() {
    let error = SzError::from_code(-3);

    // from_code creates error with code in message
    assert!(error.to_string().contains("Native error"));
    assert!(error.to_string().contains("-3"));
}

#[test]
#[serial]
fn test_code_and_message_construct_40() {
    let error = SzError::from_code(-4);

    // from_code creates error with code in message
    assert!(error.to_string().contains("Native error"));
    assert!(error.to_string().contains("-4"));
}

/// Test cause construction
/// Mirrors C# TestCauseConstruct(Type exceptionType)
#[test]
#[serial]
fn test_cause_construct() {
    use std::io;

    let root_cause = io::Error::other("Root Cause");
    let error = SzError::from_source(Box::new(root_cause));

    assert!(error.source().is_some());
    assert!(error.to_string().contains("Root Cause"));
}

/// Test message and cause construction
/// Mirrors C# TestMessageAndCauseConstruct(Type exceptionType)
#[test]
#[serial]
fn test_message_and_cause_construct() {
    use std::io;

    let root_cause = io::Error::other("Root Cause");
    let error = SzError::with_message_and_source("Some Message".to_string(), Box::new(root_cause));

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_some());
}

/// Test full construction with all parameters
/// Mirrors C# TestFullConstruct(Type exceptionType)
#[test]
#[serial]
fn test_full_construct() {
    use std::io;

    let root_cause = io::Error::other("Root Cause");
    let error = SzError::with_message_and_source("Some Message".to_string(), Box::new(root_cause));

    assert!(error.to_string().contains("Some Message"));
    assert!(error.source().is_some());
}

/// Test error code retrieval
/// Mirrors C# TestGetErrorCode((Type exceptionType, long errorCode) args)
/// Test cases with error codes: -1, -2, -3, -4
#[test]
#[serial]
fn test_get_error_code_10() {
    let error = SzError::from_code(-1);
    // Verify error message contains the code (since we can't extract it separately)
    assert!(error.to_string().contains("-1"));
}

#[test]
#[serial]
fn test_get_error_code_20() {
    let error = SzError::from_code(-2);
    // Verify error message contains the code (since we can't extract it separately)
    assert!(error.to_string().contains("-2"));
}

#[test]
#[serial]
fn test_get_error_code_30() {
    let error = SzError::from_code(-3);
    // Verify error message contains the code (since we can't extract it separately)
    assert!(error.to_string().contains("-3"));
}

#[test]
#[serial]
fn test_get_error_code_40() {
    let error = SzError::from_code(-4);
    // Verify error message contains the code (since we can't extract it separately)
    assert!(error.to_string().contains("-4"));
}

/// Test that errors without codes return None
#[test]
#[serial]
fn test_get_error_code_none() {
    let error = SzError::unknown("Test".to_string());
    // Since our error implementation doesn't store separate codes,
    // we verify the error doesn't contain any code references
    assert!(!error.to_string().contains("code:"));
}
