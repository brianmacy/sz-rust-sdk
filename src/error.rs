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
//! ## Basic Usage
//!
//! ```no_run
//! use sz_rust_sdk::error::{SzError, SzResult};
//!
//! fn example_function() -> SzResult<String> {
//!     // Simple error creation
//!     Err(SzError::configuration("Database not initialized"))
//! }
//!
//! match example_function() {
//!     Ok(result) => println!("Success: {}", result),
//!     Err(SzError::Configuration(_)) => {
//!         eprintln!("Configuration error occurred");
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```
//!
//! ## Advanced Usage with Error Context
//!
//! ```no_run
//! use sz_rust_sdk::error::{SzError, SzResult};
//!
//! fn check_error_details() -> SzResult<()> {
//!     let err = SzError::configuration("Invalid config");
//!
//!     // Access error code if available
//!     if let Some(code) = err.error_code() {
//!         eprintln!("Senzing error code: {}", code);
//!     }
//!
//!     // Check error category
//!     if err.is_retryable() {
//!         eprintln!("This error can be retried");
//!     }
//!
//!     Err(err)
//! }
//! ```
//!
//! ## Error with Source
//!
//! ```no_run
//! use sz_rust_sdk::error::{SzError, SzResult};
//!
//! fn parse_json(data: &str) -> SzResult<serde_json::Value> {
//!     let json_err = serde_json::from_str(data)
//!         .map_err(|e| SzError::bad_input("Invalid JSON").with_source(e))?;
//!     Ok(json_err)
//! }
//! ```

use std::ffi::{CStr, NulError};

/// Senzing SDK component for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SzComponent {
    Engine,
    Config,
    ConfigMgr,
    Diagnostic,
    Product,
}

/// Error categories for hierarchy-based error handling
///
/// These represent both base categories (BadInput, Retryable, Unrecoverable)
/// and specific error types. The hierarchy allows checking if an error
/// "is a" type, including parent types.
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::error::{SzError, ErrorCategory};
///
/// let err = SzError::database_transient("Deadlock");
///
/// // Check specific type
/// assert!(err.is(ErrorCategory::DatabaseTransient));
///
/// // Check parent category (polymorphic)
/// assert!(err.is(ErrorCategory::Retryable));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    // Base categories
    BadInput,
    Retryable,
    Unrecoverable,

    // Specific types under BadInput
    NotFound,
    UnknownDataSource,

    // Specific types under Retryable
    DatabaseConnectionLost,
    DatabaseTransient,
    RetryTimeoutExceeded,

    // Specific types under Unrecoverable
    Database,
    License,
    NotInitialized,
    Unhandled,

    // Standalone types
    Configuration,
    ReplaceConflict,
    EnvironmentDestroyed,
    Unknown,
}

/// Error context containing message, error code, component, and optional cause
///
/// This struct is used internally by all error variants to store common error information.
/// It reduces code duplication and makes error handling more maintainable.
///
/// # Fields
///
/// * `message` - Human-readable error description
/// * `code` - Optional Senzing native error code from getLastExceptionCode()
/// * `component` - Optional SDK component where the error originated
/// * `source` - Optional underlying error that caused this error (error chaining)
///
/// # When to use `source`
///
/// Use `source` to preserve error chains when:
/// - Wrapping a native library error
/// - An operation fails due to another error
/// - You need to preserve the error chain for debugging
///
/// Don't use `source` for:
/// - User input validation errors
/// - Simple state errors (not initialized, etc.)
#[derive(Debug)]
pub struct ErrorContext {
    /// Human-readable error message
    pub message: String,
    /// Optional Senzing native error code
    pub code: Option<i64>,
    /// Optional SDK component that generated the error
    pub component: Option<SzComponent>,
    /// Optional underlying cause of this error
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ErrorContext {
    /// Creates a new ErrorContext with just a message
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            code: None,
            component: None,
            source: None,
        }
    }

    /// Creates an ErrorContext with message, code, and component
    pub fn with_code<S: Into<String>>(message: S, code: i64, component: SzComponent) -> Self {
        Self {
            message: message.into(),
            code: Some(code),
            component: Some(component),
            source: None,
        }
    }

    /// Adds a source error to this context
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(code) = self.code {
            write!(f, " (code: {})", code)?;
        }
        Ok(())
    }
}

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

/// Extension trait for Result<T, SzError> to provide error classification helpers
///
/// This trait adds convenient methods for handling specific error categories
/// without having to manually check error types.
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::error::{SzResult, SzResultExt};
/// # use sz_rust_sdk::error::SzError;
///
/// fn example() -> SzResult<String> {
///     let result: SzResult<String> = Err(SzError::database_transient("Deadlock"));
///
///     // Map retryable errors to a retry action
///     result.or_retry(|e| {
///         println!("Retrying due to: {}", e);
///         Ok("Retry succeeded".to_string())
///     })
/// }
/// ```
pub trait SzResultExt<T> {
    /// If the error is retryable, call the provided closure; otherwise propagate the error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzResult, SzResultExt};
    /// # use sz_rust_sdk::error::SzError;
    ///
    /// # fn retry_operation() -> SzResult<String> { Ok("success".to_string()) }
    /// # fn original_operation() -> SzResult<String> { Err(SzError::database_transient("deadlock")) }
    /// let result = original_operation().or_retry(|e| {
    ///     eprintln!("Retrying due to: {}", e);
    ///     retry_operation()
    /// });
    /// ```
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>;

