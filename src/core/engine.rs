//! Core implementation of SzEngine trait
#![allow(unused_variables)]

use crate::{
    error::{SzError, SzResult},
    ffi_call,
    flags::SzFlags,
    traits::SzEngine,
    types::*,
};
use std::collections::HashSet;

/// Core implementation of the SzEngine trait
pub struct SzEngineCore;

impl SzEngineCore {
    pub fn new() -> SzResult<Self> {
        Ok(Self)
    }
}

impl SzEngine for SzEngineCore {
    fn prime_engine(&self) -> SzResult<()> {
        ffi_call!(crate::ffi::bindings::Sz_primeEngine());
        Ok(())
    }

    fn get_stats(&self) -> SzResult<JsonString> {
        unsafe {
            let result = crate::ffi::bindings::Sz_stats_helper();
            crate::ffi::helpers::process_pointer_result(result)
        }
    }

    fn add_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let record_def_c = crate::ffi::helpers::str_to_c_string(record_definition)?;
        let flags_bits = flags.unwrap_or(SzFlags::ADD_RECORD_DEFAULT).bits() as i64;

        let result = unsafe {
            crate::ffi::bindings::Sz_addRecordWithInfo_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                record_def_c.as_ptr(),
                flags_bits,
            )
        };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn get_record_preview(
        &self,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for record preview
        // For now, return a placeholder implementation
        Err(SzError::unknown("get_record_preview not yet implemented"))
    }

    fn delete_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags.unwrap_or(SzFlags::DELETE_RECORD_DEFAULT).bits() as i64;

        let result = unsafe {
            crate::ffi::bindings::Sz_deleteRecordWithInfo_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                flags_bits,
            )
        };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn reevaluate_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for reevaluate record
        Err(SzError::unknown("reevaluate_record not yet implemented"))
    }

    fn reevaluate_entity(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_value =
            flags.unwrap_or(SzFlags::REEVALUATE_ENTITY_DEFAULT).bits() as libc::c_longlong;

        let result = unsafe {
            crate::ffi::bindings::Sz_reevaluateEntityWithInfo_helper(entity_id, flags_value)
        };

        unsafe { crate::ffi::helpers::process_pointer_result(result) }
    }

    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let attributes_c = crate::ffi::helpers::str_to_c_string(attributes)?;
        let flags_bits = flags
            .unwrap_or(SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT)
            .bits() as i64;

        let result = if let Some(profile) = search_profile {
            let search_profile_c = crate::ffi::helpers::str_to_c_string(profile)?;
            unsafe {
                crate::ffi::bindings::Sz_searchByAttributes_V3_helper(
                    attributes_c.as_ptr(),
                    search_profile_c.as_ptr(),
                    flags_bits,
                )
            }
        } else {
            unsafe { crate::ffi::bindings::Sz_searchByAttributes_helper(attributes_c.as_ptr()) }
        };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn why_search(
        &self,
        attributes: &str,
        entity_id: EntityId,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for why search
        Err(SzError::unknown("why_search not yet implemented"))
    }

    fn get_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::bindings::Sz_getEntityByEntityID_helper(entity_id) };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn get_entity_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags.unwrap_or(SzFlags::GET_ENTITY_DEFAULT).bits() as i64;

        let result = unsafe {
            crate::ffi::bindings::Sz_getEntityByRecordID_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
            )
        };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for get record
        Err(SzError::unknown("get_record not yet implemented"))
    }

    fn find_interesting_entities_by_entity_id(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for find interesting entities
        Err(SzError::unknown(
            "find_interesting_entities_by_entity_id not yet implemented",
        ))
    }

    fn find_interesting_entities_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for find interesting entities by record
        Err(SzError::unknown(
            "find_interesting_entities_by_record not yet implemented",
        ))
    }

    fn find_path(
        &self,
        start_entity_id: EntityId,
        end_entity_id: EntityId,
        max_degrees: i64,
        avoid_entity_ids: Option<&HashSet<EntityId>>,
        required_data_sources: Option<&HashSet<String>>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_bits = flags.unwrap_or(SzFlags::FIND_PATH_DEFAULT).bits() as i64;

        let result_ptr = unsafe {
            crate::ffi::bindings::Sz_findPathByEntityID_helper(
                start_entity_id,
                end_entity_id,
                max_degrees,
                flags_bits,
            )
        };

        if result_ptr.is_null() {
            return Err(SzError::unknown("Failed to find path"));
        }

        unsafe { crate::ffi::helpers::c_str_to_string(result_ptr) }
    }

    fn find_network(
        &self,
        entity_list: &[EntityId],
        max_degrees: i64,
        build_out_degree: i64,
        max_entities: i64,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // Format entity list as JSON object with ENTITIES array, as expected by Senzing
        let entity_objects: Vec<serde_json::Value> = entity_list
            .iter()
            .map(|&id| serde_json::json!({"ENTITY_ID": id}))
            .collect();

        let entity_list_json = serde_json::json!({
            "ENTITIES": entity_objects
        })
        .to_string();

        let entity_list_c = crate::ffi::helpers::str_to_c_string(&entity_list_json)?;
        let flags_bits = flags.unwrap_or(SzFlags::FIND_NETWORK_DEFAULT).bits() as i64;

        let result = unsafe {
            crate::ffi::bindings::Sz_findNetworkByEntityID_V2_helper(
                entity_list_c.as_ptr(),
                max_degrees,
                build_out_degree,
                max_entities,
                flags_bits,
            )
        };

        // Process the network-specific result structure
        if result.return_code != 0 {
            crate::ffi::helpers::check_return_code(result.return_code)?;
        }

        unsafe { crate::ffi::helpers::c_str_to_string(result.response) }
    }

    fn why_entity(
        &self,
        entity_id1: EntityId,
        entity_id2: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_value = flags.unwrap_or(SzFlags::WHY_ENTITY_DEFAULT).bits() as libc::c_longlong;

        let result = unsafe {
            crate::ffi::bindings::Sz_whyEntities_helper(entity_id1, entity_id2, flags_value)
        };

        unsafe { crate::ffi::helpers::process_pointer_result(result) }
    }

    fn why_records(
        &self,
        data_source_code1: &str,
        record_id1: &str,
        data_source_code2: &str,
        record_id2: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for why records
        Err(SzError::unknown("why_records not yet implemented"))
    }

    fn how_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString> {
        // This would require a specific FFI function for how entity
        Err(SzError::unknown("how_entity not yet implemented"))
    }

    fn get_virtual_entity(
        &self,
        record_definitions: &[&str],
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        // This would require a specific FFI function for get virtual entity
        Err(SzError::unknown("get_virtual_entity not yet implemented"))
    }

    fn process_redo_record(&self) -> SzResult<JsonString> {
        // This would require a specific FFI function for process redo record
        Err(SzError::unknown("process_redo_record not yet implemented"))
    }

    fn count_redo_records(&self) -> SzResult<i64> {
        // This would require a specific FFI function for count redo records
        Err(SzError::unknown("count_redo_records not yet implemented"))
    }

    fn export_json_entity_report(&self, flags: Option<SzFlags>) -> SzResult<ExportHandle> {
        let flags_bits = flags.unwrap_or_default().bits() as i64;

        let result = unsafe { crate::ffi::bindings::Sz_exportJSONEntityReport_helper(flags_bits) };

        let export_handle_str =
            unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }?;

        // Convert the string handle to an i64 for our API
        export_handle_str
            .parse()
            .map_err(|_| SzError::ffi("Invalid export handle"))
    }

    fn export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<ExportHandle> {
        // This would require a specific FFI function for CSV export
        Err(SzError::unknown(
            "export_csv_entity_report not yet implemented",
        ))
    }

    fn fetch_next(&self, export_handle: ExportHandle) -> SzResult<JsonString> {
        let handle_str = export_handle.to_string();
        let handle_c = crate::ffi::helpers::str_to_c_string(&handle_str)?;

        let result = unsafe { crate::ffi::bindings::Sz_fetchNext_helper(handle_c.as_ptr()) };

        unsafe { crate::ffi::helpers::process_engine_pointer_result(result) }
    }

    fn close_export(&self, export_handle: ExportHandle) -> SzResult<()> {
        let handle_str = export_handle.to_string();
        let handle_c = crate::ffi::helpers::str_to_c_string(&handle_str)?;

        ffi_call!(crate::ffi::bindings::Sz_closeExportReport_helper(
            handle_c.as_ptr()
        ));
        Ok(())
    }

    fn get_export_stats(&self, export_handle: ExportHandle) -> SzResult<JsonString> {
        // This would require a specific FFI function for export stats
        Err(SzError::unknown("get_export_stats not yet implemented"))
    }
}

// Note: SzEngineCore no longer needs Drop implementation
// since it doesn't manage any resources directly
