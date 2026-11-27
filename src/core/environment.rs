//! Core implementation of SzEnvironment trait

use crate::{
    error::{SzComponent, SzError, SzResult},
    ffi::helpers::{
        check_config_mgr_return_code, check_diagnostic_return_code, check_product_return_code,
        check_return_code, str_to_c_string,
    },
    ffi_call, ffi_call_diagnostic,
    traits::*,
    types::*,
};
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Once, OnceLock, PoisonError, RwLock};

/// Locks a mutex, recovering the data if a panicking thread poisoned it.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A native init failure, kept so every later caller gets the same error.
struct InitFailure {
    message: String,
    code: Option<i64>,
    component: Option<SzComponent>,
}

impl InitFailure {
    fn from_error(error: &SzError) -> Self {
        Self {
            message: error.message().to_string(),
            code: error.error_code(),
            component: error.component(),
        }
    }

    /// Rebuilds the error with its native code, component and category intact.
    fn to_error(&self, init_name: &str) -> SzError {
        let message = format!("{init_name} failed: {}", self.message);
        match (self.code, self.component) {
            (Some(code), Some(component)) => SzError::from_native_code(message, code, component),
            _ => SzError::unrecoverable(message),
        }
    }
}

/// Runs one native subsystem's `*_init` exactly once.
///
/// The first caller runs the init; other threads block until it finishes. A
/// failure is stored and returned to every caller, keeping its native code and
/// error category.
struct InitGuard {
    /// Native function name, used in the replayed error message
    name: &'static str,
    once: Once,
    failure: Mutex<Option<InitFailure>>,
}

impl InitGuard {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            once: Once::new(),
            failure: Mutex::new(None),
        }
    }

    fn run(&self, init: impl FnOnce() -> SzResult<()>) -> SzResult<()> {
        self.once.call_once(|| {
            if let Err(error) = init() {
                *lock(&self.failure) = Some(InitFailure::from_error(&error));
            }
        });
        match lock(&self.failure).as_ref() {
            Some(failure) => Err(failure.to_error(self.name)),
            None => Ok(()),
        }
    }

    /// True once the init has run and succeeded.
    fn is_initialized(&self) -> bool {
        self.once.is_completed() && lock(&self.failure).is_none()
    }
}

/// Core implementation of the SzEnvironment trait.
///
/// `SzEnvironmentCore` manages the lifecycle of the Senzing environment and serves
/// as a factory for obtaining instances of other SDK components. It implements a
/// singleton pattern required by the Senzing native library.
///
/// # Complete Lifecycle Example
///
/// This example shows the full lifecycle: creation, usage, and cleanup.
///
/// ```no_run
/// # use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
/// use std::sync::Arc;
///
/// # let env = ExampleEnvironment::initialize("doctest_lifecycle")?;
/// // Get SDK components
/// let engine = env.get_engine()?;
/// let product = env.get_product()?;
///
/// // Use the SDK
/// let version = product.get_version()?;
/// println!("Senzing version: {}", version);
///
/// engine.add_record("TEST", "LIFECYCLE_1",
///     r#"{"NAME_FULL": "John Smith"}"#, None)?;
///
/// // Drop components first (they borrow the environment)
/// drop(engine);
/// drop(product);
///
/// // Destroy releases native resources
/// env.destroy()?;
/// # Ok::<(), SzError>(())
/// ```
///
/// # Singleton Pattern
///
/// The Senzing native library requires a single global instance per process.
/// `get_instance()` enforces this by storing one `Arc` reference in a static
/// variable. This means:
///
/// - All `get_instance()` calls with the same parameters return the same instance
/// - The returned `Arc` always has `strong_count() >= 2` (singleton + caller)
/// - Calling `destroy()` removes the singleton reference and cleans up native
///   resources only when you hold the last reference
///
/// # Thread Safety
///
/// `SzEnvironmentCore` is thread-safe (`Send + Sync`). The `Arc<SzEnvironmentCore>`
/// can be cloned and shared across threads. Each call to `get_engine()`, etc.,
/// returns an independent component instance that can be used in its own thread.
///
/// ```
/// # use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
///
/// # let env = ExampleEnvironment::initialize("doctest_thread_safety")?;
/// // Distinct people per thread: concurrent adds that resolve to the SAME entity
/// // can fail with SENZ1011 on the `internal://` in-memory database (native race;
/// // file-backed databases are unaffected).
/// let names = ["Robert Smith", "Maria Garcia", "Wei Chen", "Aisha Khan"];
/// let handles: Vec<_> = names.into_iter().enumerate().map(|(i, name)| {
///     let env = env.clone();
///     std::thread::spawn(move || {
///         let engine = env.get_engine().unwrap();
///         engine.add_record("TEST", &format!("THREAD_{i}"),
///             &format!(r#"{{"NAME_FULL": "{name}"}}"#), None)
///     })
/// }).collect();
///
/// for h in handles { h.join().unwrap()?; }
/// # drop(env);
/// # Ok::<(), SzError>(())
/// ```
pub struct SzEnvironmentCore {
    is_destroyed: Arc<AtomicBool>,
    engine_init: InitGuard,
    config_mgr_init: InitGuard,
    product_init: InitGuard,
    diagnostic_init: InitGuard,
    /// Configuration the engine and diagnostic initialize with: the builder's
    /// `config_id`, then the last successful `reinitialize`. `None` means the
    /// datastore's default configuration. Engine/diagnostic inits hold the read
    /// lock across the native call; `reinitialize` holds the write lock, so an
    /// init never starts with a configuration that is being replaced.
    config_id: RwLock<Option<ConfigId>>,
    module_name: String,
    ini_params: String,
    verbose_logging: bool,
}

