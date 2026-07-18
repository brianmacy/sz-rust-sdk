//! Datastore snapshot / restore for [`SzEnvironmentCore`].
//!
//! The `internal://` datastore is in-memory only, so its contents are lost when
//! the process exits. This module implements a portable, version-checked snapshot
//! that captures the registered engine configuration plus every loaded record, and
//! restores it into a fresh environment so a later process can warm-start from a
//! previous run.
//!
//! # Design
//!
//! The Senzing native library exposes no API for serializing the opaque in-memory
//! engine state (records, resolution state, and vector indexes). The snapshot is
//! therefore built from the two things the SDK *can* extract portably:
//!
//! 1. the active configuration (via the config manager), and
//! 2. every record's original mapped JSON (via the entity export report).
//!
//! On restore the configuration is re-registered and the records are re-added, so
//! the engine re-runs entity resolution. Because resolution is deterministic, the
//! restored engine reproduces the same entity IDs, match keys, and scoring as the
//! source. Records are stored as portable JSON, so a snapshot is independent of CPU
//! architecture and operating system.
//!
//! This preserves records and resolution state without needing the original input
//! data. It does not persist opaque internal indexes, so restore is not cheaper
//! than the original load phase; where records already carry precomputed
//! `SEMANTIC_VALUE` embeddings, restore avoids recomputing those embeddings.

use crate::error::{SzError, SzResult};
use crate::flags::SzFlags;
use crate::traits::SzEnvironment;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use super::environment::SzEnvironmentCore;

/// Format identifier written into (and verified from) every snapshot manifest.
const SNAPSHOT_FORMAT: &str = "senzing-datastore-snapshot";

/// On-disk snapshot format version. Incremented on any breaking format change so
/// that an incompatible snapshot is rejected with a clear error rather than being
/// loaded into an inconsistent state.
const SNAPSHOT_FORMAT_VERSION: u32 = 1;

/// First line of a snapshot: self-describing metadata used for version checking.
#[derive(Debug, Serialize, serde::Deserialize)]
struct SnapshotManifest {
    /// Format identifier; must equal [`SNAPSHOT_FORMAT`].
    format: String,
    /// Format version; must equal [`SNAPSHOT_FORMAT_VERSION`].
    format_version: u32,
    /// Version of the SDK crate that produced the snapshot (informational).
    sdk_version: String,
    /// Senzing engine version that produced the snapshot (informational).
    engine_version: String,
    /// Creation time as seconds since the Unix epoch (informational).
    created_epoch: u64,
}

/// Second line of a snapshot: the configuration definition to restore.
#[derive(Debug, Serialize, serde::Deserialize)]
struct SnapshotConfig {
    /// Full configuration definition JSON (as produced by `SzConfig::export`).
    config: String,
}

/// Every subsequent line of a snapshot: one loaded record.
#[derive(Debug, Serialize, serde::Deserialize)]
struct SnapshotRecord {
    /// Data source code the record belongs to.
    data_source: String,
    /// Record identifier within the data source.
    record_id: String,
    /// The record's original mapped JSON.
    record: Value,
}

/// Validates a snapshot manifest before any data is loaded.
///
/// Kept separate so the version-checking policy can be unit-tested without a
/// running engine.
fn check_manifest(manifest: &SnapshotManifest) -> SzResult<()> {
    if manifest.format != SNAPSHOT_FORMAT {
        return Err(SzError::bad_input(format!(
            "Not a Senzing datastore snapshot (found format identifier '{}')",
            manifest.format
        )));
    }
    if manifest.format_version != SNAPSHOT_FORMAT_VERSION {
        return Err(SzError::bad_input(format!(
            "Unsupported snapshot format version {} (this SDK supports version {})",
            manifest.format_version, SNAPSHOT_FORMAT_VERSION
        )));
    }
    Ok(())
}

/// Extracts the `VERSION` field from the engine's version JSON, if present.
fn engine_version(env: &SzEnvironmentCore) -> String {
    env.get_product()
        .ok()
        .and_then(|p| p.get_version().ok())
        .and_then(|v| serde_json::from_str::<Value>(&v).ok())
        .and_then(|v| v.get("VERSION").and_then(Value::as_str).map(str::to_string))
        .unwrap_or_default()
}

/// Seconds since the Unix epoch (0 if the clock is before the epoch).
fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Extracts the loadable records from one exported entity JSON line.
///
/// The export report groups records under `RESOLVED_ENTITY.RECORDS`; each entry
/// carries `DATA_SOURCE`, `RECORD_ID`, and the original `JSON_DATA`. `JSON_DATA`
/// may be an embedded object or a JSON string depending on the engine, so it is
/// normalized to an object here.
fn records_from_entity(entity_json: &str) -> SzResult<Vec<SnapshotRecord>> {
    let value: Value = serde_json::from_str(entity_json)?;
    let mut records = Vec::new();

    let record_array = value
        .get("RESOLVED_ENTITY")
        .and_then(|e| e.get("RECORDS"))
        .and_then(Value::as_array);

    if let Some(array) = record_array {
        for entry in array {
            let data_source = entry.get("DATA_SOURCE").and_then(Value::as_str);
            let record_id = entry.get("RECORD_ID").and_then(Value::as_str);
            let json_data = entry.get("JSON_DATA");

            if let (Some(data_source), Some(record_id), Some(json_data)) =
                (data_source, record_id, json_data)
            {
                let record = match json_data {
                    Value::String(s) => {
                        serde_json::from_str::<Value>(s).unwrap_or_else(|_| json_data.clone())
                    }
                    other => other.clone(),
                };
                records.push(SnapshotRecord {
                    data_source: data_source.to_string(),
                    record_id: record_id.to_string(),
                    record,
                });
            }
        }
    }

    Ok(records)
}

