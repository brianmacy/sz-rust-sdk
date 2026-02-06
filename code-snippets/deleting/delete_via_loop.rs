#![allow(clippy::borrowed_box)]
//! Delete Via Loop Example
//!
//! This example demonstrates batch deletion of records using a loop with
//! error handling, progress tracking, and retry logic. It shows how to
//! efficiently delete multiple records with proper monitoring.
//!
//! Rust equivalent of: deleting/DeleteViaLoop/Program.cs

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

// Global counters for tracking deletion statistics
static DELETED_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static RETRY_COUNT: AtomicUsize = AtomicUsize::new(0);

// Configuration constants
const BATCH_SIZE: usize = 50; // Number of records to delete in each batch
const TOTAL_RECORDS: usize = 200; // Total number of records to create and delete
const MAX_RETRIES: usize = 3; // Maximum retry attempts for failed deletions
const RETRY_DELAY_MS: u64 = 100; // Delay between retries in milliseconds
const PROGRESS_REPORT_INTERVAL: usize = 25; // Report progress every N deletions

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

    println!("Delete Via Loop Example");
    println!("=======================");
    println!(
        "This example will create {} test records and then delete them in batches.",
        TOTAL_RECORDS
    );

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Phase 1: Create test records to delete
    println!("\n1. Creating test records...");
    let start_time = Instant::now();
    create_test_records(&engine)?;
    let creation_time = start_time.elapsed();
    println!(
        "Created {} records in {:.2} seconds",
        TOTAL_RECORDS,
        creation_time.as_secs_f64()
    );

    // Phase 2: Delete records in batches with monitoring
    println!("\n2. Starting batch deletion process...");
    let deletion_start = Instant::now();
    delete_records_in_batches(&engine)?;
    let deletion_time = deletion_start.elapsed();

    // Phase 3: Report final statistics
    let total_deleted = DELETED_COUNT.load(Ordering::Relaxed);
    let total_errors = ERROR_COUNT.load(Ordering::Relaxed);
    let total_retries = RETRY_COUNT.load(Ordering::Relaxed);

    println!("\n3. Deletion Results:");
    println!("Total records deleted: {}", total_deleted);
    println!("Total errors encountered: {}", total_errors);
    println!("Total retries performed: {}", total_retries);
    println!("Deletion time: {:.2} seconds", deletion_time.as_secs_f64());

    if total_deleted > 0 {
        println!(
            "Average deletion rate: {:.2} records/second",
            total_deleted as f64 / deletion_time.as_secs_f64()
        );
    }

    // Verify deletion completion
    println!("\n4. Verifying deletion completion...");
    verify_deletion_completion(&engine)?;

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    println!("\n✅ Delete Via Loop example completed successfully!");

    Ok(())
}

/// Create test records for deletion
fn create_test_records(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    let data_source = "TEST";

    for i in 1..=TOTAL_RECORDS {
        let record_id = format!("DELETE_LOOP_REC_{:04}", i);
        let record_data = format!(
            r#"{{
            "RECORD_ID": "{}",
            "DATA_SOURCE": "{}",
            "NAME_FIRST": "TestPerson{}",
            "NAME_LAST": "ForDeletion",
            "EMAIL_ADDRESS": "test.person.{}@example.com",
            "PHONE_NUMBER": "555-{:04}",
            "ADDR_FULL": "{} Test Street, Delete City, DC 20001"
        }}"#,
            record_id,
            data_source,
            i,
            i,
            1000 + (i % 9000),
            i
        );

        engine.add_record(data_source, &record_id, &record_data, None)?;

        // Progress indicator for creation
        if i % 50 == 0 {
            println!("Created {} of {} records...", i, TOTAL_RECORDS);
        }
    }

    Ok(())
}

/// Delete records in batches with comprehensive monitoring
fn delete_records_in_batches(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    let data_source = "TEST";
    let mut batch_start = 1;

    while batch_start <= TOTAL_RECORDS {
        let batch_end = std::cmp::min(batch_start + BATCH_SIZE - 1, TOTAL_RECORDS);

        println!(
            "\nProcessing batch: records {} to {}",
            batch_start, batch_end
        );

        let batch_start_time = Instant::now();
        let mut batch_deleted = 0;
        let mut batch_errors = 0;

        for i in batch_start..=batch_end {
            let record_id = format!("DELETE_LOOP_REC_{:04}", i);

            match delete_record_with_retry(engine, data_source, &record_id) {
                Ok(_) => {
                    batch_deleted += 1;
                    DELETED_COUNT.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    batch_errors += 1;
                    ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                    eprintln!("Failed to delete {}: {}", record_id, e);
                }
            }

            // Progress reporting
            let total_processed =
                DELETED_COUNT.load(Ordering::Relaxed) + ERROR_COUNT.load(Ordering::Relaxed);
            if total_processed % PROGRESS_REPORT_INTERVAL == 0 {
                report_progress(total_processed);
            }
        }

        let batch_time = batch_start_time.elapsed();
        println!(
            "Batch completed: {} deleted, {} errors in {:.2} seconds",
            batch_deleted,
            batch_errors,
            batch_time.as_secs_f64()
        );

        batch_start = batch_end + 1;

        // Brief pause between batches to avoid overwhelming the system
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

/// Delete a single record with retry logic
fn delete_record_with_retry(
    engine: &Box<dyn SzEngine>,
    data_source: &str,
    record_id: &str,
) -> SzResult<()> {
    let mut attempts = 0;
    let delete_flags = SzFlags::DELETE_RECORD_DEFAULT;

    loop {
        attempts += 1;

        match engine.delete_record(data_source, record_id, Some(delete_flags)) {
            Ok(_delete_info) => {
                // Deletion successful
                if attempts > 1 {
                    RETRY_COUNT.fetch_add(attempts - 1, Ordering::Relaxed);
                }
                return Ok(());
            }
            Err(e) => {
                if attempts >= MAX_RETRIES {
                    // Max retries exceeded
                    return Err(e);
                }

                // Retry after a brief delay
                eprintln!(
                    "Deletion attempt {} failed for {}: {}. Retrying...",
                    attempts, record_id, e
                );
                thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
            }
        }
    }
}

/// Report progress during deletion
fn report_progress(total_processed: usize) {
    let deleted = DELETED_COUNT.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let retries = RETRY_COUNT.load(Ordering::Relaxed);

    println!(
        "Progress: {}/{} processed ({} deleted, {} errors, {} retries)",
        total_processed, TOTAL_RECORDS, deleted, errors, retries
    );
}

/// Verify that all test records have been deleted
fn verify_deletion_completion(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    let data_source = "TEST";
    let mut remaining_records = Vec::new();

    // Check a sample of record IDs to see if any still exist
    for i in 1..=TOTAL_RECORDS {
        if i % 20 == 0 {
            // Check every 20th record as a sample
            let record_id = format!("DELETE_LOOP_REC_{:04}", i);

            // Try to get the record - if it exists, it wasn't properly deleted
            match engine.get_record(data_source, &record_id, None) {
                Ok(_record_json) => {
                    remaining_records.push(record_id);
                }
                Err(_) => {
                    // Record not found - this is expected after deletion
                }
            }
        }
    }

    if remaining_records.is_empty() {
        println!("✅ Verification passed: No remaining test records found");
    } else {
        println!(
            "⚠️  Verification found {} remaining records: {:?}",
            remaining_records.len(),
            remaining_records
        );

        // Attempt to clean up any remaining records
        println!("Cleaning up remaining records...");
        for record_id in remaining_records {
            let _ = engine.delete_record(data_source, &record_id, None);
        }
    }

    Ok(())
}
