//! Core implementation of SzEnvironment trait

use crate::{
    error::{SzError, SzResult},
    ffi_call, ffi_call_i64,
    traits::*,
    types::*,
};
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// Core implementation of the SzEnvironment trait
///
/// This struct manages the lifecycle of the Senzing environment and serves
/// as a factory for obtaining instances of other SDK components.
pub struct SzEnvironmentCore {
    is_destroyed: Arc<AtomicBool>,
    is_initialized: Arc<AtomicBool>,
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
            is_initialized: Arc::new(AtomicBool::new(false)),
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

    /// Destroys the global singleton instance
    ///
    /// This allows a new instance to be created with different parameters.
    pub fn destroy_global_instance() -> SzResult<()> {
        if let Some(global_env) = GLOBAL_ENVIRONMENT.get() {
            let mut env_guard = match global_env.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    // Recover from poisoned mutex
                    poisoned.into_inner()
                }
            };
            if let Some(env) = env_guard.take() {
                // Ensure complete destruction of all Senzing modules
                if !env.is_destroyed() {
                    // Mark the environment as destroyed
                    env.is_destroyed
                        .store(true, std::sync::atomic::Ordering::Relaxed);

                    // Cleanup environment-tied components only
                    // SzConfig* functions are handled independently in their constructors/destructors
                    unsafe {
                        // These are tied to the environment lifecycle
                        let _ = crate::ffi::bindings::SzDiagnostic_destroy();
                        let _ = crate::ffi::bindings::SzProduct_destroy();
                        // Finally destroy the main Senzing environment
                        let _ = crate::ffi::bindings::Sz_destroy();

                        // Clear exception states for environment-tied components
                        crate::ffi::bindings::Sz_clearLastException();
                        crate::ffi::bindings::SzDiagnostic_clearLastException();
                        crate::ffi::bindings::SzProduct_clearLastException();
                    }

                    // Give the native library time to fully clean up internal state
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
        Ok(())
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
    /// This method is thread-safe and will only call Sz_init once.
    fn ensure_initialized(&self) -> SzResult<()> {
        if self.is_initialized.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Use compare-and-swap to ensure only one thread calls Sz_init
        if self
            .is_initialized
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            let module_name_c = crate::ffi::helpers::str_to_c_string(&self.module_name)?;
            let ini_params_c = crate::ffi::helpers::str_to_c_string(&self.ini_params)?;
            let verbose = if self.verbose_logging { 1 } else { 0 };

            ffi_call!(crate::ffi::bindings::Sz_init(
                module_name_c.as_ptr(),
                ini_params_c.as_ptr(),
                verbose as i64
            ));
        }

        Ok(())
    }
}

impl SzEnvironment for SzEnvironmentCore {
    fn destroy(&mut self) -> SzResult<()> {
        if self.is_destroyed.load(Ordering::Relaxed) {
            return Ok(());
        }

        ffi_call_i64!(crate::ffi::bindings::Sz_destroy());
        self.is_destroyed.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn is_destroyed(&self) -> bool {
        self.is_destroyed.load(Ordering::Relaxed)
    }

    fn reinitialize(&self, config_id: ConfigId) -> SzResult<()> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before reinitializing
        self.ensure_initialized()?;

        ffi_call!(crate::ffi::bindings::Sz_reinit(config_id));
        Ok(())
    }

    fn get_active_config_id(&self) -> SzResult<ConfigId> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Ensure Sz_init has been called before getting active config ID
        self.ensure_initialized()?;

        let config_id = unsafe { crate::ffi::bindings::Sz_getActiveConfigID() };
        Ok(config_id)
    }

    fn get_product(&self) -> SzResult<Box<dyn SzProduct>> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Use the same settings as the environment for consistent initialization
        let product_core = super::product::SzProductCore::new_with_params(
            &self.module_name,
            &self.ini_params,
            self.verbose_logging,
        )?;
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

        // Note: SzConfigMgr does NOT require Sz_init - it initializes independently
        // This allows config setup before engine initialization
        let config_mgr_core = super::config_manager::SzConfigManagerCore::new_with_params(
            &self.module_name,
            &self.ini_params,
            self.verbose_logging,
        )?;
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
