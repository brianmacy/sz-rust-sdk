//! Helper utilities for examples and common operations
//!
//! This module provides common functionality used across examples,
//! including environment setup and configuration management.
//!
//! The primary utility is [`ExampleEnvironment`] which handles the complex
//! initialization process for Senzing, including automatic configuration
//! setup when needed.

use crate::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

/// Global storage for the current test database path
static CURRENT_DB_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Enhanced error reporting for examples with backtrace support
///
/// This function provides comprehensive error reporting including:
/// - The main error message
/// - The full error chain (if any)
/// - Optional backtrace when `RUST_BACKTRACE=1` is set
///
/// # Arguments
///
/// * `error` - The Senzing error to report
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::helpers::print_error_with_backtrace;
/// use sz_rust_sdk::error::SzError;
///
/// let error = SzError::configuration("Database not initialized");
/// print_error_with_backtrace(&error);
/// ```
pub fn print_error_with_backtrace(error: &crate::error::SzError) {
    eprintln!("Error: {}", error);

    // Print the error chain
    let mut source = std::error::Error::source(error);
    let mut level = 1;
    while let Some(err) = source {
        eprintln!("  {}: {}", level, err);
        source = std::error::Error::source(err);
        level += 1;
    }

    // Print backtrace if requested
    if std::env::var("RUST_BACKTRACE").unwrap_or_default() != "0" {
        // Capture backtrace from the current error location
        let backtrace = std::backtrace::Backtrace::capture();
        eprintln!("\nBacktrace from FFI error detection:");
        eprintln!("{}", backtrace);
    }
}

/// Environment setup utility for examples
///
/// `ExampleEnvironment` provides a simplified interface for initializing
/// Senzing in Rust applications. It handles the complex initialization
/// process, including automatic configuration setup if none exists.
///
/// This utility follows Senzing's singleton pattern where only one
/// environment instance can exist per process.
///
/// # Key Features
///
/// * Automatic configuration detection and setup
/// * Singleton pattern enforcement
/// * Comprehensive error handling
/// * Production-ready initialization sequence
/// * Isolated test databases with automatic cleanup
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::traits::{SzEngine, SzEnvironment};
///
/// // Initialize Senzing environment with isolated database
/// let env = ExampleEnvironment::initialize("my-app")?;
///
/// // Get the engine for entity resolution operations
/// let engine = env.get_engine()?;
///
/// // Use the engine for your application
///
/// // Clean up the database when done
/// ExampleEnvironment::cleanup()?;
/// # Ok::<(), sz_rust_sdk::error::SzError>(())
/// ```
pub struct ExampleEnvironment;

