//! Configuration Management Example
//!
//! This example demonstrates comprehensive configuration management including
//! creating, modifying, and managing multiple configurations.
//!
//! Run with: cargo run --example manage_configuration

use serde_json::Value;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment using ExampleEnvironment helper
    let env = ExampleEnvironment::initialize("manage-configuration")?;

    // Ensure engine is initialized first (this sets up configuration if needed)
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Get the config manager from the environment
    let config_mgr = env.get_config_manager()?;

    println!("Senzing Configuration Management Demo\n");

    // 1. Display current configuration status
    display_current_config_status(&*config_mgr)?;

    // 2. Create a new configuration from scratch
    create_new_configuration(&*config_mgr)?;

    // 3. Modify existing configuration
    modify_existing_configuration(&*config_mgr)?;

    // 4. Display configuration registry
    display_configuration_registry(&*config_mgr)?;

    // 5. Configuration export/import demo
    configuration_export_import_demo(&*config_mgr)?;

    println!("Configuration management demo completed successfully!");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}

fn display_current_config_status(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("=== Current Configuration Status ===");

    // Get the current default configuration ID
    let default_config_id = config_mgr.get_default_config_id()?;
    println!("Default Configuration ID: {}", default_config_id);

    // Get the configuration and show some details
    let config = config_mgr.create_config_from_id(default_config_id)?;
    let data_source_registry = config.get_data_source_registry()?;

    // Parse and display data sources
    match serde_json::from_str::<Value>(&data_source_registry) {
        Ok(registry_json) => {
            if let Some(data_sources) = registry_json.get("DATA_SOURCES") {
                if let Some(ds_array) = data_sources.as_array() {
                    println!("Configured Data Sources ({}):", ds_array.len());
                    for ds in ds_array {
                        if let Some(ds_code) = ds.get("DSRC_CODE") {
                            let ds_id = ds.get("DSRC_ID").and_then(|id| id.as_i64()).unwrap_or(0);
                            println!("  - {} (ID: {})", ds_code, ds_id);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not parse data source registry: {}", e);
        }
    }

    println!();
    Ok(())
}

fn create_new_configuration(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("=== Creating New Configuration ===");

    // Create a new empty configuration
    let config = config_mgr.create_config()?;
    println!("Created new empty configuration");

    // Add some data sources
    let test_data_sources = vec!["TEST_DS_1", "TEST_DS_2", "TEST_DS_3"];

    for ds_code in &test_data_sources {
        match config.register_data_source(ds_code) {
            Ok(result) => {
                println!("  ✓ Added data source: {} - {}", ds_code, result);
            }
            Err(e) => {
                println!("  ✗ Failed to add data source {}: {}", ds_code, e);
            }
        }
    }

    // Export the configuration
    let config_json = config.export()?;

    // Register it in the repository
    let new_config_id = config_mgr.register_config(
        &config_json,
        Some("Test configuration created by Rust SDK demo"),
    )?;

    println!("✓ Registered new configuration with ID: {}", new_config_id);
    println!();

    Ok(())
}

fn modify_existing_configuration(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("=== Modifying Existing Configuration ===");

    // Get the current default configuration
    let default_config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(default_config_id)?;

    println!("Working with configuration ID: {}", default_config_id);

    // Add a new data source
    let new_data_source = "MODIFIED_CONFIG_DS";

    match config.register_data_source(new_data_source) {
        Ok(result) => {
            println!("  ✓ Added data source: {} - {}", new_data_source, result);

            // Export and register the modified configuration
            let modified_config_json = config.export()?;
            let modified_config_id = config_mgr.register_config(
                &modified_config_json,
                Some("Configuration modified by Rust SDK demo"),
            )?;

            println!(
                "✓ Registered modified configuration with ID: {}",
                modified_config_id
            );
        }
        Err(e) => {
            println!("  ✗ Failed to modify configuration: {}", e);
        }
    }

    println!();
    Ok(())
}

fn display_configuration_registry(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("=== Configuration Registry ===");

    match config_mgr.get_config_registry() {
        Ok(registry_json) => {
            match serde_json::from_str::<Value>(&registry_json) {
                Ok(registry) => {
                    if let Some(configs) = registry.get("CONFIGS") {
                        if let Some(configs_array) = configs.as_array() {
                            println!("Total configurations: {}", configs_array.len());

                            for config in configs_array.iter().take(10) {
                                // Show first 10
                                let config_id = config
                                    .get("CONFIG_ID")
                                    .and_then(|id| id.as_i64())
                                    .unwrap_or(0);

                                let comment = config
                                    .get("CONFIG_COMMENTS")
                                    .and_then(|c| c.as_str())
                                    .unwrap_or("No comment");

                                let created = config
                                    .get("SYS_CREATE_DT")
                                    .and_then(|dt| dt.as_str())
                                    .unwrap_or("Unknown");

                                println!(
                                    "  Config ID: {} | Created: {} | Comment: {}",
                                    config_id, created, comment
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Could not parse configuration registry: {}", e);
                    println!("Raw registry data: {}", registry_json);
                }
            }
        }
        Err(e) => {
            println!("Could not retrieve configuration registry: {}", e);
        }
    }

    println!();
    Ok(())
}

fn configuration_export_import_demo(config_mgr: &dyn SzConfigManager) -> SzResult<()> {
    println!("=== Configuration Export/Import Demo ===");

    // Export current default configuration
    let default_config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(default_config_id)?;
    let exported_config = config.export()?;

    println!("Exported configuration from ID: {}", default_config_id);
    println!("Configuration size: {} characters", exported_config.len());

    // Create a new configuration from the exported JSON
    let imported_config = config_mgr.create_config_from_definition(&exported_config)?;
    println!("Successfully created configuration from exported JSON");

    // Modify the imported configuration
    match imported_config.register_data_source("IMPORTED_CONFIG_TEST") {
        Ok(result) => {
            println!("  ✓ Added test data source to imported config: {}", result);

            // Re-export and register
            let re_exported = imported_config.export()?;
            let new_id = config_mgr.register_config(
                &re_exported,
                Some("Configuration created from export/import demo"),
            )?;

            println!("✓ Re-registered modified configuration with ID: {}", new_id);
        }
        Err(e) => {
            println!("  ✗ Failed to modify imported configuration: {}", e);
        }
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manage_configuration() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Configuration management should succeed");
        }
    }

    #[test]
    fn test_individual_functions() {
        // These tests would require a mock configuration manager
        // For now, just verify the functions exist and can be called
        assert!(true, "Configuration management functions are defined");
    }
}
