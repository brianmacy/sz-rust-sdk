//! Test Configuration Setup
//!
//! This example tests the automatic configuration setup functionality
//! to ensure that default configurations are registered when none exist.

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Test Configuration Setup ===\n");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    println!("Testing automatic configuration setup...");

    // This should trigger automatic configuration setup if none exists
    let env = SenzingGuard::from_env(ExampleEnvironment::initialize("test-config-setup")?);
    println!("✅ Environment initialized successfully!");

    // Test that we can get the engine
    let engine = env.get_engine()?;
    println!("✅ Engine component ready");

    // Test a simple search to verify everything works
    println!("\n🔍 Testing search operation...");
    let results = engine.search_by_attributes(r#"{"NAME_LAST": "Test"}"#, None, None)?;
    println!("✅ Search completed successfully");
    println!("   Results: {results}");

    println!("\n🎯 Configuration setup test complete");

    Ok(())
}
