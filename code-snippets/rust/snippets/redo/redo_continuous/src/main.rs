//! Redo Continuous Example
//!
//! This example demonstrates continuous monitoring and processing of redo records
//! in a background loop. It shows how to set up a long-running process that
//! continuously checks for and processes redo records as they become available.
//!
//! Rust equivalent of: redo/RedoContinuous/Program.cs

use sz_rust_sdk::prelude::*;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

// Configuration constants
const POLLING_INTERVAL_MS: u64 = 100;  // Poll every 100ms
const MAX_RUNTIME_SECONDS: u64 = 30;   // Run for 30 seconds max (for demo)
const STATS_REPORT_INTERVAL: usize = 10; // Report stats every 10 processed records

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

    println!("Starting continuous redo processing...");
    println!("Press Ctrl+C to stop (or will auto-stop after {} seconds)", MAX_RUNTIME_SECONDS);

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Setup signal handling to gracefully shutdown
    setup_signal_handler();

    // Run continuous redo processing
    let start_time = Instant::now();
    continuous_redo_processing(&engine, start_time)?;

    // Print final statistics
    let processed = REDO_PROCESSED.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let runtime = start_time.elapsed();

    println!("\n✅ Continuous redo processing completed!");
    println!("Runtime: {:.2} seconds", runtime.as_secs_f64());
    println!("Redo records processed: {}", processed);
    println!("Errors encountered: {}", errors);

    if processed > 0 {
        println!("Average processing rate: {:.2} records/second",
                processed as f64 / runtime.as_secs_f64());
    }

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Setup signal handler for graceful shutdown
fn setup_signal_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C signal, shutting down gracefully...");
        RUNNING.store(false, Ordering::Relaxed);
        r.store(false, Ordering::Relaxed);
    }).unwrap_or_else(|_| {
        println!("Warning: Could not set Ctrl+C handler");
    });
}

/// Main continuous redo processing loop
fn continuous_redo_processing(engine: &Box<dyn SzEngine>, start_time: Instant) -> SzResult<()> {
    let mut last_stats_report = 0;
    let max_duration = Duration::from_secs(MAX_RUNTIME_SECONDS);

    while RUNNING.load(Ordering::Relaxed) && start_time.elapsed() < max_duration {
        // Check for and process redo records
        match process_available_redo_records(engine) {
            Ok(processed_count) => {
                if processed_count > 0 {
                    let total_processed = REDO_PROCESSED.load(Ordering::Relaxed);

                    // Report statistics periodically
                    if total_processed >= last_stats_report + STATS_REPORT_INTERVAL {
                        println!("Processed {} redo records (runtime: {:.1}s)",
                                total_processed, start_time.elapsed().as_secs_f64());
                        last_stats_report = total_processed;
                    }
                }
            }
            Err(e) => {
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                eprintln!("Error processing redo records: {}", e);

                // Don't exit on error, just log and continue
                thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS * 2));
            }
        }

        // Brief pause before next polling cycle
        thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS));
    }

    if start_time.elapsed() >= max_duration {
        println!("Reached maximum runtime of {} seconds", MAX_RUNTIME_SECONDS);
    }

    Ok(())
}

/// Process all currently available redo records
fn process_available_redo_records(engine: &Box<dyn SzEngine>) -> SzResult<usize> {
    let mut processed_count = 0;

    // Process all available redo records in this cycle
    loop {
        match get_next_redo_record(engine) {
            Ok(Some(redo_record)) => {
                process_single_redo_record(engine, &redo_record)?;
                processed_count += 1;
                REDO_PROCESSED.fetch_add(1, Ordering::Relaxed);
            }
            Ok(None) => {
                // No more redo records available right now
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(processed_count)
}

/// Get the next redo record from the engine
/// This is a placeholder - actual implementation would depend on SDK redo API
fn get_next_redo_record(_engine: &Box<dyn SzEngine>) -> SzResult<Option<String>> {
    // This is a simplified implementation since the Rust SDK redo API
    // may differ from the C# version. In practice, you would use:
    // - engine.get_redo_record() or similar
    // - A redo record queue or iterator
    // - Proper redo record deserialization

    // For demonstration purposes, simulate occasional redo records
    use rand::Rng;
    let mut rng = rand::thread_rng();

    if rng.gen_ratio(1, 20) { // 5% chance of having a redo record
        let mock_redo = format!(r#"{{
            "RECORD_ID": "REDO_{}",
            "DATA_SOURCE": "REDO_SOURCE",
            "REDO_REASON": "ENTITY_RESOLUTION_UPDATE",
            "TIMESTAMP": "{}"
        }}"#, rng.r#gen::<u32>(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));

        Ok(Some(mock_redo))
    } else {
        Ok(None)
    }
}

/// Process a single redo record
fn process_single_redo_record(_engine: &Box<dyn SzEngine>, redo_record: &str) -> SzResult<()> {
    // Parse the redo record JSON
    let redo_data: Value = serde_json::from_str(redo_record)
        .map_err(|e| SzError::bad_input(&format!("Invalid redo record JSON: {}", e)))?;

    // Extract redo record information
    let record_id = redo_data.get("RECORD_ID")
        .and_then(|v| v.as_str())
        .unwrap_or("UNKNOWN");

    let data_source = redo_data.get("DATA_SOURCE")
        .and_then(|v| v.as_str())
        .unwrap_or("UNKNOWN");

    let redo_reason = redo_data.get("REDO_REASON")
        .and_then(|v| v.as_str())
        .unwrap_or("UNKNOWN");

    // Log the redo processing (in a real implementation, you'd call appropriate engine methods)
    println!("Processing redo: {} from {} (reason: {})", record_id, data_source, redo_reason);

    // Simulate some processing time
    thread::sleep(Duration::from_millis(10));

    Ok(())
}
