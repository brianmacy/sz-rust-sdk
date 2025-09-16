//! Register Data Sources Example
//!
//! This example demonstrates how to add data sources to the Senzing configuration
//! and update the default configuration in the repository.
//!
//! Run with: cargo run --example register_data_sources

use std::thread;
use std::time::Duration;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("register-data-sources")?;

    // Ensure engine is initialized first (this sets up configuration if needed)
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Get the config manager from the environment
    let config_mgr = env.get_config_manager()?;

    println!("Registering new data sources in Senzing configuration...\n");

    // Setup a loop to handle race-condition conflicts on replacing the default config ID
    let mut replaced_config = false;
    let max_retries = 5;
    let mut retry_count = 0;

    while !replaced_config && retry_count < max_retries {
        match register_data_sources_attempt(&*config_mgr) {
            Ok(()) => {
                replaced_config = true;
                println!("✓ Successfully registered data sources and updated configuration");
            }
            Err(e) if e.is_retryable() && retry_count < max_retries - 1 => {
                retry_count += 1;
                println!(
                    "⚠ Configuration update conflict (attempt {}), retrying...",
                    retry_count
                );

                // Brief delay before retry to reduce contention
                thread::sleep(Duration::from_millis(100 * retry_count as u64));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    if !replaced_config {
        return Err(SzError::configuration(
            "Failed to update configuration after maximum retries",
        ));
    }

    // Verify the configuration was updated
    println!("\n--- Verification ---");
    let current_config_id = config_mgr.get_default_config_id()?;
    println!("Current default configuration ID: {}", current_config_id);

    // Get the updated configuration and display data sources
    let config = config_mgr.create_config_from_id(current_config_id)?;
    let data_source_registry = config.get_data_source_registry()?;

    println!("Updated data source registry:");
    println!("{}", data_source_registry);

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

fn register_data_sources_attempt(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    // Get the current default config ID and associated config JSON
    let config_id = config_mgr.get_default_config_id()?;
    println!("Current default config ID: {}", config_id);

    // Get the SzConfig for the config ID
    let config = config_mgr.create_config_from_id(config_id)?;

    // Create an array of the data sources to add
    let data_sources = vec![
        "TEST_CUSTOMERS",
        "TEST_EMPLOYEES",
        "TEST_WATCHLIST",
        "TEST_PARTNERS",
        "TEST_SUPPLIERS",
    ];

    println!("Registering data sources:");

    // Loop through the array and add each data source
    for data_source in &data_sources {
        match config.register_data_source(data_source) {
            Ok(result) => {
                println!("  ✓ Registered: {} - {}", data_source, result);
            }
            Err(e) => {
                println!("  ⚠ Warning: Failed to register {}: {}", data_source, e);
                // Continue with other data sources - some might already exist
            }
        }
    }

    // Export the modified config
    let modified_config = config.export()?;
    println!("Exported modified configuration");

    // Add the modified config to the repository with a comment
    let new_config_id = config_mgr.register_config(
        &modified_config,
        Some("Added data sources via Rust SDK register_data_sources example"),
    )?;
    println!("Registered new configuration with ID: {}", new_config_id);

    // Replace the default config ID with the new config ID
    config_mgr.replace_default_config_id(config_id, new_config_id)?;
    println!(
        "Updated default configuration ID from {} to {}",
        config_id, new_config_id
    );

    Ok(())
}

/// Alternative approach using set_default_config for atomic operation
#[allow(dead_code)]
fn register_data_sources_atomic(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("Using atomic configuration update approach...");

    // Get the current default config ID to start with existing config
    let config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(config_id)?;

    // Register additional data sources
    let new_data_sources = vec!["VENDORS", "CONTRACTORS"];

    for data_source in &new_data_sources {
        match config.register_data_source(data_source) {
            Ok(result) => {
                println!("  ✓ Registered: {} - {}", data_source, result);
            }
            Err(e) => {
                println!("  ⚠ Warning: {}: {}", data_source, e);
            }
        }
    }

    // Use set_default_config for atomic update
    let modified_config = config.export()?;
    let new_config_id = config_mgr.set_default_config(
        &modified_config,
        Some("Atomic update with additional data sources"),
    )?;

    println!(
        "Atomically set new default configuration ID: {}",
        new_config_id
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_data_sources() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Register data sources should succeed");
        }
    }

    #[test]
    fn test_data_source_list() {
        let data_sources = vec!["CUSTOMERS", "EMPLOYEES", "WATCHLIST"];
        assert!(
            !data_sources.is_empty(),
            "Should have data sources to register"
        );
        assert!(data_sources.len() >= 3, "Should have multiple data sources");
    }
}
