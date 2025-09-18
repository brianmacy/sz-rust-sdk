use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize environment
    let env = ExampleEnvironment::initialize_verbose("quick-datasource-test")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    let config_mgr = env.get_config_manager()?;

    // Get current config
    let config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(config_id)?;

    println!("=== Before Registration ===");
    let before_export = config.export()?;
    let before_count = before_export.matches("DSRC_CODE").count();
    println!("DSRC_CODE entries before: {}", before_count);

    // Register a data source
    println!("\n=== Registering Data Source ===");
    let result = config.register_data_source("QUICK_TEST")?;
    println!("Registration result: {}", result);

    // Check after registration
    println!("\n=== After Registration ===");
    let after_export = config.export()?;
    let after_count = after_export.matches("DSRC_CODE").count();
    println!("DSRC_CODE entries after: {}", after_count);
    println!(
        "Size change: {} chars",
        after_export.len() as i64 - before_export.len() as i64
    );

    // Look for our data source in the export
    if after_export.contains("QUICK_TEST") {
        println!("✅ QUICK_TEST found in configuration export");
    } else {
        println!("❌ QUICK_TEST NOT found in configuration export");
    }

    // Try to register and persist
    println!("\n=== Persisting Configuration ===");
    let new_config_id = config_mgr.register_config(&after_export, Some("Added QUICK_TEST"))?;
    println!("New config ID: {}", new_config_id);

    // Immediately retrieve and check
    let retrieved_config = config_mgr.create_config_from_id(new_config_id)?;
    let retrieved_export = retrieved_config.export()?;
    let retrieved_count = retrieved_export.matches("DSRC_CODE").count();
    println!("Retrieved DSRC_CODE entries: {}", retrieved_count);

    if retrieved_export.contains("QUICK_TEST") {
        println!("✅ QUICK_TEST persisted successfully");
    } else {
        println!("❌ QUICK_TEST lost during persistence");
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}