    /// Maps retryable errors using the provided function, propagates non-retryable errors
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzResult, SzResultExt};
    /// # use sz_rust_sdk::error::SzError;
    ///
    /// # fn operation() -> SzResult<i32> { Err(SzError::database_transient("deadlock")) }
    /// let result = operation().map_retryable(|e| {
    ///     println!("Will retry: {}", e);
    ///     Ok(42)  // Return default value on retry
    /// });
    /// ```
    fn map_retryable<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>;

    /// Returns Ok(None) for retryable errors, Err for non-retryable errors
    ///
    /// This is useful when you want to filter out retryable errors and handle them
    /// separately from non-retryable ones.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzResult, SzResultExt};
    /// # use sz_rust_sdk::error::SzError;
    ///
    /// # fn operation() -> SzResult<String> { Err(SzError::database_transient("deadlock")) }
    /// match operation().filter_retryable() {
    ///     Ok(Some(value)) => println!("Success: {}", value),
    ///     Ok(None) => println!("Retryable error, will retry"),
    ///     Err(e) => println!("Fatal error: {}", e),
    /// }
    /// ```
    fn filter_retryable(self) -> Result<Option<T>, SzError>;

    /// Returns true if the result is an error and that error is retryable
    fn is_retryable_error(&self) -> bool;

    /// Returns true if the result is an error and that error is unrecoverable
    fn is_unrecoverable_error(&self) -> bool;

    /// Returns true if the result is an error and that error is bad input
    fn is_bad_input_error(&self) -> bool;
}

impl<T> SzResultExt<T> for SzResult<T> {
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(e) if e.is_retryable() => f(e),
            Err(e) => Err(e),
        }
    }

    fn map_retryable<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>,
    {
        self.or_retry(f)
    }

    fn filter_retryable(self) -> Result<Option<T>, SzError> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.is_retryable() => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn is_retryable_error(&self) -> bool {
        matches!(self, Err(e) if e.is_retryable())
    }

    fn is_unrecoverable_error(&self) -> bool {
        matches!(self, Err(e) if e.is_unrecoverable())
    }

    fn is_bad_input_error(&self) -> bool {
        matches!(self, Err(e) if e.is_bad_input())
    }
}

/// Base error type for all Senzing SDK operations
///
/// This enum represents all possible errors that can occur when using the
/// Senzing SDK. Each variant corresponds to a specific category of error
/// returned by the underlying Senzing C library.
///
/// The error hierarchy is designed to match the Senzing C# SDK for consistency
/// across language bindings.
///
/// # Non-exhaustive
///
/// This enum is marked `#[non_exhaustive]` to allow adding new error variants
/// in future versions without breaking existing code. Always include a catch-all
/// pattern when matching.
///
/// # Examples
///
/// ```no_run
/// use sz_rust_sdk::error::SzError;
///
/// fn handle_error(error: SzError) {
///     match error {
///         SzError::NotFound(_) => println!("Resource not found"),
///         SzError::Configuration(_) => println!("Configuration error"),
///         // Always include catch-all for non-exhaustive enums
///         _ => println!("Other error: {}", error),
///     }
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum SzError {
    /// Errors related to invalid input parameters
    BadInput(ErrorContext),

    /// Configuration-related errors
    Configuration(ErrorContext),

    /// Database operation errors
    Database(ErrorContext),

    /// License-related errors
    License(ErrorContext),

    /// Resource not found errors
    NotFound(ErrorContext),

    /// Errors that indicate the operation should be retried
    Retryable(ErrorContext),

    /// Unrecoverable errors that require reinitialization
    Unrecoverable(ErrorContext),

    /// Unknown or unexpected errors
    Unknown(ErrorContext),

    /// System not initialized errors
    NotInitialized(ErrorContext),

    /// Database connection lost errors
    DatabaseConnectionLost(ErrorContext),

    /// Database transient errors (e.g., deadlocks)
    DatabaseTransient(ErrorContext),

    /// Replace conflict errors
    ReplaceConflict(ErrorContext),

    /// Retry timeout exceeded errors
    RetryTimeoutExceeded(ErrorContext),

    /// Unhandled errors
    Unhandled(ErrorContext),

    /// Unknown data source errors
    UnknownDataSource(ErrorContext),

    /// Environment has been destroyed
    ///
    /// Corresponds to SzEnvironmentDestroyedException in C# SDK.
    /// This error occurs when attempting to use an environment that has already
    /// been destroyed.
    EnvironmentDestroyed(ErrorContext),

    /// FFI-related errors
    Ffi(ErrorContext),

    /// JSON serialization/deserialization errors
    Json(serde_json::Error),

    /// String conversion errors (C string handling)
    StringConversion(NulError),
}

// Manual implementation of Display and Error traits
impl std::fmt::Display for SzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadInput(ctx) => write!(f, "Bad input: {}", ctx),
            Self::Configuration(ctx) => write!(f, "Configuration error: {}", ctx),
            Self::Database(ctx) => write!(f, "Database error: {}", ctx),
            Self::License(ctx) => write!(f, "License error: {}", ctx),
            Self::NotFound(ctx) => write!(f, "Not found: {}", ctx),
            Self::Retryable(ctx) => write!(f, "Retryable error: {}", ctx),
            Self::Unrecoverable(ctx) => write!(f, "Unrecoverable error: {}", ctx),
            Self::Unknown(ctx) => write!(f, "Unknown error: {}", ctx),
            Self::NotInitialized(ctx) => write!(f, "Not initialized: {}", ctx),
            Self::DatabaseConnectionLost(ctx) => write!(f, "Database connection lost: {}", ctx),
            Self::DatabaseTransient(ctx) => write!(f, "Database transient error: {}", ctx),
            Self::ReplaceConflict(ctx) => write!(f, "Replace conflict: {}", ctx),
            Self::RetryTimeoutExceeded(ctx) => write!(f, "Retry timeout exceeded: {}", ctx),
            Self::Unhandled(ctx) => write!(f, "Unhandled error: {}", ctx),
            Self::UnknownDataSource(ctx) => write!(f, "Unknown data source: {}", ctx),
            Self::EnvironmentDestroyed(ctx) => write!(f, "Environment destroyed: {}", ctx),
            Self::Ffi(ctx) => write!(f, "FFI error: {}", ctx),
            Self::Json(e) => write!(f, "JSON error: {}", e),
            Self::StringConversion(e) => write!(f, "String conversion error: {}", e),
        }
    }
}

impl std::error::Error for SzError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => ctx.source.as_ref().map(|e| &**e as &dyn std::error::Error),
            Self::Json(e) => Some(e),
            Self::StringConversion(e) => Some(e),
        }
    }
}

// Implement From for automatic conversions
impl From<serde_json::Error> for SzError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<NulError> for SzError {
    fn from(err: NulError) -> Self {
        Self::StringConversion(err)
    }
}

