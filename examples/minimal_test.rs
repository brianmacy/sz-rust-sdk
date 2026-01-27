//! Minimal Senzing SDK test - basic operations without segfaults

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Guard automatically cleans up when it goes out of scope
    let env = SenzingGuard::from_env(ExampleEnvironment::initialize("minimal-test")?);
    let engine = env.get_engine()?;

    let search_attrs = r#"{"NAME_LAST": "Test"}"#;
    match engine.search_by_attributes(search_attrs, None, None) {
        Ok(results) => println!("Search results: {}", results),
        Err(e) => println!("Search failed: {}", e),
    }

    // Cleanup happens automatically when env guard goes out of scope
    Ok(())
}