impl ExampleEnvironment {
    /// Generate a random database path in /tmp/
    fn generate_random_db_path() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let random_suffix = format!("{:x}", timestamp);
        format!("/tmp/senzing_test_{}.db", random_suffix)
    }

    /// Set up a new random database from the template with default configuration
    fn setup_test_database() -> SzResult<String> {
        let db_path = Self::generate_random_db_path();
        let template_path = "/opt/senzing/er/resources/templates/G2C.db";

        // Check if template exists
        if !Path::new(template_path).exists() {
            return Err(SzError::configuration(
                "Template database not found at /opt/senzing/er/resources/templates/G2C.db",
            ));
        }

        // Copy template to random location
        if let Err(e) = fs::copy(template_path, &db_path) {
            return Err(SzError::configuration(format!(
                "Failed to copy template database to {}: {}",
                db_path, e
            )));
        }

        // Store the path for cleanup
        if let Ok(mut current_path) = CURRENT_DB_PATH.lock() {
            *current_path = Some(db_path.clone());
        }

        println!("Created test database: {}", db_path);

        // Set up initial configuration in the new database
        Self::setup_initial_configuration(&db_path)?;

        Ok(db_path)
    }

    /// Set up initial configuration in a fresh database
    fn setup_initial_configuration(db_path: &str) -> SzResult<()> {
        let settings = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
            db_path
        );

        println!("Setting up initial configuration in database...");

        // Create config manager for the new database
        let config_mgr = crate::core::config_manager::SzConfigManagerCore::new_with_params(
            "SzRustSDK-InitialSetup",
            &settings,
            false,
        )?;

        // Create and register default configuration
        let config_core = crate::core::config::SzConfigCore::new_with_params(
            "SzRustSDK-InitialConfig",
            &settings,
            false,
        )?;
        let config = Box::new(config_core) as Box<dyn crate::traits::SzConfig>;
        let config_definition = config.export()?;
        let config_id = config_mgr.register_config(
            &config_definition,
            Some("Initial default configuration for isolated test"),
        )?;

        // Set as default
        config_mgr.set_default_config_id(config_id)?;

        println!(
            "✅ Initial configuration setup complete with ID: {}",
            config_id
        );
        Ok(())
    }

    /// Clean up the current test database
    pub fn cleanup() -> SzResult<()> {
        if let Ok(mut current_path) = CURRENT_DB_PATH.lock() {
            if let Some(db_path) = current_path.take() {
                if Path::new(&db_path).exists() {
                    if let Err(e) = fs::remove_file(&db_path) {
                        eprintln!("Warning: Failed to remove test database {}: {}", db_path, e);
                    } else {
                        println!("Cleaned up test database: {}", db_path);
                    }
                }
            }
        }
        Ok(())
    }
    /// Create and initialize a Senzing environment for examples using singleton pattern
    ///
    /// This creates a working Senzing environment that can perform
    /// basic operations like search and entity resolution.
    /// If no configuration exists, it registers a default one.
    /// Uses the singleton pattern to ensure only one environment exists per process.
    ///
    /// # Arguments
    ///
    /// * `instance_name` - A unique name for this environment instance
    ///
    /// # Returns
    ///
    /// Returns a ready-to-use `Arc<SzEnvironmentCore>` instance that can be
    /// safely shared across threads.
    ///
    /// # Errors
    ///
    /// Returns `SzError` if:
    /// - Senzing libraries cannot be initialized
    /// - Database connection fails
    /// - Configuration setup fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::helpers::ExampleEnvironment;
    ///
    /// let env = ExampleEnvironment::initialize("my-rust-app")?;
    /// println!("Senzing environment initialized successfully");
    /// # Ok::<(), sz_rust_sdk::error::SzError>(())
    /// ```
    pub fn initialize(instance_name: &str) -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
        let settings = Self::get_configuration()?;

        // Use singleton pattern to get or create the environment
        println!("Getting singleton SzEnvironmentCore instance with isolated database");
        SzEnvironmentCore::get_instance(instance_name, &settings, false)
    }

    /// Get the standard Senzing configuration for examples with isolated database
    fn get_configuration() -> SzResult<String> {
        if let Ok(config) = std::env::var("SENZING_ENGINE_CONFIGURATION_JSON") {
            return Ok(config);
        }

        // Check if we already have a database path (for singleton pattern support)
        let db_path = if let Ok(current_path) = CURRENT_DB_PATH.lock() {
            if let Some(existing_path) = current_path.as_ref() {
                // Reuse existing database path to maintain singleton behavior
                existing_path.clone()
            } else {
                // No existing path, create new one
                drop(current_path); // Release lock before calling setup_test_database
                Self::setup_test_database()?
            }
        } else {
            return Err(SzError::configuration(
                "Failed to access database path mutex",
            ));
        };

        let config = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
            db_path
        );

        Ok(config)
    }

    /// Initialize with verbose logging for debugging using singleton pattern
    pub fn initialize_verbose(instance_name: &str) -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
        let settings = Self::get_configuration()?;

        // Try with verbose logging enabled using singleton pattern
        match SzEnvironmentCore::get_instance(instance_name, &settings, false) {
            Ok(env) => Ok(env),
            Err(e) => {
                // Use proper error hierarchy instead of string matching
                if matches!(e, SzError::Configuration { .. }) {
                    // Try to set up default configuration with verbose logging
                    let temp_env = SzEnvironmentCore::get_instance(
                        &format!("{}-setup", instance_name),
                        &settings,
                        true,
                    )?;
                    let config_mgr = temp_env.get_config_manager()?;
                    let config = config_mgr.create_config()?;
                    let config_definition = config.export()?;
                    let config_id = config_mgr.register_config(
                        &config_definition,
                        Some("Default configuration for examples"),
                    )?;
                    config_mgr.set_default_config_id(config_id)?;

                    SzEnvironmentCore::get_instance(instance_name, &settings, true)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Get engine from environment - configuration is set up during database initialization
    ///
    /// This is a convenience method that simply calls env.get_engine() since configuration
    /// is now automatically set up during database creation.
    pub fn get_engine_with_setup(
        env: &std::sync::Arc<SzEnvironmentCore>,
    ) -> SzResult<Box<dyn crate::traits::SzEngine>> {
        env.get_engine()
    }
}
