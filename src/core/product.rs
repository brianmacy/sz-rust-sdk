//! Core implementation of SzProduct trait

use crate::{error::SzResult, traits::SzProduct, types::JsonString};

/// Core implementation of the SzProduct trait
pub struct SzProductCore;

impl SzProductCore {
    /// Creates a new SzProductCore without initializing the native library.
    /// Caller must ensure SzProduct_init has already been called.
    pub(crate) fn new() -> SzResult<Self> {
        Ok(Self)
    }
}

impl SzProduct for SzProductCore {
    fn get_license(&self) -> SzResult<JsonString> {
        let license_ptr = unsafe { crate::ffi::SzProduct_getLicense() };
        if license_ptr.is_null() {
            return Err(crate::error::SzError::unknown("Failed to get license"));
        }
        unsafe { crate::ffi::helpers::c_str_to_string_no_free(license_ptr) }
    }

    fn get_version(&self) -> SzResult<JsonString> {
        let version_ptr = unsafe { crate::ffi::SzProduct_getVersion() };
        if version_ptr.is_null() {
            return Err(crate::error::SzError::unknown("Failed to get version"));
        }
        unsafe { crate::ffi::helpers::c_str_to_string_no_free(version_ptr) }
    }
}

// Note: SzProductCore no longer needs Drop implementation
// since it doesn't manage any resources directly
