//! Core implementation of SzDiagnostic trait

use crate::{
    error::SzResult,
    ffi_call_diagnostic, process_diagnostic_result,
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
        Ok(Self)
    }

    pub fn new_with_params(
        _module_name: &str,
        _ini_params: &str,
        _verbose_logging: bool,
    ) -> SzResult<Self> {
        Self::new()
    }
}

impl SzDiagnostic for SzDiagnosticCore {
    fn check_repository_performance(&self, seconds_to_run: i64) -> SzResult<JsonString> {
        let result =
            unsafe { crate::ffi::SzDiagnostic_checkRepositoryPerformance_helper(seconds_to_run) };
        process_diagnostic_result!(result)
    }

    fn get_feature(&self, feature_id: FeatureId) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::SzDiagnostic_getFeature_helper(feature_id) };
        process_diagnostic_result!(result)
    }

    fn get_repository_info(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::SzDiagnostic_getRepositoryInfo_helper() };
        process_diagnostic_result!(result)
    }

    fn purge_repository(&self) -> SzResult<()> {
        ffi_call_diagnostic!(crate::ffi::SzDiagnostic_purgeRepository());
        Ok(())
    }
}
