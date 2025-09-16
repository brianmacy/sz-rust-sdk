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
    pub fn new() -> SzResult<Self> {
        // Use minimal settings - config module may not need full database settings
        Self::new_with_params("SzRustSDK-Config", "{}", false)
    }

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
        let config = Self::new()?;
        config.load_definition(config_definition)?;
        Ok(config)
    }

    fn load_definition(&self, config_definition: &str) -> SzResult<()> {
        let config_def_c = crate::ffi::helpers::str_to_c_string(config_definition)?;
        ffi_call_config!(crate::ffi::bindings::SzConfig_load_helper(
            self.handle,
            config_def_c.as_ptr()
        ));
        Ok(())
    }
}

impl SzConfig for SzConfigCore {
    fn export(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::SzConfig_export_helper(self.handle) };

        unsafe { crate::ffi::helpers::process_config_pointer_result(result) }
    }

    fn get_data_source_registry(&self) -> SzResult<JsonString> {
        // This would require a specific FFI function for getting data source registry
        // For now, we'll use export and extract the data source information
        self.export()
    }

    fn register_data_source(&self, data_source_code: &str) -> SzResult<JsonString> {
        // Create the proper JSON format for data source registration
        let data_source_json = format!(r#"{{"DSRC_CODE": "{}"}}"#, data_source_code);
        let data_source_c = crate::ffi::helpers::str_to_c_string(&data_source_json)?;

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
    }
}
