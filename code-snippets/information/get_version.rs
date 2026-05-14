//! Get Version Example
//!
//! This example demonstrates how to get Senzing version information.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Environment initialization
//! - Getting the product interface
//! - Retrieving version information with get_version()
//! - Processing JSON version data

use serde_json::Value;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment
    let env = get_environment()?;

    // Step 2: Get the product interface for version information
    let product = env.get_product()?;

    println!("Getting Senzing version information...");

    // Step 3: Get version information
    // get_version() returns JSON with version details
    let version_json = product.get_version()?;
    println!("✓ Retrieved version information");

    // Step 4: Parse and display the version information
    if let Ok(version_data) = serde_json::from_str::<Value>(&version_json) {
        // Display key version information
        if let Some(product_name) = version_data["PRODUCT_NAME"].as_str() {
            println!("Product: {}", product_name);
        }

        if let Some(version) = version_data["VERSION"].as_str() {
            println!("Version: {}", version);
        }

        if let Some(build_version) = version_data["BUILD_VERSION"].as_str() {
            println!("Build: {}", build_version);
        }

        if let Some(build_date) = version_data["BUILD_DATE"].as_str() {
            println!("Build Date: {}", build_date);
        }
    } else {
        // If JSON parsing fails, just display the raw JSON
        println!("Raw version info:\n{}", version_json);
    }

    println!("✅ Version information retrieved successfully");

    Ok(())
}

/// Simple helper to get a configured Senzing environment
/// Handles database setup and configuration automatically
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("get_version_example")
}
