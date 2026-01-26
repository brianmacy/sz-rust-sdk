//! Core implementation of SzEnvironment trait

use crate::{
    error::{SzError, SzResult},
    ffi_call,
    traits::*,
    types::*,
};
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once, OnceLock};

/// Core implementation of the SzEnvironment trait
///
/// This struct manages the lifecycle of the Senzing environment and serves
/// as a factory for obtaining instances of other SDK components.
pub struct SzEnvironmentCore {
    is_destroyed: Arc<AtomicBool>,
    /// Guards Sz_init() - ensures it runs exactly once and other threads wait
    init_once: Arc<Once>,
    /// Stores any error that occurred during Sz_init
    init_error: Arc<Mutex<Option<String>>>,
    /// Guards SzConfigMgr_init() - ensures it runs exactly once and other threads wait
    config_mgr_init_once: Arc<Once>,
    /// Stores any error that occurred during SzConfigMgr_init
    config_mgr_init_error: Arc<Mutex<Option<String>>>,
    /// Guards SzProduct_init() - ensures it runs exactly once and other threads wait
    product_init_once: Arc<Once>,
    /// Stores any error that occurred during SzProduct_init
    product_init_error: Arc<Mutex<Option<String>>>,
    module_name: String,
    ini_params: String,
    verbose_logging: bool,
}

// Singleton storage for the global SzEnvironmentCore instance
// Using ManuallyDrop to prevent static destructor from running at exit,
// which avoids conflicts with Senzing's internal static mutex destruction order
static GLOBAL_ENVIRONMENT: OnceLock<ManuallyDrop<Mutex<Option<Arc<SzEnvironmentCore>>>>> =
    OnceLock::new();

impl SzEnvironmentCore {
    /// Creates a new SzEnvironment instance
    ///
    /// # Arguments
    ///
    /// * `module_name` - Name of the module for logging purposes
    /// * `ini_params` - JSON string containing initialization parameters
    /// * `verbose_logging` - Whether to enable verbose logging
    pub fn new(module_name: &str, ini_params: &str, verbose_logging: bool) -> SzResult<Self> {
        Ok(Self {
            is_destroyed: Arc::new(AtomicBool::new(false)),
            init_once: Arc::new(Once::new()),
            init_error: Arc::new(Mutex::new(None)),
            config_mgr_init_once: Arc::new(Once::new()),
            config_mgr_init_error: Arc::new(Mutex::new(None)),
            product_init_once: Arc::new(Once::new()),
            product_init_error: Arc::new(Mutex::new(None)),
            module_name: module_name.to_string(),
            ini_params: ini_params.to_string(),
            verbose_logging,
        })
    }

    /// Creates a new SzEnvironment instance with default parameters
    pub fn new_default() -> SzResult<Self> {
        Self::new("SzRustSDK", "{}", false)
    }

    /// Gets or creates the global singleton SzEnvironmentCore instance
    ///
    /// This method ensures that only one SzEnvironmentCore instance exists
    /// per process, which is required by the Senzing SDK.
    ///
    /// # Arguments
    ///
    /// * `module_name` - Name of the module for logging purposes
    /// * `ini_params` - JSON string containing initialization parameters
    /// * `verbose_logging` - Whether to enable verbose logging
    pub fn get_instance(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
    ) -> SzResult<Arc<Self>> {
        let global_env = GLOBAL_ENVIRONMENT.get_or_init(|| ManuallyDrop::new(Mutex::new(None)));
        let mut env_guard = global_env.lock().unwrap();

        match env_guard.as_ref() {
            Some(existing_env) => {
                // Check if the existing environment is still valid
                if existing_env.is_destroyed() {
                    let new_env = Arc::new(Self::new(module_name, ini_params, verbose_logging)?);
                    *env_guard = Some(new_env.clone());
                    Ok(new_env)
                } else {
                    // Validate critical parameters match existing instance (ini_params and verbose_logging)
                    // Module name can be different as it's only used for logging
                    if existing_env.ini_params != ini_params
                        || existing_env.verbose_logging != verbose_logging
                    {
                        return Err(SzError::configuration(
                            "Cannot change critical initialization parameters (ini_params, verbose_logging) after SzEnvironmentCore instance is created",
                        ));
                    }
                    // Return the existing valid environment (module name can be different)
                    Ok(existing_env.clone())
                }
            }
            None => {
                // Create the first instance
                let new_env = Arc::new(Self::new(module_name, ini_params, verbose_logging)?);
                *env_guard = Some(new_env.clone());
                Ok(new_env)
            }
        }
    }