impl SzError {
    // ========================================================================
    // Error Construction - Simple Constructors
    // ========================================================================

    /// Creates a new BadInput error
    pub fn bad_input<S: Into<String>>(message: S) -> Self {
        Self::BadInput(ErrorContext::new(message))
    }

    /// Creates a new Configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration(ErrorContext::new(message))
    }

    /// Creates a new Database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database(ErrorContext::new(message))
    }

    /// Creates a new License error
    pub fn license<S: Into<String>>(message: S) -> Self {
        Self::License(ErrorContext::new(message))
    }

    /// Creates a new NotFound error
    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Self::NotFound(ErrorContext::new(message))
    }

    /// Creates a new Retryable error
    pub fn retryable<S: Into<String>>(message: S) -> Self {
        Self::Retryable(ErrorContext::new(message))
    }

    /// Creates a new Unrecoverable error
    pub fn unrecoverable<S: Into<String>>(message: S) -> Self {
        Self::Unrecoverable(ErrorContext::new(message))
    }

    /// Creates a new Unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown(ErrorContext::new(message))
    }

    /// Creates a new FFI error
    pub fn ffi<S: Into<String>>(message: S) -> Self {
        Self::Ffi(ErrorContext::new(message))
    }

    /// Creates a new NotInitialized error
    pub fn not_initialized<S: Into<String>>(message: S) -> Self {
        Self::NotInitialized(ErrorContext::new(message))
    }

    /// Creates a new DatabaseConnectionLost error
    pub fn database_connection_lost<S: Into<String>>(message: S) -> Self {
        Self::DatabaseConnectionLost(ErrorContext::new(message))
    }

    /// Creates a new DatabaseTransient error
    pub fn database_transient<S: Into<String>>(message: S) -> Self {
        Self::DatabaseTransient(ErrorContext::new(message))
    }

    /// Creates a new ReplaceConflict error
    pub fn replace_conflict<S: Into<String>>(message: S) -> Self {
        Self::ReplaceConflict(ErrorContext::new(message))
    }

    /// Creates a new RetryTimeoutExceeded error
    pub fn retry_timeout_exceeded<S: Into<String>>(message: S) -> Self {
        Self::RetryTimeoutExceeded(ErrorContext::new(message))
    }

    /// Creates a new Unhandled error
    pub fn unhandled<S: Into<String>>(message: S) -> Self {
        Self::Unhandled(ErrorContext::new(message))
    }

    /// Creates a new UnknownDataSource error
    pub fn unknown_data_source<S: Into<String>>(message: S) -> Self {
        Self::UnknownDataSource(ErrorContext::new(message))
    }

    /// Creates a new EnvironmentDestroyed error
    pub fn environment_destroyed<S: Into<String>>(message: S) -> Self {
        Self::EnvironmentDestroyed(ErrorContext::new(message))
    }

    // ========================================================================
    // Error Inspection - Helper Methods
    // ========================================================================

    /// Returns the error code if available
    ///
    /// Error codes are populated when errors are created from native Senzing
    /// error codes via `from_code()` or `from_code_with_message()`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::from_code(999);
    /// assert_eq!(error.error_code(), Some(999));
    /// ```
    pub fn error_code(&self) -> Option<i64> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => ctx.code,
            Self::Json(_) | Self::StringConversion(_) => None,
        }
    }

    /// Returns the component that generated this error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzError, SzComponent};
    ///
    /// let error = SzError::from_code_with_message(999, SzComponent::Engine);
    /// assert_eq!(error.component(), Some(SzComponent::Engine));
    /// ```
    pub fn component(&self) -> Option<SzComponent> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => ctx.component,
            Self::Json(_) | Self::StringConversion(_) => None,
        }
    }

    /// Returns the error message
    ///
    /// This extracts just the message string without the error type prefix.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::database_connection_lost("Connection failed");
    /// assert_eq!(error.message(), "Connection failed");
    ///
    /// let error = SzError::from_code(1008);
    /// // Returns the message from getLastException()
    /// assert!(!error.message().is_empty());
    /// ```
    pub fn message(&self) -> &str {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => &ctx.message,
            Self::Json(_) => "JSON error",
            Self::StringConversion(_) => "String conversion error",
        }
    }

    /// Returns true if this error indicates the operation should be retried
    ///
    /// This includes Retryable and its subtypes:
    /// - DatabaseConnectionLost
    /// - DatabaseTransient
    /// - RetryTimeoutExceeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::database_connection_lost("Connection lost");
    /// assert!(error.is_retryable());
    /// ```
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SzError::Retryable(_)
                | SzError::DatabaseConnectionLost(_)
                | SzError::DatabaseTransient(_)
                | SzError::RetryTimeoutExceeded(_)
        )
    }

    /// Returns true if this error is unrecoverable
    ///
    /// This includes Unrecoverable and its subtypes:
    /// - Database
    /// - License
    /// - NotInitialized
    /// - Unhandled
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::license("License expired");
    /// assert!(error.is_unrecoverable());
    /// ```
    pub fn is_unrecoverable(&self) -> bool {
        matches!(
            self,
            SzError::Unrecoverable(_)
                | SzError::Database(_)
                | SzError::License(_)
                | SzError::NotInitialized(_)
                | SzError::Unhandled(_)
        )
    }

    /// Returns true if this error is a bad input error
    ///
    /// This includes BadInput and its subtypes:
    /// - NotFound
    /// - UnknownDataSource
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::not_found("Entity not found");
    /// assert!(error.is_bad_input());
    /// ```
    pub fn is_bad_input(&self) -> bool {
        matches!(
            self,
            SzError::BadInput(_) | SzError::NotFound(_) | SzError::UnknownDataSource(_)
        )
    }

    /// Returns true if this is a database-related error
    ///
    /// This includes ALL database errors regardless of retryability:
    /// - Database (unrecoverable)
    /// - DatabaseConnectionLost (retryable)
    /// - DatabaseTransient (retryable)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// // Unrecoverable database error
    /// let error = SzError::database("Schema error");
    /// assert!(error.is_database());
    /// assert!(error.is_unrecoverable());
    ///
    /// // Retryable database error
    /// let error = SzError::database_transient("Deadlock");
    /// assert!(error.is_database());
    /// assert!(error.is_retryable());
    /// ```
    pub fn is_database(&self) -> bool {
        matches!(
            self,
            SzError::Database(_)
                | SzError::DatabaseConnectionLost(_)
                | SzError::DatabaseTransient(_)
        )
    }

    /// Returns true if this is a license-related error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::license("License expired");
    /// assert!(error.is_license());
    /// ```
    pub fn is_license(&self) -> bool {
        matches!(self, SzError::License(_))
    }

    /// Returns true if this is a configuration-related error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::configuration("Invalid config");
    /// assert!(error.is_configuration());
    /// ```
    pub fn is_configuration(&self) -> bool {
        matches!(self, SzError::Configuration(_))
    }

    /// Returns true if this is an initialization-related error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::not_initialized("SDK not initialized");
    /// assert!(error.is_initialization());
    /// ```
    pub fn is_initialization(&self) -> bool {
        matches!(self, SzError::NotInitialized(_))
    }

    /// Returns this error's type hierarchy from most specific to least
    ///
    /// This makes parent-child relationships explicit and queryable at runtime.
    /// The first element is always the most specific type, followed by parent
    /// categories in order.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzError, ErrorCategory};
    ///
    /// let err = SzError::database_transient("Deadlock");
    ///
    /// // Get the full hierarchy
    /// let hierarchy = err.hierarchy();
    /// assert_eq!(hierarchy, vec![
    ///     ErrorCategory::DatabaseTransient,
    ///     ErrorCategory::Retryable,
    /// ]);
    ///
    /// // Check if error "is a" Retryable (polymorphic check)
    /// assert!(err.is(ErrorCategory::Retryable));
    /// assert!(err.is(ErrorCategory::DatabaseTransient));
    /// ```
    pub fn hierarchy(&self) -> Vec<ErrorCategory> {
        // If we have an error code, use the generated hierarchy
        if let Some(code) = self.error_code() {
            let generated = crate::error_mappings_generated::get_error_hierarchy(code);
            if !generated.is_empty() {
                return generated;
            }
        }

        // Fallback to manual mapping for errors without codes
        match self {
            // BadInput family
            Self::BadInput(_) => vec![ErrorCategory::BadInput],
            Self::NotFound(_) => vec![ErrorCategory::NotFound, ErrorCategory::BadInput],
            Self::UnknownDataSource(_) => {
                vec![ErrorCategory::UnknownDataSource, ErrorCategory::BadInput]
            }

            // Retryable family
            Self::Retryable(_) => vec![ErrorCategory::Retryable],
            Self::DatabaseConnectionLost(_) => {
                vec![
                    ErrorCategory::DatabaseConnectionLost,
                    ErrorCategory::Retryable,
                ]
            }
            Self::DatabaseTransient(_) => {
                vec![ErrorCategory::DatabaseTransient, ErrorCategory::Retryable]
            }
            Self::RetryTimeoutExceeded(_) => {
                vec![
                    ErrorCategory::RetryTimeoutExceeded,
                    ErrorCategory::Retryable,
                ]
            }

            // Unrecoverable family
            Self::Unrecoverable(_) => vec![ErrorCategory::Unrecoverable],
            Self::Database(_) => vec![ErrorCategory::Database, ErrorCategory::Unrecoverable],
            Self::License(_) => vec![ErrorCategory::License, ErrorCategory::Unrecoverable],
            Self::NotInitialized(_) => {
                vec![ErrorCategory::NotInitialized, ErrorCategory::Unrecoverable]
            }
            Self::Unhandled(_) => vec![ErrorCategory::Unhandled, ErrorCategory::Unrecoverable],

            // Standalone types
            Self::Configuration(_) => vec![ErrorCategory::Configuration],
            Self::ReplaceConflict(_) => vec![ErrorCategory::ReplaceConflict],
            Self::EnvironmentDestroyed(_) => vec![ErrorCategory::EnvironmentDestroyed],
            Self::Unknown(_) => vec![ErrorCategory::Unknown],

            // FFI errors (no hierarchy)
            Self::Ffi(_) | Self::Json(_) | Self::StringConversion(_) => vec![],
        }
    }

    /// Checks if this error belongs to a category (polymorphic check)
    ///
    /// This checks the entire hierarchy, so `DatabaseTransient` will return
    /// true for both `ErrorCategory::DatabaseTransient` and `ErrorCategory::Retryable`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzError, ErrorCategory};
    ///
    /// let err = SzError::database_transient("Deadlock");
    ///
    /// // Check specific type
    /// assert!(err.is(ErrorCategory::DatabaseTransient));
    ///
    /// // Check parent category (polymorphic)
    /// assert!(err.is(ErrorCategory::Retryable));
    ///
    /// // Not in this category
    /// assert!(!err.is(ErrorCategory::BadInput));
    /// ```
    pub fn is(&self, category: ErrorCategory) -> bool {
        self.hierarchy().contains(&category)
    }

    // ========================================================================
    // Error Metadata - For Error Reporting Integration
    // ========================================================================

    /// Returns the error category as a string
    ///
    /// This is useful for error reporting tools and logging systems that need
    /// to categorize errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::from_code(1008);
    /// assert_eq!(error.category(), "database_transient");
    ///
    /// let error = SzError::license("Expired");
    /// assert_eq!(error.category(), "license");
    /// ```
    pub fn category(&self) -> &'static str {
        match self {
            Self::BadInput(_) | Self::NotFound(_) | Self::UnknownDataSource(_) => "bad_input",
            Self::Configuration(_) => "configuration",
            Self::Database(_) => "database",
            Self::DatabaseConnectionLost(_) => "database_connection",
            Self::DatabaseTransient(_) => "database_transient",
            Self::License(_) => "license",
            Self::NotInitialized(_) => "not_initialized",
            Self::Retryable(_) | Self::RetryTimeoutExceeded(_) => "retryable",
            Self::Unrecoverable(_) | Self::Unhandled(_) => "unrecoverable",
            Self::ReplaceConflict(_) => "replace_conflict",
            Self::EnvironmentDestroyed(_) => "environment_destroyed",
            Self::Unknown(_) => "unknown",
            Self::Ffi(_) => "ffi",
            Self::Json(_) => "json",
            Self::StringConversion(_) => "string_conversion",
        }
    }

    /// Returns the severity level of this error
    ///
    /// Severity levels:
    /// - `"critical"`: License failures, unhandled errors
    /// - `"high"`: Database errors, not initialized
    /// - `"medium"`: Connection issues, transient errors, configuration
    /// - `"low"`: Input validation, not found
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// let error = SzError::license("Expired");
    /// assert_eq!(error.severity(), "critical");
    ///
    /// let error = SzError::database_transient("Deadlock");
    /// assert_eq!(error.severity(), "medium");
    /// ```
    pub fn severity(&self) -> &'static str {
        match self {
            Self::License(_) | Self::Unrecoverable(_) | Self::Unhandled(_) => "critical",
            Self::Database(_) | Self::NotInitialized(_) => "high",
            Self::DatabaseConnectionLost(_)
            | Self::DatabaseTransient(_)
            | Self::Configuration(_) => "medium",
            _ => "low",
        }
    }

    // ========================================================================
    // Error Code Mapping - From Native Senzing Errors
    // ========================================================================

    /// Creates an error from getLastExceptionCode() with message from getLastException()
    ///
    /// This method maps native Senzing error codes to the appropriate Rust error type.
    /// The mapping is auto-generated from szerrors.json and covers all 456 Senzing error codes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::{SzError, SzComponent};
    ///
    /// let error = SzError::from_code_with_message(999, SzComponent::Engine);
    /// assert!(matches!(error, SzError::License(_)));
    /// assert_eq!(error.error_code(), Some(999));
    /// ```
    pub fn from_code_with_message(error_code: i64, component: SzComponent) -> Self {
        let error_msg = Self::get_last_exception_message(component, error_code);
        let ctx = ErrorContext::with_code(error_msg, error_code, component);

        // Use generated error mapping (456 error codes from szerrors.json)
        crate::error_mappings_generated::map_error_code(error_code, ctx)
    }

    /// Gets the last exception message from the specified component
    fn get_last_exception_message(component: SzComponent, error_code: i64) -> String {
        use crate::ffi;
        use libc::c_char;

        const BUFFER_SIZE: usize = 4096;
        let mut buffer = vec![0 as c_char; BUFFER_SIZE];

        let result = unsafe {
            match component {
                SzComponent::Engine => {
                    ffi::Sz_getLastException(buffer.as_mut_ptr() as *mut c_char, BUFFER_SIZE)
                }
                SzComponent::Config => {
                    ffi::SzConfig_getLastException(buffer.as_mut_ptr() as *mut c_char, BUFFER_SIZE)
                }
                SzComponent::ConfigMgr => ffi::SzConfigMgr_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE,
                ),
                SzComponent::Diagnostic => ffi::SzDiagnostic_getLastException(
                    buffer.as_mut_ptr() as *mut c_char,
                    BUFFER_SIZE,
                ),
                SzComponent::Product => {
                    ffi::SzProduct_getLastException(buffer.as_mut_ptr() as *mut c_char, BUFFER_SIZE)
                }
            }
        };

        if result > 0 {
            // Successfully got exception message
            unsafe {
                match CStr::from_ptr(buffer.as_ptr()).to_str() {
                    Ok(message) if !message.is_empty() => message.to_string(),
                    _ => format!("Native error (code: {error_code})"),
                }
            }
        } else {
            // Failed to get exception message, use generic message
            format!("Native error (code: {error_code})")
        }
    }

    /// Creates an error from getLastExceptionCode() (legacy method for compatibility)
    pub fn from_code(error_code: i64) -> Self {
        // Default to Engine component for backward compatibility
        Self::from_code_with_message(error_code, SzComponent::Engine)
    }

    /// Creates an Unknown error from a source error
    pub fn from_source(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        let message = source.to_string();
        Self::Unknown(ErrorContext {
            message,
            code: None,
            component: None,
            source: Some(source),
        })
    }

    /// Creates an Unknown error with a custom message and source
    pub fn with_message_and_source<S: Into<String>>(
        message: S,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::Unknown(ErrorContext {
            message: message.into(),
            code: None,
            component: None,
            source: Some(source),
        })
    }
}

