//! Internal helper utilities for examples only
//!
//! **⚠️ INTERNAL MODULE - NOT PART OF PUBLIC API**
//!
//! This module provides internal utilities used by examples and code snippets.
//! These functions are not intended for use by SDK consumers and should not be
//! considered part of the stable public API.
//!
//! The primary utility is [`ExampleEnvironment`] which handles the complex
//! initialization process for Senzing examples, including automatic configuration
//! setup when needed.

use crate::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::path::Path;

/// Get the Senzing CONFIGPATH from environment variable or default
fn get_config_path() -> String {
    std::env::var("SENZING_CONFIGPATH")
        .unwrap_or_else(|_| "/opt/senzing/er/resources/templates".to_string())
}

/// Get the Senzing RESOURCEPATH from environment variable or default
fn get_resource_path() -> String {
    std::env::var("SENZING_RESOURCEPATH")
        .unwrap_or_else(|_| "/opt/senzing/er/resources".to_string())
}

/// Get the Senzing SUPPORTPATH from environment variable or default
fn get_support_path() -> String {
    std::env::var("SENZING_SUPPORTPATH").unwrap_or_else(|_| "/opt/senzing/data".to_string())
}

/// Get the Senzing template database path from environment variable or default
fn get_template_db_path() -> String {
    std::env::var("SENZING_TEMPLATE_DB").unwrap_or_else(|_| format!("{}/G2C.db", get_config_path()))
}

// Thread-local storage for test database cleanup
thread_local! {
    static CURRENT_TEST_DB: RefCell<Option<String>> = const { RefCell::new(None) };
}

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
        let template_path = get_template_db_path();

        // Check if template exists
        if !Path::new(&template_path).exists() {
            return Err(SzError::configuration(format!(
                "Template database not found at {}. Set SENZING_TEMPLATE_DB or SENZING_CONFIGPATH environment variable.",
                template_path
            )));
        }

        // Copy template to random location
        if let Err(e) = fs::copy(template_path, &db_path) {
            return Err(SzError::configuration(format!(
                "Failed to copy template database to {}: {}",
                db_path, e
            )));
        }

        // Store the path for cleanup in thread-local storage
        CURRENT_TEST_DB.with(|db| {
            *db.borrow_mut() = Some(db_path.clone());
        });

        println!("Created test database: {}", db_path);

        // Set up initial configuration in the new database
        Self::setup_initial_configuration(&db_path)?;

        Ok(db_path)
    }

    /// Set up initial configuration in a fresh database
    /// This uses a separate, temporary config manager instance that doesn't interfere with the main environment
    fn setup_initial_configuration(db_path: &str) -> SzResult<()> {
        let settings = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"{}","RESOURCEPATH":"{}","SUPPORTPATH":"{}"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
            get_config_path(),
            get_resource_path(),
            get_support_path(),
            db_path
        );

        println!("Setting up initial configuration in database...");

        // Use direct config manager creation for setup - separate from main environment
        let config_mgr = crate::core::config_manager::SzConfigManagerCore::new_with_params(
            "SzRustSDK-Setup",
            &settings,
            false,
        )?;

        // Create and register default configuration using direct config core creation
        let config_core = crate::core::config::SzConfigCore::new_with_params(
            "SzRustSDK-SetupConfig",
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

        // config_mgr and config will be cleaned up automatically when they go out of scope

        Ok(())
    }

    /// Clean up the current test database and destroy the global environment
    /// This allows a new environment to be created that will pick up updated configurations
    pub fn cleanup() -> SzResult<()> {
        println!("Cleaning up environment for configuration reinitialization...");

        // Destroy the global environment instance so the next initialization
        // will pick up any configuration changes that were made
        crate::core::environment::SzEnvironmentCore::destroy_global_instance()?;

        println!("✅ Environment cleanup complete");
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
        match SzEnvironmentCore::get_instance(instance_name, &settings, false) {
            Ok(env) => Ok(env),
            Err(e) => {
                // Check if this is a configuration error - if so, try to set up default configuration
                if matches!(e, SzError::Configuration { .. }) {
                    println!("No configuration found, setting up default configuration...");
                    // Try to set up default configuration
                    let temp_env = SzEnvironmentCore::get_instance(
                        &format!("{}-setup", instance_name),
                        &settings,
                        false,
                    )?;
                    let config_mgr = temp_env.get_config_manager()?;
                    let config = config_mgr.create_config()?;
                    let config_definition = config.export()?;
                    let config_id = config_mgr.register_config(
                        &config_definition,
                        Some("Default configuration for tests"),
                    )?;
                    config_mgr.set_default_config_id(config_id)?;
                    println!("✅ Configuration setup complete with ID: {}", config_id);

                    // Now try again with the main instance name
                    SzEnvironmentCore::get_instance(instance_name, &settings, false)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Get the standard Senzing configuration for examples with shared database
    fn get_configuration_verbose() -> SzResult<String> {
        if let Ok(config) = std::env::var("SENZING_ENGINE_CONFIGURATION_JSON") {
            return Ok(config);
        }

        // Use a single shared database path for all tests to avoid Senzing library conflicts
        // The isolation will come from destroying and recreating the environment between tests
        static SHARED_DB_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let db_path = SHARED_DB_PATH.get_or_init(|| {
            let path = Self::setup_test_database()
                .unwrap_or_else(|_| "/tmp/senzing_shared_test.db".to_string());
            println!("Using shared test database: {}", path);
            path
        });

        let config = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"{}","RESOURCEPATH":"{}","SUPPORTPATH":"{}"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}","DEBUGLEVEL":"2"}}}}"#,
            get_config_path(),
            get_resource_path(),
            get_support_path(),
            db_path
        );

        Ok(config)
    }

    /// Get the standard Senzing configuration for examples with shared database
    fn get_configuration() -> SzResult<String> {
        if let Ok(config) = std::env::var("SENZING_ENGINE_CONFIGURATION_JSON") {
            return Ok(config);
        }

        // Use a single shared database path for all tests to avoid Senzing library conflicts
        // The isolation will come from destroying and recreating the environment between tests
        static SHARED_DB_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let db_path = SHARED_DB_PATH.get_or_init(|| {
            let path = Self::setup_test_database()
                .unwrap_or_else(|_| "/tmp/senzing_shared_test.db".to_string());
            println!("Using shared test database: {}", path);
            path
        });

        let config = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"{}","RESOURCEPATH":"{}","SUPPORTPATH":"{}"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
            get_config_path(),
            get_resource_path(),
            get_support_path(),
            db_path
        );

        Ok(config)
    }

    /// Initialize with verbose logging for debugging using singleton pattern
    pub fn initialize_verbose(instance_name: &str) -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
        let settings = Self::get_configuration_verbose()?;

        // Try with verbose logging enabled using singleton pattern
        match SzEnvironmentCore::get_instance(instance_name, &settings, true) {
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
