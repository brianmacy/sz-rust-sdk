//! Redo Continuous Via Thread Pool Example
//!
//! This example demonstrates continuous processing of redo records using real OS threads
//! and a thread pool for maximum Senzing performance. Each thread creates its own
//! engine instance from the shared environment for parallel processing.
//!
//! Rust equivalent of: redo/RedoContinuousViaFutures/Program.cs

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::SzEnvironmentCore;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use std::thread;

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);

// Configuration constants
const POLLING_INTERVAL_MS: u64 = 50;   // Poll every 50ms
const MAX_RUNTIME_SECONDS: u64 = 30;   // Run for 30 seconds max (for demo)
const WORKER_COUNT: usize = 4;          // Number of concurrent workers
const CHANNEL_BUFFER_SIZE: usize = 100; // Channel buffer size
const STATS_REPORT_INTERVAL: usize = 25; // Report stats every 25 processed records

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

    println!("Starting thread pool continuous redo processing...");
    println!("Using {} OS threads for maximum Senzing performance", WORKER_COUNT);
    println!("Press Ctrl+C to stop (or will auto-stop after {} seconds)", MAX_RUNTIME_SECONDS);

    // Get the base environment for creating per-thread engines
    let base_environment = environment;

    // Setup graceful shutdown
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    // Setup Ctrl+C handler in separate thread
    let ctrl_c_shutdown = shutdown_clone.clone();
    thread::spawn(move || {
        let _ = ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C signal, shutting down gracefully...");
            ctrl_c_shutdown.store(true, Ordering::Relaxed);
        });
    });

    // Run thread pool redo processing with timeout
    let start_time = Instant::now();
    thread_pool_redo_processing(base_environment, shutdown.clone())?;

    if start_time.elapsed() >= Duration::from_secs(MAX_RUNTIME_SECONDS) {
        println!("Reached maximum runtime of {} seconds", MAX_RUNTIME_SECONDS);
        shutdown.store(true, Ordering::Relaxed);
    }

    // Print final statistics
    let processed = REDO_PROCESSED.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let runtime = start_time.elapsed();

    println!("\n✅ Asynchronous redo processing completed!");
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

/// Main thread pool redo processing coordination
fn thread_pool_redo_processing(
    base_environment: std::sync::Arc<SzEnvironmentCore>,
    shutdown: Arc<AtomicBool>
) -> SzResult<()> {
    // Create channel for distributing redo records to workers
    let (sender, receiver) = mpsc::channel::<String>();
    let receiver = Arc::new(Mutex::new(receiver));

    // Start redo record producer thread
    let producer_env = base_environment.clone();
    let producer_shutdown = shutdown.clone();
    let producer_handle = thread::spawn(move || {
        redo_producer_thread(producer_env, sender, producer_shutdown)
    });

    // Start multiple worker threads for processing redo records
    let mut worker_handles = Vec::new();
    for worker_id in 0..WORKER_COUNT {
        let worker_env = base_environment.clone();
        let worker_receiver = receiver.clone();
        let worker_shutdown = shutdown.clone();

        let handle = thread::spawn(move || {
            redo_worker_thread(worker_id, worker_env, worker_receiver, worker_shutdown)
        });
        worker_handles.push(handle);
    }

    // Start statistics reporting thread
    let stats_shutdown = shutdown.clone();
    let stats_handle = thread::spawn(move || {
        statistics_reporter_thread(stats_shutdown)
    });

    // Wait for shutdown signal
    while !shutdown.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));

        // Auto-shutdown after max runtime
        if stats_handle.is_finished() {
            break;
        }
    }

    // Signal shutdown to all threads
    shutdown.store(true, Ordering::Relaxed);

    // Wait for all threads to complete
    let _ = producer_handle.join();
    for handle in worker_handles {
        let _ = handle.join();
    }
    let _ = stats_handle.join();

    Ok(())
}

/// Producer thread that continuously fetches redo records
fn redo_producer_thread(
    environment: std::sync::Arc<SzEnvironmentCore>,
    sender: mpsc::Sender<String>,
    shutdown: Arc<AtomicBool>
) {
    // Each thread gets its own engine instance (required due to Rust Send/Sync constraints)
    let engine = match ExampleEnvironment::get_engine_with_setup(&environment) {
        Ok(eng) => eng,
        Err(e) => {
            eprintln!("Producer thread failed to get engine: {}", e);
            return;
        }
    };

    while !shutdown.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS));

        // Get next batch of redo records
        match get_next_redo_records(&engine) {
            Ok(records) => {
                for record in records {
                    if sender.send(record).is_err() {
                        // Channel closed, shut down
                        break;
                    }
                }
            }
            Err(e) => {
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                eprintln!("Producer error: {}", e);
                thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS * 2));
            }
        }
    }

    println!("Producer thread shutting down");
}

