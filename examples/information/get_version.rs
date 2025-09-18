//! Get Version Example
//!
//! This example demonstrates how to retrieve Senzing product version
//! and license information.
//!
//! Run with: cargo run --example get_version

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    // Initialize Senzing SDK using ExampleEnvironment helper
    println!("Initializing Senzing SDK...");
    let env = ExampleEnvironment::initialize("get-version-example")?;
    println!("✅ Environment initialized successfully!");

    match env.get_product() {
        Ok(product) => {
            println!("\n=== Senzing Version Information ===");
            match product.get_version() {
                Ok(version) => println!("{}", version),
                Err(e) => println!("⚠️  Version info not available: {}", e),
            }

            println!("\n=== Senzing License Information ===");
            match product.get_license() {
                Ok(license) => println!("{}", license),
                Err(e) => println!("⚠️  License info not available: {}", e),
            }
        }
        Err(e) => {
            println!("⚠️  Product component not available: {}", e);
            println!("This indicates Senzing needs proper configuration.");
        }
    }

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
