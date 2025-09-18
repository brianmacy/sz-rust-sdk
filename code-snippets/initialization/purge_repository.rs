//! Purge Repository Example
//!
//! This example demonstrates how to safely purge all data from a Senzing repository
//! with user confirmation and proper safety warnings.
//!
//! Rust equivalent of: initialization/PurgeRepository/Program.cs

use std::io::{self, Write};
use sz_rust_sdk::prelude::*;

const PURGE_MESSAGE: &str = r#"

**************************************** WARNING ****************************************

This example will purge all currently loaded data from the Senzing datastore!
Before proceeding, all instances of Senzing (custom code, tools, etc.) must be shut down.

*****************************************************************************************

Are you sure you want to continue and purge the Senzing datastore? (y/n) "#;

const YES_ANSWERS: &[&str] = &["y", "Y", "Yes", "yes", "YES"];

fn main() -> SzResult<()> {
    // Check for non-interactive mode (for automated testing)
    let args: Vec<String> = std::env::args().collect();
    let auto_confirm = args.contains(&"--auto-confirm".to_string());

    if auto_confirm {
        println!("Running in non-interactive mode (auto-confirmed for testing)");
    } else {
        // Display warning and get user confirmation
        print!("{}", PURGE_MESSAGE);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| SzError::unknown(format!("Failed to read user input: {}", e)))?;

        let response = input.trim();
        if response.is_empty() || !YES_ANSWERS.contains(&response) {
            println!("Purge operation cancelled.");
            std::process::exit(1);
        }
    }

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

    println!("Starting repository purge...");

    // Get the diagnostic hub to perform the purge
    let diagnostic = environment.get_diagnostic()?;

    // Perform the purge operation
    diagnostic.purge_repository()?;

    println!("✅ Repository purge completed successfully!");
    println!("All data has been removed from the Senzing datastore.");

    // Clean up resources
    ExampleEnvironment::cleanup()?;

    Ok(())
}
