//! Core implementation of SzProduct trait

use crate::{error::SzResult, ffi_call_product, traits::SzProduct, types::JsonString};

/// Core implementation of the SzProduct trait
pub struct SzProductCore;

impl SzProductCore {
    pub fn new() -> SzResult<Self> {
        // Use minimal settings - product module may not need full database settings
        Self::new_with_params("SzRustSDK-Product", "{}", false)
    }

    pub fn new_with_params(
        module_name: &str,
        ini_params: &str,
        verbose_logging: bool,
    ) -> SzResult<Self> {
        // Initialize the product module with parameters
        let module_name_c = crate::ffi::helpers::str_to_c_string(module_name)?;
        let ini_params_c = crate::ffi::helpers::str_to_c_string(ini_params)?;
        let verbose = if verbose_logging { 1 } else { 0 };

        ffi_call_product!(crate::ffi::bindings::SzProduct_init(
            module_name_c.as_ptr(),
            ini_params_c.as_ptr(),
            verbose
        ));
        Ok(Self)
    }
}

impl SzProduct for SzProductCore {
    fn get_license(&self) -> SzResult<JsonString> {
        let license_ptr = unsafe { crate::ffi::bindings::SzProduct_getLicense() };
        if license_ptr.is_null() {
            return Err(crate::error::SzError::unknown("Failed to get license"));
        }
        unsafe { crate::ffi::helpers::c_str_to_string_no_free(license_ptr) }
    }

    fn get_version(&self) -> SzResult<JsonString> {
        let version_ptr = unsafe { crate::ffi::bindings::SzProduct_getVersion() };
        if version_ptr.is_null() {
            return Err(crate::error::SzError::unknown("Failed to get version"));
        }
        unsafe { crate::ffi::helpers::c_str_to_string_no_free(version_ptr) }
    }
}

// Note: SzProductCore no longer needs Drop implementation
// since it doesn't manage any resources directly
