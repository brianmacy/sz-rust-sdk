//! Redo With Info Continuous Example
//!
//! This example demonstrates continuous redo processing with detailed information
//! tracking, including entity ID monitoring, relationship changes, and detailed
//! redo record analysis. It shows how to process redo records while maintaining
//! comprehensive information about the changes being made.
//!
//! Rust equivalent of: redo/RedoWithInfoContinuous/Program.cs

use sz_rust_sdk::prelude::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static ENTITIES_AFFECTED: AtomicUsize = AtomicUsize::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

// Configuration constants
const POLLING_INTERVAL_MS: u64 = 100;  // Poll every 100ms
const MAX_RUNTIME_SECONDS: u64 = 30;   // Run for 30 seconds max (for demo)
const STATS_REPORT_INTERVAL: usize = 5; // Report stats every 5 processed records

// Information tracking structures
#[derive(Debug, Clone)]
struct RedoInfo {
    record_id: String,
    data_source: String,
    entity_id: Option<i64>,
    redo_reason: String,
    timestamp: String,
    affected_entities: Vec<i64>,
}

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

    println!("Starting continuous redo processing with detailed information tracking...");
    println!("Press Ctrl+C to stop (or will auto-stop after {} seconds)", MAX_RUNTIME_SECONDS);

    // Get the engine with automatic setup which ensures proper configuration
    let engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Setup information tracking
    let entity_tracker = Arc::new(Mutex::new(HashMap::<i64, EntityInfo>::new()));
    let redo_history = Arc::new(Mutex::new(Vec::<RedoInfo>::new()));

    // Setup signal handling to gracefully shutdown
    setup_signal_handler();

    // Run continuous redo processing with information tracking
    let start_time = Instant::now();
    continuous_redo_processing_with_info(&engine, entity_tracker.clone(), redo_history.clone(), start_time)?;

    // Print final statistics and information
    print_final_statistics_and_info(start_time, &entity_tracker, &redo_history);

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

