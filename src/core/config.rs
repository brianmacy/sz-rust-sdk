//! Core implementation of SzConfig trait

use crate::{
    error::SzResult, ffi_call_config, process_config_result, traits::SzConfig, types::JsonString,
};

/// Config handle type (matches C uintptr_t)
type ConfigHandle = usize;

/// Core implementation of the SzConfig trait
pub struct SzConfigCore {
    handle: ConfigHandle,
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
        let module_name_c = crate::ffi::helpers::str_to_c_string(module_name)?;
        let ini_params_c = crate::ffi::helpers::str_to_c_string(ini_params)?;
        let verbose = if verbose_logging { 1 } else { 0 };

        ffi_call_config!(crate::ffi::SzConfig_init(
            module_name_c.as_ptr(),
            ini_params_c.as_ptr(),
            verbose
        ));

        let result = unsafe { crate::ffi::SzConfig_create_helper() };
        if result.returnCode != 0 {
            crate::ffi::helpers::check_config_return_code(result.returnCode)?;
        }
        let handle = result.response;

        Ok(Self { handle })
    }

    pub fn new_with_definition(config_definition: &str) -> SzResult<Self> {
        match super::environment::SzEnvironmentCore::get_existing_instance() {
            Ok(existing_env) => {
                let module_name_c = crate::ffi::helpers::str_to_c_string("SzRustSDK-Config")?;
                let ini_params_c =
                    crate::ffi::helpers::str_to_c_string(existing_env.get_ini_params())?;
                let verbose = if existing_env.get_verbose_logging() {
                    1
                } else {
                    0
                };

                ffi_call_config!(crate::ffi::SzConfig_init(
                    module_name_c.as_ptr(),
                    ini_params_c.as_ptr(),
                    verbose
                ));

                let config_def_c = crate::ffi::helpers::str_to_c_string(config_definition)?;
                let result = unsafe { crate::ffi::SzConfig_load_helper(config_def_c.as_ptr()) };
                if result.returnCode != 0 {
                    crate::ffi::helpers::check_config_return_code(result.returnCode)?;
                }
                // SzConfig_load_helper returns *mut c_void, cast to usize for our handle
                let handle = result.response as usize;

                Ok(Self { handle })
            }
            Err(e) => Err(crate::error::SzError::configuration(format!(
                "Cannot create config with definition without initialized environment: {e}"
            ))),
        }
    }
}

impl SzConfig for SzConfigCore {
    fn export(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::SzConfig_export_helper(self.handle) };
        process_config_result!(result)
    }

    fn get_data_source_registry(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::SzConfig_getDataSourceRegistry_helper(self.handle) };
        process_config_result!(result)
    }

    fn register_data_source(&self, data_source_code: &str) -> SzResult<JsonString> {
        let json_input = format!(r#"{{"DSRC_CODE": "{data_source_code}"}}"#);
        let data_source_c = crate::ffi::helpers::str_to_c_string(&json_input)?;

        let result = unsafe {
            crate::ffi::SzConfig_registerDataSource_helper(self.handle, data_source_c.as_ptr())
        };

        process_config_result!(result)
    }

    fn unregister_data_source(&self, data_source_code: &str) -> SzResult<()> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;

        ffi_call_config!(crate::ffi::SzConfig_unregisterDataSource_helper(
            self.handle,
            data_source_c.as_ptr()
        ));

        Ok(())
    }
}

impl Drop for SzConfigCore {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe {
                let _ = crate::ffi::SzConfig_close_helper(self.handle);
            }
        }
    }
}
