//! Environment and Hubs Example
//!
//! This example shows how to initialize the Senzing environment and access all
//! available hub components (Engine, Product, ConfigManager, Diagnostic).
//!
//! Rust equivalent of: initialization/EnvironmentAndHubs/Program.cs

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Create a descriptive instance name (can be anything)
    let instance_name = env!("CARGO_PKG_NAME");

    // Remove any existing environment configuration to use isolated database
    unsafe { std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON") };

    // Initialize the Senzing environment using the singleton pattern
    let environment = match ExampleEnvironment::initialize(instance_name) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to initialize environment: {}", e);
            return Err(e);
        }
    };

    // Access all hub components from the environment
    println!("Accessing Senzing hub components...");

    // Get the Product hub for version and license information
    let product = environment.get_product()?;
    println!("✓ Product hub: {:p}", &*product);

    // Get the Config Manager hub for configuration lifecycle management
    let config_manager = environment.get_config_manager()?;
    println!("✓ Config Manager hub: {:p}", &*config_manager);

    // Get the Diagnostic hub for system monitoring and health checks
    let diagnostic = environment.get_diagnostic()?;
    println!("✓ Diagnostic hub: {:p}", &*diagnostic);

    // Get the Engine hub for core entity resolution operations
    let engine = environment.get_engine()?;
    println!("✓ Engine hub: {:p}", &*engine);

    println!("\n✅ All hub components successfully accessed!");
    println!("These hub handles are valid until environment cleanup is called.");

    // Note: In Rust, we don't need explicit hub cleanup as they're automatically
    // managed through RAII when the environment is cleaned up

    // Clean up resources
    ExampleEnvironment::cleanup(environment)?;

    Ok(())
}