// ========================================================================
// ErrorContext Extension Methods
// ========================================================================

impl ErrorContext {
    /// Adds a source error (builder pattern)
    ///
    /// This is useful for chaining error construction:
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::ErrorContext;
    ///
    /// let ctx = ErrorContext::new("Parse failed")
    ///     .with_source(std::io::Error::other("IO error"));
    /// ```
    pub fn chain_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

// ========================================================================
// SzError Extension Methods for Builder Pattern
// ========================================================================

impl SzError {
    /// Adds a source error to this error (builder pattern)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sz_rust_sdk::error::SzError;
    ///
    /// fn parse_config(data: &str) -> Result<(), SzError> {
    ///     let json_result: Result<serde_json::Value, _> = serde_json::from_str(data);
    ///     json_result.map_err(|e|
    ///         SzError::configuration("Invalid JSON config")
    ///             .with_source(e)
    ///     )?;
    ///     Ok(())
    /// }
    /// ```
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        match &mut self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => {
                ctx.source = Some(Box::new(source));
            }
            // Json and StringConversion already have their source
            Self::Json(_) | Self::StringConversion(_) => {}
        }
        self
    }
}

// ========================================================================
// Tests
// ========================================================================

#[cfg(test)]
mod test_error_mapping {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_error_code_10_maps_to_retry_timeout() {
        let error = SzError::from_code(10);
        assert!(
            matches!(error, SzError::RetryTimeoutExceeded(_)),
            "Error code 10 should map to RetryTimeoutExceeded, got: {error:?}"
        );
        assert_eq!(error.error_code(), Some(10));
    }

