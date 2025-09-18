#![allow(clippy::borrowed_box)]
//! Search Thread Pool Example
//!
//! This example demonstrates thread pool patterns for concurrent search operations
//! using real OS threads for maximum Senzing performance. Each worker thread
//! gets its own engine instance for parallel search processing.
//!
//! Thread pool patterns for search operations

use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::prelude::*;

// Global counters for tracking search statistics
static SEARCHES_COMPLETED: AtomicUsize = AtomicUsize::new(0);
static MATCHES_FOUND: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);

// Configuration constants
const SEARCH_QUERIES: usize = 20; // Number of search queries to execute
const WORKER_COUNT: usize = 4; // Number of concurrent worker threads
const CHANNEL_BUFFER_SIZE: usize = 25; // Channel buffer size
const STATS_REPORT_INTERVAL: usize = 5; // Report stats every N searches

#[derive(Debug, Clone)]
struct SearchTask {
    search_id: usize,
    search_name: String,
    search_attributes: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SearchResult {
    search_id: usize,
    search_name: String,
    match_count: usize,
    search_time_ms: f64,
    results: String,
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

    println!("Thread Pool Search Example");
    println!("===========================");
    println!(
        "Executing {} search queries using {} OS threads for maximum performance",
        SEARCH_QUERIES, WORKER_COUNT
    );

    // First, load some test data for searching
    println!("\n1. Loading test data for searching...");
    let main_engine = ExampleEnvironment::get_engine_with_setup(&environment)?;
    load_test_data(&main_engine)?;

    // Create channels for distributing search tasks and collecting results
    let (task_sender, task_receiver) = mpsc::sync_channel::<SearchTask>(CHANNEL_BUFFER_SIZE);
    let (result_sender, result_receiver) = mpsc::sync_channel::<SearchResult>(CHANNEL_BUFFER_SIZE);

    let task_receiver = Arc::new(std::sync::Mutex::new(task_receiver));
    let running = Arc::new(AtomicBool::new(true));

    println!(
        "\n2. Starting {} worker threads for concurrent search operations...",
        WORKER_COUNT
    );

    // Spawn worker threads
    let mut workers = Vec::new();
    for worker_id in 0..WORKER_COUNT {
        let task_receiver = Arc::clone(&task_receiver);
        let result_sender = result_sender.clone();
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

            // Process search tasks from the channel
            while running.load(Ordering::Relaxed) {
                let task = {
                    let recv = task_receiver.lock().unwrap();
                    recv.try_recv()
                };

                match task {
                    Ok(search_task) => {
                        let result = process_search_task(&engine, &search_task);
                        match result {
                            Ok(search_result) => {
                                let current =
                                    SEARCHES_COMPLETED.fetch_add(1, Ordering::Relaxed) + 1;
                                MATCHES_FOUND
                                    .fetch_add(search_result.match_count, Ordering::Relaxed);

                                if current % STATS_REPORT_INTERVAL == 0 {
                                    println!(
                                        "Worker {}: Completed {} searches",
                                        worker_id, current
                                    );
                                }

                                if result_sender.send(search_result).is_err() {
                                    // Result channel closed, exit
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("Worker {}: Search failed: {}", worker_id, e);
                                ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
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

    // Generate and distribute search tasks
    let start_time = Instant::now();

    println!("\n3. Generating {} search tasks...", SEARCH_QUERIES);
    let search_tasks = generate_search_tasks();

    for task in search_tasks {
        if let Err(e) = task_sender.send(task) {
            eprintln!("Failed to send search task: {}", e);
            break;
        }
    }

    // Close task channel to signal no more work
    drop(task_sender);

    // Collect results
    let mut search_results = Vec::new();
    drop(result_sender); // Close the sender so receiver will know when to stop

    // Wait for all workers to complete
    println!("Waiting for all workers to complete...");
    for (i, worker) in workers.into_iter().enumerate() {
        if let Err(e) = worker.join() {
            eprintln!("Worker {} panicked: {:?}", i, e);
        }
    }

    // Collect remaining results
    while let Ok(result) = result_receiver.try_recv() {
        search_results.push(result);
    }

    running.store(false, Ordering::Relaxed);
    let total_time = start_time.elapsed();

    // Analyze and display results
    println!("\n4. Search Results Analysis:");
    analyze_search_results(&search_results, total_time);

    // Print final statistics
    let searches = SEARCHES_COMPLETED.load(Ordering::Relaxed);
    let matches = MATCHES_FOUND.load(Ordering::Relaxed);
    let errors = ERROR_COUNT.load(Ordering::Relaxed);

    println!("\n✅ Thread pool search operations completed!");
    println!("Total time: {:.2} seconds", total_time.as_secs_f64());
    println!("Searches completed: {}", searches);
    println!("Total matches found: {}", matches);
    println!("Errors encountered: {}", errors);
    println!("Worker threads used: {}", WORKER_COUNT);

    if searches > 0 {
        println!(
            "Average search rate: {:.2} searches/second",
            searches as f64 / total_time.as_secs_f64()
        );
        println!(
            "Per-thread rate: {:.2} searches/second/thread",
            searches as f64 / total_time.as_secs_f64() / WORKER_COUNT as f64
        );
    }

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Load test data for searching
fn load_test_data(engine: &Box<dyn SzEngine>) -> SzResult<()> {
    let test_records = vec![
        (
            r#"{"RECORD_ID": "SEARCH_TEST_001", "DATA_SOURCE": "TEST", "NAME_FIRST": "Alice", "NAME_LAST": "Johnson", "EMAIL_ADDRESS": "alice.johnson@email.com", "PHONE_NUMBER": "555-0001"}"#,
            "SEARCH_TEST_001",
        ),
        (
            r#"{"RECORD_ID": "SEARCH_TEST_002", "DATA_SOURCE": "TEST", "NAME_FIRST": "Bob", "NAME_LAST": "Smith", "EMAIL_ADDRESS": "bob.smith@email.com", "PHONE_NUMBER": "555-0002"}"#,
            "SEARCH_TEST_002",
        ),
        (
            r#"{"RECORD_ID": "SEARCH_TEST_003", "DATA_SOURCE": "TEST", "NAME_FIRST": "Carol", "NAME_LAST": "Williams", "EMAIL_ADDRESS": "carol.williams@email.com", "PHONE_NUMBER": "555-0003"}"#,
            "SEARCH_TEST_003",
        ),
        (
            r#"{"RECORD_ID": "SEARCH_TEST_004", "DATA_SOURCE": "TEST", "NAME_FIRST": "David", "NAME_LAST": "Brown", "EMAIL_ADDRESS": "david.brown@email.com", "PHONE_NUMBER": "555-0004"}"#,
            "SEARCH_TEST_004",
        ),
        (
            r#"{"RECORD_ID": "SEARCH_TEST_005", "DATA_SOURCE": "TEST", "NAME_FIRST": "Emma", "NAME_LAST": "Davis", "EMAIL_ADDRESS": "emma.davis@email.com", "PHONE_NUMBER": "555-0005"}"#,
            "SEARCH_TEST_005",
        ),
    ];

    for (record_data, record_id) in test_records {
        engine.add_record(
            "TEST",
            record_id,
            record_data,
            Some(SzFlags::ADD_RECORD_DEFAULT),
        )?;
    }

    println!("Loaded {} test records for searching", 5);
    Ok(())
}

/// Generate search tasks
fn generate_search_tasks() -> Vec<SearchTask> {
    let search_patterns = vec![
        ("Name Search - Alice", r#"{"NAME_FIRST": "Alice"}"#),
        ("Name Search - Smith", r#"{"NAME_LAST": "Smith"}"#),
        (
            "Email Search",
            r#"{"EMAIL_ADDRESS": "alice.johnson@email.com"}"#,
        ),
        ("Phone Search", r#"{"PHONE_NUMBER": "555-0002"}"#),
        (
            "Combined Search",
            r#"{"NAME_FIRST": "Bob", "NAME_LAST": "Smith"}"#,
        ),
        ("Partial Name", r#"{"NAME_FIRST": "Car"}"#),
        ("Domain Search", r#"{"EMAIL_ADDRESS": "@email.com"}"#),
        ("Phone Pattern", r#"{"PHONE_NUMBER": "555-000"}"#),
        (
            "Full Name",
            r#"{"NAME_FIRST": "David", "NAME_LAST": "Brown"}"#,
        ),
        ("Empty Search", r#"{"NAME_FIRST": "NonExistent"}"#),
    ];

    let mut tasks = Vec::new();
    for i in 0..SEARCH_QUERIES {
        let (name, attributes) = &search_patterns[i % search_patterns.len()];
        tasks.push(SearchTask {
            search_id: i + 1,
            search_name: format!("{} #{}", name, i + 1),
            search_attributes: attributes.to_string(),
        });
    }

    tasks
}

/// Process a single search task
fn process_search_task(engine: &Box<dyn SzEngine>, task: &SearchTask) -> SzResult<SearchResult> {
    let start_time = Instant::now();

    let search_response = engine.search_by_attributes(
        &task.search_attributes,
        None, // search_profile
        Some(SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT),
    )?;

    let search_time = start_time.elapsed();

    // Parse the response to count matches
    let match_count = match serde_json::from_str::<Value>(&search_response) {
        Ok(json) => {
            if let Some(entities) = json.get("RESOLVED_ENTITIES") {
                if let Some(array) = entities.as_array() {
                    array.len()
                } else {
                    0
                }
            } else {
                0
            }
        }
        Err(_) => 0,
    };

    Ok(SearchResult {
        search_id: task.search_id,
        search_name: task.search_name.clone(),
        match_count,
        search_time_ms: search_time.as_millis() as f64,
        results: search_response,
    })
}

/// Analyze and display search results
fn analyze_search_results(results: &[SearchResult], total_time: Duration) {
    if results.is_empty() {
        println!("No search results to analyze");
        return;
    }

    let total_matches: usize = results.iter().map(|r| r.match_count).sum();
    let avg_search_time: f64 =
        results.iter().map(|r| r.search_time_ms as f64).sum::<f64>() / results.len() as f64;

    let fastest = results.iter().min_by(|a, b| a.search_time_ms.partial_cmp(&b.search_time_ms).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    let slowest = results.iter().max_by(|a, b| a.search_time_ms.partial_cmp(&b.search_time_ms).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    let most_matches = results.iter().max_by_key(|r| r.match_count).unwrap();

    println!("📊 Search Performance Analysis:");
    println!("  Searches executed: {}", results.len());
    println!("  Total matches found: {}", total_matches);
    println!("  Average search time: {:.2}ms", avg_search_time);
    println!(
        "  Fastest search: {} ({:.2}ms)",
        fastest.search_name, fastest.search_time_ms
    );
    println!(
        "  Slowest search: {} ({:.2}ms)",
        slowest.search_name, slowest.search_time_ms
    );
    println!(
        "  Most productive: {} ({} matches)",
        most_matches.search_name, most_matches.match_count
    );

    if total_time.as_secs_f64() > 0.0 {
        println!(
            "  Concurrent efficiency: {:.2}x faster than sequential",
            avg_search_time * results.len() as f64 / 1000.0 / total_time.as_secs_f64()
        );
    }
}
