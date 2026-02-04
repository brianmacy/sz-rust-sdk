//! Core implementation of SzConfigManager trait

use crate::{
    error::SzResult,
    ffi_call_config_mgr, process_config_mgr_long_result, process_config_mgr_result,
    traits::{SzConfig, SzConfigManager},
    types::{ConfigId, JsonString},
};

/// Core implementation of the SzConfigManager trait
///
/// This is a zero-sized type as the config manager uses module-level
/// functions in the native library after initialization.
pub struct SzConfigManagerCore;

impl SzConfigManagerCore {
    /// Creates a new SzConfigManagerCore without initializing the native library.
    /// Caller must ensure SzConfigMgr_init has already been called.
    pub(crate) fn new() -> SzResult<Self> {
        Ok(Self)
    }
}

impl SzConfigManager for SzConfigManagerCore {
    fn create_config(&self) -> SzResult<Box<dyn SzConfig>> {
        match super::environment::SzEnvironmentCore::get_existing_instance() {
            Ok(existing_env) => {
                let config_core = super::config::SzConfigCore::new_with_params(
                    "SzRustSDK-Config",
                    existing_env.get_ini_params(),
                    existing_env.get_verbose_logging(),
                )?;
                Ok(Box::new(config_core))
            }
            Err(e) => Err(crate::error::SzError::configuration(format!(
                "Cannot create config without initialized environment: {e}"
            ))),
        }
    }

    fn create_config_from_id(&self, config_id: ConfigId) -> SzResult<Box<dyn SzConfig>> {
        let result = unsafe { crate::ffi::SzConfigMgr_getConfig_helper(config_id) };
        let config_definition = process_config_mgr_result!(result)?;

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
        let result = unsafe { crate::ffi::SzConfigMgr_getConfigRegistry_helper() };
        process_config_mgr_result!(result)
    }

    fn get_default_config_id(&self) -> SzResult<ConfigId> {
        let result = unsafe { crate::ffi::SzConfigMgr_getDefaultConfigID_helper() };
        process_config_mgr_long_result!(result)
    }

    fn register_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId> {
        let config_def_c = crate::ffi::helpers::str_to_c_string(config_definition)?;
        let comment_c = crate::ffi::helpers::str_to_c_string(config_comment.unwrap_or(""))?;

        let result = unsafe {
            crate::ffi::SzConfigMgr_registerConfig_helper(config_def_c.as_ptr(), comment_c.as_ptr())
        };

        process_config_mgr_long_result!(result)
    }

    fn replace_default_config_id(
        &self,
        current_default_config_id: ConfigId,
        new_default_config_id: ConfigId,
    ) -> SzResult<()> {
        ffi_call_config_mgr!(crate::ffi::SzConfigMgr_replaceDefaultConfigID(
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
        ffi_call_config_mgr!(crate::ffi::SzConfigMgr_setDefaultConfigID(config_id));
        Ok(())
    }
}
