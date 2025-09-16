//! Core implementation of SzConfigManager trait

use crate::{
    error::SzResult,
    ffi_call_config_mgr,
    traits::{SzConfig, SzConfigManager},
    types::{ConfigId, JsonString},
};
use std::ptr;

/// Core implementation of the SzConfigManager trait
pub struct SzConfigManagerCore {
    #[allow(dead_code)]
    handle: *mut std::ffi::c_void,
}

impl SzConfigManagerCore {
    pub fn new() -> SzResult<Self> {
        Self::new_with_params("SzRustSDK-ConfigMgr", "{}", false)
    }

    pub fn new_with_params(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
    ) -> SzResult<Self> {
        // Initialize the config manager module with parameters
        let module_name_c = crate::ffi::helpers::str_to_c_string(module_name)?;
        let ini_params_c = crate::ffi::helpers::str_to_c_string(ini_params)?;
        let verbose = if verbose_logging { 1 } else { 0 };

        ffi_call_config_mgr!(crate::ffi::bindings::SzConfigMgr_init(
            module_name_c.as_ptr(),
            ini_params_c.as_ptr(),
            verbose
        ));

        // Config manager doesn't need a handle for the new API
        Ok(Self {
            handle: ptr::null_mut(),
        })
    }
}

impl SzConfigManager for SzConfigManagerCore {
    fn create_config(&self) -> SzResult<Box<dyn SzConfig>> {
        // Create config with empty settings - the config module will use defaults
        let config_core = super::config::SzConfigCore::new_with_params(
            "SzRustSDK-Config",
            "{}",  // Empty settings for config creation
            false, // Use quiet logging to match environment
        )?;
        Ok(Box::new(config_core))
    }

    fn create_config_from_id(&self, config_id: ConfigId) -> SzResult<Box<dyn SzConfig>> {
        let result = unsafe { crate::ffi::bindings::SzConfigMgr_getConfig_helper(config_id) };
        let config_definition = unsafe { crate::ffi::helpers::process_pointer_result(result) }?;
        let config_core = super::config::SzConfigCore::new_with_definition(&config_definition)?;
        Ok(Box::new(config_core))
    }

    fn create_config_from_definition(
        &self,
        config_definition: &str,
    ) -> SzResult<Box<dyn SzConfig>> {
        let config_core = super::config::SzConfigCore::new_with_definition(config_definition)?;
        Ok(Box::new(config_core))
    }

    fn get_config_registry(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::SzConfigMgr_getConfigRegistry_helper() };
        unsafe { crate::ffi::helpers::process_config_mgr_pointer_result(result) }
    }

    fn get_default_config_id(&self) -> SzResult<ConfigId> {
        let result = unsafe { crate::ffi::bindings::SzConfigMgr_getDefaultConfigID_helper() };
        crate::ffi::helpers::process_config_mgr_long_result(result)
    }

    fn register_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId> {
        let config_def_c = crate::ffi::helpers::str_to_c_string(config_definition)?;
        let comment_c = crate::ffi::helpers::str_to_c_string(config_comment.unwrap_or(""))?;

        let result = unsafe {
            crate::ffi::bindings::SzConfigMgr_registerConfig_helper(
                config_def_c.as_ptr(),
                comment_c.as_ptr(),
            )
        };

        crate::ffi::helpers::process_config_mgr_long_result(result)
    }

    fn replace_default_config_id(
        &self,
        current_default_config_id: ConfigId,
        new_default_config_id: ConfigId,
    ) -> SzResult<()> {
        ffi_call_config_mgr!(crate::ffi::bindings::SzConfigMgr_replaceDefaultConfigID(
            current_default_config_id,
            new_default_config_id
        ));
        Ok(())
    }

    fn set_default_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId> {
        let config_id = self.register_config(config_definition, config_comment)?;
        self.set_default_config_id(config_id)?;
        Ok(config_id)
    }

    fn set_default_config_id(&self, config_id: ConfigId) -> SzResult<()> {
        ffi_call_config_mgr!(crate::ffi::bindings::SzConfigMgr_setDefaultConfigID(
            config_id
        ));
        Ok(())
    }
}

impl Drop for SzConfigManagerCore {
    fn drop(&mut self) {
        // Config manager doesn't need cleanup in the new API
    }
}
