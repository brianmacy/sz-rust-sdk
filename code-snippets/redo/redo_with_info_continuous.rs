#![allow(clippy::borrowed_box)]
//! Redo With Info Continuous Example
//!
//! This example demonstrates continuous redo processing with detailed information
//! tracking using the real Senzing SDK API. It shows how to:
//! 1. Use count_redo_records() to check for available redo records
//! 2. Use get_redo_record() to retrieve redo records
//! 3. Use process_redo_record() to process them with detailed logging
//! 4. Parse and analyze redo record JSON for insights
//!
//! Rust equivalent of: redo/RedoWithInfoContinuous/Program.cs

use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

/// Initialize the Senzing environment for the redo with info continuous example
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("redo_with_info_continuous_example")
}

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

// Configuration constants
const POLLING_INTERVAL_MS: u64 = 100; // Poll every 100ms
const MAX_RUNTIME_SECONDS: u64 = 5; // Run for 5 seconds max (for demo)

// Information tracking
#[derive(Debug, Default)]
struct RedoProcessingInfo {
    by_data_source: HashMap<String, usize>,
    by_entity_type: HashMap<String, usize>,
    total_entities_affected: usize,
    total_relationships_changed: usize,
}

fn main() -> SzResult<()> {
    println!("=== Senzing Redo With Info Continuous Processing ===");

    // Initialize the Senzing environment
    let environment = get_environment()?;
    let engine = environment.get_engine()?;

    println!("✅ Environment initialized");
    println!(
        "Starting detailed redo processing for {} seconds...",
        MAX_RUNTIME_SECONDS
    );

    // Setup signal handling
    setup_signal_handler();

    // Run continuous redo processing with detailed tracking
    let start_time = Instant::now();
    let mut info_tracker = RedoProcessingInfo::default();
    continuous_redo_processing_with_info(&engine, start_time, &mut info_tracker)?;

    // Print final detailed statistics
    let processed = REDO_PROCESSED.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let runtime = start_time.elapsed();

    println!("\n=== FINAL DETAILED STATISTICS ===");
    println!("Runtime: {:.2} seconds", runtime.as_secs_f64());
    println!("Redo records processed: {}", processed);
    println!("Errors encountered: {}", errors);

    if processed > 0 {
        println!(
            "Average processing rate: {:.2} records/second",
            processed as f64 / runtime.as_secs_f64()
        );

        // Detailed breakdown
        println!("\n📊 Breakdown by Data Source:");
        for (data_source, count) in &info_tracker.by_data_source {
            println!("  {} → {} records", data_source, count);
        }

        println!("\n📈 Entity Analysis:");
        println!(
            "  Total entities affected: {}",
            info_tracker.total_entities_affected
        );
        println!(
            "  Total relationships changed: {}",
            info_tracker.total_relationships_changed
        );

        if !info_tracker.by_entity_type.is_empty() {
            println!("\n🏷️ Breakdown by Entity Type:");
            for (entity_type, count) in &info_tracker.by_entity_type {
                println!("  {} → {} occurrences", entity_type, count);
            }
        }
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

/// Main continuous redo processing loop with detailed information tracking
fn continuous_redo_processing_with_info(
    engine: &Box<dyn SzEngine>,
    start_time: Instant,
    info_tracker: &mut RedoProcessingInfo,
) -> SzResult<()> {
    let max_duration = Duration::from_secs(MAX_RUNTIME_SECONDS);

    while RUNNING.load(Ordering::Relaxed) && start_time.elapsed() < max_duration {
        // Use Senzing API to check for redo records
        match engine.count_redo_records() {
            Ok(redo_count) => {
                if redo_count > 0 {
                    println!("📋 Found {} redo records available", redo_count);

                    // Process available redo records with detailed tracking
                    match process_redo_records_with_info(engine, redo_count, info_tracker) {
                        Ok(processed_count) => {
                            if processed_count > 0 {
                                let total_processed = REDO_PROCESSED.load(Ordering::Relaxed);
                                println!(
                                    "✅ Processed {} redo records (total: {}, runtime: {:.1}s)",
                                    processed_count,
                                    total_processed,
                                    start_time.elapsed().as_secs_f64()
                                );
                            }
                        }
                        Err(e) => {
                            ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                            eprintln!("❌ Error processing redo records: {}", e);
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

/// Process redo records with detailed information tracking
fn process_redo_records_with_info(
    engine: &Box<dyn SzEngine>,
    available_count: i64,
    info_tracker: &mut RedoProcessingInfo,
) -> SzResult<usize> {
    let mut processed_count = 0;
    let max_to_process = std::cmp::min(available_count as usize, 5); // Process up to 5 per cycle

    for i in 0..max_to_process {
        if !RUNNING.load(Ordering::Relaxed) {
            break;
        }

        // Get the redo record using Senzing API
        match engine.get_redo_record() {
            Ok(redo_record) => {
                if !redo_record.is_empty() {
                    // Analyze the redo record before processing
                    analyze_redo_record(&redo_record, info_tracker)?;

                    // Process the redo record using Senzing API
                    match engine.process_redo_record(&redo_record, None) {
                        Ok(result) => {
                            println!(
                                "✅ Processed redo record {}/{}: {} bytes result",
                                i + 1,
                                max_to_process,
                                result.len()
                            );

                            // Analyze the processing result if it contains useful info
                            if !result.is_empty() {
                                analyze_processing_result(&result, info_tracker)?;
                            }

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

/// Analyze redo record JSON for detailed information
fn analyze_redo_record(redo_record: &str, info_tracker: &mut RedoProcessingInfo) -> SzResult<()> {
    // Parse the redo record JSON to extract detailed information
    match serde_json::from_str::<Value>(redo_record) {
        Ok(redo_data) => {
            // Extract data source information
            if let Some(data_source) = redo_data.get("DATA_SOURCE").and_then(|v| v.as_str()) {
                *info_tracker
                    .by_data_source
                    .entry(data_source.to_string())
                    .or_insert(0) += 1;
                println!("📝 Redo record from data source: {}", data_source);
            }

            // Extract entity information if available
            if let Some(entity_id) = redo_data.get("ENTITY_ID").and_then(|v| v.as_i64()) {
                info_tracker.total_entities_affected += 1;
                println!("🔗 Entity affected: {}", entity_id);
            }

            // Extract record information
            if let Some(record_id) = redo_data.get("RECORD_ID").and_then(|v| v.as_str()) {
                println!("📄 Record ID: {}", record_id);
            }

            // Extract redo reason/type
            if let Some(redo_reason) = redo_data
                .get("REDO_REASON")
                .or_else(|| redo_data.get("REASON"))
                .and_then(|v| v.as_str())
            {
                *info_tracker
                    .by_entity_type
                    .entry(redo_reason.to_string())
                    .or_insert(0) += 1;
                println!("🔄 Redo reason: {}", redo_reason);
            }

            Ok(())
        }
        Err(e) => {
            println!("⚠️ Could not parse redo record JSON (may be normal): {}", e);
            // Don't fail on JSON parsing errors - redo records may have different formats
            Ok(())
        }
    }
}

/// Analyze processing result for additional insights
fn analyze_processing_result(result: &str, info_tracker: &mut RedoProcessingInfo) -> SzResult<()> {
    // Parse the processing result to extract information about changes made
    match serde_json::from_str::<Value>(result) {
        Ok(result_data) => {
            // Look for relationship changes
            if let Some(relationships) = result_data
                .get("RELATIONSHIPS")
                .or_else(|| result_data.get("AFFECTED_ENTITIES"))
                .and_then(|v| v.as_array())
            {
                info_tracker.total_relationships_changed += relationships.len();
                println!("🔗 Relationships affected: {}", relationships.len());
            }

            // Look for entity resolution information
            if let Some(entity_info) = result_data
                .get("ENTITY")
                .or_else(|| result_data.get("RESOLVED_ENTITY"))
            {
                if let Some(entity_id) = entity_info.get("ENTITY_ID").and_then(|v| v.as_i64()) {
                    println!("🎯 Resolved to entity: {}", entity_id);
                }
            }

            Ok(())
        }
        Err(_) => {
            // Processing result may not be JSON or may be in a different format
            println!(
                "📊 Processing result: {} bytes (non-JSON format)",
                result.len()
            );
            Ok(())
        }
    }
}
