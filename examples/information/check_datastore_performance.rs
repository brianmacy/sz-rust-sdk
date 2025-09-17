//! Check Datastore Performance Example
//!
//! This example demonstrates how to test Senzing datastore performance using the Senzing v4 Rust SDK.
//!
//! ## Senzing Operations Demonstrated
//!
//! * Initialize Senzing environment with automatic configuration
//! * Access diagnostic capabilities for performance testing
//! * Run repository performance benchmarks
//! * Retrieve and analyze performance metrics
//! * Display repository statistics
//!
//! ## Prerequisites
//!
//! * Senzing v4 installed at `/opt/senzing/er/`
//! * Uses SQLite database at `/tmp/G2C.db` by default
//! * No manual configuration required - automatic setup included
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example check_datastore_performance
//! ```
//!
//! ## Performance Tests
//!
//! The example runs three Senzing performance tests:
//! * Quick test (1 second) - basic performance check
//! * Standard test (3 seconds) - standard benchmark
//! * Extended test (10 seconds) - comprehensive assessment

use serde_json::Value;
use std::time::Instant;
use sz_rust_sdk::prelude::*;

/// Main entry point for the performance check example
fn main() -> SzResult<()> {
    // Enable backtrace for Senzing error debugging
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    if let Err(e) = run_performance_check() {
        // Use Senzing SDK's enhanced error reporting
        sz_rust_sdk::helpers::print_error_with_backtrace(&e);
        return Err(e);
    }

    Ok(())
}