/// Serializes `value` as a single JSON line.
fn write_line<W: Write, T: Serialize>(writer: &mut W, value: &T) -> SzResult<()> {
    let json = serde_json::to_string(value)?;
    writer
        .write_all(json.as_bytes())
        .and_then(|()| writer.write_all(b"\n"))
        .map_err(|e| SzError::bad_input(format!("Failed writing snapshot: {e}")))
}

/// Reads and deserializes a single required JSON line.
fn read_line<R: BufRead, T: DeserializeOwned>(reader: &mut R, what: &str) -> SzResult<T> {
    let mut line = String::new();
    let read = reader
        .read_line(&mut line)
        .map_err(|e| SzError::bad_input(format!("Failed reading snapshot {what}: {e}")))?;
    if read == 0 {
        return Err(SzError::bad_input(format!(
            "Snapshot is truncated: missing {what}"
        )));
    }
    serde_json::from_str(line.trim_end()).map_err(SzError::from)
}

/// Implements [`SzEnvironment::export_datastore_snapshot`].
pub(crate) fn export_snapshot(env: &SzEnvironmentCore, path: &Path) -> SzResult<()> {
    if env.is_destroyed() {
        return Err(SzError::unrecoverable("Environment has been destroyed"));
    }

    let engine = env.get_engine()?;
    let config_mgr = env.get_config_manager()?;

    // Capture the active configuration so it can be restored verbatim.
    let config_id = env.get_active_config_id()?;
    if config_id == 0 {
        return Err(SzError::configuration(
            "No configuration is registered in the datastore; nothing to snapshot",
        ));
    }
    let config = config_mgr.create_config_from_id(config_id)?;
    let config_definition = config.export()?;

    let file = std::fs::File::create(path).map_err(|e| {
        SzError::bad_input(format!(
            "Cannot create snapshot file '{}': {e}",
            path.display()
        ))
    })?;
    let mut writer = BufWriter::new(file);

    let manifest = SnapshotManifest {
        format: SNAPSHOT_FORMAT.to_string(),
        format_version: SNAPSHOT_FORMAT_VERSION,
        sdk_version: env!("CARGO_PKG_VERSION").to_string(),
        engine_version: engine_version(env),
        created_epoch: now_epoch_secs(),
    };
    write_line(&mut writer, &manifest)?;
    write_line(
        &mut writer,
        &SnapshotConfig {
            config: config_definition,
        },
    )?;

    // Stream every record out of the datastore, writing each line as it arrives so
    // that even very large datastores are exported with bounded memory use.
    let flags = SzFlags::EXPORT_INCLUDE_ALL_ENTITIES | SzFlags::ENTITY_INCLUDE_RECORD_JSON_DATA;
    let handle = engine.export_json_entity_report(Some(flags))?;
    let result = (|| -> SzResult<()> {
        loop {
            let chunk = engine.fetch_next(handle)?;
            if chunk.is_empty() {
                break;
            }
            for record in records_from_entity(&chunk)? {
                write_line(&mut writer, &record)?;
            }
        }
        Ok(())
    })();
    // Always release the export handle, even if writing failed midway.
    let _ = engine.close_export_report(handle);
    result?;

    writer
        .flush()
        .map_err(|e| SzError::bad_input(format!("Failed to flush snapshot: {e}")))?;
    Ok(())
}

