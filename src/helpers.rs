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

/// Get the path to the SQLite schema creation SQL file
fn get_schema_sql_path() -> String {
    format!(
        "{}/schema/szcore-schema-sqlite-create.sql",
        get_resource_path()
    )
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

/// RAII guard for automatic environment cleanup
///
/// This guard ensures that `ExampleEnvironment::cleanup()` is automatically
/// called when the guard goes out of scope, following Rust's RAII pattern.
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::helpers::EnvironmentGuard;
/// use sz_rust_sdk::traits::{SzEngine, SzEnvironment};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Guard automatically cleans up when it goes out of scope
///     let env = EnvironmentGuard::new("my-app")?;
///     let engine = env.get_engine()?;
///
///     // Do work with engine...
///
///     Ok(()) // Cleanup happens automatically here
/// }
/// ```
pub struct EnvironmentGuard {
    env: Option<std::sync::Arc<crate::core::SzEnvironmentCore>>,
}

impl EnvironmentGuard {
    /// Create a new environment guard with automatic cleanup
    pub fn new(instance_name: &str) -> SzResult<Self> {
        let env = ExampleEnvironment::initialize(instance_name)?;
        Ok(Self { env: Some(env) })
    }

    /// Get a reference to the environment
    pub fn env(&self) -> &std::sync::Arc<crate::core::SzEnvironmentCore> {
        self.env.as_ref().expect("Environment already dropped")
    }
}

impl Drop for EnvironmentGuard {
    fn drop(&mut self) {
        // Explicitly drop our reference BEFORE calling cleanup
        // This ensures cleanup() can successfully destroy the environment
        self.env.take();

        // Now attempt cleanup - ignore errors during drop
        let _ = ExampleEnvironment::cleanup();
    }
}

impl std::ops::Deref for EnvironmentGuard {
    type Target = std::sync::Arc<crate::core::SzEnvironmentCore>;

    fn deref(&self) -> &Self::Target {
        self.env.as_ref().expect("Environment already dropped")
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
/// ## Manual cleanup (legacy)
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
/// drop(engine);
/// drop(env);
/// ExampleEnvironment::cleanup()?;
/// # Ok::<(), sz_rust_sdk::error::SzError>(())
/// ```
///
/// ## RAII guard (recommended)
///
/// ```no_run
/// use sz_rust_sdk::helpers::EnvironmentGuard;
/// use sz_rust_sdk::traits::{SzEngine, SzEnvironment};
///
/// // Guard automatically cleans up when it goes out of scope
/// let env = EnvironmentGuard::new("my-app")?;
/// let engine = env.get_engine()?;
///
/// // Use the engine...
///
/// // Cleanup happens automatically here
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

    /// Set up a new database by executing the SQLite schema SQL
    fn setup_test_database() -> SzResult<String> {
        let db_path = Self::generate_random_db_path();
        let schema_path = get_schema_sql_path();

        // Check if schema SQL exists
        if !Path::new(&schema_path).exists() {
            return Err(SzError::configuration(format!(
                "SQLite schema SQL not found at {}. Set SENZING_RESOURCEPATH environment variable.",
                schema_path
            )));
        }

        // Read the schema SQL
        let schema_sql = std::fs::read_to_string(&schema_path).map_err(|e| {
            SzError::configuration(format!(
                "Failed to read schema SQL from {}: {}",
                schema_path, e
            ))
        })?;

        // Create database and execute schema SQL using rusqlite
        let conn = rusqlite::Connection::open(&db_path).map_err(|e| {
            SzError::configuration(format!("Failed to create database at {}: {}", db_path, e))
        })?;

        // Execute each statement in the schema (split by semicolons)
        // The schema file contains multiple statements separated by semicolons
        conn.execute_batch(&schema_sql).map_err(|e| {
            // Clean up the partially created database
            let _ = std::fs::remove_file(&db_path);
            SzError::configuration(format!("Failed to execute schema SQL: {}", e))
        })?;

        // Store the path for cleanup in thread-local storage
        CURRENT_TEST_DB.with(|db| {
            *db.borrow_mut() = Some(db_path.clone());
        });

        println!("Created test database from SQL schema: {}", db_path);

        // Set up initial configuration in the new database
        Self::setup_initial_configuration(&db_path)?;

        Ok(db_path)
    }

    /// Set up initial configuration in a fresh database
    /// Uses a temporary environment to access config components through traits
    fn setup_initial_configuration(db_path: &str) -> SzResult<()> {
        let settings = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"{}","RESOURCEPATH":"{}","SUPPORTPATH":"{}"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
            get_config_path(),
            get_resource_path(),
            get_support_path(),
            db_path
        );

        println!("Setting up initial configuration in database...");

        // Create environment - this works without config being set up yet
        let temp_env = SzEnvironmentCore::get_instance("SzRustSDK-Setup", &settings, false)?;

        // Get config manager through traits - SzConfigMgr initializes independently of Sz_init
        let config_mgr = temp_env.get_config_manager()?;
        let config = config_mgr.create_config()?;

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

        // Destroy the temporary environment so the main environment can be created fresh
        temp_env.destroy()?;

        Ok(())
    }

    /// Clean up the current test database and destroy the global environment
    /// This allows a new environment to be created that will pick up updated configurations
    pub fn cleanup() -> SzResult<()> {
        println!("Cleaning up environment for configuration reinitialization...");

        // Destroy the global environment instance so the next initialization
        // will pick up any configuration changes that were made
        // Use try_get_instance to get the singleton and destroy it
        if let Some(env) = SzEnvironmentCore::try_get_instance() {
            env.destroy()?;
        }

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
