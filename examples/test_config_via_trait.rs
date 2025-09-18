//! Test Config Manager via Trait
//!
//! This example tests that the ExampleEnvironment uses the SzEnvironment trait
//! to get the ConfigManager instead of creating it directly.

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    println!("=== Test Config Manager via SzEnvironment Trait ===\n");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    println!("Testing configuration setup via singleton environment trait...");

    // This should trigger the configuration setup process
    let env = ExampleEnvironment::initialize("config-trait-test")?;
    println!("✅ Environment initialized successfully using trait pattern");

    // Verify we can get the config manager through the trait
    let config_mgr = env.get_config_manager()?;
    println!("✅ Config manager obtained via SzEnvironment trait");

    // Test that we can get the default config ID
    let config_id = config_mgr.get_default_config_id()?;
    println!("✅ Default config ID: {}", config_id);

    // Test that we can use the engine
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    println!("✅ Engine obtained successfully");

    // Test a simple search
    println!("\n🔍 Testing search operation...");
    let results = engine.search_by_attributes(r#"{"NAME_LAST": "TestTrait"}"#, None, None)?;
    println!("✅ Search completed successfully");
    println!("   Results: {}", results);

    println!("\n🎯 Config manager trait test complete");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
