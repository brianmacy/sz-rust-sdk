//! Error types for the Senzing Rust SDK
//!
//! This module defines a comprehensive error hierarchy that mirrors the
//! Senzing C# SDK exception hierarchy while leveraging Rust's `Result<T, E>` types.
//!
//! The error system provides detailed error information from the underlying
//! Senzing C library with proper error chains and backtrace support. When errors
//! occur, the SDK automatically calls the appropriate `getLastException` function
//! to retrieve detailed error messages from the native library.
//!
//! # Error Categories (Matching C# SDK Hierarchy)
//!
//! **Base errors:**
//! * [`SzError::BadInput`] - Invalid input parameters or data (SzBadInputException)
//!   * [`SzError::NotFound`] - Resource or entity not found (extends BadInput)
//!   * [`SzError::UnknownDataSource`] - Unknown data source errors (extends BadInput)
//! * [`SzError::Configuration`] - Configuration and setup errors (SzConfigurationException)
//! * [`SzError::Retryable`] - Temporary errors that can be retried (SzRetryableException)
//!   * [`SzError::DatabaseConnectionLost`] - Database connectivity lost (extends Retryable)
//!   * [`SzError::DatabaseTransient`] - Temporary database issues (extends Retryable)
//!   * [`SzError::RetryTimeoutExceeded`] - Retry timeout exceeded (extends Retryable)
//! * [`SzError::Unrecoverable`] - Fatal errors requiring reinitialization (SzUnrecoverableException)
//!   * [`SzError::Database`] - Database operation errors (extends Unrecoverable)
//!   * [`SzError::License`] - Licensing issues (extends Unrecoverable)
//!   * [`SzError::NotInitialized`] - System not initialized errors (extends Unrecoverable)
//!   * [`SzError::Unhandled`] - Unhandled errors (extends Unrecoverable)
//! * [`SzError::ReplaceConflict`] - Data replacement conflicts (SzReplaceConflictException)
//! * [`SzError::EnvironmentDestroyed`] - Environment already destroyed (SzEnvironmentDestroyedException)
//! * [`SzError::Unknown`] - Unexpected or unclassified errors
//!
//! # Examples
//!
//! ```no_run
//! use sz_rust_sdk::error::{SzError, SzResult};
//!
//! fn example_function() -> SzResult<String> {
//!     // This would normally come from a Senzing operation
//!     Err(SzError::configuration("Database not initialized"))
//! }
//!
//! match example_function() {
//!     Ok(result) => println!("Success: {}", result),
//!     Err(SzError::Configuration { message, .. }) => {
//!         eprintln!("Configuration error: {}", message);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```

use std::ffi::{CStr, NulError};

/// Senzing SDK component for error reporting
#[derive(Debug, Clone, Copy)]
pub enum SzComponent {
    Engine,
    Config,
    ConfigMgr,
    Diagnostic,
    Product,
}
use thiserror::Error;

/// Result type alias for Senzing SDK operations
///
/// This is the standard Result type used throughout the Senzing Rust SDK.
/// All Senzing operations return `SzResult<T>` instead of `Result<T, SzError>`.
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::error::SzResult;
///
/// fn senzing_operation() -> SzResult<String> {
///     // Your Senzing operation here
///     Ok("Success".to_string())
/// }
/// ```
pub type SzResult<T> = Result<T, SzError>;