    /// Gets the existing global singleton SzEnvironmentCore instance
    ///
    /// This method returns the existing singleton instance without creating a new one.
    /// It will return an error if no instance has been created yet.
    ///
    /// # Returns
    ///
    /// Returns the existing singleton instance or an error if none exists.
    pub fn get_existing_instance() -> SzResult<Arc<Self>> {
        let global_env = GLOBAL_ENVIRONMENT.get_or_init(|| ManuallyDrop::new(Mutex::new(None)));
        let env_guard = global_env.lock().unwrap();

        match env_guard.as_ref() {
            Some(existing_env) => {
                if existing_env.is_destroyed() {
                    Err(SzError::unrecoverable(
                        "SzEnvironmentCore instance has been destroyed",
                    ))
                } else {
                    Ok(existing_env.clone())
                }
            }
            None => Err(SzError::unrecoverable(
                "No SzEnvironmentCore instance has been created yet. Call get_instance() first.",
            )),
        }
    }

    /// Gets the global singleton instance if it exists
    ///
    /// Returns None if no instance has been created yet.
    pub fn try_get_instance() -> Option<Arc<Self>> {
        GLOBAL_ENVIRONMENT
            .get()?
            .lock()
            .unwrap()
            .as_ref()
            .map(|env| env.clone())
    }

    /// Destroys the environment, consuming the Arc and releasing all native resources.
    ///
    /// This method uses Rust's ownership semantics to ensure safe cleanup:
    /// - Only succeeds if the caller holds the sole reference to the environment
    /// - If other references exist (e.g., other threads still using the environment),
    ///   returns an error and the environment remains valid
    ///
    /// # Ownership Requirements
    ///
    /// The Senzing native library is a global resource. Destroying the environment
    /// while other code still holds references would cause undefined behavior.
    /// This method enforces that you can only destroy when you're the sole owner.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sz_rust_sdk::prelude::*;
    ///
    /// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
    /// // ... use env ...
    ///
    /// // When done, destroy (only works if this is the only reference)
    /// env.destroy()?;
    ///
    /// // Can now create a new environment with different settings
    /// let env2 = SzEnvironmentCore::get_instance("my_app", &new_settings, false)?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `SzError::Unrecoverable` if:
    /// - Other references to the environment still exist
    /// - The environment was already destroyed
    pub fn destroy(self: Arc<Self>) -> SzResult<()> {
        // First, remove from global singleton storage
        // This drops the global reference, leaving only the caller's reference
        if let Some(global_env) = GLOBAL_ENVIRONMENT.get() {
            let mut env_guard = match global_env.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            // Only take if it's the same instance
            if let Some(stored) = env_guard.as_ref()
                && Arc::ptr_eq(stored, &self)
            {
                env_guard.take();
            }
        }

        // Now try to get exclusive ownership
        match Arc::try_unwrap(self) {
            Ok(env) => {
                // We have sole ownership - safe to destroy
                if env.is_destroyed.load(Ordering::Relaxed) {
                    return Ok(()); // Already destroyed, nothing to do
                }

                // Mark as destroyed
                env.is_destroyed.store(true, Ordering::Relaxed);

                // Cleanup all Senzing modules
                // Note: SzConfig_destroy() is not needed here - it manages config handles,
                // not the config system itself. Config handles have their own lifecycle.
                unsafe {
                    let _ = crate::ffi::SzDiagnostic_destroy();
                    let _ = crate::ffi::SzProduct_destroy();
                    let _ = crate::ffi::SzConfigMgr_destroy(); // CRITICAL: Clears cached config state
                    let _ = crate::ffi::Sz_destroy();

                    // Clear exception states
                    crate::ffi::Sz_clearLastException();
                    crate::ffi::SzDiagnostic_clearLastException();
                    crate::ffi::SzProduct_clearLastException();
                    crate::ffi::SzConfigMgr_clearLastException();
                }

                // Give the native library time to fully clean up internal state
                std::thread::sleep(std::time::Duration::from_millis(100));

                Ok(())
            }
            Err(arc) => {
                // Other references exist - put it back in global storage and return error
                // We must restore it since we removed it earlier
                if let Some(global_env) = GLOBAL_ENVIRONMENT.get()
                    && let Ok(mut env_guard) = global_env.lock()
                {
                    *env_guard = Some(arc);
                }
                Err(SzError::unrecoverable(
                    "Cannot destroy environment: other references still exist. \
                     Ensure all Arc<SzEnvironmentCore> clones are dropped before calling destroy().",
                ))
            }
        }
    }