/// Implements [`SzEnvironment::import_datastore_snapshot`].
pub(crate) fn import_snapshot(env: &SzEnvironmentCore, path: &Path) -> SzResult<()> {
    if env.is_destroyed() {
        return Err(SzError::unrecoverable("Environment has been destroyed"));
    }

    let file = std::fs::File::open(path).map_err(|e| {
        SzError::bad_input(format!(
            "Cannot open snapshot file '{}': {e}",
            path.display()
        ))
    })?;
    let mut reader = BufReader::new(file);

    // Line 1: manifest. Validate before loading anything so an incompatible
    // snapshot fails fast rather than partially loading.
    let manifest: SnapshotManifest = read_line(&mut reader, "manifest")?;
    check_manifest(&manifest)?;

    // Line 2: configuration. Register it and make it the default so the engine
    // initializes against the restored configuration.
    let snapshot_config: SnapshotConfig = read_line(&mut reader, "configuration")?;
    let config_mgr = env.get_config_manager()?;
    let config_id = config_mgr.set_default_config(
        &snapshot_config.config,
        Some("Restored from datastore snapshot"),
    )?;
    // `reinitialize` initializes the engine if needed, then activates the restored
    // configuration. This works whether or not the engine has been used yet.
    env.reinitialize(config_id)?;

    // Remaining lines: records. Re-add each so the engine re-resolves them into the
    // same entities as the source datastore.
    let engine = env.get_engine()?;
    for line in reader.lines() {
        let line = line.map_err(|e| SzError::bad_input(format!("Failed reading snapshot: {e}")))?;
        if line.trim().is_empty() {
            continue;
        }
        let record: SnapshotRecord = serde_json::from_str(&line)?;
        engine.add_record(
            &record.data_source,
            &record.record_id,
            &record.record.to_string(),
            None,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> SnapshotManifest {
        SnapshotManifest {
            format: SNAPSHOT_FORMAT.to_string(),
            format_version: SNAPSHOT_FORMAT_VERSION,
            sdk_version: "0.0.0".to_string(),
            engine_version: "4.0.0".to_string(),
            created_epoch: 1_700_000_000,
        }
    }

    #[test]
    fn test_manifest_roundtrips_and_validates() -> SzResult<()> {
        let json = serde_json::to_string(&sample_manifest())?;
        let parsed: SnapshotManifest = serde_json::from_str(&json)?;
        check_manifest(&parsed)?;
        Ok(())
    }

    #[test]
    fn test_check_manifest_rejects_wrong_format() {
        let mut manifest = sample_manifest();
        manifest.format = "some-other-format".to_string();
        let err = check_manifest(&manifest).expect_err("wrong format must be rejected");
        assert!(err.is_bad_input(), "expected BadInput, got {err:?}");
    }

    #[test]
    fn test_check_manifest_rejects_incompatible_version() {
        let mut manifest = sample_manifest();
        manifest.format_version = SNAPSHOT_FORMAT_VERSION + 1;
        let err = check_manifest(&manifest).expect_err("newer format version must be rejected");
        assert!(err.is_bad_input(), "expected BadInput, got {err:?}");
    }

    #[test]
    fn test_records_from_entity_extracts_object_json_data() -> SzResult<()> {
        let entity = r#"{
            "RESOLVED_ENTITY": {
                "ENTITY_ID": 1,
                "RECORDS": [
                    {"DATA_SOURCE": "TEST", "RECORD_ID": "R1",
                     "JSON_DATA": {"NAME_FULL": "John Smith", "RECORD_ID": "R1"}}
                ]
            }
        }"#;
        let records = records_from_entity(entity)?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].data_source, "TEST");
        assert_eq!(records[0].record_id, "R1");
        assert_eq!(records[0].record["NAME_FULL"], "John Smith");
        Ok(())
    }

    #[test]
    fn test_records_from_entity_normalizes_stringified_json_data() -> SzResult<()> {
        // Some engine builds emit JSON_DATA as a JSON string; it must be normalized
        // to an object so it re-adds cleanly.
        let entity = r#"{
            "RESOLVED_ENTITY": {
                "RECORDS": [
                    {"DATA_SOURCE": "TEST", "RECORD_ID": "R2",
                     "JSON_DATA": "{\"NAME_FULL\": \"Jane Doe\"}"}
                ]
            }
        }"#;
        let records = records_from_entity(entity)?;
        assert_eq!(records.len(), 1);
        assert!(
            records[0].record.is_object(),
            "JSON_DATA should be an object"
        );
        assert_eq!(records[0].record["NAME_FULL"], "Jane Doe");
        Ok(())
    }

    #[test]
    fn test_records_from_entity_handles_no_records() -> SzResult<()> {
        let records = records_from_entity(r#"{"RESOLVED_ENTITY": {"ENTITY_ID": 1}}"#)?;
        assert!(records.is_empty());
        Ok(())
    }

    #[test]
    fn test_read_line_reports_truncation() {
        let mut reader = std::io::Cursor::new(Vec::new());
        let err: SzError =
            read_line::<_, SnapshotManifest>(&mut reader, "manifest").expect_err("empty is error");
        assert!(err.is_bad_input(), "expected BadInput, got {err:?}");
    }

    #[test]
    fn test_record_line_roundtrips() -> SzResult<()> {
        let record = SnapshotRecord {
            data_source: "CUSTOMERS".to_string(),
            record_id: "1001".to_string(),
            record: serde_json::json!({"NAME_FULL": "Ada Lovelace"}),
        };
        let mut buffer = Vec::new();
        write_line(&mut buffer, &record)?;
        let parsed: SnapshotRecord = serde_json::from_str(std::str::from_utf8(&buffer).unwrap())?;
        assert_eq!(parsed.data_source, "CUSTOMERS");
        assert_eq!(parsed.record_id, "1001");
        assert_eq!(parsed.record["NAME_FULL"], "Ada Lovelace");
        Ok(())
    }
}
