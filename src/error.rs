//! Error types for the Senzing Rust SDK
//!
//! This module defines a comprehensive error hierarchy that mirrors the
//! Senzing C# SDK exception hierarchy while leveraging Rust's `Result<T, E>` types.
//!
//! The error system provides detailed error information from the underlying
//! Senzing C library with proper error chains and backtrace support.
//!
//! # Error Categories
//!
//! * [`SzError::Configuration`] - Configuration and setup errors
//! * [`SzError::BadInput`] - Invalid input parameters or data
//! * [`SzError::Database`] - Database connection or operation errors
//! * [`SzError::NotFound`] - Resource or entity not found
//! * [`SzError::Retryable`] - Temporary errors that can be retried
//! * [`SzError::Unrecoverable`] - Fatal errors requiring reinitialization
//! * [`SzError::License`] - Licensing issues
//! * [`SzError::Unknown`] - Unexpected or unclassified errors
//!
//! # Examples
//!
//! ```
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
use thiserror::Error;

/// Result type alias for Senzing SDK operations
///
/// This is the standard Result type used throughout the Senzing Rust SDK.
/// All Senzing operations return `SzResult<T>` instead of `Result<T, SzError>`.
///
/// # Examples
///
/// ```
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

    /// Returns true if this error indicates the operation should be retried
    pub fn is_retryable(&self) -> bool {
        matches!(self, SzError::Retryable { .. })
    }

    /// Returns true if this error is unrecoverable
    pub fn is_unrecoverable(&self) -> bool {
        matches!(self, SzError::Unrecoverable { .. })
    }

    /// Creates an error from a return code
    pub fn from_code(return_code: i64) -> Self {
        let error_msg = format!("Native error (code: {})", return_code);

        match return_code {
            -1 => Self::unknown(error_msg),
            -2 => Self::configuration(error_msg),
            -3 => Self::bad_input(error_msg),
            -4 => Self::retryable(error_msg),
            -5 => Self::unrecoverable(error_msg),
            -6 => Self::not_found(error_msg),
            -7 => Self::license(error_msg),
            -8 => Self::database(error_msg),
            _ => Self::unknown(error_msg),
        }
    }
}

/// Utility function to convert C string errors to SzError
///
/// # Safety
///
/// The caller must ensure that `c_str` is either null or points to a valid null-terminated C string.
pub unsafe fn c_str_to_sz_error(c_str: *const i8) -> SzError {
    if c_str.is_null() {
        return SzError::unknown("Unknown error occurred");
    }

    match CStr::from_ptr(c_str).to_str() {
        Ok(error_msg) => SzError::unknown(error_msg),
        Err(_) => SzError::ffi("Failed to convert C string to Rust string"),
    }
}
