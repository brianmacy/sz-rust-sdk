#![allow(clippy::borrowed_box)]
//! Redo Continuous Example
//!
//! This example demonstrates continuous monitoring and processing of redo records
//! using the real Senzing SDK API. It shows how to:
//! 1. Check for available redo records using count_redo_records()
//! 2. Retrieve redo records using get_redo_record()
//! 3. Process redo records using process_redo_record()
//!
//! Rust equivalent of: redo/RedoContinuous/Program.cs

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::prelude::*;

/// Initialize the Senzing environment for the redo continuous example
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("redo_continuous_example")
}

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

// Configuration constants
const POLLING_INTERVAL_MS: u64 = 100; // Poll every 100ms
const MAX_RUNTIME_SECONDS: u64 = 5; // Run for 5 seconds max (for demo)
const STATS_REPORT_INTERVAL: usize = 5; // Report stats every 5 processed records

fn main() -> SzResult<()> {
    println!("=== Senzing Redo Continuous Processing ===");

    // Initialize the Senzing environment
    let environment = get_environment()?;
    let engine = environment.get_engine()?;

    println!("✅ Environment initialized");
    println!(
        "Starting continuous redo processing for {} seconds...",
        MAX_RUNTIME_SECONDS
    );

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
        println!(
            "Average processing rate: {:.2} records/second",
            processed as f64 / runtime.as_secs_f64()
        );
    } else {
        println!("ℹ️ No redo records were available during this run");
    }

    Ok(())
}

/// Setup signal handler for graceful shutdown
fn setup_signal_handler() {
    ctrlc::set_handler(move || {
        println!("\n🛑 Received Ctrl+C signal, shutting down gracefully...");
        RUNNING.store(false, Ordering::Relaxed);
    })
    .unwrap_or_else(|_| {
        println!("Warning: Could not set Ctrl+C handler");
    });
}

/// Main continuous redo processing loop using real Senzing API
fn continuous_redo_processing(engine: &Box<dyn SzEngine>, start_time: Instant) -> SzResult<()> {
    let mut last_stats_report = 0;
    let max_duration = Duration::from_secs(MAX_RUNTIME_SECONDS);

    while RUNNING.load(Ordering::Relaxed) && start_time.elapsed() < max_duration {
        // Use Senzing API to check for redo records
        match engine.count_redo_records() {
            Ok(redo_count) => {
                if redo_count > 0 {
                    println!("📋 Found {} redo records available", redo_count);

                    // Process available redo records
                    match process_available_redo_records(engine, redo_count) {
                        Ok(processed_count) => {
                            if processed_count > 0 {
                                let total_processed = REDO_PROCESSED.load(Ordering::Relaxed);

                                // Report statistics periodically
                                if total_processed >= last_stats_report + STATS_REPORT_INTERVAL {
                                    println!(
                                        "✅ Processed {} redo records (runtime: {:.1}s)",
                                        total_processed,
                                        start_time.elapsed().as_secs_f64()
                                    );
                                    last_stats_report = total_processed;
                                }
                            }
                        }
                        Err(e) => {
                            ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                            eprintln!("❌ Error processing redo records: {}", e);

                            // Don't exit on error, just log and continue
                            thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS * 2));
                        }
                    }
                }
            }
            Err(e) => {
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                eprintln!("❌ Error counting redo records: {}", e);
                thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS * 2));
            }
        }

        // Brief pause before next polling cycle
        thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS));
    }

    if start_time.elapsed() >= max_duration {
        println!(
            "⏰ Reached maximum runtime of {} seconds",
            MAX_RUNTIME_SECONDS
        );
    }

    Ok(())
}

/// Process available redo records using real Senzing API
fn process_available_redo_records(
    engine: &Box<dyn SzEngine>,
    available_count: i64,
) -> SzResult<usize> {
    let mut processed_count = 0;
    let max_to_process = std::cmp::min(available_count as usize, 10); // Process up to 10 per cycle

    for i in 0..max_to_process {
        if !RUNNING.load(Ordering::Relaxed) {
            break;
        }

        // Get the next redo record using Senzing API
        match engine.get_redo_record() {
            Ok(redo_record) => {
                if !redo_record.is_empty() {
                    // Process the redo record using Senzing API
                    match engine.process_redo_record(&redo_record, None) {
                        Ok(result) => {
                            println!(
                                "✅ Processed redo record {}/{}: {} bytes result",
                                i + 1,
                                max_to_process,
                                result.len()
                            );
                            processed_count += 1;
                            REDO_PROCESSED.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            eprintln!("❌ Error processing redo record {}: {}", i + 1, e);
                            ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                } else {
                    println!("ℹ️ No more redo records available");
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ Error getting redo record {}: {}", i + 1, e);
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    Ok(processed_count)
}
