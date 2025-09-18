//! Basic Senzing SDK operations: search, path finding, and network analysis

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("basic-usage")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Search for entities
    let search_attributes = r#"{"NAME_LAST": "Smith", "NAME_FIRST": "John"}"#;
    match engine.search_by_attributes(search_attributes, None, None) {
        Ok(results) => println!("Search results: {}", results),
        Err(e) => println!("Search failed: {}", e),
    }

    // Find path between entities
    match engine.find_path(1, 2, 3, None, None, None) {
        Ok(path_result) => println!("Path: {}", path_result),
        Err(e) => println!("Find path failed: {} (expected - no entities loaded)", e),
    }

    // Network analysis
    match engine.find_network(&[1, 2, 3], 2, 1, 10, None) {
        Ok(network_result) => println!("Network: {}", network_result),
        Err(e) => println!(
            "Network analysis failed: {} (expected - no entities loaded)",
            e
        ),
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}
