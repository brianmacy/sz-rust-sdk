#![allow(clippy::borrowed_box)]
//! Retry with Exponential Backoff
//!
//! Demonstrates a production-ready retry pattern using SzErrorInspect to
//! detect retryable Senzing errors across mixed error types.
//!
//! Key Senzing SDK concepts demonstrated:
//! - is_sz_retryable() for retry decisions on Box<dyn Error>
//! - Bounded retries with exponential backoff
//! - Distinguishing retryable vs fatal errors in a processing loop

use serde_json::json;
use std::thread;
use std::time::Duration;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// A unified result type for operations mixing Senzing and other errors
type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const MAX_RETRIES: u32 = 3;

fn main() -> SzResult<()> {
    let env = get_environment()?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    println!("=== Retry with Backoff Demonstration ===\n");

    // Load a batch of records with retry logic
    let records = vec![
        ("1001", json!({"NAME_FULL": "Alice Smith", "EMAIL_ADDRESS": "alice@example.com"})),
        ("1002", json!({"NAME_FULL": "Bob Jones", "PHONE_NUMBER": "555-1234"})),
        ("1003", json!({"NAME_FULL": "Carol White", "EMAIL_ADDRESS": "carol@example.com"})),
    ];

    for (id, record) in &records {
        match add_record_with_retry(&engine, "TEST", id, &record.to_string(), MAX_RETRIES) {
            Ok(_info) => println!("Added record {id}"),
            Err(ref e) if e.is_sz_bad_input() => {
                // Bad input won't succeed on retry -- skip it
                eprintln!("Skipping {id}: bad input - {e}");
            }
            Err(e) => {
                eprintln!("Failed {id} after retries: {e}");
                return Err(SzError::unrecoverable(format!("Batch aborted: {e}")));
            }
        }
    }

    println!("\nAll records processed.");
    Ok(())
}

/// Add a record with bounded exponential backoff.
///
/// Uses is_sz_retryable() to detect transient Senzing errors (deadlocks,
/// connection drops, etc.) and retries them. Non-retryable errors and
/// non-Senzing errors propagate immediately.
fn add_record_with_retry(
    engine: &Box<dyn SzEngine>,
    data_source: &str,
    record_id: &str,
    record_json: &str,
    max_retries: u32,
) -> AppResult<String> {
    let mut attempt = 0;
    loop {
        match engine.add_record(data_source, record_id, record_json, None) {
            Ok(info) => return Ok(info),

            // is_sz_retryable() works on SzError directly -- walks the chain,
            // returns true for DatabaseTransient, DatabaseConnectionLost, etc.
            Err(ref e) if e.is_sz_retryable() && attempt < max_retries => {
                attempt += 1;
                let delay = Duration::from_millis(100 * 2u64.pow(attempt));
                eprintln!(
                    "  Retryable error on {record_id} (attempt {attempt}/{max_retries}): {e}"
                );
                thread::sleep(delay);
            }

            Err(e) => return Err(e.into()),
        }
    }
}

fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    ExampleEnvironment::initialize("retry_backoff_example")
}