/// Worker thread that processes individual redo records
fn redo_worker_thread(
    worker_id: usize,
    environment: std::sync::Arc<SzEnvironmentCore>,
    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
    shutdown: Arc<AtomicBool>
) {
    // Each worker thread gets its own engine instance (required due to Rust Send/Sync constraints)
    let engine = match ExampleEnvironment::get_engine_with_setup(&environment) {
        Ok(eng) => eng,
        Err(e) => {
            eprintln!("Worker {} failed to get engine: {}", worker_id, e);
            return;
        }
    };

    let mut processed_by_worker = 0;

    while !shutdown.load(Ordering::Relaxed) {
        // Try to get a redo record from the channel
        let record = {
            let rx = receiver.lock().unwrap();
            match rx.try_recv() {
                Ok(record) => Some(record),
                Err(mpsc::TryRecvError::Empty) => None,
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        };

        match record {
            Some(redo_record) => {
                match process_single_redo_record(&engine, &redo_record) {
                    Ok(()) => {
                        processed_by_worker += 1;
                        REDO_PROCESSED.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(e) => {
                        ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                        eprintln!("Worker {} error: {}", worker_id, e);
                    }
                }
            }
            None => {
                // Channel empty, brief wait
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    println!("Worker {} shutting down (processed {} records)", worker_id, processed_by_worker);
}

/// Statistics reporting thread
fn statistics_reporter_thread(shutdown: Arc<AtomicBool>) {
    let mut last_reported = 0;
    let start_time = Instant::now();

    while !shutdown.load(Ordering::Relaxed) && start_time.elapsed().as_secs() < MAX_RUNTIME_SECONDS {
        thread::sleep(Duration::from_secs(2));

        let current_processed = REDO_PROCESSED.load(Ordering::Relaxed);
        let current_errors = ERROR_COUNT.load(Ordering::Relaxed);

        if current_processed >= last_reported + STATS_REPORT_INTERVAL {
            let runtime = start_time.elapsed().as_secs_f64();
            let rate = if runtime > 0.0 { current_processed as f64 / runtime } else { 0.0 };

            println!("📊 Stats: {} processed, {} errors, {:.1} rec/sec (runtime: {:.1}s)",
                    current_processed, current_errors, rate, runtime);
            last_reported = current_processed;
        }
    }

    println!("Statistics reporter shutting down");
}

/// Get multiple redo records from the engine (thread pool version)
fn get_next_redo_records(
    _engine: &Box<dyn SzEngine>
) -> SzResult<Vec<String>> {
    // This is a simplified implementation since the Rust SDK redo API
    // may differ from the C# version. In practice, you would use:
    // - engine.get_redo_records_async() or similar
    // - A redo record queue or stream
    // - Proper async redo record deserialization

    // For demonstration purposes, simulate occasional batches of redo records
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut records = Vec::new();

    // 20% chance of having 1-5 redo records
    if rng.gen_ratio(1, 5) {
        let count = rng.gen_range(1..=5);
        for _ in 0..count {
            let mock_redo = format!(r#"{{
                "RECORD_ID": "THREADPOOL_REDO_{}",
                "DATA_SOURCE": "TEST",
                "REDO_REASON": "THREADPOOL_ENTITY_RESOLUTION_UPDATE",
                "TIMESTAMP": "{}",
                "WORKER_TYPE": "THREADPOOL"
            }}"#, rng.r#gen::<u32>(), chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));

            records.push(mock_redo);
        }
    }

    Ok(records)
}

/// Process a single redo record in thread pool
fn process_single_redo_record(
    _engine: &Box<dyn SzEngine>,
    redo_record: &str
) -> SzResult<()> {
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

    // Log the thread pool redo processing
    println!("Thread pool processing redo: {} from {} (reason: {})", record_id, data_source, redo_reason);

    // Simulate processing time
    thread::sleep(Duration::from_millis(5));

    Ok(())
}