/// Demonstrates the complete Senzing performance testing workflow
fn run_performance_check() -> SzResult<()> {
    // Step 1: Initialize Senzing environment
    // ExampleEnvironment handles complex initialization including automatic
    // configuration setup if no configuration exists in the database
    let env = ExampleEnvironment::initialize("sz-rust-sdk-performance-check")?;

    println!("=== Senzing Datastore Performance Check ===\n");

    // Step 2: Initialize the Senzing engine
    // This ensures configuration is properly set up before running diagnostics
    println!("Ensuring engine is ready...");
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    println!("✅ Engine initialized successfully");

    // Step 3: Get the diagnostic interface for performance testing
    let diagnostic = env.get_diagnostic()?;

    // Step 4: Run Senzing performance tests
    run_quick_performance_test(&*diagnostic)?;
    run_standard_performance_test(&*diagnostic)?;
    run_extended_performance_test(&*diagnostic)?;

    // Step 5: Display repository statistics
    display_repository_statistics(&*diagnostic)?;

    println!("Performance testing completed!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Runs a quick Senzing performance test (1 second duration)
fn run_quick_performance_test(diagnostic: &dyn SzDiagnostic) -> SzResult<()> {
    println!("--- Quick Performance Test (1 second) ---");

    let start_time = Instant::now();
    let test_duration = 1; // seconds

    // Call Senzing's repository performance check
    let result = diagnostic.check_repository_performance(test_duration)?;
    let elapsed = start_time.elapsed();

    println!("Test completed in {:.2} seconds", elapsed.as_secs_f64());

    // Parse and display the performance metrics returned by Senzing
    parse_and_display_performance_results(&result, "Quick Test")?;

    println!();
    Ok(())
}

fn run_standard_performance_test(diagnostic: &dyn SzDiagnostic) -> SzResult<()> {
    println!("--- Standard Performance Test (3 seconds) ---");

    let start_time = Instant::now();
    let test_duration = 3; // seconds

    println!("Running 3-second performance test...");

    let result = diagnostic.check_repository_performance(test_duration)?;
    let elapsed = start_time.elapsed();

    println!("Test completed in {:.2} seconds", elapsed.as_secs_f64());

    // Parse and display results
    parse_and_display_performance_results(&result, "Standard Test")?;

    println!();
    Ok(())
}

fn run_extended_performance_test(diagnostic: &dyn SzDiagnostic) -> SzResult<()> {
    println!("--- Extended Performance Test (10 seconds) ---");
    println!("Note: This test will take approximately 10 seconds to complete");

    let start_time = Instant::now();
    let test_duration = 10; // seconds

    // Show progress indicator
    println!("Starting extended performance test...");

    let result = diagnostic.check_repository_performance(test_duration)?;
    let elapsed = start_time.elapsed();

    println!(
        "Extended test completed in {:.2} seconds",
        elapsed.as_secs_f64()
    );

    // Parse and display results
    parse_and_display_performance_results(&result, "Extended Test")?;

    println!();
    Ok(())
}

/// Parses and displays Senzing performance test results
///
/// Processes the JSON performance data returned by Senzing's check_repository_performance
/// and displays key metrics in a readable format.
fn parse_and_display_performance_results(result_json: &str, test_name: &str) -> SzResult<()> {
    match serde_json::from_str::<Value>(result_json) {
        Ok(perf_data) => {
            println!("{} Results:", test_name);

            // Display test duration
            if let Some(duration) = perf_data.get("durationSeconds") {
                println!("  Duration: {} seconds", duration);
            }

            // Display operations per second metric
            if let Some(ops_per_sec) = perf_data.get("operationsPerSecond") {
                println!(
                    "  Operations/Second: {:.2}",
                    ops_per_sec.as_f64().unwrap_or(0.0)
                );
            }

            // Total operations
            if let Some(total_ops) = perf_data.get("totalOperations") {
                println!("  Total Operations: {}", total_ops);
            }

            // Average response time
            if let Some(avg_response) = perf_data.get("averageResponseTimeMs") {
                println!(
                    "  Average Response Time: {:.2} ms",
                    avg_response.as_f64().unwrap_or(0.0)
                );
            }

            // Min/Max response times
            if let Some(min_response) = perf_data.get("minResponseTimeMs") {
                println!(
                    "  Min Response Time: {:.2} ms",
                    min_response.as_f64().unwrap_or(0.0)
                );
            }

            if let Some(max_response) = perf_data.get("maxResponseTimeMs") {
                println!(
                    "  Max Response Time: {:.2} ms",
                    max_response.as_f64().unwrap_or(0.0)
                );
            }

            // Database performance metrics
            if let Some(db_metrics) = perf_data.get("databaseMetrics") {
                println!("  Database Metrics:");
                if let Some(db_obj) = db_metrics.as_object() {
                    for (metric, value) in db_obj {
                        match value {
                            Value::Number(n) => println!("    {}: {}", metric, n),
                            Value::String(s) => println!("    {}: {}", metric, s),
                            _ => println!("    {}: {:?}", metric, value),
                        }
                    }
                }
            }

            // System resource usage
            if let Some(resources) = perf_data.get("systemResources") {
                println!("  System Resources:");
                if let Some(cpu_usage) = resources.get("cpuUsagePercent") {
                    println!("    CPU Usage: {:.1}%", cpu_usage.as_f64().unwrap_or(0.0));
                }
                if let Some(memory_usage) = resources.get("memoryUsageMB") {
                    println!(
                        "    Memory Usage: {:.1} MB",
                        memory_usage.as_f64().unwrap_or(0.0)
                    );
                }
            }

            // Performance rating/assessment
            assess_performance(&perf_data)?;
        }
        Err(e) => {
            println!("Could not parse performance results: {}", e);
            println!("Raw performance data:");
            println!("{}", result_json);
        }
    }

    Ok(())
}

fn assess_performance(perf_data: &Value) -> SzResult<()> {
    // Provide basic performance assessment based on operations per second
    if let Some(ops_per_sec) = perf_data.get("operationsPerSecond")
        && let Some(ops_rate) = ops_per_sec.as_f64()
    {
        println!("  Performance Assessment:");

        match ops_rate {
            rate if rate >= 1000.0 => {
                println!("    ✓ Excellent performance ({:.0} ops/sec)", rate)
            }
            rate if rate >= 500.0 => println!("    ✓ Good performance ({:.0} ops/sec)", rate),
            rate if rate >= 100.0 => println!("    ⚠ Fair performance ({:.0} ops/sec)", rate),
            rate if rate >= 50.0 => {
                println!("    ⚠ Below average performance ({:.0} ops/sec)", rate)
            }
            rate => println!(
                "    ✗ Poor performance ({:.0} ops/sec) - consider optimization",
                rate
            ),
        }
    }

    // Check average response time
    if let Some(avg_response) = perf_data.get("averageResponseTimeMs")
        && let Some(response_time) = avg_response.as_f64()
    {
        match response_time {
            time if time <= 10.0 => println!("    ✓ Excellent response time ({:.2} ms)", time),
            time if time <= 50.0 => println!("    ✓ Good response time ({:.2} ms)", time),
            time if time <= 100.0 => println!("    ⚠ Fair response time ({:.2} ms)", time),
            time if time <= 500.0 => println!("    ⚠ Slow response time ({:.2} ms)", time),
            time => println!("    ✗ Very slow response time ({:.2} ms)", time),
        }
    }

    Ok(())
}

fn display_repository_statistics(diagnostic: &dyn SzDiagnostic) -> SzResult<()> {
    println!("--- Repository Information ---");

    let repo_info = diagnostic.get_repository_info()?;

    match serde_json::from_str::<Value>(&repo_info) {
        Ok(repo_json) => {
            // Display key repository statistics
            if let Some(entity_count) = repo_json.get("entityCount") {
                println!("Total Entities: {}", entity_count);
            }

            if let Some(record_count) = repo_json.get("recordCount") {
                println!("Total Records: {}", record_count);
            }

            if let Some(relationship_count) = repo_json.get("relationshipCount") {
                println!("Total Relationships: {}", relationship_count);
            }

            // Database size information
            if let Some(db_size) = repo_json.get("databaseSize")
                && let Some(size_mb) = db_size.get("sizeMB")
            {
                println!("Database Size: {:.1} MB", size_mb.as_f64().unwrap_or(0.0));
            }

            // Data source breakdown
            if let Some(data_sources) = repo_json.get("dataSources")
                && let Some(ds_array) = data_sources.as_array()
            {
                println!("Data Sources: {}", ds_array.len());
                for ds in ds_array.iter().take(5) {
                    // Show first 5
                    if let Some(ds_code) = ds.get("dataSourceCode") {
                        let record_count = ds
                            .get("recordCount")
                            .and_then(|rc| rc.as_i64())
                            .unwrap_or(0);
                        println!("  {}: {} records", ds_code, record_count);
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not parse repository information: {}", e);
            println!("Raw repository info: {}", repo_info);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_check() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Performance check should succeed");
        }
    }

    #[test]
    fn test_performance_assessment() {
        let sample_perf = serde_json::json!({
            "operationsPerSecond": 750.5,
            "averageResponseTimeMs": 25.3,
            "durationSeconds": 30
        });

        let result = assess_performance(&sample_perf);
        assert!(result.is_ok(), "Should assess sample performance data");
    }

    #[test]
    fn test_parse_performance_results() {
        let sample_result = r#"{
            "durationSeconds": 30,
            "operationsPerSecond": 500.0,
            "totalOperations": 15000,
            "averageResponseTimeMs": 50.0
        }"#;

        let result = parse_and_display_performance_results(sample_result, "Test");
        assert!(result.is_ok(), "Should parse sample performance results");
    }
}