    #[test]
    fn test_error_code_87_maps_to_unhandled() {
        let error = SzError::from_code(87);
        assert!(
            matches!(error, SzError::Unhandled(_)),
            "Error code 87 should map to Unhandled, got: {error:?}"
        );
        assert_eq!(error.error_code(), Some(87));
    }

    #[test]
    fn test_error_code_1006_maps_to_connection_lost() {
        let error = SzError::from_code(1006);
        assert!(
            matches!(error, SzError::DatabaseConnectionLost(_)),
            "Error code 1006 should map to DatabaseConnectionLost, got: {error:?}"
        );
        assert!(error.is_retryable());
        assert_eq!(error.error_code(), Some(1006));
    }

    #[test]
    fn test_error_code_1007_maps_to_connection_lost() {
        let error = SzError::from_code(1007);
        assert!(
            matches!(error, SzError::DatabaseConnectionLost(_)),
            "Error code 1007 should map to DatabaseConnectionLost, got: {error:?}"
        );
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_code_1008_maps_to_database_transient() {
        let error = SzError::from_code(1008);
        assert!(
            matches!(error, SzError::DatabaseTransient(_)),
            "Error code 1008 should map to DatabaseTransient, got: {error:?}"
        );
        assert!(error.is_retryable());
        assert_eq!(error.error_code(), Some(1008));
    }

    #[test]
    fn test_not_initialized_error_codes() {
        for code in [48, 49, 50, 53] {
            let error = SzError::from_code(code);
            assert!(
                matches!(error, SzError::NotInitialized(_)),
                "Error code {code} should map to NotInitialized, got: {error:?}"
            );
        }
    }

    #[test]
    fn test_license_error_code_999() {
        let error = SzError::from_code(999);
        assert!(
            matches!(error, SzError::License(_)),
            "Error code 999 should map to License, got: {error:?}"
        );
        assert_eq!(error.error_code(), Some(999));
    }

    #[test]
    fn test_database_error_range() {
        let error = SzError::from_code(1010);
        assert!(
            matches!(error, SzError::Database(_)),
            "Error code 1010 should map to Database, got: {error:?}"
        );
    }

    #[test]
    fn test_bad_input_range() {
        // Test codes that map to BadInput (excluding NotFound/UnknownDataSource subtypes)
        for code in [2, 7, 22, 51, 88] {
            let error = SzError::from_code(code);
            assert!(
                matches!(error, SzError::BadInput(_)),
                "Error code {code} should map to BadInput, got: {error:?}"
            );
        }

        // Code 33 is NotFound (subtype of BadInput)
        let error = SzError::from_code(33);
        assert!(
            matches!(error, SzError::NotFound(_)),
            "Error code 33 should map to NotFound, got: {error:?}"
        );
        // But it should still be recognized as BadInput category
        assert!(error.is_bad_input());
    }

    #[test]
    fn test_configuration_range() {
        let error = SzError::from_code(2001);
        assert!(
            matches!(error, SzError::Configuration(_)),
            "Error code 2001 should map to Configuration, got: {error:?}"
        );
    }

    #[test]
    fn test_unknown_error_default() {
        let error = SzError::from_code(99999);
        assert!(
            matches!(error, SzError::Unknown(_)),
            "Error code 99999 should map to Unknown, got: {error:?}"
        );
    }

    #[test]
    fn test_from_code_with_message() {
        let error = SzError::from_code_with_message(999, SzComponent::Config);
        assert!(matches!(error, SzError::License(_)));
        assert_eq!(error.component(), Some(SzComponent::Config));
        assert_eq!(error.error_code(), Some(999));
    }

    #[test]
    fn test_error_with_source() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let error = SzError::configuration("Parse failed").with_source(json_err);

        assert!(matches!(error, SzError::Configuration(_)));
        assert!(error.source().is_some());
    }

