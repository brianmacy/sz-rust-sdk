//! Core implementation of SzDiagnostic trait

use crate::{
    error::SzResult,
    ffi_call,
    traits::SzDiagnostic,
    types::{FeatureId, JsonString},
};

/// Core implementation of the SzDiagnostic trait
///
/// This is a zero-sized type as the diagnostic component uses module-level
/// functions in the native library after environment initialization.
pub struct SzDiagnosticCore;

impl SzDiagnosticCore {
    pub fn new() -> SzResult<Self> {
        // Don't call SzDiagnostic_init - rely on the main environment initialization
        // The diagnostic functions should work once the main Sz_init has been called
        Ok(Self)
    }

    pub fn new_with_params(
        _module_name: &str,
        _ini_params: &str,
        _verbose_logging: bool,
    ) -> SzResult<Self> {
        // For now, treat this the same as new() - the diagnostic component may not need
        // separate initialization if the main environment is already initialized
        Self::new()
    }
}

impl SzDiagnostic for SzDiagnosticCore {
    fn check_repository_performance(&self, seconds_to_run: i64) -> SzResult<JsonString> {
        let result = unsafe {
            crate::ffi::bindings::SzDiagnostic_checkRepositoryPerformance_helper(seconds_to_run)
        };

        unsafe { crate::ffi::helpers::process_pointer_result(result) }
    }

    fn get_feature(&self, feature_id: FeatureId) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::SzDiagnostic_getFeature_helper(feature_id) };

        unsafe { crate::ffi::helpers::process_pointer_result(result) }
    }

    fn get_repository_info(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::SzDiagnostic_getRepositoryInfo_helper() };

        unsafe { crate::ffi::helpers::process_pointer_result(result) }
    }

    fn purge_repository(&self) -> SzResult<()> {
        ffi_call!(crate::ffi::bindings::SzDiagnostic_purgeRepository());
        Ok(())
    }
}
