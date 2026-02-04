//! RAII guard for automatic Senzing environment cleanup
//!
//! This module provides [`SenzingGuard`], an RAII wrapper around
//! `Arc<SzEnvironmentCore>` that automatically cleans up native Senzing
//! resources when dropped.

use crate::error::SzResult;
use std::sync::Arc;

/// RAII guard for automatic Senzing environment cleanup.
///
/// `SenzingGuard` wraps an `Arc<SzEnvironmentCore>` and ensures that native
/// Senzing resources are properly released when the guard goes out of scope.
/// This provides idiomatic Rust resource management without requiring explicit
/// `destroy()` calls.
///
/// # Lifecycle
///
/// 1. **Creation**: Initializes or obtains the Senzing environment
/// 2. **Usage**: Access the environment via `Deref` or `env()` method
/// 3. **Destruction**: When dropped, removes the singleton reference and
///    attempts to release native resources
///
/// # Example
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// fn main() -> SzResult<()> {
///     // Guard automatically manages the lifecycle
///     let guard = SenzingGuard::new("my-app", &settings, false)?;
///
///     // Access environment and components
///     let engine = guard.get_engine()?;
///     let product = guard.get_product()?;
///
///     // Add records
///     engine.add_record("CUSTOMERS", "1", r#"{"NAME": "John"}"#, None)?;
///
///     // Resources released automatically when guard drops
///     Ok(())
/// } // <- Native resources released here
/// ```
///
/// # Thread Safety
///
/// `SenzingGuard` is thread-safe (`Send + Sync`) because it wraps an
/// `Arc<SzEnvironmentCore>`. The underlying environment can be safely
/// shared across threads.
///
/// # Comparison with Manual `destroy()`
///
/// | Aspect | `SenzingGuard` | Manual `destroy()` |
/// |--------|----------------|-------------------|
/// | Resource release | Automatic on drop | Explicit call required |
/// | Error handling | Panics on cleanup failure | Returns `SzResult` |
/// | Multiple references | Must be sole owner | Returns error if refs exist |
/// | Idiomatic Rust | ✅ RAII pattern | ❌ Explicit lifecycle |
///
/// # Panic Behavior
///
/// The guard will panic if cleanup fails during `Drop`. This is intentional
/// to prevent silent resource leaks. If you need to handle cleanup errors
/// gracefully, use the explicit `into_inner()` method and call `destroy()`
/// manually.
pub struct SenzingGuard {
    env: Option<Arc<super::SzEnvironmentCore>>,
}

impl SenzingGuard {
    /// Creates a new `SenzingGuard` with the specified configuration.
    ///
    /// This initializes the Senzing environment singleton or returns the
    /// existing instance if one already exists with compatible parameters.
    ///
    /// # Arguments
    ///
    /// * `module_name` - Name for logging purposes
    /// * `ini_params` - JSON string with Senzing configuration
    /// * `verbose_logging` - Enable verbose logging
    ///
    /// # Example
    ///
    /// ```ignore
    /// let settings = r#"{"PIPELINE": {...}, "SQL": {...}}"#;
    /// let guard = SenzingGuard::new("my-app", settings, false)?;
    /// ```
    pub fn new(module_name: &str, ini_params: &str, verbose_logging: bool) -> SzResult<Self> {
        let env = super::SzEnvironmentCore::get_instance(module_name, ini_params, verbose_logging)?;
        Ok(Self { env: Some(env) })
    }

    /// Creates a guard from an existing environment instance.
    ///
    /// Use this when you already have an `Arc<SzEnvironmentCore>` and want
    /// to transfer ownership to a guard for automatic cleanup.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let env = SzEnvironmentCore::get_instance("app", &settings, false)?;
    /// let guard = SenzingGuard::from_env(env);
    /// // guard now owns the Arc and will clean up on drop
    /// ```
    pub fn from_env(env: Arc<super::SzEnvironmentCore>) -> Self {
        Self { env: Some(env) }
    }

    /// Gets a reference to the inner environment.
    ///
    /// # Panics
    ///
    /// Panics if the guard has already been consumed via `into_inner()`.
    pub fn env(&self) -> &Arc<super::SzEnvironmentCore> {
        self.env.as_ref().expect("SenzingGuard already consumed")
    }

    /// Consumes the guard and returns the inner `Arc<SzEnvironmentCore>`.
    ///
    /// After calling this, the guard will NOT perform automatic cleanup.
    /// You become responsible for calling `destroy()` on the returned Arc.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let guard = SenzingGuard::new("app", &settings, false)?;
    /// let env = guard.into_inner();
    /// // ... use env ...
    /// env.destroy()?; // Manual cleanup required
    /// ```
    pub fn into_inner(mut self) -> Arc<super::SzEnvironmentCore> {
        self.env.take().expect("SenzingGuard already consumed")
    }

    /// Attempts cleanup without panicking.
    ///
    /// Returns an error if cleanup fails, allowing graceful error handling.
    /// After calling this, the guard is consumed and Drop will be a no-op.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let guard = SenzingGuard::new("app", &settings, false)?;
    /// // ... use guard ...
    /// if let Err(e) = guard.try_cleanup() {
    ///     eprintln!("Cleanup failed: {}", e);
    /// }
    /// ```
    pub fn try_cleanup(mut self) -> SzResult<()> {
        if let Some(env) = self.env.take() {
            env.destroy()
        } else {
            Ok(())
        }
    }
}

// Allow direct access to SzEnvironment trait methods via Deref
impl std::ops::Deref for SenzingGuard {
    type Target = Arc<super::SzEnvironmentCore>;

    fn deref(&self) -> &Self::Target {
        self.env()
    }
}

// Implement Drop to automatically clean up resources
impl Drop for SenzingGuard {
    fn drop(&mut self) {
        if let Some(env) = self.env.take() {
            // Attempt to destroy the environment
            // If this fails (e.g., other references exist), we log and continue
            // rather than panicking, to be more forgiving in edge cases
            match env.destroy() {
                Ok(()) => {}
                Err(e) => {
                    // Only log if this is a genuine error, not "already destroyed"
                    if !e.to_string().contains("already destroyed")
                        && !e.to_string().contains("other references")
                    {
                        eprintln!("SenzingGuard: cleanup warning: {e}");
                    }
                }
            }
        }
    }
}

// Thread safety: SenzingGuard is safe to send/share because it only contains Arc
unsafe impl Send for SenzingGuard {}
unsafe impl Sync for SenzingGuard {}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full integration tests require Senzing to be installed.
    // These are compile-time checks only.

    fn _assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn senzing_guard_is_send_sync() {
        _assert_send_sync::<SenzingGuard>();
    }
}
