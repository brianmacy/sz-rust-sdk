use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("debug-existing-datasources")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    let config_mgr = env.get_config_manager()?;

    let config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(config_id)?;

    // Get the data source registry
    match config.get_data_source_registry() {
        Ok(registry) => {
            println!("Data Source Registry:");
            println!("{}", registry);
        }
        Err(e) => {
            println!("Failed to get data source registry: {}", e);
        }
    }

    // Also check the full export for DSRC_CODE entries
    let export = config.export()?;
    let dsrc_count = export.matches("DSRC_CODE").count();
    println!("\nFound {} DSRC_CODE entries in configuration", dsrc_count);

    if let Some(dsrc_start) = export.find("DSRC_CODE") {
        let excerpt_start = dsrc_start.saturating_sub(100);
        let excerpt_end = (dsrc_start + 500).min(export.len());
        println!("\nExample DSRC_CODE structure:");
        println!("{}", &export[excerpt_start..excerpt_end]);
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}