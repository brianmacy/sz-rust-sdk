#![allow(clippy::borrowed_box)]
//! Load Via Loop Example
//!
//! This example demonstrates advanced record loading with comprehensive error handling,
//! retry logic, and performance tracking. It processes JSONL files with detailed
//! error recovery and logging.
//!
//! Rust equivalent of: loading/LoadViaLoop/Program.cs

use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

// Constants for processing
const RETRY_PREFIX: &str = "retry-";
const RETRY_SUFFIX: &str = ".jsonl";
const DATA_SOURCE_KEY: &str = "DATA_SOURCE";
const RECORD_ID_KEY: &str = "RECORD_ID";
const NUM_TEST_RECORDS: usize = 50; // Number of test records to generate

// Global counters for tracking processing statistics
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static SUCCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
static RETRY_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    // Initialize the Senzing environment using the singleton pattern
    let environment = match ExampleEnvironment::initialize(instance_name) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to initialize environment: {}", e);
            return Err(e);
        }
    };

    println!(
        "Loading {} test records using loop pattern...",
        NUM_TEST_RECORDS
    );

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Setup retry file for failed records
    let retry_file_path = format!("{}test_records{}", RETRY_PREFIX, RETRY_SUFFIX);
    let mut retry_writer = File::create(&retry_file_path).map_err(|e| {
        SzError::unknown(format!(
            "Failed to create retry file {}: {}",
            retry_file_path, e
        ))
    })?;

    // Generate and process test records
    for record_num in 1..=NUM_TEST_RECORDS {
        let test_record = generate_test_record(record_num);
        let line = serde_json::to_string(&test_record).map_err(|e| {
            SzError::unknown(format!(
                "Failed to serialize test record {}: {}",
                record_num, e
            ))
        })?;

        // Process the JSON record
        match process_record(&engine, &line, record_num) {
            Ok(()) => {
                SUCCESS_COUNT.fetch_add(1, Ordering::Relaxed);
                if SUCCESS_COUNT.load(Ordering::Relaxed) % 10 == 0 {
                    println!(
                        "Processed {} records successfully",
                        SUCCESS_COUNT.load(Ordering::Relaxed)
                    );
                }
            }
            Err(e) => {
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                eprintln!("ERROR processing record {}: {}", record_num, e);

                // Write failed record to retry file
                if let Err(write_err) = writeln!(retry_writer, "{}", line) {
                    eprintln!("WARNING: Failed to write to retry file: {}", write_err);
                } else {
                    RETRY_COUNT.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    // Print final statistics
    let success = SUCCESS_COUNT.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let retries = RETRY_COUNT.load(Ordering::Relaxed);

    println!("\n✅ Processing completed!");
    println!("Records processed successfully: {}", success);
    println!("Records with errors: {}", errors);
    println!("Records written to retry file: {}", retries);

    if retries > 0 {
        println!("Retry file created: {}", retry_file_path);
    }

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Generate a test record
fn generate_test_record(record_num: usize) -> Value {
    json!({
        "RECORD_ID": format!("LOOP_TEST_{:06}", record_num),
        "DATA_SOURCE": "TEST",
        "NAME_FIRST": format!("Person{}", record_num),
        "NAME_LAST": "TestLoop",
        "EMAIL_ADDRESS": format!("person.{}@testloop.com", record_num),
        "PHONE_NUMBER": format!("555-{:04}", 1000 + (record_num % 9000)),
        "ADDR_FULL": format!("{} Loop Avenue, Test City, TC 20001", record_num),
        "DATE_OF_BIRTH": format!("198{}-{:02}-{:02}",
            record_num % 10,
            (record_num % 12) + 1,
            (record_num % 28) + 1
        )
    })
}

/// Process a single JSON record
fn process_record(engine: &Box<dyn SzEngine>, json_line: &str, line_number: usize) -> SzResult<()> {
    // Parse the JSON record
    let record: Value = serde_json::from_str(json_line)
        .map_err(|e| SzError::bad_input(format!("Invalid JSON on line {}: {}", line_number, e)))?;

    // Extract required fields
    let data_source = record
        .get(DATA_SOURCE_KEY)
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            SzError::bad_input(format!(
                "Missing {} on line {}",
                DATA_SOURCE_KEY, line_number
            ))
        })?;

    let record_id = record
        .get(RECORD_ID_KEY)
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            SzError::bad_input(format!(
                "Missing {} on line {}",
                RECORD_ID_KEY, line_number
            ))
        })?;

    // Add the record to the repository with default flags
    engine.add_record(
        data_source,
        record_id,
        json_line,
        Some(SzFlags::ADD_RECORD_DEFAULT),
    )?;

    Ok(())
}
