//! Get License Example
//!
//! This example demonstrates how to retrieve licensing information from the
//! Senzing engine, including license details, record limits, and expiration.
//!
//! Rust equivalent of: information/GetLicense/Program.cs

use serde_json::Value;
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
    ExampleEnvironment::cleanup()?;

    println!("\n✅ Get License example completed successfully!");

    Ok(())
}

/// Parse and display license information in a user-friendly format
fn parse_and_display_license(license_json: &str) -> SzResult<()> {
    let license: Value = serde_json::from_str(license_json)
        .map_err(|e| SzError::bad_input(format!("Failed to parse license JSON: {}", e)))?;

    // Display license type and customer information
    if let Some(customer) = license.get("customer").and_then(|v| v.as_str()) {
        println!("📋 Customer: {}", customer);
    }

    if let Some(license_type) = license.get("licenseType").and_then(|v| v.as_str()) {
        println!("📄 License Type: {}", license_type);
    }

    // Display contract information
    if let Some(contract) = license.get("contract") {
        if let Some(contract_id) = contract.get("id").and_then(|v| v.as_str()) {
            println!("📝 Contract ID: {}", contract_id);
        }

        if let Some(start_date) = contract.get("startDate").and_then(|v| v.as_str()) {
            println!("🕐 Contract Start: {}", start_date);
        }

        if let Some(end_date) = contract.get("endDate").and_then(|v| v.as_str()) {
            println!("⏰ Contract End: {}", end_date);
        }
    }

    // Display record limits
    if let Some(record_limit) = license.get("recordLimit").and_then(|v| v.as_i64()) {
        if record_limit > 0 {
            println!("📊 Record Limit: {}", format_number(record_limit));
        } else {
            println!("📊 Record Limit: Unlimited");
        }
    }

    // Display expiration information
    if let Some(expiration_warning) = license
        .get("expirationWarningDays")
        .and_then(|v| v.as_i64())
    {
        println!("⚠️  Expiration Warning: {} days", expiration_warning);
    }

    // Display any warnings or status
    if let Some(warning) = license.get("warning").and_then(|v| v.as_str()) {
        println!("🔔 Warning: {}", warning);
    }

    if let Some(billing) = license.get("billing") {
        if let Some(billing_type) = billing.get("billingType").and_then(|v| v.as_str()) {
            println!("💳 Billing Type: {}", billing_type);
        }
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
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}