/// Builder for the [`SzEnvironmentCore`] singleton.
///
/// Mirrors `SzCoreEnvironment.newBuilder()` in the Java and C# SDKs. Use it to
/// start the engine and diagnostic with an explicit registered configuration
/// instead of the datastore's default one; otherwise
/// [`SzEnvironmentCore::get_instance`] is equivalent.
///
/// ```
/// # use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
///
/// # let env = ExampleEnvironment::initialize("doctest_builder")?;
/// # let settings = env.get_ini_params().to_string();
/// # let config_id = env.get_config_manager()?.get_default_config_id()?;
/// # env.reinitialize(config_id)?; // the shared doctest singleton now pins config_id
/// # drop(env);
/// let env = SzEnvironmentCore::builder()
///     .module_name("my-app")
///     .settings(&settings)
///     .config_id(config_id)
///     .build()?;
/// assert_eq!(env.get_active_config_id()?, config_id);
/// # Ok::<(), SzError>(())
/// ```
#[derive(Debug, Clone)]
pub struct SzEnvironmentCoreBuilder {
    module_name: String,
    settings: String,
    verbose_logging: bool,
    config_id: Option<ConfigId>,
}

impl Default for SzEnvironmentCoreBuilder {
    fn default() -> Self {
        Self {
            module_name: "SzRustSDK".to_string(),
            settings: "{}".to_string(),
            verbose_logging: false,
            config_id: None,
        }
    }
}

impl SzEnvironmentCoreBuilder {
    /// Sets the module (instance) name used in native logs.
    pub fn module_name(mut self, module_name: impl Into<String>) -> Self {
        self.module_name = module_name.into();
        self
    }

    /// Sets the engine settings JSON (the `ini_params` of [`SzEnvironmentCore::get_instance`]).
    pub fn settings(mut self, settings: impl Into<String>) -> Self {
        self.settings = settings.into();
        self
    }

    /// Enables or disables verbose native logging.
    pub fn verbose_logging(mut self, verbose_logging: bool) -> Self {
        self.verbose_logging = verbose_logging;
        self
    }

    /// Starts the engine and diagnostic with this registered configuration
    /// (`Sz_initWithConfigID` / `SzDiagnostic_initWithConfigID`) instead of the
    /// datastore's default. Accepts a [`ConfigId`] or an `Option<ConfigId>`;
    /// `None` (the default) uses the datastore's default configuration.
    pub fn config_id(mut self, config_id: impl Into<Option<ConfigId>>) -> Self {
        self.config_id = config_id.into();
        self
    }

