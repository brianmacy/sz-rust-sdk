//! Redo Continuous Via Futures Example
//!
//! This example demonstrates continuous redo processing using Rust futures
//! and thread pools with the real Senzing SDK API. It shows how to:
//! 1. Process redo records using multiple OS threads
//! 2. Use real Senzing API calls: count_redo_records(), get_redo_record(), process_redo_record()
//! 3. Coordinate processing using thread-safe channels
//!
//! Rust equivalent of: redo/RedoContinuousViaFutures/Program.cs

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::prelude::*;

/// Initialize the Senzing environment for the redo continuous via futures example
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("redo_continuous_via_futures_example")
}

// Global counters for tracking processing statistics
static REDO_PROCESSED: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

// Configuration constants
const THREAD_POOL_SIZE: usize = 4; // Number of processing threads
const POLLING_INTERVAL_MS: u64 = 100; // Poll every 100ms
const MAX_RUNTIME_SECONDS: u64 = 5; // Run for 5 seconds max (for demo)
const CHANNEL_CAPACITY: usize = 100; // Channel buffer size

fn main() -> SzResult<()> {
    println!("=== Senzing Redo Continuous Processing via Thread Pool ===");

    // Initialize the Senzing environment
    let environment = get_environment()?;

    println!("✅ Environment initialized");
    println!(
        "Starting redo processing with {} threads for {} seconds...",
        THREAD_POOL_SIZE, MAX_RUNTIME_SECONDS
    );

    // Setup signal handling
    setup_signal_handler();

    // Run redo processing with thread pool
    let start_time = Instant::now();
    run_redo_processing_with_thread_pool(environment, start_time)?;

    // Print final statistics
    let processed = REDO_PROCESSED.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);
    let runtime = start_time.elapsed();

    println!("\n✅ Thread pool redo processing completed!");
    println!("Runtime: {:.2} seconds", runtime.as_secs_f64());
    println!("Redo records processed: {}", processed);
    println!("Errors encountered: {}", errors);
    println!("Thread pool size: {}", THREAD_POOL_SIZE);

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

/// Run redo processing using a thread pool with real Senzing API
fn run_redo_processing_with_thread_pool(
    environment: std::sync::Arc<SzEnvironmentCore>,
    start_time: Instant,
) -> SzResult<()> {
    // Create channel for distributing redo records to worker threads
    let (tx, rx) = mpsc::sync_channel::<String>(CHANNEL_CAPACITY);
    let rx = Arc::new(Mutex::new(rx));

    // Spawn worker threads - each gets its own engine instance
    let mut handles = Vec::new();
    for thread_id in 0..THREAD_POOL_SIZE {
        let rx_clone = Arc::clone(&rx);
        let env_clone = environment.clone(); // Clone environment for each thread

        let handle = thread::spawn(move || {
            // Each thread gets its own engine instance (thread-safe design)
            match env_clone.get_engine() {
                Ok(engine) => {
                    worker_thread_redo_processor(thread_id, rx_clone, engine);
                }
                Err(e) => {
                    eprintln!("❌ Thread {} failed to get engine: {}", thread_id, e);
                    ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    // Producer thread - finds and distributes redo records
    let producer_handle = {
        let env_clone = environment.clone();
        thread::spawn(move || match env_clone.get_engine() {
            Ok(engine) => {
                redo_producer_thread(tx, engine, start_time);
            }
            Err(e) => {
                eprintln!("❌ Producer thread failed to get engine: {}", e);
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        })
    };

    // Wait for producer to complete
    producer_handle.join().unwrap_or_else(|_| {
        eprintln!("❌ Producer thread panicked");
        ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
    });

    // Signal workers to shutdown
    RUNNING.store(false, Ordering::Relaxed);

    // Wait for all worker threads to complete
    for handle in handles {
        handle.join().unwrap_or_else(|_| {
            eprintln!("❌ Worker thread panicked");
            ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
        });
    }

    Ok(())
}

/// Producer thread that finds redo records using Senzing API
fn redo_producer_thread(
    tx: mpsc::SyncSender<String>,
    engine: Box<dyn SzEngine>,
    start_time: Instant,
) {
    let max_duration = Duration::from_secs(MAX_RUNTIME_SECONDS);

    while RUNNING.load(Ordering::Relaxed) && start_time.elapsed() < max_duration {
        // Use Senzing API to check for available redo records
        match engine.count_redo_records() {
            Ok(redo_count) => {
                if redo_count > 0 {
                    println!("🔍 Producer found {} redo records", redo_count);

                    // Get and distribute redo records to workers
                    let max_to_get = std::cmp::min(redo_count as usize, 10);
                    for i in 0..max_to_get {
                        if !RUNNING.load(Ordering::Relaxed) {
                            break;
                        }

                        match engine.get_redo_record() {
                            Ok(redo_record) => {
                                if !redo_record.is_empty() {
                                    // Send to worker thread via channel
                                    match tx.try_send(redo_record) {
                                        Ok(()) => {
                                            // Successfully queued for processing
                                        }
                                        Err(mpsc::TrySendError::Full(_)) => {
                                            println!("⚠️ Channel full, slowing down producer");
                                            thread::sleep(Duration::from_millis(50));
                                        }
                                        Err(mpsc::TrySendError::Disconnected(_)) => {
                                            println!("📡 Channel disconnected, stopping producer");
                                            return;
                                        }
                                    }
                                } else {
                                    println!("ℹ️ No more redo records available");
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Producer error getting redo record {}: {}", i + 1, e);
                                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Producer error counting redo records: {}", e);
                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }

        // Brief pause before next polling cycle
        thread::sleep(Duration::from_millis(POLLING_INTERVAL_MS));
    }

    println!("🏁 Producer thread completed");
}

/// Worker thread that processes redo records using Senzing API
fn worker_thread_redo_processor(
    thread_id: usize,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
    engine: Box<dyn SzEngine>,
) {
    let mut local_processed = 0;

    loop {
        // Try to receive a redo record from the channel
        let redo_record = {
            let receiver = rx.lock().unwrap();
            match receiver.try_recv() {
                Ok(record) => Some(record),
                Err(mpsc::TryRecvError::Empty) => None,
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        };

        match redo_record {
            Some(record) => {
                // Process the redo record using real Senzing API
                match engine.process_redo_record(&record, None) {
                    Ok(result) => {
                        local_processed += 1;
                        REDO_PROCESSED.fetch_add(1, Ordering::Relaxed);

                        if local_processed % 5 == 0 {
                            println!(
                                "⚙️ Thread {} processed {} records (latest: {} bytes)",
                                thread_id,
                                local_processed,
                                result.len()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Thread {} error processing record: {}", thread_id, e);
                        ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            None => {
                // No work available, check if we should continue
                if !RUNNING.load(Ordering::Relaxed) {
                    break;
                }
                // Brief pause to avoid busy waiting
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    println!(
        "🏁 Worker thread {} completed ({} records processed)",
        thread_id, local_processed
    );
}