    /// Get the initialization parameters used by this environment
    pub fn get_ini_params(&self) -> &str {
        &self.ini_params
    }

    /// Get the verbose logging setting used by this environment
    pub fn get_verbose_logging(&self) -> bool {
        self.verbose_logging
    }

    /// Ensures Sz_init has been called - should be called before any engine operations
    ///
    /// This method is thread-safe: the first thread to call this will run Sz_init(),
    /// and all other threads will block until initialization is complete.
    fn ensure_initialized(&self) -> SzResult<()> {
        // Clone Arcs for use in closure (can't capture &self in call_once)
        let module_name = self.module_name.clone();
        let ini_params = self.ini_params.clone();
        let verbose_logging = self.verbose_logging;
        let init_error = Arc::clone(&self.init_error);

        // call_once blocks all threads until the closure completes
        self.init_once.call_once(|| {
            let result = (|| -> SzResult<()> {
                let module_name_c = crate::ffi::helpers::str_to_c_string(&module_name)?;
                let ini_params_c = crate::ffi::helpers::str_to_c_string(&ini_params)?;
                let verbose = if verbose_logging { 1 } else { 0 };

                ffi_call!(crate::ffi::Sz_init(
                    module_name_c.as_ptr(),
                    ini_params_c.as_ptr(),
                    verbose as i64
                ));
                Ok(())
            })();

            // Store any error for other threads to see
            if let Err(e) = result
                && let Ok(mut guard) = init_error.lock()
            {
                *guard = Some(e.to_string());
            }
        });

        // Check if initialization failed
        if let Ok(guard) = self.init_error.lock()
            && let Some(err_msg) = guard.as_ref()
        {
            return Err(SzError::unrecoverable(format!(
                "Sz_init failed: {}",
                err_msg
            )));
        }

        Ok(())
    }

    /// Ensures SzConfigMgr_init has been called - should be called before any config manager operations
    ///
    /// This method is thread-safe: the first thread to call this will run SzConfigMgr_init(),
    /// and all other threads will block until initialization is complete.
    fn ensure_config_mgr_initialized(&self) -> SzResult<()> {
        // Clone Arcs for use in closure (can't capture &self in call_once)
        let module_name = self.module_name.clone();
        let ini_params = self.ini_params.clone();
        let verbose_logging = self.verbose_logging;
        let init_error = Arc::clone(&self.config_mgr_init_error);

        // call_once blocks all threads until the closure completes
        self.config_mgr_init_once.call_once(|| {
            let result = (|| -> SzResult<()> {
                let module_name_c = crate::ffi::helpers::str_to_c_string(&module_name)?;
                let ini_params_c = crate::ffi::helpers::str_to_c_string(&ini_params)?;
                let verbose = if verbose_logging { 1 } else { 0 };

                // Call the FFI directly and check with the proper config_mgr error handler
                let return_code = unsafe {
                    crate::ffi::SzConfigMgr_init(
                        module_name_c.as_ptr(),
                        ini_params_c.as_ptr(),
                        verbose,
                    )
                };
                crate::ffi::helpers::check_config_mgr_return_code(return_code)?;
                Ok(())
            })();

            // Store any error for other threads to see
            if let Err(e) = result
                && let Ok(mut guard) = init_error.lock()
            {
                *guard = Some(e.to_string());
            }
        });

        // Check if initialization failed
        if let Ok(guard) = self.config_mgr_init_error.lock()
            && let Some(err_msg) = guard.as_ref()
        {
            return Err(SzError::unrecoverable(format!(
                "SzConfigMgr_init failed: {}",
                err_msg
            )));
        }

        Ok(())
    }

