//! Datastore snapshot / restore
//!
//! The `internal://` datastore is in-memory only, so its contents are lost when
//! the process exits. This example shows how to persist the datastore to a single
//! portable file at the end of one run (the "producer") and warm-start a later run
//! (the "consumer") from that file without re-ingesting the source data.
//!
//! Run with: cargo run --example snapshot_restore

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

const DATA_SOURCE: &str = "TEST";

/// Sample records. 1001 and 1002 resolve together; 1003 is distinct.
fn records() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "1001",
            r#"{"DATA_SOURCE":"TEST","RECORD_ID":"1001","NAME_FULL":"Robert Smith","PHONE_NUMBER":"555-1212","ADDR_FULL":"123 Main Street, Anytown, ST 12345"}"#,
        ),
        (
            "1002",
            r#"{"DATA_SOURCE":"TEST","RECORD_ID":"1002","NAME_FULL":"Bob J Smith","PHONE_NUMBER":"555-1212","ADDR_FULL":"123 Main St, Anytown, ST 12345"}"#,
        ),
        (
            "1003",
            r#"{"DATA_SOURCE":"TEST","RECORD_ID":"1003","NAME_FULL":"John Doe","PHONE_NUMBER":"555-9876","ADDR_FULL":"456 Oak Avenue, Another City, ST 67890"}"#,
        ),
    ]
}

fn main() -> SzResult<()> {
    let snapshot_path =
        std::env::temp_dir().join(format!("sz_demo_{}.szsnapshot", std::process::id()));

    // --- Producer: load records and snapshot the datastore -------------------
    println!("=== Producer: loading records and taking a snapshot ===");
    let env = ExampleEnvironment::initialize("snapshot-restore-producer")?;
    let engine = env.get_engine()?;
    for (record_id, record) in records() {
        engine.add_record(DATA_SOURCE, record_id, record, None)?;
    }
    println!("Loaded {} records", records().len());

    env.export_datastore_snapshot(&snapshot_path)?;
    println!("Snapshot written to {}", snapshot_path.display());

    // Tear down the environment. With internal:// this destroys the in-memory
    // datastore completely — the data now lives only in the snapshot file.
    drop(engine);
    ExampleEnvironment::cleanup(env)?;

    // --- Consumer: warm-start from the snapshot ------------------------------
    println!("\n=== Consumer: restoring from the snapshot ===");
    let env = ExampleEnvironment::initialize("snapshot-restore-consumer")?;
    env.import_datastore_snapshot(&snapshot_path)?;
    println!("Datastore restored from {}", snapshot_path.display());

    let engine = env.get_engine()?;

    // The restored engine is fully live: search and why work without re-ingesting.
    let search = engine.search_by_attributes(
        r#"{"NAME_FULL":"Robert Smith","ADDR_FULL":"123 Main Street, Anytown, ST 12345"}"#,
        None,
        None,
    )?;
    println!(
        "Search against the restored engine returned {} bytes",
        search.len()
    );

    let record = engine.get_record(DATA_SOURCE, "1001", None)?;
    println!("Restored record 1001: {record}");

    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    let _ = std::fs::remove_file(&snapshot_path);
    println!("\n✅ Snapshot / restore round-trip complete");
    Ok(())
}
