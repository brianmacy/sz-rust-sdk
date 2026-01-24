//! Simple example demonstrating the SDK usage pattern:
//! 1. Create SzEnvironmentCore (the only concrete type you use)
//! 2. Access components through traits
//! 3. Set up configuration and add a record

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Build settings JSON - adjust paths for your platform
    let settings = r#"{
        "PIPELINE": {
            "CONFIGPATH": "/opt/homebrew/opt/senzing/runtime/er/etc",
            "RESOURCEPATH": "/opt/homebrew/opt/senzing/runtime/er/resources",
            "SUPPORTPATH": "/opt/homebrew/opt/senzing/runtime/data"
        },
        "SQL": {"CONNECTION": "sqlite3://na:na@/tmp/simple_example.db"}
    }"#;

    // Copy template database (required for fresh databases)
    let template = "/opt/homebrew/opt/senzing/runtime/er/resources/templates/G2C.db";
    std::fs::copy(template, "/tmp/simple_example.db").expect("Copy template database");

    // Step 1: Create environment - this is the ONLY concrete type you use directly
    let env = SzEnvironmentCore::get_instance("simple-example", settings, false)?;

    // Step 2: Set up configuration through traits (Box<dyn SzConfigManager>, Box<dyn SzConfig>)
    let config_mgr = env.get_config_manager()?;
    let config = config_mgr.create_config()?;

    // Register a data source
    config.register_data_source("CUSTOMERS")?;

    // Export and register the configuration
    let config_json = config.export()?;
    let config_id = config_mgr.set_default_config(&config_json, Some("Added CUSTOMERS"))?;
    println!("Configuration registered with ID: {}", config_id);

    // Step 3: Destroy and recreate environment to pick up new config
    env.destroy()?;
    let env = SzEnvironmentCore::get_instance("simple-example", settings, false)?;

    // Step 4: Get engine through traits (Box<dyn SzEngine>)
    let engine = env.get_engine()?;

    // Step 5: Add a record
    let record = r#"{"NAME_FULL": "John Smith", "EMAIL_ADDRESS": "john@example.com"}"#;
    let result = engine.add_record("CUSTOMERS", "CUST001", record, None)?;
    println!("Record added: {}", result);

    // Step 6: Search for the entity
    let search_attrs = r#"{"NAME_FULL": "Jon Smith"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("Search results: {}", results);

    // Cleanup - destroy environment before removing database
    env.destroy()?;
    std::fs::remove_file("/tmp/simple_example.db").ok();

    Ok(())
}
