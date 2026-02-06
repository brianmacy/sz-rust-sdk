#![allow(clippy::borrowed_box)]
//! Database Demo Example
//!
//! This example demonstrates how to retrieve database performance statistics
//! and system information from the Senzing engine, including datastore
//! performance metrics and configuration details.
//!
//! Rust equivalent of: information/DatabaseDemo/Program.cs

use serde_json::Value;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

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

    println!("Retrieving database and system information...");

    // Initialize the engine with automatic setup to ensure proper configuration
    let _engine = ExampleEnvironment::get_engine_with_setup(&environment)?;

    // Get diagnostic and config manager components
    let diagnostic = environment.get_diagnostic()?;
    let config_manager = environment.get_config_manager()?;

    // Display various types of database information
    display_repository_info(&diagnostic)?;
    display_database_info(&diagnostic)?;
    display_active_config_id(&config_manager)?;

    println!("✅ Database information retrieved successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    Ok(())
}

/// Display repository information and statistics
fn display_repository_info(diagnostic: &Box<dyn SzDiagnostic>) -> SzResult<()> {
    println!("\n--- Repository Information ---");

    // Get repository information (handle errors gracefully)
    let repo_json = match diagnostic.get_repository_info() {
        Ok(json) => json,
        Err(e) => {
            println!("Repository information not available: {}", e);
            return Ok(());
        }
    };

    // Parse JSON to display formatted information
    let repo_info: Value = serde_json::from_str(&repo_json)
        .map_err(|e| SzError::unknown(format!("Failed to parse repository JSON: {}", e)))?;

    // Display data sources
    if let Some(data_sources) = repo_info.get("datastore") {
        println!("Datastore Information:");
        if let Ok(pretty_json) = serde_json::to_string_pretty(data_sources) {
            println!("{}", pretty_json);
        }
    }

    // Display entity information
    if let Some(entities) = repo_info.get("entities") {
        if let Some(entity_count) = entities.get("count").and_then(|v| v.as_i64()) {
            println!("Total Entities: {}", entity_count);
        }
    }

    // Display record information
    if let Some(records) = repo_info.get("records") {
        if let Some(record_count) = records.get("count").and_then(|v| v.as_i64()) {
            println!("Total Records: {}", record_count);
        }
    }

    Ok(())
}

/// Display statistics for a specific database operation type
#[allow(dead_code)]
fn display_operation_stats(operation_name: &str, stats: &Value) {
    println!("  {}:", operation_name);

    if let Some(count) = stats.get("count").and_then(|v| v.as_i64()) {
        println!("    Count: {}", count);
    }

    if let Some(avg_time) = stats.get("averageTime").and_then(|v| v.as_f64()) {
        println!("    Average Time: {:.2} ms", avg_time);
    }

    if let Some(min_time) = stats.get("minTime").and_then(|v| v.as_f64()) {
        println!("    Min Time: {:.2} ms", min_time);
    }

    if let Some(max_time) = stats.get("maxTime").and_then(|v| v.as_f64()) {
        println!("    Max Time: {:.2} ms", max_time);
    }

    if let Some(total_time) = stats.get("totalTime").and_then(|v| v.as_f64()) {
        println!("    Total Time: {:.2} ms", total_time);
    }
}

/// Display general database information
fn display_database_info(diagnostic: &Box<dyn SzDiagnostic>) -> SzResult<()> {
    println!("\n--- Database Performance Check ---");

    // Check repository performance
    let performance_json = diagnostic.check_repository_performance(1)?;
    let performance_info: Value = serde_json::from_str(&performance_json)
        .map_err(|e| SzError::unknown(format!("Failed to parse performance JSON: {}", e)))?;

    println!("Repository Performance Check Results:");
    if let Ok(pretty_json) = serde_json::to_string_pretty(&performance_info) {
        println!("{}", pretty_json);
    }

    // Get feature information if available (handle errors gracefully)
    let feature_json = match diagnostic.get_feature(1) {
        Ok(json) => json,
        Err(e) => {
            println!("\nFeature information not available: {}", e);
            return Ok(());
        }
    };
    let feature_info: Value = serde_json::from_str(&feature_json)
        .map_err(|e| SzError::unknown(format!("Failed to parse feature JSON: {}", e)))?;

    println!("\nFeature Information:");
    if let Some(feature_id) = feature_info.get("LIB_FEAT_ID").and_then(|v| v.as_i64()) {
        println!("Feature ID: {}", feature_id);
    }

    if let Some(feature_desc) = feature_info.get("FELEM_DESC").and_then(|v| v.as_str()) {
        println!("Feature Description: {}", feature_desc);
    }

    Ok(())
}

/// Display active configuration ID
fn display_active_config_id(config_manager: &Box<dyn SzConfigManager>) -> SzResult<()> {
    println!("\n--- Active Configuration ---");

    // Get active configuration ID from config manager
    let active_config_id = config_manager.get_default_config_id()?;
    println!("Active Config ID: {}", active_config_id);

    Ok(())
}
