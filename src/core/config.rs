//! Core implementation of SzConfig trait

use crate::{error::SzResult, ffi_call_config, traits::SzConfig, types::JsonString};
use libc;

/// Core implementation of the SzConfig trait
pub struct SzConfigCore {
    handle: *mut libc::c_char,
}

// SAFETY: SzConfigCore is safe to send between threads as the handle is managed by Senzing
unsafe impl Send for SzConfigCore {}
unsafe impl Sync for SzConfigCore {}

impl SzConfigCore {
    pub fn new_with_params(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
    ) -> SzResult<Self> {
        // Initialize the config module with parameters
        let module_name_c = crate::ffi::helpers::str_to_c_string(module_name)?;
        let ini_params_c = crate::ffi::helpers::str_to_c_string(ini_params)?;
        let verbose = if verbose_logging { 1 } else { 0 };

        ffi_call_config!(crate::ffi::bindings::SzConfig_init(
            module_name_c.as_ptr(),
            ini_params_c.as_ptr(),
            verbose
        ));

        let result = unsafe { crate::ffi::bindings::SzConfig_create_helper() };
        if result.return_code != 0 {
            crate::ffi::helpers::check_config_return_code(result.return_code)?;
        }
        let handle = result.response;

        Ok(Self { handle })
    }

    pub fn new_with_definition(config_definition: &str) -> SzResult<Self> {
        // Get environment parameters for proper initialization
        match super::environment::SzEnvironmentCore::get_existing_instance() {
            Ok(existing_env) => {
                // Initialize the config module with parameters
                let module_name_c = crate::ffi::helpers::str_to_c_string("SzRustSDK-Config")?;
                let ini_params_c =
                    crate::ffi::helpers::str_to_c_string(existing_env.get_ini_params())?;
                let verbose = if existing_env.get_verbose_logging() {
                    1
                } else {
                    0
                };

                ffi_call_config!(crate::ffi::bindings::SzConfig_init(
                    module_name_c.as_ptr(),
                    ini_params_c.as_ptr(),
                    verbose
                ));

                // Load the config definition directly - this returns the handle
                let config_def_c = crate::ffi::helpers::str_to_c_string(config_definition)?;
                let result =
                    unsafe { crate::ffi::bindings::SzConfig_load_helper(config_def_c.as_ptr()) };
                if result.return_code != 0 {
                    crate::ffi::helpers::check_config_return_code(result.return_code)?;
                }
                let handle = result.response;

                Ok(Self { handle })
            }
            Err(e) => {
                // Error if no environment exists - don't create fake objects
                Err(crate::error::SzError::configuration(format!(
                    "Cannot create config with definition without initialized environment: {}",
                    e
                )))
            }
        }
    }
}

impl SzConfig for SzConfigCore {
    fn export(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::SzConfig_export_helper(self.handle) };

        unsafe { crate::ffi::helpers::process_config_pointer_result(result) }
    }

    fn get_data_source_registry(&self) -> SzResult<JsonString> {
        // Use the native SzConfig_getDataSourceRegistry function instead of export
        let result =
            unsafe { crate::ffi::bindings::SzConfig_getDataSourceRegistry_helper(self.handle) };
        unsafe { crate::ffi::helpers::process_config_pointer_result(result) }
    }

    fn register_data_source(&self, data_source_code: &str) -> SzResult<JsonString> {
        // Create JSON input as expected by native function (matching C# SDK behavior)
        let json_input = format!(r#"{{"DSRC_CODE": "{}"}}"#, data_source_code);
        let data_source_c = crate::ffi::helpers::str_to_c_string(&json_input)?;

        let result = unsafe {
            crate::ffi::bindings::SzConfig_registerDataSource_helper(
                self.handle,
                data_source_c.as_ptr(),
            )
        };

        unsafe { crate::ffi::helpers::process_config_pointer_result(result) }
    }

    fn unregister_data_source(&self, data_source_code: &str) -> SzResult<()> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;

        ffi_call_config!(crate::ffi::bindings::SzConfig_unregisterDataSource_helper(
            self.handle,
            data_source_c.as_ptr()
        ));

        Ok(())
    }
}

impl Drop for SzConfigCore {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = crate::ffi::bindings::SzConfig_close_helper(self.handle);
            }
        }
        // Config cleanup is handled by the native library at process exit.
        // No explicit destroy call needed.
    }
}
