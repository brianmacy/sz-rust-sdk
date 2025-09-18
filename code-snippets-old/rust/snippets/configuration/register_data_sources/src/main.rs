//! Register Data Sources Example
//!
//! This example demonstrates how to register new data sources in Senzing configuration.
//! Note: Most examples use the "TEST" data source which is available by default.
//! This example shows how to add additional data sources when needed.
//!
//! Key Senzing SDK concepts demonstrated:
//! - Environment initialization
//! - Getting configuration manager
//! - Creating and modifying configurations
//! - Registering data sources with register_data_source()
//! - Setting default configuration

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Step 1: Get a configured Senzing environment
    let env = get_environment()?;

    // Step 2: Get the configuration manager
    let config_mgr = env.get_config_manager()?;

    println!("Registering data sources in Senzing configuration...");

    // Step 3: Create a new configuration
    let config = config_mgr.create_config()?;
    println!("✓ Created new configuration");

    // Step 4: Register data sources
    // register_data_source(data_source_code)
    config.register_data_source("CUSTOMERS")?;
    println!("✓ Registered CUSTOMERS data source");

    config.register_data_source("EMPLOYEES")?;
    println!("✓ Registered EMPLOYEES data source");

    config.register_data_source("PARTNERS")?;
    println!("✓ Registered PARTNERS data source");

    // Step 5: Export the configuration to JSON
    let config_json = config.export()?;
    println!("✓ Exported configuration ({} chars)", config_json.len());

    // Step 6: Register the configuration and set it as default
    let config_id = config_mgr.register_config(&config_json, Some("Added data sources"))?;
    println!("✓ Registered configuration with ID: {}", config_id);

    config_mgr.set_default_config_id(config_id)?;
    println!("✓ Set as default configuration");

    println!("✅ Data source registration complete");

    Ok(())
}

/// Simple helper to get a configured Senzing environment
/// Handles database setup and configuration automatically
fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("register_data_sources_example")
}