    /// Returns the singleton environment, creating it if none is active.
    ///
    /// Native subsystems still initialize lazily, on first use, so an invalid
    /// configuration ID is reported by the first `get_engine()` /
    /// `get_diagnostic()` call, not here.
    ///
    /// # Errors
    ///
    /// Returns `SzError::Configuration` if an environment is already active
    /// with different settings or verbose logging, or with a different
    /// configuration ID than one set here. To switch the configuration of a
    /// running environment, use [`SzEnvironment::reinitialize`].
    pub fn build(self) -> SzResult<Arc<SzEnvironmentCore>> {
        SzEnvironmentCore::get_or_create(
            &self.module_name,
            &self.settings,
            self.verbose_logging,
            self.config_id,
        )
    }
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
        Ok(Self::with_config_id(
            module_name,
            ini_params,
            verbose_logging,
            None,
        ))
    }

    fn with_config_id(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
        config_id: Option<ConfigId>,
    ) -> Self {
        Self {
            is_destroyed: Arc::new(AtomicBool::new(false)),
            engine_init: InitGuard::new("Sz_init"),
            config_mgr_init: InitGuard::new("SzConfigMgr_init"),
            product_init: InitGuard::new("SzProduct_init"),
            diagnostic_init: InitGuard::new("SzDiagnostic_init"),
            config_id: RwLock::new(config_id),
            module_name: module_name.to_string(),
            ini_params: ini_params.to_string(),
            verbose_logging,
        }
    }

    /// Returns a builder for the singleton environment.
    ///
    /// The builder can set an explicit configuration ID, which
    /// [`get_instance`](Self::get_instance) cannot. See [`SzEnvironmentCoreBuilder`].
    pub fn builder() -> SzEnvironmentCoreBuilder {
        SzEnvironmentCoreBuilder::default()
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
    ///
    /// # Arc Reference Count Behavior
    ///
    /// **Important**: The returned `Arc<SzEnvironmentCore>` will always have
    /// `strong_count() >= 2` due to the singleton pattern:
    ///
    /// - **Reference 1**: Stored in `GLOBAL_ENVIRONMENT` (the singleton storage)
    /// - **Reference 2+**: Returned to the caller (and any clones they make)
    ///
    /// This is by design and is NOT a memory leak. The singleton reference ensures
    /// that subsequent calls to `get_instance()` return the same instance.
    ///
    /// ```
    /// # use sz_rust_sdk::helpers::ExampleEnvironment;
    /// use sz_rust_sdk::prelude::*;
    /// use std::sync::Arc;
    ///
    /// # let env = ExampleEnvironment::initialize("doctest_get_instance_arc")?;
    /// assert!(Arc::strong_count(&env) >= 2); // Expected: singleton + caller
    ///
    /// let env2 = SzEnvironmentCore::get_existing_instance()?;
    /// assert!(Arc::ptr_eq(&env, &env2)); // Same instance
    /// assert!(Arc::strong_count(&env) >= 3); // singleton + 2 callers
    /// # drop(env2);
    /// # drop(env);
    /// # Ok::<(), SzError>(())
    /// ```
    ///
    /// # Cleanup
    ///
    /// To properly release resources, use `destroy()` which removes the singleton
    /// reference and calls native cleanup when you hold the last reference:
    ///
    /// ```no_run
    /// # use sz_rust_sdk::helpers::ExampleEnvironment;
    /// use sz_rust_sdk::prelude::*;
    ///
    /// # let env = ExampleEnvironment::initialize("doctest_get_instance_cleanup")?;
    /// let env2 = SzEnvironmentCore::get_existing_instance()?;
    /// // Drop all other references first
    /// drop(env2);
    /// // Then destroy - this removes singleton ref and cleans up native resources
    /// env.destroy()?;
    /// # Ok::<(), SzError>(())
    /// ```
    pub fn get_instance(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
    ) -> SzResult<Arc<Self>> {
        Self::get_or_create(module_name, ini_params, verbose_logging, None)
    }

    /// Returns the active singleton, or creates it. A `config_id` of `None`
    /// accepts whatever configuration the active environment uses.
    fn get_or_create(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
        config_id: Option<ConfigId>,
    ) -> SzResult<Arc<Self>> {
        let global_env = GLOBAL_ENVIRONMENT.get_or_init(|| ManuallyDrop::new(Mutex::new(None)));
        let mut env_guard = lock(global_env);

        if let Some(existing_env) = env_guard.as_ref()
            && !existing_env.is_destroyed()
        {
            // Validate critical parameters match existing instance (ini_params and verbose_logging)
            // Module name can be different as it's only used for logging
            if existing_env.ini_params != ini_params
                || existing_env.verbose_logging != verbose_logging
            {
                return Err(SzError::configuration(
                    "Cannot change critical initialization parameters (ini_params, verbose_logging) after SzEnvironmentCore instance is created",
                ));
            }
            if let Some(requested) = config_id
                && existing_env.get_config_id() != Some(requested)
            {
                return Err(SzError::configuration(format!(
                    "Cannot start with configuration ID {requested}: the active SzEnvironmentCore \
                     uses {:?} (None = datastore default). Use reinitialize() to switch configurations.",
                    existing_env.get_config_id()
                )));
            }
            return Ok(existing_env.clone());
        }

        // No active instance (none yet, or the stored one was destroyed)
        let new_env = Arc::new(Self::with_config_id(
            module_name,
            ini_params,
            verbose_logging,
            config_id,
        ));
        *env_guard = Some(new_env.clone());
        Ok(new_env)
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
    /// ```no_run
    /// # use sz_rust_sdk::helpers::ExampleEnvironment;
    /// use sz_rust_sdk::prelude::*;
    ///
    /// # let env = ExampleEnvironment::initialize("doctest_destroy_basic")?;
    /// // ... use env ...
    ///
    /// // When done, destroy (only works if this is the only reference)
    /// env.destroy()?;
    /// # Ok::<(), SzError>(())
    /// ```
    ///
    /// # Calling `destroy()` from a `Drop` Implementation
    ///
    /// Because `destroy()` takes `Arc<Self>` by value, calling it from a `Drop`
    /// impl requires moving the Arc out of your struct. **Use `Option<Arc<...>>`
    /// with `.take()`** — this is exactly what [`SenzingGuard`](super::SenzingGuard) does internally.
    ///
    /// ```no_run
    /// use sz_rust_sdk::prelude::*;
    /// use std::sync::Arc;
    ///
    /// // CORRECT: Use Option<Arc<...>> + take()
    /// struct MyWrapper {
    ///     env: Option<Arc<SzEnvironmentCore>>,
    /// }
    ///
    /// impl Drop for MyWrapper {
    ///     fn drop(&mut self) {
    ///         if let Some(env) = self.env.take() {
    ///             let _ = env.destroy();
    ///         }
    ///     }
    /// }
    ///
    /// # use sz_rust_sdk::helpers::ExampleEnvironment;
    /// # let env = ExampleEnvironment::initialize("doctest_destroy_drop_correct")?;
    /// let wrapper = MyWrapper { env: Some(env) };
    /// drop(wrapper); // Cleanup happens via Drop
    /// # Ok::<(), SzError>(())
    /// ```
    ///
    /// ```no_run
    /// use sz_rust_sdk::prelude::*;
    /// use std::sync::Arc;
    ///
    /// // WRONG: ptr::read causes double-free / heap corruption
    /// struct MyWrapper {
    ///     env: Arc<SzEnvironmentCore>,  // Not wrapped in Option!
    /// }
    ///
    /// impl Drop for MyWrapper {
    ///     fn drop(&mut self) {
    ///         // BUG: ptr::read creates a second Arc without incrementing the
    ///         // reference count. After destroy() frees the inner data, the
    ///         // compiler also drops self.env -> double-free -> heap corruption.
    ///         let env = unsafe { std::ptr::read(&self.env) };
    ///         let _ = env.destroy();
    ///         // <- compiler drops self.env here (use-after-free!)
    ///     }
    /// }
    /// ```
    ///
    /// **Note:** Using `ManuallyDrop` instead of `ptr::read` avoids the
    /// double-free but is unnecessarily complex. Prefer `Option::take()` or
    /// use [`SenzingGuard`](super::SenzingGuard) directly — it handles all of this correctly.
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

    /// Returns the configuration ID the engine and diagnostic initialize with.
    ///
    /// This is the ID given to [`SzEnvironmentCoreBuilder::config_id`], replaced
    /// by each successful [`reinitialize`](SzEnvironment::reinitialize). `None`
    /// means the datastore's default configuration. Mirrors `configID` in the
    /// Swift SDK and the Java/C# environment's config ID.
    pub fn get_config_id(&self) -> Option<ConfigId> {
        *self
            .config_id
            .read()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Converts the module name and settings for a native `*_init` call.
    ///
    /// Done outside the init guards, so an invalid string (interior NUL) is
    /// reported to every caller as the same `StringConversion` error.
    fn native_init_params(&self) -> SzResult<(CString, CString, i64)> {
        Ok((
            str_to_c_string(&self.module_name)?,
            str_to_c_string(&self.ini_params)?,
            i64::from(self.verbose_logging),
        ))
    }

    /// Ensures the engine is initialized: `Sz_initWithConfigID` when a
    /// configuration ID is set, otherwise `Sz_init`. Runs once; other threads
    /// block until it finishes.
    fn ensure_initialized(&self) -> SzResult<()> {
        let (module, ini, verbose) = self.native_init_params()?;
        self.engine_init.run(|| {
            let config_id = self
                .config_id
                .read()
                .unwrap_or_else(PoisonError::into_inner);
            let return_code = unsafe {
                match *config_id {
                    Some(id) => {
                        crate::ffi::Sz_initWithConfigID(module.as_ptr(), ini.as_ptr(), id, verbose)
                    }
                    None => crate::ffi::Sz_init(module.as_ptr(), ini.as_ptr(), verbose),
                }
            };
            check_return_code(return_code)
        })
    }

    /// Ensures the diagnostic is initialized: `SzDiagnostic_initWithConfigID`
    /// when a configuration ID is set, otherwise `SzDiagnostic_init`. Runs once;
    /// other threads block until it finishes.
    fn ensure_diagnostic_initialized(&self) -> SzResult<()> {
        let (module, ini, verbose) = self.native_init_params()?;
        self.diagnostic_init.run(|| {
            let config_id = self
                .config_id
                .read()
                .unwrap_or_else(PoisonError::into_inner);
            let return_code = unsafe {
                match *config_id {
                    Some(id) => crate::ffi::SzDiagnostic_initWithConfigID(
                        module.as_ptr(),
                        ini.as_ptr(),
                        id,
                        verbose,
                    ),
                    None => crate::ffi::SzDiagnostic_init(module.as_ptr(), ini.as_ptr(), verbose),
                }
            };
            check_diagnostic_return_code(return_code)
        })
    }

    /// Ensures `SzConfigMgr_init` has run. It does not need the engine, so a
    /// configuration can be registered before the engine starts.
    fn ensure_config_mgr_initialized(&self) -> SzResult<()> {
        let (module, ini, verbose) = self.native_init_params()?;
        self.config_mgr_init.run(|| {
            let return_code =
                unsafe { crate::ffi::SzConfigMgr_init(module.as_ptr(), ini.as_ptr(), verbose) };
            check_config_mgr_return_code(return_code)
        })
    }

    /// Ensures `SzProduct_init` has run.
    fn ensure_product_initialized(&self) -> SzResult<()> {
        let (module, ini, verbose) = self.native_init_params()?;
        self.product_init.run(|| {
            let return_code =
                unsafe { crate::ffi::SzProduct_init(module.as_ptr(), ini.as_ptr(), verbose) };
            check_product_return_code(return_code)
        })
    }
}

impl SzEnvironment for SzEnvironmentCore {
    fn is_destroyed(&self) -> bool {
        self.is_destroyed.load(Ordering::Relaxed)
    }

    fn get_logging(&self) -> SzResult<Box<dyn crate::traits::SzLogging>> {
        // No destroyed check: logging is process-global native state that needs no engine
        // initialization, and the 4.5.0 usage rule changes it after destroy.
        Ok(Box::new(crate::core::logging::SzLoggingCore))
    }

    fn reinitialize(&self, config_id: ConfigId) -> SzResult<()> {
        if self.is_destroyed() {
            return Err(SzError::unrecoverable("Environment has been destroyed"));
        }

        // Start the engine first if needed (as Java/C# do), so the new ID is
        // always validated by Sz_reinit.
        self.ensure_initialized()?;

        // Hold the write lock so no engine/diagnostic init runs with the old ID
        // while it is being replaced.
        let mut active = self
            .config_id
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        ffi_call!(crate::ffi::Sz_reinit(config_id));
        if self.diagnostic_init.is_initialized() {
            ffi_call_diagnostic!(crate::ffi::SzDiagnostic_reinit(config_id));
        }
        // Recorded only on success; a diagnostic started later inits with it.
        *active = Some(config_id);
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

        // SzDiagnostic_init / SzDiagnostic_initWithConfigID, once. Like Java/C#,
        // the diagnostic initializes on its own; it does not start the engine.
        self.ensure_diagnostic_initialized()?;

        Ok(Box::new(super::diagnostic::SzDiagnosticCore::new()?))
    }

    fn export_datastore_snapshot(&self, path: &std::path::Path) -> SzResult<()> {
        super::snapshot::export_snapshot(self, path)
    }

    fn import_datastore_snapshot(&self, path: &std::path::Path) -> SzResult<()> {
        super::snapshot::import_snapshot(self, path)
    }
}

/// # Drop Behavior - Intentionally Does NOT Clean Up Native Resources
///
/// The `Drop` implementation for `SzEnvironmentCore` is **intentionally a no-op**
/// that only marks the environment as destroyed without releasing native Senzing
/// resources. This design is deliberate for several important reasons:
///
/// ## Why Drop Doesn't Clean Up
///
/// 1. **Singleton Pattern Complexity**: Due to the global singleton pattern,
///    multiple `Arc` references exist (one in `GLOBAL_ENVIRONMENT`, one or more
///    held by callers). The `Drop` trait cannot detect if this is the "last"
///    reference being dropped.
///
/// 2. **Native Library Safety**: The Senzing native library has specific
///    destruction ordering requirements. Calling `Sz_destroy()` while other
///    threads might still be using the library can cause undefined behavior.
///
/// 3. **Controlled Shutdown**: Users need explicit control over when native
///    resources are released, especially in applications that may reinitialize
///    with different configurations.
///
/// ## Proper Cleanup Pattern
///
/// **Recommended:** Use [`SenzingGuard`](super::SenzingGuard) for automatic RAII cleanup:
///
/// ```no_run
/// # use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
///
/// // Explicit destroy (ownership-based)
/// # let env = ExampleEnvironment::initialize("doctest_drop_cleanup")?;
/// let engine = env.get_engine()?;
/// // ... use engine ...
/// drop(engine);
/// env.destroy()?;  // Explicitly release native resources
/// # Ok::<(), SzError>(())
/// ```
///
/// If you need to call `destroy()` from your own `Drop` implementation,
/// see the safety notes on [`destroy()`](Self::destroy) — you **must** store
/// the Arc in an `Option` and use `.take()` to avoid double-free bugs.
///
/// ## What This Implementation Does
///
/// - Marks the environment as "destroyed" to prevent further API calls
/// - Does **NOT** call `Sz_destroy()` or other native cleanup functions
/// - Allows garbage collection of Rust-side resources
impl Drop for SzEnvironmentCore {
    fn drop(&mut self) {
        // Mark as destroyed to prevent further use through any remaining references.
        // We intentionally do NOT call Sz_destroy() here because:
        // 1. We can't detect if this is the last Arc reference (singleton has one too)
        // 2. Calling native cleanup during Drop can race with other threads
        // 3. Users should use destroy() for explicit cleanup or SenzingGuard for RAII
        if !self.is_destroyed() {
            self.is_destroyed
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod logging_tests {
    use super::*;

    #[test]
    fn test_get_logging_needs_no_native_init() {
        let env = SzEnvironmentCore::new("logging-unit", "{}", false).unwrap();
        assert!(env.get_logging().is_ok());
        assert!(
            !env.engine_init.once.is_completed(),
            "get_logging must not init"
        );
    }

    #[test]
    fn test_get_logging_on_destroyed_environment() {
        let env = SzEnvironmentCore::new("logging-unit", "{}", false).unwrap();
        env.is_destroyed.store(true, Ordering::Relaxed);
        // Unlike the component getters: the 4.5.0 usage rule changes logging after destroy.
        assert!(env.get_product().is_err(), "component getters refuse");
        assert!(env.get_logging().is_ok(), "get_logging works after destroy");
    }
}

#[cfg(test)]
mod init_guard_tests {
    use super::*;

    #[test]
    fn test_init_guard_replays_native_code_and_category() {
        let guard = InitGuard::new("Sz_init");
        let native = || {
            Err(SzError::from_native_code(
                "no config",
                7221,
                SzComponent::Engine,
            ))
        };
        for attempt in 0..2 {
            let error = guard.run(native).expect_err("failure must be replayed");
            assert!(
                matches!(error, SzError::Configuration(_)),
                "attempt {attempt}: {error}"
            );
            assert_eq!(error.error_code(), Some(7221));
            assert_eq!(error.component(), Some(SzComponent::Engine));
            assert_eq!(error.message(), "Sz_init failed: no config");
        }
        assert!(!guard.is_initialized());
    }

    #[test]
    fn test_init_guard_replays_codeless_error_as_unrecoverable() {
        let guard = InitGuard::new("SzProduct_init");
        let error = guard
            .run(|| Err(SzError::unknown("no native code")))
            .expect_err("failure must be returned");
        assert!(matches!(error, SzError::Unrecoverable(_)), "{error}");
        assert_eq!(error.message(), "SzProduct_init failed: no native code");
    }

    #[test]
    fn test_init_guard_runs_init_once() {
        let guard = InitGuard::new("SzDiagnostic_init");
        assert!(!guard.is_initialized());
        assert!(guard.run(|| Ok(())).is_ok());
        assert!(guard.run(|| panic!("init must not run twice")).is_ok());
        assert!(guard.is_initialized());
    }
}
