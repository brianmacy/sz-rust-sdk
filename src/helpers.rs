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
use std::path::Path;

/// Detected Senzing installation paths for building the engine init JSON.
///
/// These correspond to the `PIPELINE` section of the Senzing engine configuration:
/// <https://www.senzing.com/docs/tutorials/senzing_engine_config/>
struct SenzingPaths {
    config_path: String,
    resource_path: String,
    support_path: String,
}

/// Auto-detect Senzing installation paths based on platform.
///
/// Checks standard installation locations in priority order:
/// 1. macOS Homebrew official cask (Apple Silicon): `/opt/homebrew/opt/senzing`
/// 2. macOS Homebrew official cask (Intel): `/usr/local/opt/senzing`
/// 3. macOS Homebrew legacy unofficial tap: `.../senzing/runtime`
/// 4. Linux standard: `/opt/senzing` with `/etc/opt/senzing` for config
///
/// This matches the detection logic used by `build.rs` for library linking.
fn detect_senzing_paths() -> SenzingPaths {
    // Check macOS Homebrew locations — official cask first, then legacy unofficial
    for homebrew_base in [
        "/opt/homebrew/opt/senzing",
        "/usr/local/opt/senzing",
        "/opt/homebrew/opt/senzing/runtime",
        "/usr/local/opt/senzing/runtime",
    ] {
        if Path::new(&format!("{homebrew_base}/er/resources")).exists() {
            return SenzingPaths {
                config_path: format!("{homebrew_base}/er/resources/templates"),
                resource_path: format!("{homebrew_base}/er/resources"),
                support_path: format!("{homebrew_base}/data"),
            };
        }
    }

    // Linux: use /etc/opt/senzing for config if it exists, otherwise resources/templates
    let config_path = if Path::new("/etc/opt/senzing").exists() {
        "/etc/opt/senzing".to_string()
    } else {
        "/opt/senzing/er/resources/templates".to_string()
    };

    SenzingPaths {
        config_path,
        resource_path: "/opt/senzing/er/resources".to_string(),
        support_path: "/opt/senzing/data".to_string(),
    }
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
    eprintln!("Error: {error}");

    // Print the error chain
    let mut source = std::error::Error::source(error);
    let mut level = 1;
    while let Some(err) = source {
        eprintln!("  {level}: {err}");
        source = std::error::Error::source(err);
        level += 1;
    }

    // Print backtrace if requested
    if std::env::var("RUST_BACKTRACE").unwrap_or_default() != "0" {
        // Capture backtrace from the current error location
        let backtrace = std::backtrace::Backtrace::capture();
        eprintln!("\nBacktrace from FFI error detection:");
        eprintln!("{backtrace}");
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
/// * In-memory `internal://` database (v4.3+) — no temp files or schema setup
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
/// // Clean up when done - cleanup takes ownership of env
/// drop(engine);
/// ExampleEnvironment::cleanup(env)?;
/// # Ok::<(), sz_rust_sdk::error::SzError>(())
/// ```
///
/// ## RAII guard (recommended)
///
/// ```no_run
/// use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
///
/// // Guard automatically cleans up when it goes out of scope
/// let env = SenzingGuard::from_env(ExampleEnvironment::initialize("my-app")?);
/// let engine = env.get_engine()?;
///
/// // Use the engine...
///
/// // Cleanup happens automatically here
/// # Ok::<(), sz_rust_sdk::error::SzError>(())
/// ```
pub struct ExampleEnvironment;

impl ExampleEnvironment {
    /// Clean up the environment by destroying native resources.
    ///
    /// Takes ownership of the environment Arc, ensuring clean ownership semantics.
    /// This consumes the env - you cannot use it after calling cleanup.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use sz_rust_sdk::helpers::ExampleEnvironment;
    /// # use sz_rust_sdk::prelude::*;
    /// let env = ExampleEnvironment::initialize("doctest_cleanup")?;
    /// // ... use env ...
    /// ExampleEnvironment::cleanup(env)?;  // Consumes env
    /// # Ok::<(), SzError>(())
    /// ```
    pub fn cleanup(env: std::sync::Arc<SzEnvironmentCore>) -> SzResult<()> {
        println!("Cleaning up environment...");
        env.destroy()?;
        println!("✅ Environment cleanup complete");
        Ok(())
    }

    /// Create and initialize a Senzing environment for examples using singleton pattern
    ///
    /// Uses the `internal://` in-memory database (v4.3+) for zero-setup isolation.
    /// If no configuration exists, it registers a default one.
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
        Self::initialize_with_logging(instance_name, false)
    }

    /// Initialize with verbose logging for debugging using singleton pattern
    pub fn initialize_verbose(instance_name: &str) -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
        Self::initialize_with_logging(instance_name, true)
    }

    fn initialize_with_logging(
        instance_name: &str,
        verbose: bool,
    ) -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
        let settings = Self::get_configuration(verbose)?;

        println!("Getting singleton SzEnvironmentCore instance with in-memory database");
        let env = SzEnvironmentCore::get_instance(instance_name, &settings, verbose)?;

        // With internal://, each environment has its own ephemeral in-memory DB.
        // Register a default config BEFORE Sz_init (triggered by get_engine) so
        // the engine finds it in the same DB instance.
        // Note: get_default_config_id() returns Ok(0) (not Err) when no config exists
        // with internal://, so check the value, not just Ok/Err.
        let config_mgr = env.get_config_manager()?;
        let needs_config = config_mgr
            .get_default_config_id()
            .map_or(true, |id| id == 0);
        if needs_config {
            println!("No configuration found, setting up default configuration...");
            let config = config_mgr.create_config()?;
            let config_definition = config.export()?;
            let config_id =
                config_mgr.register_config(&config_definition, Some("Default configuration"))?;
            config_mgr.set_default_config_id(config_id)?;
            println!("✅ Configuration setup complete with ID: {config_id}");
        }

        Ok(env)
    }

    fn get_configuration(verbose: bool) -> SzResult<String> {
        if let Ok(config) = std::env::var("SENZING_ENGINE_CONFIGURATION_JSON") {
            if !config.contains("CONNECTION") {
                return Err(SzError::configuration(
                    "SENZING_ENGINE_CONFIGURATION_JSON is set but missing SQL.CONNECTION",
                ));
            }
            return Ok(config);
        }

        let paths = detect_senzing_paths();
        let debug = if verbose { r#","DEBUGLEVEL":"2""# } else { "" };
        let config = format!(
            r#"{{"PIPELINE":{{"CONFIGPATH":"{}","RESOURCEPATH":"{}","SUPPORTPATH":"{}"}},"SQL":{{"CONNECTION":"internal://"{debug}}}}}"#,
            paths.config_path, paths.resource_path, paths.support_path
        );

        Ok(config)
    }

    /// Get engine from environment.
    ///
    /// Convenience method that calls `env.get_engine()`.
    pub fn get_engine_with_setup(
        env: &std::sync::Arc<SzEnvironmentCore>,
    ) -> SzResult<Box<dyn crate::traits::SzEngine>> {
        env.get_engine()
    }
}