/// Base error type for all Senzing SDK operations
///
/// This enum represents all possible errors that can occur when using the
/// Senzing SDK. Each variant corresponds to a specific category of error
/// returned by the underlying Senzing C library.
///
/// The error hierarchy is designed to match the Senzing C# SDK for consistency
/// across language bindings.
#[derive(Error, Debug)]
pub enum SzError {
    /// Errors related to invalid input parameters
    #[error("Bad input: {message}")]
    BadInput {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Configuration {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Database operation errors
    #[error("Database error: {message}")]
    Database {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// License-related errors
    #[error("License error: {message}")]
    License {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Resource not found errors
    #[error("Not found: {message}")]
    NotFound {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errors that indicate the operation should be retried
    #[error("Retryable error: {message}")]
    Retryable {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unrecoverable errors that require reinitialization
    #[error("Unrecoverable error: {message}")]
    Unrecoverable {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unknown or unexpected errors
    #[error("Unknown error: {message}")]
    Unknown {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// System not initialized errors
    #[error("Not initialized: {message}")]
    NotInitialized {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Database connection lost errors
    #[error("Database connection lost: {message}")]
    DatabaseConnectionLost {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Database transient errors
    #[error("Database transient error: {message}")]
    DatabaseTransient {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Replace conflict errors
    #[error("Replace conflict: {message}")]
    ReplaceConflict {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Retry timeout exceeded errors
    #[error("Retry timeout exceeded: {message}")]
    RetryTimeoutExceeded {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unhandled errors
    #[error("Unhandled error: {message}")]
    Unhandled {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unknown data source errors
    #[error("Unknown data source: {message}")]
    UnknownDataSource {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Environment has been destroyed
    ///
    /// Corresponds to SzEnvironmentDestroyedException in C# SDK.
    /// This error occurs when attempting to use an environment that has already
    /// been destroyed.
    #[error("Environment destroyed: {message}")]
    EnvironmentDestroyed {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// FFI-related errors
    #[error("FFI error: {message}")]
    Ffi {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// String conversion errors (C string handling)
    #[error("String conversion error: {0}")]
    StringConversion(#[from] NulError),
}

impl SzError {
    /// Creates a new BadInput error
    pub fn bad_input<S: Into<String>>(message: S) -> Self {
        Self::BadInput {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new License error
    pub fn license<S: Into<String>>(message: S) -> Self {
        Self::License {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new NotFound error
    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Self::NotFound {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Retryable error
    pub fn retryable<S: Into<String>>(message: S) -> Self {
        Self::Retryable {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Unrecoverable error
    pub fn unrecoverable<S: Into<String>>(message: S) -> Self {
        Self::Unrecoverable {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new FFI error
    pub fn ffi<S: Into<String>>(message: S) -> Self {
        Self::Ffi {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new NotInitialized error
    pub fn not_initialized<S: Into<String>>(message: S) -> Self {
        Self::NotInitialized {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new DatabaseConnectionLost error
    pub fn database_connection_lost<S: Into<String>>(message: S) -> Self {
        Self::DatabaseConnectionLost {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new DatabaseTransient error
    pub fn database_transient<S: Into<String>>(message: S) -> Self {
        Self::DatabaseTransient {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new ReplaceConflict error
    pub fn replace_conflict<S: Into<String>>(message: S) -> Self {
        Self::ReplaceConflict {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new RetryTimeoutExceeded error
    pub fn retry_timeout_exceeded<S: Into<String>>(message: S) -> Self {
        Self::RetryTimeoutExceeded {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Unhandled error
    pub fn unhandled<S: Into<String>>(message: S) -> Self {
        Self::Unhandled {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new UnknownDataSource error
    pub fn unknown_data_source<S: Into<String>>(message: S) -> Self {
        Self::UnknownDataSource {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new EnvironmentDestroyed error
    pub fn environment_destroyed<S: Into<String>>(message: S) -> Self {
        Self::EnvironmentDestroyed {
            message: message.into(),
            source: None,
        }
    }

    /// Returns true if this error indicates the operation should be retried
    ///
    /// This includes Retryable and its subtypes:
    /// - DatabaseConnectionLost
    /// - DatabaseTransient
    /// - RetryTimeoutExceeded
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SzError::Retryable { .. }
                | SzError::DatabaseConnectionLost { .. }
                | SzError::DatabaseTransient { .. }
                | SzError::RetryTimeoutExceeded { .. }
        )
    }

    /// Returns true if this error is unrecoverable
    ///
    /// This includes Unrecoverable and its subtypes:
    /// - Database
    /// - License
    /// - NotInitialized
    /// - Unhandled
    pub fn is_unrecoverable(&self) -> bool {
        matches!(
            self,
            SzError::Unrecoverable { .. }
                | SzError::Database { .. }
                | SzError::License { .. }
                | SzError::NotInitialized { .. }
                | SzError::Unhandled { .. }
        )
    }

    /// Returns true if this error is a bad input error
    ///
    /// This includes BadInput and its subtypes:
    /// - NotFound
    /// - UnknownDataSource
    pub fn is_bad_input(&self) -> bool {
        matches!(
            self,
            SzError::BadInput { .. } | SzError::NotFound { .. } | SzError::UnknownDataSource { .. }
        )
    }

    /// Creates an error from getLastExceptionCode() with message from getLastException()
    pub fn from_code_with_message(error_code: i64, component: SzComponent) -> Self {
        let error_msg = Self::get_last_exception_message(component, error_code);

        match error_code {
            // Specific error codes that map to new error types (check these first)
            47..=63 => Self::not_initialized(error_msg), // Not initialized errors

            // Detailed error code ranges from getLastExceptionCode()
            0..=46 | 64..=100 => Self::bad_input(error_msg), // Bad input range (excluding not_initialized)
            999 => Self::license(error_msg),                 // License error
            1000..=1020 => Self::database(error_msg),        // Database errors
            2000..=2300 => Self::configuration(error_msg),   // Configuration errors
            7200..=7299 => Self::configuration(error_msg),   // Configuration errors (extended)
            7301..=7400 => Self::bad_input(error_msg),       // Bad input errors (extended)
            8500..=8600 => Self::database(error_msg),        // Secure storage/database
            9000..=9099 | 9201..=9999 => Self::license(error_msg), // License errors (extended)
            9100..=9200 => Self::configuration(error_msg),   // Configuration errors (extended)

            // Default to unknown for any other codes
            _ => Self::unknown(error_msg),
        }
    }

    /// Gets the last exception message from the specified component
    fn get_last_exception_message(component: SzComponent, error_code: i64) -> String {
        use crate::ffi;
        use libc::c_char;

        const BUFFER_SIZE: usize = 4096;
        let mut buffer = vec![0i8; BUFFER_SIZE];

        let result = unsafe {
            match component {
                SzComponent::Engine => ffi::bindings::Sz_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE as i64,
                ),
                SzComponent::Config => ffi::bindings::SzConfig_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE as i64,
                ),
                SzComponent::ConfigMgr => ffi::bindings::SzConfigMgr_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE as i64,
                ),
                SzComponent::Diagnostic => ffi::bindings::SzDiagnostic_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE as i64,
                ),
                SzComponent::Product => ffi::bindings::SzProduct_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE as i64,
                ),
            }
        };

        if result > 0 {
            // Successfully got exception message
            unsafe {
                match CStr::from_ptr(buffer.as_ptr()).to_str() {
                    Ok(message) if !message.is_empty() => message.to_string(),
                    _ => format!("Native error (code: {})", error_code),
                }
            }
        } else {
            // Failed to get exception message, use generic message
            format!("Native error (code: {})", error_code)
        }
    }

    /// Creates an error from getLastExceptionCode() (legacy method for compatibility)
    pub fn from_code(error_code: i64) -> Self {
        // Default to Engine component for backward compatibility
        Self::from_code_with_message(error_code, SzComponent::Engine)
    }

    /// Creates an Unknown error from a source error
    pub fn from_source(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Unknown {
            message: source.to_string(),
            source: Some(source),
        }
    }

    /// Creates an Unknown error with a custom message and source
    pub fn with_message_and_source<S: Into<String>>(
        message: S,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::Unknown {
            message: message.into(),
            source: Some(source),
        }
    }
}

#[cfg(test)]
mod test_error_mapping {
    use super::*;

    #[test]
    fn test_error_code_7220_maps_to_configuration() {
        let error = SzError::from_code(7220);
        match error {
            SzError::Configuration { message, .. } => {
                // Message should either be from getLastException or fallback format
                assert!(message.contains("7220") || !message.is_empty());
            }
            _ => panic!(
                "Error code 7220 should map to Configuration, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_not_initialized_error_codes() {
        for code in 47..=63 {
            let error = SzError::from_code(code);
            match error {
                SzError::NotInitialized { .. } => {
                    // Expected
                }
                _ => panic!(
                    "Error code {} should map to NotInitialized, got: {:?}",
                    code, error
                ),
            }
        }
    }

    #[test]
    fn test_license_error_code_999() {
        let error = SzError::from_code(999);
        match error {
            SzError::License { .. } => {
                // Expected
            }
            _ => panic!("Error code 999 should map to License, got: {:?}", error),
        }
    }

    #[test]
    fn test_database_error_range() {
        let error = SzError::from_code(1010);
        match error {
            SzError::Database { .. } => {
                // Expected
            }
            _ => panic!("Error code 1010 should map to Database, got: {:?}", error),
        }
    }

    #[test]
    fn test_unknown_error_default() {
        let error = SzError::from_code(99999);
        match error {
            SzError::Unknown { .. } => {
                // Expected
            }
            _ => panic!("Error code 99999 should map to Unknown, got: {:?}", error),
        }
    }

    #[test]
    fn test_from_code_with_message() {
        let error = SzError::from_code_with_message(7220, SzComponent::Config);
        match error {
            SzError::Configuration { message, .. } => {
                // Message should either be from getLastException or fallback format
                assert!(!message.is_empty());
            }
            _ => panic!(
                "Error code 7220 should map to Configuration, got: {:?}",
                error
            ),
        }
    }
}
