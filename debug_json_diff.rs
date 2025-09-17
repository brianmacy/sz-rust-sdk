use sz_rust_sdk::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() -> SzResult<()> {
    // Initialize environment
    let env = ExampleEnvironment::initialize("debug-json-diff")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    let config_mgr = env.get_config_manager()?;

    // Get baseline configuration
    let current_config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(current_config_id)?;
    let baseline_export = config.export()?;

    // Save baseline
    let mut baseline_file = File::create("baseline_config.json")?;
    baseline_file.write_all(baseline_export.as_bytes())?;
    println!("Saved baseline config to baseline_config.json ({} chars)", baseline_export.len());

    // Register one data source
    config.register_data_source("TEST_SOURCE")?;
    let modified_export = config.export()?;

    // Save modified
    let mut modified_file = File::create("modified_config.json")?;
    modified_file.write_all(modified_export.as_bytes())?;
    println!("Saved modified config to modified_config.json ({} chars)", modified_export.len());

    println!("Size difference: {} chars", modified_export.len() as i64 - baseline_export.len() as i64);
    println!("DSRC_CODE count - baseline: {}, modified: {}",
        baseline_export.matches("DSRC_CODE").count(),
        modified_export.matches("DSRC_CODE").count());

    ExampleEnvironment::cleanup()?;
    Ok(())
}