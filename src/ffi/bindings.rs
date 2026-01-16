//! Low-level FFI bindings to the native Senzing library
//!
//! These bindings match the native function calls used by the official C# SDK,
//! including the use of helper functions where available.

use libc::{c_char, c_longlong};

/// Result structure for helper functions that return pointers
#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct SzPointerResult {
    pub response: *mut c_char,
    pub return_code: c_longlong,
}

/// Result structure for helper functions that return long values
#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct SzLongResult {
    pub response: c_longlong,
    pub return_code: c_longlong,
}

/// Network analysis result structure
#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct SzNetworkResult {
    pub response: *mut c_char,
    pub return_code: c_longlong,
}

unsafe extern "C" {
    // Environment/Core functions
    pub fn Sz_init(
        module_name: *const c_char,
        ini_params: *const c_char,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn Sz_initWithConfigID(
        module_name: *const c_char,
        ini_params: *const c_char,
        config_id: c_longlong,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn Sz_reinit(config_id: c_longlong) -> c_longlong;

    pub fn Sz_destroy() -> c_longlong;

    // Module-specific destroy functions
    pub fn SzConfig_destroy() -> c_longlong;
    pub fn SzConfigMgr_destroy() -> c_longlong;
    pub fn SzDiagnostic_destroy() -> c_longlong;
    pub fn SzProduct_destroy() -> c_longlong;

    pub fn Sz_getActiveConfigID() -> c_longlong;

    // Engine functions using helper variants
    pub fn Sz_primeEngine() -> c_longlong;

    pub fn Sz_stats_helper() -> SzPointerResult;

    pub fn Sz_addRecord(
        data_source_code: *const c_char,
        record_id: *const c_char,
        record_definition: *const c_char,
    ) -> c_longlong;

    pub fn Sz_addRecordWithInfo_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        record_definition: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_deleteRecord(data_source_code: *const c_char, record_id: *const c_char)
    -> c_longlong;

    pub fn Sz_deleteRecordWithInfo_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_getEntityByEntityID_helper(entity_id: c_longlong) -> SzPointerResult;

    pub fn Sz_getEntityByRecordID_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
    ) -> SzPointerResult;

    pub fn Sz_getRecord_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_getRecordPreview_helper(
        record_definition: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_searchByAttributes_helper(attributes: *const c_char) -> SzPointerResult;

    pub fn Sz_searchByAttributes_V3_helper(
        attributes: *const c_char,
        search_profile: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_whyEntityByEntityID_helper(
        entity_id1: c_longlong,
        entity_id2: c_longlong,
        flags: c_longlong,
    ) -> *mut c_char;

    pub fn Sz_findPathByEntityID_helper(
        start_entity_id: c_longlong,
        end_entity_id: c_longlong,
        max_degrees: c_longlong,
        flags: c_longlong,
    ) -> *mut c_char;

    pub fn Sz_findNetworkByEntityID_V2_helper(
        entity_list: *const c_char,
        max_degrees: c_longlong,
        build_out_degree: c_longlong,
        max_entities: c_longlong,
        flags: c_longlong,
    ) -> SzNetworkResult;

    pub fn Sz_exportJSONEntityReport_helper(flags: c_longlong) -> SzPointerResult;

    pub fn Sz_exportCSVEntityReport_helper(
        csv_column_list: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_fetchNext_helper(export_handle: *const c_char) -> SzPointerResult;

    pub fn Sz_closeExportReport_helper(export_handle: *const c_char) -> c_longlong;

    pub fn Sz_processRedoRecord() -> *mut c_char;

    pub fn Sz_processRedoRecordWithInfo_helper(
        redo_record: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_getRedoRecord_helper() -> SzPointerResult;

    pub fn Sz_countRedoRecords() -> c_longlong;

    // Critical missing stewardship and analysis functions for 100% C# SDK parity
    pub fn Sz_reevaluateEntityWithInfo_helper(
        entity_id: c_longlong,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_reevaluateRecordWithInfo_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_whyRecordInEntity_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_whyEntities_helper(
        entity_id1: c_longlong,
        entity_id2: c_longlong,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_whySearch_helper(
        attributes: *const c_char,
        entity_id: c_longlong,
        search_profile: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_findInterestingEntitiesByEntityID_helper(
        entity_id: c_longlong,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_findInterestingEntitiesByRecordID_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_whyRecords_helper(
        data_source_code1: *const c_char,
        record_id1: *const c_char,
        data_source_code2: *const c_char,
        record_id2: *const c_char,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_howEntityByEntityID_helper(
        entity_id: c_longlong,
        flags: c_longlong,
    ) -> SzPointerResult;

    pub fn Sz_getVirtualEntityByRecordID_helper(
        data_source_code: *const c_char,
        record_id: *const c_char,
    ) -> SzPointerResult;

    // Config functions
    pub fn SzConfig_init(
        module_name: *const c_char,
        ini_params: *const c_char,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn SzConfig_create_helper() -> SzPointerResult;

    pub fn SzConfig_close_helper(config_handle: *const c_char) -> c_longlong;

    pub fn SzConfig_load_helper(config_definition: *const c_char) -> SzPointerResult;

    pub fn SzConfig_export_helper(config_handle: *const c_char) -> SzPointerResult;

    pub fn SzConfig_getDataSourceRegistry_helper(config_handle: *const c_char) -> SzPointerResult;

    pub fn SzConfig_registerDataSource(
        config_handle: *const c_char,
        data_source_code: *const c_char,
        response: *mut *mut c_char,
        response_size: *mut usize,
    ) -> c_longlong;

    pub fn SzConfig_registerDataSource_helper(
        config_handle: *const c_char,
        data_source_code: *const c_char,
    ) -> SzPointerResult;

    pub fn SzConfig_unregisterDataSource_helper(
        config_handle: *const c_char,
        data_source_code: *const c_char,
    ) -> c_longlong;

    // ConfigManager functions
    pub fn SzConfigMgr_init(
        module_name: *const c_char,
        ini_params: *const c_char,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn SzConfigMgr_getDefaultConfigID_helper() -> SzLongResult;

    pub fn SzConfigMgr_registerConfig_helper(
        config_definition: *const c_char,
        config_comment: *const c_char,
    ) -> SzLongResult;

    pub fn SzConfigMgr_getConfig_helper(config_id: c_longlong) -> SzPointerResult;

    pub fn SzConfigMgr_getConfigRegistry_helper() -> SzPointerResult;

    pub fn SzConfigMgr_replaceDefaultConfigID(
        current_default_config_id: c_longlong,
        new_default_config_id: c_longlong,
    ) -> c_longlong;

    pub fn SzConfigMgr_setDefaultConfigID(config_id: c_longlong) -> c_longlong;

    // Diagnostic functions
    pub fn SzDiagnostic_init(
        module_name: *const c_char,
        ini_params: *const c_char,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn SzDiagnostic_checkRepositoryPerformance_helper(
        seconds_to_run: c_longlong,
    ) -> SzPointerResult;

    pub fn SzDiagnostic_getFeature_helper(feature_id: c_longlong) -> SzPointerResult;

    pub fn SzDiagnostic_getRepositoryInfo_helper() -> SzPointerResult;

    pub fn SzDiagnostic_purgeRepository() -> c_longlong;

    // Product functions
    pub fn SzProduct_init(
        module_name: *const c_char,
        ini_params: *const c_char,
        verbose_logging: c_longlong,
    ) -> c_longlong;

    pub fn SzProduct_getLicense() -> *mut c_char;

    pub fn SzProduct_getVersion() -> *mut c_char;

    // Memory management
    pub fn Sz_free(ptr: *mut c_char);

    // Error handling
    pub fn Sz_getLastException(buf: *mut c_char, length: c_longlong) -> c_longlong;

    pub fn Sz_getLastExceptionCode() -> c_longlong;

    pub fn Sz_clearLastException();

    pub fn SzConfig_getLastException(buf: *mut c_char, length: c_longlong) -> c_longlong;

    pub fn SzConfig_getLastExceptionCode() -> c_longlong;

    pub fn SzConfig_clearLastException();

    pub fn SzConfigMgr_getLastException(buf: *mut c_char, length: c_longlong) -> c_longlong;

    pub fn SzConfigMgr_getLastExceptionCode() -> c_longlong;

    pub fn SzConfigMgr_clearLastException();

    pub fn SzDiagnostic_getLastException(buf: *mut c_char, length: c_longlong) -> c_longlong;

    pub fn SzDiagnostic_getLastExceptionCode() -> c_longlong;

    pub fn SzDiagnostic_clearLastException();

    pub fn SzProduct_getLastException(buf: *mut c_char, length: c_longlong) -> c_longlong;

    pub fn SzProduct_getLastExceptionCode() -> c_longlong;

    pub fn SzProduct_clearLastException();
}
