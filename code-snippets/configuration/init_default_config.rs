//! Initialize Default Configuration Example
//!
//! This example demonstrates how to create and set a default Senzing configuration
//! using the SetDefaultConfig method, which creates, registers, and sets the
//! configuration as default in a single operation.
//!
//! Rust equivalent of C# SzConfigManagerDemo.SetDefaultConfig pattern

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    println!("Initialize Default Configuration Example");
    println!("=======================================");

    // Initialize the Senzing environment
    let environment = ExampleEnvironment::initialize(instance_name)?;

    // Get configuration manager to manage default configuration
    let config_manager = environment.get_config_manager()?;

    // Step 1: Check current default configuration (if any)
    println!("1. Checking for existing default configuration...");
    let current_default = config_manager.get_default_config_id()?;
    if current_default > 0 {
        println!("   Current default configuration ID: {}", current_default);
    } else {
        println!("   No default configuration currently set");
    }

    // Step 2: Create a basic default configuration
    println!("2. Creating new default configuration...");
    let config = config_manager.create_config()?;
    let config_json = config.export()?;
    println!("   Configuration created ({} bytes)", config_json.len());

    // Step 3: Set this as the default configuration using SetDefaultConfig
    // This method creates, registers, and sets as default in one operation
    println!("3. Setting new configuration as default...");
    let config_comment = format!("Default configuration initialized by {}", instance_name);
    let new_default_id = config_manager.set_default_config(&config_json, Some(&config_comment))?;
    println!("✅ New default configuration set with ID: {}", new_default_id);

    // Step 4: Verify the default configuration
    println!("4. Verifying default configuration...");
    let verified_default = config_manager.get_default_config_id()?;
    println!("✅ Verified default configuration ID: {}", verified_default);

    // Step 5: Reinitialize environment with new default configuration
    println!("5. Reinitializing environment with new default...");
    environment.reinitialize(new_default_id)?;
    println!("✅ Environment reinitialized with new default configuration");

    println!("\n✅ Default configuration successfully initialized!");
    println!("   Configuration ID: {}", new_default_id);
    println!("   The Senzing environment now has a properly initialized default configuration.");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}
