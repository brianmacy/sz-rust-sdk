//! Simple example demonstrating the SDK usage pattern:
//! 1. Create SzEnvironmentCore (the only concrete type you use)
//! 2. Access components through traits
//! 3. Set up configuration and add a record

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Build settings JSON - adjust paths for your platform
    let settings = r#"{
        "PIPELINE": {
            "CONFIGPATH": "/opt/homebrew/opt/senzing/er/resources/templates",
            "RESOURCEPATH": "/opt/homebrew/opt/senzing/er/resources",
            "SUPPORTPATH": "/opt/homebrew/opt/senzing/data"
        },
        "SQL": {"CONNECTION": "internal://"}
    }"#;

    // Step 1: Create environment - this is the ONLY concrete type you use directly
    let env = SzEnvironmentCore::get_instance("simple-example", settings, false)?;

    // Step 2: Set up configuration through traits (Box<dyn SzConfigManager>, Box<dyn SzConfig>)
    // With internal://, register config BEFORE getting the engine
    let config_mgr = env.get_config_manager()?;
    let config = config_mgr.create_config()?;

    // Register a data source
    config.register_data_source("CUSTOMERS")?;

    // Export and register the configuration
    let config_json = config.export()?;
    let config_id = config_mgr.set_default_config(&config_json, Some("Added CUSTOMERS"))?;
    println!("Configuration registered with ID: {config_id}");

    // Step 3: Get engine through traits (Box<dyn SzEngine>)
    let engine = env.get_engine()?;

    // Step 4: Add a record
    let record = r#"{"NAME_FULL": "John Smith", "EMAIL_ADDRESS": "john@example.com"}"#;
    let result = engine.add_record("CUSTOMERS", "CUST001", record, None)?;
    println!("Record added: {result}");

    // Step 5: Search for the entity
    let search_attrs = r#"{"NAME_FULL": "Jon Smith"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;
    println!("Search results: {results}");

    // Cleanup
    env.destroy()?;

    Ok(())
}
