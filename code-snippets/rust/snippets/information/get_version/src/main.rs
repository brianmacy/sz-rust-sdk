//! Get Version Example
//!
//! This example demonstrates how to retrieve version and license information
//! from the Senzing SDK, including API version and build details.
//!
//! Rust equivalent of: information/GetVersion/Program.cs

use sz_rust_sdk::prelude::*;
use serde_json::Value;

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

    println!("Retrieving Senzing version and license information...");

    // Get the product component from the environment
    let product = environment.get_product()?;

    // Display version information
    display_version_info(&product)?;

    // Display license information
    display_license_info(&product)?;

    println!("✅ Version information retrieved successfully!");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}

/// Display version information
fn display_version_info(product: &Box<dyn SzProduct>) -> SzResult<()> {
    println!("\n--- Version Information ---");

    // Get version as JSON string
    let version_json = product.get_version()?;

    // Parse JSON to display formatted information
    let version_info: Value = serde_json::from_str(&version_json)
        .map_err(|e| SzError::unknown(&format!("Failed to parse version JSON: {}", e)))?;

    // Display API version
    if let Some(api_version) = version_info.get("API_VERSION").and_then(|v| v.as_str()) {
        println!("API Version: {}", api_version);
    }

    // Display native API version
    if let Some(native_api_version) = version_info.get("NATIVE_API_VERSION").and_then(|v| v.as_str()) {
        println!("Native API Version: {}", native_api_version);
    }

    // Display build version
    if let Some(build_version) = version_info.get("BUILD_VERSION").and_then(|v| v.as_str()) {
        println!("Build Version: {}", build_version);
    }

    // Display build date
    if let Some(build_date) = version_info.get("BUILD_DATE").and_then(|v| v.as_str()) {
        println!("Build Date: {}", build_date);
    }

    // Display build number
    if let Some(build_number) = version_info.get("BUILD_NUMBER").and_then(|v| v.as_str()) {
        println!("Build Number: {}", build_number);
    }

    // Display compatibility version
    if let Some(compatibility_version) = version_info.get("COMPATIBILITY_VERSION") {
        if let Some(config_version) = compatibility_version.get("CONFIG_VERSION").and_then(|v| v.as_str()) {
            println!("Config Compatibility Version: {}", config_version);
        }
    }

    // Display the raw JSON for complete information
    println!("\nRaw Version JSON:");
    if let Ok(pretty_json) = serde_json::to_string_pretty(&version_info) {
        println!("{}", pretty_json);
    } else {
        println!("{}", version_json);
    }

    Ok(())
}

/// Display license information
fn display_license_info(product: &Box<dyn SzProduct>) -> SzResult<()> {
    println!("\n--- License Information ---");

    // Get license information as JSON string
    let license_json = product.get_license()?;

    // Parse JSON to display formatted information
    let license_info: Value = serde_json::from_str(&license_json)
        .map_err(|e| SzError::unknown(&format!("Failed to parse license JSON: {}", e)))?;

    // Display customer information
    if let Some(customer) = license_info.get("customer").and_then(|v| v.as_str()) {
        println!("Customer: {}", customer);
    }

    // Display contract information
    if let Some(contract) = license_info.get("contract").and_then(|v| v.as_str()) {
        println!("Contract: {}", contract);
    }

    // Display issue date
    if let Some(issue_date) = license_info.get("issueDate").and_then(|v| v.as_str()) {
        println!("Issue Date: {}", issue_date);
    }

    // Display license type
    if let Some(license_type) = license_info.get("licenseType").and_then(|v| v.as_str()) {
        println!("License Type: {}", license_type);
    }

    // Display license level
    if let Some(license_level) = license_info.get("licenseLevel").and_then(|v| v.as_str()) {
        println!("License Level: {}", license_level);
    }

    // Display billing information
    if let Some(billing) = license_info.get("billing").and_then(|v| v.as_str()) {
        println!("Billing: {}", billing);
    }

    // Display expiration date
    if let Some(expire_date) = license_info.get("expireDate").and_then(|v| v.as_str()) {
        println!("Expiration Date: {}", expire_date);
    }

    // Display record limit
    if let Some(record_limit) = license_info.get("recordLimit").and_then(|v| v.as_i64()) {
        if record_limit > 0 {
            println!("Record Limit: {}", record_limit);
        } else {
            println!("Record Limit: Unlimited");
        }
    }

    // Display the raw JSON for complete license information
    println!("\nRaw License JSON:");
    if let Ok(pretty_json) = serde_json::to_string_pretty(&license_info) {
        println!("{}", pretty_json);
    } else {
        println!("{}", license_json);
    }

    Ok(())
}
