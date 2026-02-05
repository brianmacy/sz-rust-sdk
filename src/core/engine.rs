//! Core implementation of SzEngine trait

use crate::{
    error::{SzError, SzResult},
    ffi_call,
    flags::SzFlags,
    process_engine_result,
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
        ffi_call!(crate::ffi::Sz_primeEngine());
        Ok(())
    }

    fn get_stats(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::Sz_stats_helper() };
        process_engine_result!(result)
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
        let flags_bits = flags.unwrap_or(SzFlags::ADD_RECORD_DEFAULT_FLAGS).bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_addRecordWithInfo_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                record_def_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn get_record_preview(
        &self,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let record_def_c = crate::ffi::helpers::str_to_c_string(record_definition)?;
        let flags_bits = flags.unwrap_or(SzFlags::RECORD_DEFAULT_FLAGS).bits() as i64;

        let result =
            unsafe { crate::ffi::Sz_getRecordPreview_helper(record_def_c.as_ptr(), flags_bits) };

        process_engine_result!(result)
    }

    fn delete_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags.unwrap_or(SzFlags::DELETE_RECORD_DEFAULT_FLAGS).bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_deleteRecordWithInfo_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn reevaluate_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags
            .unwrap_or(SzFlags::REEVALUATE_RECORD_DEFAULT_FLAGS)
            .bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_reevaluateRecordWithInfo_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn reevaluate_entity(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_bits = flags
            .unwrap_or(SzFlags::REEVALUATE_ENTITY_DEFAULT_FLAGS)
            .bits() as i64;

        let result =
            unsafe { crate::ffi::Sz_reevaluateEntityWithInfo_helper(entity_id, flags_bits) };

        process_engine_result!(result)
    }

    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let attributes_c = crate::ffi::helpers::str_to_c_string(attributes)?;
        let flags_bits = flags
            .unwrap_or(SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS)
            .bits() as i64;

        // V2 and V3 have different result types, so handle separately
        if let Some(profile) = search_profile {
            let search_profile_c = crate::ffi::helpers::str_to_c_string(profile)?;
            let result = unsafe {
                crate::ffi::Sz_searchByAttributes_V3_helper(
                    attributes_c.as_ptr(),
                    search_profile_c.as_ptr(),
                    flags_bits,
                )
            };
            process_engine_result!(result)
        } else {
            let result = unsafe {
                crate::ffi::Sz_searchByAttributes_V2_helper(attributes_c.as_ptr(), flags_bits)
            };
            process_engine_result!(result)
        }
    }

    fn why_search(
        &self,
        attributes: &str,
        entity_id: EntityId,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let attributes_c = crate::ffi::helpers::str_to_c_string(attributes)?;
        let search_profile_c = search_profile
            .map(crate::ffi::helpers::str_to_c_string)
            .transpose()?;
        let search_profile_ptr = search_profile_c
            .as_ref()
            .map(|c_str| c_str.as_ptr())
            .unwrap_or(std::ptr::null());
        let flags_bits = flags.unwrap_or(SzFlags::WHY_SEARCH_DEFAULT_FLAGS).bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_whySearch_V2_helper(
                attributes_c.as_ptr(),
                entity_id,
                search_profile_ptr,
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn get_entity(&self, entity_ref: EntityRef, flags: Option<SzFlags>) -> SzResult<JsonString> {
        let flags_bits = flags.unwrap_or(SzFlags::ENTITY_DEFAULT_FLAGS).bits() as i64;

        match entity_ref {
            EntityRef::Id(entity_id) => {
                let result =
                    unsafe { crate::ffi::Sz_getEntityByEntityID_V2_helper(entity_id, flags_bits) };
                process_engine_result!(result)
            }
            EntityRef::Record {
                data_source,
                record_id,
            } => {
                let data_source_c = crate::ffi::helpers::str_to_c_string(data_source)?;
                let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
                let result = unsafe {
                    crate::ffi::Sz_getEntityByRecordID_V2_helper(
                        data_source_c.as_ptr(),
                        record_id_c.as_ptr(),
                        flags_bits,
                    )
                };
                process_engine_result!(result)
            }
        }
    }

    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags.unwrap_or(SzFlags::RECORD_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result = unsafe {
            crate::ffi::Sz_getRecord_V2_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn find_interesting_entities(
        &self,
        entity_ref: EntityRef,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_bits = flags
            .unwrap_or(SzFlags::FIND_INTERESTING_ENTITIES_DEFAULT_FLAGS)
            .bits() as i64;

        match entity_ref {
            EntityRef::Id(entity_id) => {
                let result = unsafe {
                    crate::ffi::Sz_findInterestingEntitiesByEntityID_helper(entity_id, flags_bits)
                };
                process_engine_result!(result)
            }
            EntityRef::Record {
                data_source,
                record_id,
            } => {
                let data_source_c = crate::ffi::helpers::str_to_c_string(data_source)?;
                let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
                let result = unsafe {
                    crate::ffi::Sz_findInterestingEntitiesByRecordID_helper(
                        data_source_c.as_ptr(),
                        record_id_c.as_ptr(),
                        flags_bits,
                    )
                };
                process_engine_result!(result)
            }
        }
    }

    fn find_path(
        &self,
        start_entity_id: EntityId,
        end_entity_id: EntityId,
        max_degrees: i64,
        _avoid_entity_ids: Option<&HashSet<EntityId>>,
        _required_data_sources: Option<&HashSet<String>>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_bits = flags.unwrap_or(SzFlags::FIND_PATH_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result = unsafe {
            crate::ffi::Sz_findPathByEntityID_V2_helper(
                start_entity_id,
                end_entity_id,
                max_degrees,
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn find_network(
        &self,
        entity_list: &[EntityId],
        max_degrees: i64,
        build_out_degree: i64,
        max_entities: i64,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let entity_objects: Vec<serde_json::Value> = entity_list
            .iter()
            .map(|&id| serde_json::json!({"ENTITY_ID": id}))
            .collect();

        let entity_list_json = serde_json::json!({
            "ENTITIES": entity_objects
        })
        .to_string();

        let entity_list_c = crate::ffi::helpers::str_to_c_string(&entity_list_json)?;
        let flags_bits = flags.unwrap_or(SzFlags::FIND_NETWORK_DEFAULT_FLAGS).bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_findNetworkByEntityID_V2_helper(
                entity_list_c.as_ptr(),
                max_degrees,
                build_out_degree,
                max_entities,
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn why_entities(
        &self,
        entity_id1: EntityId,
        entity_id2: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let flags_bits = flags.unwrap_or(SzFlags::WHY_ENTITIES_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result =
            unsafe { crate::ffi::Sz_whyEntities_V2_helper(entity_id1, entity_id2, flags_bits) };

        process_engine_result!(result)
    }

    fn why_records(
        &self,
        data_source_code1: &str,
        record_id1: &str,
        data_source_code2: &str,
        record_id2: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source1_c = crate::ffi::helpers::str_to_c_string(data_source_code1)?;
        let record_id1_c = crate::ffi::helpers::str_to_c_string(record_id1)?;
        let data_source2_c = crate::ffi::helpers::str_to_c_string(data_source_code2)?;
        let record_id2_c = crate::ffi::helpers::str_to_c_string(record_id2)?;
        let flags_bits = flags.unwrap_or(SzFlags::WHY_RECORDS_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result = unsafe {
            crate::ffi::Sz_whyRecords_V2_helper(
                data_source1_c.as_ptr(),
                record_id1_c.as_ptr(),
                data_source2_c.as_ptr(),
                record_id2_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn why_record_in_entity(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let data_source_c = crate::ffi::helpers::str_to_c_string(data_source_code)?;
        let record_id_c = crate::ffi::helpers::str_to_c_string(record_id)?;
        let flags_bits = flags.unwrap_or(SzFlags::WHY_RECORDS_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result = unsafe {
            crate::ffi::Sz_whyRecordInEntity_V2_helper(
                data_source_c.as_ptr(),
                record_id_c.as_ptr(),
                flags_bits,
            )
        };

        process_engine_result!(result)
    }

    fn how_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString> {
        let flags_bits = flags.unwrap_or(SzFlags::HOW_ENTITY_DEFAULT_FLAGS).bits() as i64;

        // Use V2 helper which accepts flags
        let result = unsafe { crate::ffi::Sz_howEntityByEntityID_V2_helper(entity_id, flags_bits) };

        process_engine_result!(result)
    }

    fn get_virtual_entity(
        &self,
        record_keys: &[(String, String)],
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        if record_keys.is_empty() {
            return Err(SzError::configuration("No record keys provided"));
        }

        let record_objects: Vec<serde_json::Value> = record_keys
            .iter()
            .map(|(data_source, record_id)| {
                serde_json::json!({
                    "DATA_SOURCE": data_source,
                    "RECORD_ID": record_id
                })
            })
            .collect();

        let record_list_json = serde_json::json!({
            "RECORDS": record_objects
        })
        .to_string();

        let record_list_c = crate::ffi::helpers::str_to_c_string(&record_list_json)?;
        let flags_bits = flags
            .unwrap_or(SzFlags::VIRTUAL_ENTITY_DEFAULT_FLAGS)
            .bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_getVirtualEntityByRecordID_V2_helper(record_list_c.as_ptr(), flags_bits)
        };

        process_engine_result!(result)
    }

    fn process_redo_record(
        &self,
        redo_record: &str,
        _flags: Option<SzFlags>,
    ) -> SzResult<JsonString> {
        let redo_record_c = crate::ffi::helpers::str_to_c_string(redo_record)?;
        // Note: Sz_processRedoRecordWithInfo_helper does not accept flags in the C API
        let result =
            unsafe { crate::ffi::Sz_processRedoRecordWithInfo_helper(redo_record_c.as_ptr()) };

        process_engine_result!(result)
    }

    fn get_redo_record(&self) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::Sz_getRedoRecord_helper() };
        process_engine_result!(result)
    }

    fn count_redo_records(&self) -> SzResult<i64> {
        let count = unsafe { crate::ffi::Sz_countRedoRecords() };
        Ok(count)
    }

    fn export_json_entity_report(&self, flags: Option<SzFlags>) -> SzResult<ExportHandle> {
        let flags_bits = flags.unwrap_or_default().bits() as i64;

        let result = unsafe { crate::ffi::Sz_exportJSONEntityReport_helper(flags_bits) };

        crate::ffi::helpers::check_return_code(result.returnCode)?;
        Ok(result.exportHandle as ExportHandle)
    }

    fn export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<ExportHandle> {
        let csv_columns_c = crate::ffi::helpers::str_to_c_string(csv_column_list)?;
        let flags_bits = flags.unwrap_or_default().bits() as i64;

        let result = unsafe {
            crate::ffi::Sz_exportCSVEntityReport_helper(csv_columns_c.as_ptr(), flags_bits)
        };

        crate::ffi::helpers::check_return_code(result.returnCode)?;
        Ok(result.exportHandle as ExportHandle)
    }

    fn fetch_next(&self, export_handle: ExportHandle) -> SzResult<JsonString> {
        let result = unsafe { crate::ffi::Sz_fetchNext_helper(export_handle as usize) };

        process_engine_result!(result)
    }

    fn close_export(&self, export_handle: ExportHandle) -> SzResult<()> {
        ffi_call!(crate::ffi::Sz_closeExportReport_helper(
            export_handle as usize
        ));
        Ok(())
    }
}