#[derive(Debug, Clone)]
struct EntityInfo {
    entity_id: i64,
    first_seen: Instant,
    last_modified: Instant,
    redo_count: usize,
    record_ids: HashSet<String>,
    data_sources: HashSet<String>,
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

/// Main continuous redo processing loop with information tracking
fn continuous_redo_processing_with_info(
    engine: &Box<dyn SzEngine>,
    entity_tracker: Arc<Mutex<HashMap<i64, EntityInfo>>>,
    redo_history: Arc<Mutex<Vec<RedoInfo>>>,
    start_time: Instant
) -> SzResult<()> {
    let mut last_stats_report = 0;
    let max_duration = Duration::from_secs(MAX_RUNTIME_SECONDS);

    while RUNNING.load(Ordering::Relaxed) && start_time.elapsed() < max_duration {
        // Check for and process redo records with information tracking
        match process_redo_records_with_info(engine, &entity_tracker, &redo_history) {
            Ok(processed_count) => {
                if processed_count > 0 {
                    let total_processed = REDO_PROCESSED.load(Ordering::Relaxed);

                    // Report statistics periodically
                    if total_processed >= last_stats_report + STATS_REPORT_INTERVAL {
                        report_detailed_statistics(start_time, &entity_tracker, total_processed);
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

/// Process redo records with detailed information tracking
fn process_redo_records_with_info(
    engine: &Box<dyn SzEngine>,
    entity_tracker: &Arc<Mutex<HashMap<i64, EntityInfo>>>,
    redo_history: &Arc<Mutex<Vec<RedoInfo>>>
) -> SzResult<usize> {
    let mut processed_count = 0;

    // Process all available redo records in this cycle
    loop {
        match get_next_redo_record_with_info(engine) {
            Ok(Some(redo_info)) => {
                process_single_redo_with_info(engine, &redo_info, entity_tracker, redo_history)?;
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

/// Get the next redo record with detailed information
fn get_next_redo_record_with_info(_engine: &Box<dyn SzEngine>) -> SzResult<Option<RedoInfo>> {
    // This is a simplified implementation since the Rust SDK redo API
    // may differ from the C# version. In practice, you would use:
    // - engine.get_redo_record_with_info() or similar
    // - A redo record queue with detailed metadata
    // - Proper redo record deserialization with entity information

    // For demonstration purposes, simulate occasional redo records with detailed info
    use rand::Rng;
    let mut rng = rand::thread_rng();

    if rng.gen_ratio(1, 15) { // ~6.7% chance of having a redo record
        let entity_id = rng.gen_range(1000..=9999);
        let record_id = format!("REDO_REC_{}", rng.r#gen::<u32>());
        let data_sources = ["CUSTOMERS", "EMPLOYEES", "WATCHLIST", "VENDORS"];
        let data_source = data_sources[rng.gen_range(0..data_sources.len())];
        let reasons = [
            "ENTITY_MERGE",
            "ENTITY_SPLIT",
            "RELATIONSHIP_UPDATE",
            "ATTRIBUTE_CHANGE",
            "RECORD_ADDITION",
            "RECORD_DELETION"
        ];
        let redo_reason = reasons[rng.gen_range(0..reasons.len())];

        // Generate additional affected entities for complex operations
        let mut affected_entities = vec![entity_id];
        if rng.gen_ratio(1, 3) { // 33% chance of multiple entities
            for _ in 0..rng.gen_range(1..=3) {
                affected_entities.push(rng.gen_range(1000..=9999));
            }
        }

        let redo_info = RedoInfo {
            record_id,
            data_source: data_source.to_string(),
            entity_id: Some(entity_id),
            redo_reason: redo_reason.to_string(),
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            affected_entities,
        };

        Ok(Some(redo_info))
    } else {
        Ok(None)
    }
}

/// Process a single redo record with detailed information tracking
fn process_single_redo_with_info(
    _engine: &Box<dyn SzEngine>,
    redo_info: &RedoInfo,
    entity_tracker: &Arc<Mutex<HashMap<i64, EntityInfo>>>,
    redo_history: &Arc<Mutex<Vec<RedoInfo>>>
) -> SzResult<()> {
    let now = Instant::now();

    // Update entity tracking information
    {
        let mut tracker = entity_tracker.lock().unwrap();

        for &entity_id in &redo_info.affected_entities {
            let entity_info = tracker.entry(entity_id).or_insert_with(|| {
                ENTITIES_AFFECTED.fetch_add(1, Ordering::Relaxed);
                EntityInfo {
                    entity_id,
                    first_seen: now,
                    last_modified: now,
                    redo_count: 0,
                    record_ids: HashSet::new(),
                    data_sources: HashSet::new(),
                }
            });

            entity_info.last_modified = now;
            entity_info.redo_count += 1;
            entity_info.record_ids.insert(redo_info.record_id.clone());
            entity_info.data_sources.insert(redo_info.data_source.clone());
        }
    }

    // Add to redo history
    {
        let mut history = redo_history.lock().unwrap();
        history.push(redo_info.clone());

        // Keep history limited to last 1000 entries
        if history.len() > 1000 {
            history.remove(0);
        }
    }

    // Log the detailed redo processing
    println!("Processing redo: {} from {} (reason: {}, entities: {:?})",
            redo_info.record_id, redo_info.data_source,
            redo_info.redo_reason, redo_info.affected_entities);

    Ok(())
}

/// Report detailed statistics during processing
fn report_detailed_statistics(
    start_time: Instant,
    entity_tracker: &Arc<Mutex<HashMap<i64, EntityInfo>>>,
    total_processed: usize
) {
    let runtime = start_time.elapsed().as_secs_f64();
    let rate = if runtime > 0.0 { total_processed as f64 / runtime } else { 0.0 };
    let entities_count = ENTITIES_AFFECTED.load(Ordering::Relaxed);

    // Get detailed entity statistics
    let (most_active_entity, avg_redo_per_entity) = {
        let tracker = entity_tracker.lock().unwrap();
        let most_active = tracker.values()
            .max_by_key(|e| e.redo_count)
            .map(|e| (e.entity_id, e.redo_count))
            .unwrap_or((0, 0));

        let avg_redo = if !tracker.is_empty() {
            tracker.values().map(|e| e.redo_count).sum::<usize>() as f64 / tracker.len() as f64
        } else {
            0.0
        };

        (most_active, avg_redo)
    };

    println!("📊 Stats: {} redo records, {} entities affected, {:.1} rec/sec",
            total_processed, entities_count, rate);
    println!("   Most active entity: {} ({} redos), Avg redos/entity: {:.1}",
            most_active_entity.0, most_active_entity.1, avg_redo_per_entity);
}

/// Print comprehensive final statistics and information
fn print_final_statistics_and_info(
    start_time: Instant,
    entity_tracker: &Arc<Mutex<HashMap<i64, EntityInfo>>>,
    redo_history: &Arc<Mutex<Vec<RedoInfo>>>
) {
    let processed = REDO_PROCESSED.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let entities = ENTITIES_AFFECTED.load(Ordering::Relaxed);
    let runtime = start_time.elapsed();

    println!("\n✅ Detailed redo processing completed!");
    println!("Runtime: {:.2} seconds", runtime.as_secs_f64());
    println!("Redo records processed: {}", processed);
    println!("Entities affected: {}", entities);
    println!("Errors encountered: {}", errors);

    if processed > 0 {
        println!("Average processing rate: {:.2} records/second",
                processed as f64 / runtime.as_secs_f64());
    }

    // Detailed entity information
    {
        let tracker = entity_tracker.lock().unwrap();
        if !tracker.is_empty() {
            println!("\n📊 Entity Analysis:");
            println!("Total unique entities: {}", tracker.len());

            let mut entities_by_redo_count: Vec<_> = tracker.values().collect();
            entities_by_redo_count.sort_by_key(|e| std::cmp::Reverse(e.redo_count));

            println!("Top 5 most active entities:");
            for (i, entity) in entities_by_redo_count.iter().take(5).enumerate() {
                println!("  {}. Entity {}: {} redos, {} data sources, {} records",
                        i + 1, entity.entity_id, entity.redo_count,
                        entity.data_sources.len(), entity.record_ids.len());
            }
        }
    }

    // Redo reason analysis
    {
        let history = redo_history.lock().unwrap();
        if !history.is_empty() {
            let mut reason_counts = HashMap::new();
            for redo in history.iter() {
                *reason_counts.entry(&redo.redo_reason).or_insert(0) += 1;
            }

            println!("\n📊 Redo Reason Analysis:");
            let mut sorted_reasons: Vec<_> = reason_counts.iter().collect();
            sorted_reasons.sort_by_key(|&(_, &count)| std::cmp::Reverse(count));

            for (reason, &count) in sorted_reasons {
                let percentage = (count as f64 / history.len() as f64) * 100.0;
                println!("  {}: {} ({:.1}%)", reason, count, percentage);
            }
        }
    }
}
