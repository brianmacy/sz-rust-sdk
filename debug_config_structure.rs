use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize environment to get baseline configuration
    let env = ExampleEnvironment::initialize("debug-config-structure")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    let config_mgr = env.get_config_manager()?;

    // Get the baseline configuration
    let config_id = config_mgr.get_default_config_id()?;
    let baseline_config = config_mgr.create_config_from_id(config_id)?;
    let baseline_export = baseline_config.export()?;

    println!("=== Baseline Configuration Analysis ===");
    println!("Size: {} characters", baseline_export.len());

    // Look for data source entries in the baseline
    if let Some(start) = baseline_export.find("\"CFG_DSRC\"") {
        let excerpt = &baseline_export[start..std::cmp::min(start + 2000, baseline_export.len())];
        println!("\nBaseline CFG_DSRC section:");
        println!("{}", excerpt);
    }

    // Try registering a data source manually and see what changes
    println!("\n=== Registering TEST_DS and checking structure ===");
    let result = baseline_config.register_data_source("TEST_DS")?;
    println!("Registration result: {}", result);

    let updated_export = baseline_config.export()?;
    println!("Updated size: {} characters", updated_export.len());

    // Look for the new data source structure
    if let Some(start) = updated_export.find("TEST_DS") {
        let before_start = std::cmp::max(start.saturating_sub(200), 0);
        let after_end = std::cmp::min(start + 400, updated_export.len());
        let excerpt = &updated_export[before_start..after_end];
        println!("\nTEST_DS registration structure:");
        println!("{}", excerpt);
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}