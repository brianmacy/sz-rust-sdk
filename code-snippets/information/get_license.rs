//! Get License Example
//!
//! This example demonstrates how to retrieve licensing information from the
//! Senzing engine, including license details, record limits, and expiration.
//!
//! Rust equivalent of: information/GetLicense/Program.cs

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

    println!("Get License Information Example");
    println!("===============================");

    // Get the product interface from the environment
    let product = environment.get_product()?;

    // Retrieve license information
    println!("\n1. Retrieving license information...");
    let license_info = product.get_license()?;

    println!("Raw license information:");
    println!("{}", license_info);

    // Parse and display license details
    println!("\n2. Parsed license details:");
    parse_and_display_license(&license_info)?;

    // Also get version information for context
    println!("\n3. Related version information:");
    let version_info = product.get_version()?;
    parse_and_display_version(&version_info)?;

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    println!("\n✅ Get License example completed successfully!");

    Ok(())
}

/// Parse and display license information in a user-friendly format
fn parse_and_display_license(license_json: &str) -> SzResult<()> {
    let license: Value = serde_json::from_str(license_json)
        .map_err(|e| SzError::bad_input(format!("Failed to parse license JSON: {}", e)))?;

    // Fields documented for the getLicense response (all strings except the
    // integers recordLimit and advSearch); the raw JSON above shows every field.
    let text = |key: &str| license.get(key).and_then(|v| v.as_str()).unwrap_or("");

    println!("📋 Customer: {}", text("customer"));
    println!("📝 Contract: {}", text("contract"));
    println!("📄 License Type: {}", text("licenseType"));
    println!("🏷️  License Level: {}", text("licenseLevel"));
    println!("🕐 Issued: {}", text("issueDate"));
    println!("⏰ Expires: {}", text("expireDate"));

    if let Some(record_limit) = license.get("recordLimit").and_then(|v| v.as_i64()) {
        if record_limit > 0 {
            println!("📊 Record Limit: {}", format_number(record_limit));
        } else {
            println!("📊 Record Limit: Unlimited");
        }
    }

    if let Some(adv_search) = license.get("advSearch").and_then(|v| v.as_i64()) {
        println!("🔎 Advanced Search: {}", adv_search);
    }

    Ok(())
}

/// Parse and display version information for context
fn parse_and_display_version(version_json: &str) -> SzResult<()> {
    let version: Value = serde_json::from_str(version_json)
        .map_err(|e| SzError::bad_input(format!("Failed to parse version JSON: {}", e)))?;

    if let Some(product_name) = version.get("PRODUCT_NAME").and_then(|v| v.as_str()) {
        println!("🏷️  Product: {}", product_name);
    }

    if let Some(version_string) = version.get("VERSION").and_then(|v| v.as_str()) {
        println!("🔢 Version: {}", version_string);
    }

    if let Some(build_version) = version.get("BUILD_VERSION").and_then(|v| v.as_str()) {
        println!("🏗️  Build: {}", build_version);
    }

    if let Some(build_date) = version.get("BUILD_DATE").and_then(|v| v.as_str()) {
        println!("📅 Build Date: {}", build_date);
    }

    if let Some(compatibility) = version.get("COMPATIBILITY_VERSION") {
        if let Some(config_version) = compatibility.get("CONFIG_VERSION").and_then(|v| v.as_str()) {
            println!("⚙️  Config Version: {}", config_version);
        }
    }

    Ok(())
}

/// Format a number with commas for readability
fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}