    /// Ensures SzProduct_init has been called - should be called before any product operations
    ///
    /// This method is thread-safe: the first thread to call this will run SzProduct_init(),
    /// and all other threads will block until initialization is complete.
    fn ensure_product_initialized(&self) -> SzResult<()> {
        // Clone Arcs for use in closure (can't capture &self in call_once)
        let module_name = self.module_name.clone();
        let ini_params = self.ini_params.clone();
        let verbose_logging = self.verbose_logging;
        let init_error = Arc::clone(&self.product_init_error);

        // call_once blocks all threads until the closure completes
        self.product_init_once.call_once(|| {
            let result = (|| -> SzResult<()> {
                let module_name_c = crate::ffi::helpers::str_to_c_string(&module_name)?;
                let ini_params_c = crate::ffi::helpers::str_to_c_string(&ini_params)?;
                let verbose = if verbose_logging { 1 } else { 0 };

                // Call the FFI directly and check with the proper product error handler
                let return_code = unsafe {
                    crate::ffi::SzProduct_init(
                        module_name_c.as_ptr(),
                        ini_params_c.as_ptr(),
                        verbose,
                    )
                };
                crate::ffi::helpers::check_product_return_code(return_code)?;
                Ok(())
            })();

            // Store any error for other threads to see
            if let Err(e) = result
                && let Ok(mut guard) = init_error.lock()
            {
                *guard = Some(e.to_string());
            }
        });

        // Check if initialization failed
        if let Ok(guard) = self.product_init_error.lock()
            && let Some(err_msg) = guard.as_ref()
        {
            return Err(SzError::unrecoverable(format!(
                "SzProduct_init failed: {}",
                err_msg
            )));
        }

        Ok(())
    }
}

impl SzEnvironment for SzEnvironmentCore {
    fn is_destroyed(&self) -> bool {
        self.is_destroyed.load(Ordering::Relaxed)
    }

    fn reinitialize(&self, config_id: ConfigId) -> SzResult<()> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before reinitializing
        self.ensure_initialized()?;

        ffi_call!(crate::ffi::Sz_reinit(config_id));
        Ok(())
    }

    fn get_active_config_id(&self) -> SzResult<ConfigId> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before getting active config ID
        self.ensure_initialized()?;

        let mut config_id: i64 = 0;
        let return_code = unsafe { crate::ffi::Sz_getActiveConfigID(&mut config_id) };
        crate::ffi::helpers::check_return_code(return_code)?;
        Ok(config_id)
    }

    fn get_product(&self) -> SzResult<Box<dyn SzProduct>> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure SzProduct_init has been called (thread-safe, all threads wait for completion)
        self.ensure_product_initialized()?;

        // Create product instance (init already done, so this is safe)
        let product_core = super::product::SzProductCore::new()?;
        Ok(Box::new(product_core))
    }

    fn get_engine(&self) -> SzResult<Box<dyn SzEngine>> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before creating engine
        self.ensure_initialized()?;

        let engine_core = super::engine::SzEngineCore::new()?;
        Ok(Box::new(engine_core))
    }

    fn get_config_manager(&self) -> SzResult<Box<dyn SzConfigManager>> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure SzConfigMgr_init has been called (thread-safe, all threads wait for completion)
        // Note: SzConfigMgr does NOT require Sz_init - it initializes independently
        // This allows config setup before engine initialization
        self.ensure_config_mgr_initialized()?;

        // Create config manager instance (init already done, so this is safe)
        let config_mgr_core = super::config_manager::SzConfigManagerCore::new()?;
        Ok(Box::new(config_mgr_core))
    }

    fn get_diagnostic(&self) -> SzResult<Box<dyn SzDiagnostic>> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before creating diagnostic
        self.ensure_initialized()?;

        let diagnostic_core = super::diagnostic::SzDiagnosticCore::new_with_params(
            &self.module_name,
            &self.ini_params,
            self.verbose_logging,
        )?;
        Ok(Box::new(diagnostic_core))
    }
}

impl Drop for SzEnvironmentCore {
    fn drop(&mut self) {
        // Disable automatic cleanup for now to avoid segfaults
        // The user should call destroy() explicitly if needed
        // TODO: Investigate proper cleanup sequence with Senzing library
        if !self.is_destroyed() {
            // Mark as destroyed without calling Sz_destroy()
            self.is_destroyed
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
