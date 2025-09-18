//! Minimal Senzing SDK test - basic operations without segfaults

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("minimal-test")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let search_attrs = r#"{"NAME_LAST": "Test"}"#;
    match engine.search_by_attributes(search_attrs, None, None) {
        Ok(results) => println!("Search results: {}", results),
        Err(e) => println!("Search failed: {}", e),
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}