    #[test]
    fn test_is_retryable_methods() {
        assert!(SzError::retry_timeout_exceeded("Timeout").is_retryable());
        assert!(SzError::database_connection_lost("Lost").is_retryable());
        assert!(SzError::database_transient("Deadlock").is_retryable());
        assert!(!SzError::bad_input("Invalid").is_retryable());
    }

    #[test]
    fn test_is_unrecoverable_methods() {
        assert!(SzError::license("Expired").is_unrecoverable());
        assert!(SzError::not_initialized("Not init").is_unrecoverable());
        assert!(SzError::database("DB error").is_unrecoverable());
        assert!(!SzError::bad_input("Invalid").is_unrecoverable());
    }

    #[test]
    fn test_is_bad_input_methods() {
        assert!(SzError::bad_input("Invalid").is_bad_input());
        assert!(SzError::not_found("Missing").is_bad_input());
        assert!(SzError::unknown_data_source("Unknown").is_bad_input());
        assert!(!SzError::configuration("Config").is_bad_input());
    }

    #[test]
    fn test_error_context_preservation() {
        let error = SzError::from_code_with_message(1008, SzComponent::Engine);

        assert_eq!(error.error_code(), Some(1008));
        assert_eq!(error.component(), Some(SzComponent::Engine));
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_category() {
        assert_eq!(
            SzError::database_transient("test").category(),
            "database_transient"
        );
        assert_eq!(SzError::license("test").category(), "license");
        assert_eq!(SzError::bad_input("test").category(), "bad_input");
        assert_eq!(SzError::not_found("test").category(), "bad_input");
        assert_eq!(SzError::configuration("test").category(), "configuration");
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(SzError::license("test").severity(), "critical");
        assert_eq!(SzError::unhandled("test").severity(), "critical");
        assert_eq!(SzError::database("test").severity(), "high");
        assert_eq!(SzError::database_transient("test").severity(), "medium");
        assert_eq!(SzError::bad_input("test").severity(), "low");
    }

    #[test]
    fn test_error_metadata_complete() {
        let error = SzError::from_code_with_message(1008, SzComponent::Engine);

        // Verify all metadata is accessible
        assert_eq!(error.error_code(), Some(1008));
        assert_eq!(error.component(), Some(SzComponent::Engine));
        assert_eq!(error.category(), "database_transient");
        assert_eq!(error.severity(), "medium");
        assert!(error.is_retryable());
        assert!(!error.is_unrecoverable());
        assert!(!error.is_bad_input());
    }

    #[test]
    fn test_result_ext_or_retry() {
        use super::SzResultExt;

        // Retryable error should trigger retry
        let result: SzResult<i32> = Err(SzError::database_transient("Deadlock"));
        let retried = result.or_retry(|e| {
            assert!(e.is_retryable());
            Ok(42)
        });
        assert_eq!(retried.unwrap(), 42);

        // Non-retryable error should propagate
        let result: SzResult<i32> = Err(SzError::license("Expired"));
        let retried = result.or_retry(|_| Ok(42));
        assert!(retried.is_err());
        assert!(retried.unwrap_err().is_unrecoverable());
    }

    #[test]
    fn test_result_ext_filter_retryable() {
        use super::SzResultExt;

        // Success should return Some
        let result: SzResult<i32> = Ok(42);
        assert_eq!(result.filter_retryable().unwrap(), Some(42));

        // Retryable error should return None
        let result: SzResult<i32> = Err(SzError::database_transient("Deadlock"));
        assert_eq!(result.filter_retryable().unwrap(), None);

        // Non-retryable error should propagate
        let result: SzResult<i32> = Err(SzError::license("Expired"));
        assert!(result.filter_retryable().is_err());
    }

    #[test]
    fn test_result_ext_is_retryable_error() {
        use super::SzResultExt;

        let ok_result: SzResult<i32> = Ok(42);
        assert!(!ok_result.is_retryable_error());

        let retryable: SzResult<i32> = Err(SzError::database_transient("Deadlock"));
        assert!(retryable.is_retryable_error());

        let not_retryable: SzResult<i32> = Err(SzError::license("Expired"));
        assert!(!not_retryable.is_retryable_error());
    }

    #[test]
    fn test_result_ext_is_unrecoverable_error() {
        use super::SzResultExt;

        let ok_result: SzResult<i32> = Ok(42);
        assert!(!ok_result.is_unrecoverable_error());

        let unrecoverable: SzResult<i32> = Err(SzError::license("Expired"));
        assert!(unrecoverable.is_unrecoverable_error());

        let recoverable: SzResult<i32> = Err(SzError::bad_input("Invalid"));
        assert!(!recoverable.is_unrecoverable_error());
    }

    #[test]
    fn test_result_ext_is_bad_input_error() {
        use super::SzResultExt;

        let ok_result: SzResult<i32> = Ok(42);
        assert!(!ok_result.is_bad_input_error());

        let bad_input: SzResult<i32> = Err(SzError::bad_input("Invalid"));
        assert!(bad_input.is_bad_input_error());

        let not_bad_input: SzResult<i32> = Err(SzError::license("Expired"));
        assert!(!not_bad_input.is_bad_input_error());
    }

    #[test]
    fn test_is_database_methods() {
        // All database-related errors
        assert!(SzError::database("Schema error").is_database());
        assert!(SzError::database_connection_lost("Lost").is_database());
        assert!(SzError::database_transient("Deadlock").is_database());

        // Non-database errors
        assert!(!SzError::license("Expired").is_database());
        assert!(!SzError::configuration("Invalid").is_database());

        // Database errors can be retryable or unrecoverable
        assert!(SzError::database("Schema").is_database());
        assert!(SzError::database("Schema").is_unrecoverable());

        assert!(SzError::database_transient("Deadlock").is_database());
        assert!(SzError::database_transient("Deadlock").is_retryable());
    }

    #[test]
    fn test_is_license_methods() {
        assert!(SzError::license("Expired").is_license());
        assert!(!SzError::database("Error").is_license());
    }

    #[test]
    fn test_is_configuration_methods() {
        assert!(SzError::configuration("Invalid").is_configuration());
        assert!(!SzError::database("Error").is_configuration());
    }

    #[test]
    fn test_is_initialization_methods() {
        assert!(SzError::not_initialized("Not init").is_initialization());
        assert!(!SzError::database("Error").is_initialization());
    }

    #[test]
    fn test_error_domain_and_behavior_combined() {
        // Database error that's retryable
        let error = SzError::database_transient("Deadlock");
        assert!(error.is_database());
        assert!(error.is_retryable());
        assert!(!error.is_unrecoverable());

        // Database error that's unrecoverable
        let error = SzError::database("Schema error");
        assert!(error.is_database());
        assert!(error.is_unrecoverable());
        assert!(!error.is_retryable());
    }

    // ========================================================================
    // Error Hierarchy Tests
    // ========================================================================

    #[test]
    fn test_hierarchy_database_transient() {
        let err = SzError::database_transient("Deadlock");
        let hierarchy = err.hierarchy();

        assert_eq!(hierarchy.len(), 2);
        assert_eq!(hierarchy[0], ErrorCategory::DatabaseTransient);
        assert_eq!(hierarchy[1], ErrorCategory::Retryable);
    }

    #[test]
    fn test_hierarchy_not_found() {
        let err = SzError::not_found("Entity 123");
        let hierarchy = err.hierarchy();

        assert_eq!(hierarchy.len(), 2);
        assert_eq!(hierarchy[0], ErrorCategory::NotFound);
        assert_eq!(hierarchy[1], ErrorCategory::BadInput);
    }

    #[test]
    fn test_hierarchy_database() {
        let err = SzError::database("Schema error");
        let hierarchy = err.hierarchy();

        assert_eq!(hierarchy.len(), 2);
        assert_eq!(hierarchy[0], ErrorCategory::Database);
        assert_eq!(hierarchy[1], ErrorCategory::Unrecoverable);
    }

    #[test]
    fn test_hierarchy_license() {
        let err = SzError::license("License expired");
        let hierarchy = err.hierarchy();

        assert_eq!(hierarchy.len(), 2);
        assert_eq!(hierarchy[0], ErrorCategory::License);
        assert_eq!(hierarchy[1], ErrorCategory::Unrecoverable);
    }

    #[test]
    fn test_hierarchy_configuration() {
        let err = SzError::configuration("Invalid config");
        let hierarchy = err.hierarchy();

        assert_eq!(hierarchy.len(), 1);
        assert_eq!(hierarchy[0], ErrorCategory::Configuration);
    }

    #[test]
    fn test_is_method_specific_type() {
        let err = SzError::database_transient("Deadlock");

        // Should match specific type
        assert!(err.is(ErrorCategory::DatabaseTransient));
    }

    #[test]
    fn test_is_method_parent_type() {
        let err = SzError::database_transient("Deadlock");

        // Should match parent type (polymorphic)
        assert!(err.is(ErrorCategory::Retryable));
    }

    #[test]
    fn test_is_method_negative() {
        let err = SzError::database_transient("Deadlock");

        // Should NOT match unrelated types
        assert!(!err.is(ErrorCategory::BadInput));
        assert!(!err.is(ErrorCategory::Unrecoverable));
        assert!(!err.is(ErrorCategory::Configuration));
    }

    #[test]
    fn test_is_method_all_retryable_subtypes() {
        // All Retryable subtypes should match Retryable category
        assert!(SzError::database_connection_lost("Lost").is(ErrorCategory::Retryable));
        assert!(SzError::database_transient("Deadlock").is(ErrorCategory::Retryable));
        assert!(SzError::retry_timeout_exceeded("Timeout").is(ErrorCategory::Retryable));
    }

    #[test]
    fn test_is_method_all_unrecoverable_subtypes() {
        // All Unrecoverable subtypes should match Unrecoverable category
        assert!(SzError::database("DB error").is(ErrorCategory::Unrecoverable));
        assert!(SzError::license("Expired").is(ErrorCategory::Unrecoverable));
        assert!(SzError::not_initialized("Not init").is(ErrorCategory::Unrecoverable));
        assert!(SzError::unhandled("Unhandled").is(ErrorCategory::Unrecoverable));
    }

    #[test]
    fn test_is_method_all_bad_input_subtypes() {
        // All BadInput subtypes should match BadInput category
        assert!(SzError::not_found("Missing").is(ErrorCategory::BadInput));
        assert!(SzError::unknown_data_source("Unknown").is(ErrorCategory::BadInput));
        assert!(SzError::bad_input("Invalid").is(ErrorCategory::BadInput));
    }

    // ========================================================================
    // Generated Error Code Mapping Tests
    // ========================================================================

    #[test]
    fn test_generated_mapping_sample_codes() {
        // Test a sample of error codes from different ranges to verify generated mappings

        // BadInput range
        let err = SzError::from_code(2);
        assert!(matches!(err, SzError::BadInput(_)));

        let err = SzError::from_code(7);
        assert!(matches!(err, SzError::BadInput(_)));

        // RetryTimeoutExceeded (specific code 10)
        let err = SzError::from_code(10);
        assert!(matches!(err, SzError::RetryTimeoutExceeded(_)));

        // Configuration range
        let err = SzError::from_code(14);
        assert!(matches!(err, SzError::Configuration(_)));

        // NotInitialized (specific codes)
        let err = SzError::from_code(48);
        assert!(matches!(err, SzError::NotInitialized(_)));

        // Unhandled (specific code 87)
        let err = SzError::from_code(87);
        assert!(matches!(err, SzError::Unhandled(_)));

        // License (specific code 999)
        let err = SzError::from_code(999);
        assert!(matches!(err, SzError::License(_)));

        // DatabaseConnectionLost (specific codes)
        let err = SzError::from_code(1006);
        assert!(matches!(err, SzError::DatabaseConnectionLost(_)));

        let err = SzError::from_code(1007);
        assert!(matches!(err, SzError::DatabaseConnectionLost(_)));

        // DatabaseTransient (specific code 1008)
        let err = SzError::from_code(1008);
        assert!(matches!(err, SzError::DatabaseTransient(_)));

        // Database (other codes in range)
        let err = SzError::from_code(1010);
        assert!(matches!(err, SzError::Database(_)));

        // Configuration (2000-2300 range)
        let err = SzError::from_code(2001);
        assert!(matches!(err, SzError::Configuration(_)));
    }

    #[test]
    fn test_generated_mapping_with_hierarchy() {
        // Test that generated mappings provide correct hierarchy
        let err = SzError::from_code(1008); // DatabaseTransient

        // Verify error code is preserved
        assert_eq!(err.error_code(), Some(1008));

        // Verify hierarchy is correct
        assert!(err.is(ErrorCategory::DatabaseTransient));
        assert!(err.is(ErrorCategory::Retryable));

        // Verify it's recognized as retryable
        assert!(err.is_retryable());
    }

    #[test]
    fn test_all_specific_error_codes() {
        // Test all the specific error codes that have special handling
        let test_cases = vec![
            (
                10,
                ErrorCategory::RetryTimeoutExceeded,
                ErrorCategory::Retryable,
            ),
            (87, ErrorCategory::Unhandled, ErrorCategory::Unrecoverable),
            (
                48,
                ErrorCategory::NotInitialized,
                ErrorCategory::Unrecoverable,
            ),
            (
                49,
                ErrorCategory::NotInitialized,
                ErrorCategory::Unrecoverable,
            ),
            (
                50,
                ErrorCategory::NotInitialized,
                ErrorCategory::Unrecoverable,
            ),
            (
                53,
                ErrorCategory::NotInitialized,
                ErrorCategory::Unrecoverable,
            ),
            (999, ErrorCategory::License, ErrorCategory::Unrecoverable),
            (
                1006,
                ErrorCategory::DatabaseConnectionLost,
                ErrorCategory::Retryable,
            ),
            (
                1007,
                ErrorCategory::DatabaseConnectionLost,
                ErrorCategory::Retryable,
            ),
            (
                1008,
                ErrorCategory::DatabaseTransient,
                ErrorCategory::Retryable,
            ),
        ];

        for (code, specific, parent) in test_cases {
            let err = SzError::from_code(code);
            assert!(
                err.is(specific),
                "Error code {} should be {:?}",
                code,
                specific
            );
            assert!(
                err.is(parent),
                "Error code {} should also be {:?} (parent)",
                code,
                parent
            );
        }
    }

    #[test]
    fn test_error_code_preservation() {
        // Verify that error codes are preserved through the mapping process
        for code in [2, 10, 48, 87, 999, 1006, 1008, 2001] {
            let err = SzError::from_code(code);
            assert_eq!(
                err.error_code(),
                Some(code),
                "Error code {} should be preserved",
                code
            );
        }
    }
}
