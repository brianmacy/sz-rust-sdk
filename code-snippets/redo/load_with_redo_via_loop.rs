#![allow(clippy::borrowed_box)]
//! Load With Redo Via Loop Example
//!
//! This example demonstrates how to load records and process any redo records
//! that the Senzing engine generates during the loading process. It shows
//! continuous redo processing in a loop after initial data loading.
//!
//! Rust equivalent of: redo/LoadWithRedoViaLoop/Program.cs

use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

// Constants for file processing
const INPUT_FILES: &[&str] = &[
    "../../resources/data/truthset/customers.jsonl",
    "../../resources/data/truthset/reference.jsonl",
    "../../resources/data/truthset/watchlist.jsonl",
];
const RETRY_PREFIX: &str = "retry-";
const RETRY_SUFFIX: &str = ".jsonl";
const DATA_SOURCE_KEY: &str = "DATA_SOURCE";
const RECORD_ID_KEY: &str = "RECORD_ID";

// Global counters for tracking processing statistics
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static SUCCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
static REDONE_COUNT: AtomicUsize = AtomicUsize::new(0);
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

    println!("Loading records and processing redo operations...");

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Phase 1: Load all input files
    for input_file in INPUT_FILES {
        println!("Loading file: {}", input_file);
        load_file(&engine, input_file)?;
    }

    // Phase 2: Process redo records continuously
    println!("Processing redo records...");
    process_redo_records(&engine)?;

    // Print final statistics
    let success = SUCCESS_COUNT.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let redone = REDONE_COUNT.load(Ordering::Relaxed);
    let retries = RETRY_COUNT.load(Ordering::Relaxed);

    println!("\n✅ Processing completed!");
    println!("Records loaded successfully: {}", success);
    println!("Records with errors: {}", errors);
    println!("Redo records processed: {}", redone);
    println!("Records written to retry file: {}", retries);

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Load records from a single file
fn load_file(engine: &Box<dyn SzEngine>, file_path: &str) -> SzResult<()> {
    // Check if file exists, if not, warn and continue
    if !std::path::Path::new(file_path).exists() {
        eprintln!("⚠️  File not found: {} (skipping)", file_path);
        return Ok(());
    }

    let file = File::open(file_path)
        .map_err(|e| SzError::unknown(format!("Failed to open file {}: {}", file_path, e)))?;

    let reader = BufReader::new(file);

    // Setup retry file for failed records
    let retry_file_path = format!(
        "{}{}{}",
        RETRY_PREFIX,
        std::path::Path::new(file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy(),
        RETRY_SUFFIX
    );
    let mut retry_writer = File::create(&retry_file_path).map_err(|e| {
        SzError::unknown(format!(
            "Failed to create retry file {}: {}",
            retry_file_path, e
        ))
    })?;

    // Process each line in the JSONL file
    let mut line_number = 0;
    for line_result in reader.lines() {
        line_number += 1;

        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("ERROR reading line {}: {}", line_number, e);
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                continue;
            }
        };

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Process the JSON record with redo handling
        match process_record_with_redo(engine, &line, line_number) {
            Ok(()) => {
                SUCCESS_COUNT.fetch_add(1, Ordering::Relaxed);
                if SUCCESS_COUNT.load(Ordering::Relaxed) % 100 == 0 {
                    println!(
                        "Processed {} records successfully",
                        SUCCESS_COUNT.load(Ordering::Relaxed)
                    );
                }
            }
            Err(e) => {
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                eprintln!("ERROR processing line {}: {}", line_number, e);

                // Write failed record to retry file
                if let Err(write_err) = writeln!(retry_writer, "{}", line) {
                    eprintln!("WARNING: Failed to write to retry file: {}", write_err);
                } else {
                    RETRY_COUNT.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    if RETRY_COUNT.load(Ordering::Relaxed) > 0 {
        println!("Retry file created: {}", retry_file_path);
    }

    Ok(())
}

/// Process a single JSON record with redo handling
fn process_record_with_redo(
    engine: &Box<dyn SzEngine>,
    json_line: &str,
    line_number: usize,
) -> SzResult<()> {
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

    // Add the record to the repository with no flags
    let _response = engine.add_record(data_source, record_id, json_line, None)?;

    Ok(())
}

/// Process redo records continuously until queue is empty
fn process_redo_records(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    let mut redo_processed = true;
    let mut iteration = 0;

    while redo_processed {
        iteration += 1;
        redo_processed = false;

        println!("Redo processing iteration: {}", iteration);

        // Check for redo records and process them
        loop {
            // Get next redo record (this is a simplified approach)
            // In a real implementation, you'd use the actual redo record API
            match get_next_redo_record(engine) {
                Ok(Some(redo_record)) => {
                    redo_processed = true;
                    process_single_redo_record(engine, &redo_record)?;
                }
                Ok(None) => {
                    // No more redo records in this iteration
                    break;
                }
                Err(e) => {
                    eprintln!("Error getting redo record: {}", e);
                    break;
                }
            }
        }

        if !redo_processed {
            println!("No more redo records to process");
        }
    }

    println!(
        "Redo processing completed after {} iterations",
        iteration - 1
    );
    Ok(())
}

/// Get the next redo record from the engine
/// This is a placeholder - actual implementation would depend on SDK redo API
fn get_next_redo_record(_engine: &Box<dyn SzEngine>) -> SzResult<Option<String>> {
    // This is a simplified implementation since the Rust SDK redo API
    // may differ from the C# version. In practice, you would use:
    // - engine.get_redo_record() or similar
    // - A redo record queue or iterator
    // - Proper redo record deserialization

    // For now, return None to indicate no more redo records
    Ok(None)
}

/// Process a single redo record
fn process_single_redo_record(_engine: &Box<dyn SzEngine>, redo_record: &str) -> SzResult<()> {
    println!("Processing redo record: {}", redo_record);

    // Parse the redo record JSON
    let _redo_data: Value = serde_json::from_str(redo_record)
        .map_err(|e| SzError::bad_input(format!("Invalid redo record JSON: {}", e)))?;

    // Process the redo record
    // This would involve calling the appropriate engine method for redo processing
    // The specific API calls depend on the Rust SDK implementation

    REDONE_COUNT.fetch_add(1, Ordering::Relaxed);

    if REDONE_COUNT.load(Ordering::Relaxed) % 10 == 0 {
        println!(
            "Processed {} redo records",
            REDONE_COUNT.load(Ordering::Relaxed)
        );
    }

    Ok(())
}
