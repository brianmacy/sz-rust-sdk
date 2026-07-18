//! Datastore snapshot / restore integration tests.
//!
//! These tests exercise a real round-trip against the live Senzing engine using
//! the in-memory `internal://` datastore (no mocks). They verify that a snapshot
//! taken from one environment can be restored into a fresh environment — after the
//! original in-memory datastore has been destroyed — and that the restored engine
//! reproduces the same entity-resolution state.
//!
//! This is the core use case from the feature request: `internal://` is otherwise
//! lost when the process exits, so `cleanup()` (which destroys the environment)
//! wipes the datastore. Only a restore from the snapshot brings the data back.

use serial_test::serial;
use std::path::PathBuf;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

const DATA_SOURCE: &str = "CUSTOMERS";

/// Test records. Records 1001 and 1002 share name, address, and phone, so the
/// engine resolves them into one entity; 1003 and 1004 are distinct. Expected
/// result: three resolved entities.
fn test_records() -> Vec<(&'static str, String)> {
    vec![
        (
            "1001",
            r#"{"DATA_SOURCE":"CUSTOMERS","RECORD_ID":"1001","NAME_FULL":"John Smith","ADDR_FULL":"123 Main St, Las Vegas, NV 89132","PHONE_NUMBER":"702-555-1212"}"#
                .to_string(),
        ),
        (
            "1002",
            r#"{"DATA_SOURCE":"CUSTOMERS","RECORD_ID":"1002","NAME_FULL":"John Smith","ADDR_FULL":"123 Main St, Las Vegas, NV 89132","PHONE_NUMBER":"702-555-1212","EMAIL_ADDRESS":"jsmith@example.com"}"#
                .to_string(),
        ),
        (
            "1003",
            r#"{"DATA_SOURCE":"CUSTOMERS","RECORD_ID":"1003","NAME_FULL":"Jane Doe","ADDR_FULL":"456 Oak Ave, Reno, NV 89501","PHONE_NUMBER":"775-555-8989"}"#
                .to_string(),
        ),
        (
            "1004",
            r#"{"DATA_SOURCE":"CUSTOMERS","RECORD_ID":"1004","NAME_FULL":"Robert Jones","ADDR_FULL":"789 Pine Rd, Carson City, NV 89701","PHONE_NUMBER":"775-555-4545"}"#
                .to_string(),
        ),
    ]
}

/// Registers the CUSTOMERS data source and activates the configuration in the
/// current (still-live) environment so records can be added to it.
fn register_data_source(env: &std::sync::Arc<SzEnvironmentCore>) -> SzResult<()> {
    let config_mgr = env.get_config_manager()?;
    let config = config_mgr.create_config()?;
    config.register_data_source(DATA_SOURCE)?;
    let definition = config.export()?;
    let config_id =
        config_mgr.set_default_config(&definition, Some("Snapshot test config with CUSTOMERS"))?;
    env.reinitialize(config_id)?;
    Ok(())
}

/// Exports the datastore and returns the resolved-entity groupings as a sorted
/// list of sorted record-id sets. Entity IDs are intentionally ignored (they may
/// renumber across a restore); the grouping is what must be preserved.
fn entity_groups(engine: &dyn SzEngine) -> SzResult<Vec<Vec<String>>> {
    let flags = SzFlags::EXPORT_INCLUDE_ALL_ENTITIES | SzFlags::ENTITY_INCLUDE_RECORD_DATA;
    let handle = engine.export_json_entity_report(Some(flags))?;
    let mut groups = Vec::new();
    loop {
        let chunk = engine.fetch_next(handle)?;
        if chunk.is_empty() {
            break;
        }
        let value: serde_json::Value = serde_json::from_str(&chunk)?;
        if let Some(records) = value
            .get("RESOLVED_ENTITY")
            .and_then(|e| e.get("RECORDS"))
            .and_then(|r| r.as_array())
        {
            let mut ids: Vec<String> = records
                .iter()
                .filter_map(|r| r.get("RECORD_ID").and_then(|v| v.as_str()))
                .map(str::to_string)
                .collect();
            ids.sort();
            groups.push(ids);
        }
    }
    engine.close_export_report(handle)?;
    groups.sort();
    Ok(groups)
}

