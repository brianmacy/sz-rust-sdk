#![allow(clippy::borrowed_box)]
//! Error Inspection with SzErrorInspect
//!
//! Demonstrates how to inspect Senzing errors when they are wrapped inside
//! other error types (Box<dyn Error>, custom enums, etc.).
//!
//! Key Senzing SDK concepts demonstrated:
//! - SzErrorInspect trait for cross-error-type inspection
//! - is_sz_retryable(), is_sz_bad_input(), is_sz_unrecoverable()
//! - is_sz(ErrorCategory) for specific category matching
//! - sz_error() to extract the underlying SzError for detailed info

use std::fs;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// A unified result type that can hold any error (SzError, io::Error, etc.)
type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> SzResult<()> {
    let env = get_environment()?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("=== SzErrorInspect Demonstration ===\n");

    // Demo 1: Inspecting errors from mixed operations
    println!("1. Mixed error handling with is_sz_retryable()");
    match mixed_operations(&engine) {
        Ok(()) => println!("   All operations succeeded"),
        Err(ref e) if e.is_sz_retryable() => {
            println!("   Retryable Senzing error: {e}");
        }
        Err(ref e) if e.is_sz_bad_input() => {
            println!("   Bad input (expected): {e}");
        }
        Err(e) => {
            println!("   Other error: {e}");
        }
    }

    // Demo 2: Using is_sz() with specific ErrorCategory values
    println!("\n2. Category-specific inspection with is_sz()");
    match engine.get_record("TEST", "NONEXISTENT_RECORD", None) {
        Ok(result) => println!("   Found record: {result}"),
        Err(ref e) if e.is_sz(ErrorCategory::NotFound) => {
            println!("   NotFound (expected): record does not exist");
        }
        Err(ref e) if e.is_sz(ErrorCategory::BadInput) => {
            // NotFound extends BadInput, so this would also match if NotFound
            // wasn't checked first. Always check specific types before broader categories.
            println!("   BadInput: {e}");
        }
        Err(e) => {
            println!("   Unexpected error: {e}");
        }
    }

    // Demo 3: Extracting the underlying SzError for detailed inspection
    println!("\n3. Detailed error inspection with sz_error()");
    match mixed_operations(&engine) {
        Err(ref e) => {
            if let Some(sz) = e.sz_error() {
                println!("   Category: {}", sz.category());
                println!("   Severity: {}", sz.severity());
                println!("   Retryable: {}", sz.is_retryable());
                println!("   Hierarchy: {:?}", sz.hierarchy());
                if let Some(code) = sz.error_code() {
                    println!("   Native code: {code}");
                }
            } else {
                println!("   Non-Senzing error: {e}");
            }
        }
        Ok(()) => println!("   (no error to inspect)"),
    }

    // Demo 4: Non-Senzing errors return false for all checks
    println!("\n4. Non-Senzing errors");
    let io_err: Box<dyn std::error::Error + Send + Sync> = Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file missing",
    ));
    println!("   is_sz_retryable: {}", io_err.is_sz_retryable());
    println!("   is_sz_bad_input: {}", io_err.is_sz_bad_input());
    println!("   sz_error present: {}", io_err.sz_error().is_some());

    println!("\nDone.");
    Ok(())
}

/// A function that mixes file I/O and Senzing calls, returning Box<dyn Error>.
/// The `?` operator propagates both io::Error and SzError into the unified type.
fn mixed_operations(engine: &Box<dyn SzEngine>) -> AppResult<()> {
    // This will fail with io::Error if the file doesn't exist,
    // or with SzError if the Senzing call fails.
    let _data = fs::read_to_string("/nonexistent/path.json")
        .unwrap_or_else(|_| r#"{"NAME_FULL": "Test Person"}"#.to_string());

    // add_record with a bad data source to trigger a Senzing error
    engine.add_record("NONEXISTENT_SOURCE", "1", &_data, None)?;
    Ok(())
}

fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    ExampleEnvironment::initialize("error_inspection_example")
}
