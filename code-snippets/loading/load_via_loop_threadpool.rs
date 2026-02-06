#![allow(clippy::borrowed_box)]
//! Load Via Loop Thread Pool Example
//!
//! This example demonstrates thread pool patterns for batch loading operations
//! using real OS threads for maximum Senzing performance. Each worker thread
//! gets its own engine instance for parallel processing.
//!
//! Thread pool patterns for loading inspired by: loading/LoadViaLoop/Program.cs

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

// Global counters for tracking loading statistics
static LOADED_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);

// Configuration constants
const TOTAL_RECORDS: usize = 100; // Total number of records to load
const WORKER_COUNT: usize = 4; // Number of concurrent worker threads
const CHANNEL_BUFFER_SIZE: usize = 50; // Channel buffer size
const PROGRESS_REPORT_INTERVAL: usize = 25; // Report progress every N loads

#[derive(Debug, Clone)]
struct LoadTask {
    record_id: String,
    data_source: String,
    record_data: String,
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

    println!("Thread Pool Load Via Loop Example");
    println!("==================================");
    println!(
        "Loading {} records using {} OS threads for maximum performance",
        TOTAL_RECORDS, WORKER_COUNT
    );

    // Create a channel for distributing work to threads
    let (sender, receiver) = mpsc::sync_channel::<LoadTask>(CHANNEL_BUFFER_SIZE);
    let receiver = Arc::new(std::sync::Mutex::new(receiver));

    // Shared stopping flag
    let running = Arc::new(AtomicBool::new(true));

    println!("Starting {} worker threads...", WORKER_COUNT);

    // Spawn worker threads
    let mut workers = Vec::new();
    for worker_id in 0..WORKER_COUNT {
        let receiver = Arc::clone(&receiver);
        let running = Arc::clone(&running);
        let environment = environment.clone();

        let worker = thread::spawn(move || {
            // Each thread gets its own engine instance
            let engine = match ExampleEnvironment::get_engine_with_setup(&environment) {
                Ok(eng) => eng,
                Err(e) => {
                    eprintln!("Worker {}: Failed to get engine: {}", worker_id, e);
                    return;
                }
            };

            // Process records from the channel
            while running.load(Ordering::Relaxed) {
                let task = {
                    let recv = receiver.lock().unwrap();
                    recv.try_recv()
                };

                match task {
                    Ok(load_task) => {
                        if let Err(e) = process_load_task(&engine, &load_task) {
                            eprintln!("Worker {}: Load failed: {}", worker_id, e);
                            ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                        } else {
                            let current = LOADED_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                            if current % PROGRESS_REPORT_INTERVAL == 0 {
                                println!("Processed {} records successfully", current);
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        // No work available, brief pause
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        // Channel closed, exit
                        break;
                    }
                }
            }
            println!("Worker {} finished", worker_id);
        });

        workers.push(worker);
    }

    // Generate and distribute work
    let start_time = Instant::now();

    println!("Generating {} load tasks...", TOTAL_RECORDS);
    for i in 1..=TOTAL_RECORDS {
        let record_id = format!("THREAD_LOAD_REC_{:06}", i);
        let data_source = "TEST".to_string();
        let record_data = format!(
            r#"{{
            "RECORD_ID": "{}",
            "DATA_SOURCE": "{}",
            "NAME_FIRST": "ThreadPerson{}",
            "NAME_LAST": "TestRecord",
            "EMAIL_ADDRESS": "thread.person.{}@example.com",
            "PHONE_NUMBER": "555-{:04}",
            "ADDR_FULL": "{} Thread Avenue, Load City, LC 30001",
            "DATE_OF_BIRTH": "199{}-{:02}-{:02}"
        }}"#,
            record_id,
            data_source,
            i,
            i,
            1000 + (i % 9000),
            i,
            i % 10,
            (i % 12) + 1,
            (i % 28) + 1
        );

        let task = LoadTask {
            record_id,
            data_source,
            record_data,
        };

        // Send task to worker threads
        if let Err(e) = sender.send(task) {
            eprintln!("Failed to send task: {}", e);
            break;
        }
    }

    // Close the channel to signal no more work
    drop(sender);

    // Wait for all workers to complete
    println!("Waiting for all workers to complete...");
    for (i, worker) in workers.into_iter().enumerate() {
        if let Err(e) = worker.join() {
            eprintln!("Worker {} panicked: {:?}", i, e);
        }
    }

    // Stop the running flag
    running.store(false, Ordering::Relaxed);

    let total_time = start_time.elapsed();

    // Print final statistics
    let loaded = LOADED_COUNT.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);

    println!("\n✅ Thread pool loading completed!");
    println!("Total time: {:.2} seconds", total_time.as_secs_f64());
    println!("Records loaded: {}", loaded);
    println!("Errors encountered: {}", errors);
    println!("Worker threads used: {}", WORKER_COUNT);

    if loaded > 0 {
        println!(
            "Average loading rate: {:.2} records/second",
            loaded as f64 / total_time.as_secs_f64()
        );
        println!(
            "Per-thread rate: {:.2} records/second/thread",
            loaded as f64 / total_time.as_secs_f64() / WORKER_COUNT as f64
        );
    }

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    Ok(())
}

/// Process a single load task using the engine
fn process_load_task(engine: &Box<dyn SzEngine>, task: &LoadTask) -> SzResult<()> {
    engine.add_record(
        &task.data_source,
        &task.record_id,
        &task.record_data,
        Some(SzFlags::ADD_RECORD_DEFAULT),
    )?;
    Ok(())
}