fn unique_snapshot_path() -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sz_snapshot_test_{}_{}.szsnapshot",
        std::process::id(),
        nanos
    ))
}

/// Full round-trip: load records, snapshot, destroy the in-memory datastore,
/// restore into a fresh environment, and verify the resolution state matches.
#[test]
#[serial]
fn test_snapshot_restore_roundtrip() -> SzResult<()> {
    let snapshot_path = unique_snapshot_path();

    // --- Phase 1: producer environment ---------------------------------------
    let env = ExampleEnvironment::initialize("sz-snapshot-roundtrip-source")?;
    register_data_source(&env)?;

    let engine = env.get_engine()?;
    for (record_id, record) in test_records() {
        engine.add_record(DATA_SOURCE, record_id, &record, None)?;
    }

    let baseline_groups = entity_groups(engine.as_ref())?;
    assert_eq!(
        baseline_groups.len(),
        3,
        "expected 3 resolved entities, got {baseline_groups:?}"
    );
    assert!(
        baseline_groups.contains(&vec!["1001".to_string(), "1002".to_string()]),
        "records 1001 and 1002 should resolve together, got {baseline_groups:?}"
    );

    // Snapshot the live datastore, then tear everything down. With internal://
    // this destroys the in-memory data completely.
    env.export_datastore_snapshot(&snapshot_path)?;
    assert!(snapshot_path.exists(), "snapshot file should be written");
    drop(engine);
    ExampleEnvironment::cleanup(env)?;

    // --- Phase 2: consumer environment (fresh, empty datastore) ---------------
    let restored = ExampleEnvironment::initialize("sz-snapshot-roundtrip-restore")?;
    restored.import_datastore_snapshot(&snapshot_path)?;

    let restored_engine = restored.get_engine()?;
    let restored_groups = entity_groups(restored_engine.as_ref())?;

    assert_eq!(
        restored_groups, baseline_groups,
        "restored entity groupings must match the source datastore"
    );

    // Records are individually retrievable after restore.
    for (record_id, _) in test_records() {
        let record = restored_engine.get_record(DATA_SOURCE, record_id, None)?;
        assert!(
            record.contains(record_id),
            "restored record {record_id} should be retrievable"
        );
    }

    // A live search against the restored engine finds the resolved John Smith.
    let search = restored_engine.search_by_attributes(
        r#"{"NAME_FULL":"John Smith","ADDR_FULL":"123 Main St, Las Vegas, NV 89132"}"#,
        None,
        None,
    )?;
    assert!(
        search.contains("RESOLVED_ENTITY") || search.contains("ENTITY_ID"),
        "search should return the restored entity, got: {search}"
    );

    drop(restored_engine);
    ExampleEnvironment::cleanup(restored)?;
    let _ = std::fs::remove_file(&snapshot_path);
    Ok(())
}

/// Importing a file that is not a valid snapshot must fail with a clear error
/// rather than corrupting the datastore.
#[test]
#[serial]
fn test_import_rejects_invalid_snapshot() -> SzResult<()> {
    let bad_path = unique_snapshot_path();
    std::fs::write(&bad_path, "this is not a snapshot\n")
        .map_err(|e| SzError::bad_input(format!("write failed: {e}")))?;

    let env = ExampleEnvironment::initialize("sz-snapshot-invalid")?;
    let result = env.import_datastore_snapshot(&bad_path);
    assert!(result.is_err(), "importing garbage must fail");

    ExampleEnvironment::cleanup(env)?;
    let _ = std::fs::remove_file(&bad_path);
    Ok(())
}
