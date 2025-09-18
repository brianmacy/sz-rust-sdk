//! Minimal test to isolate segfault issue
//!
//! This example performs the absolute minimum to test if basic operations work
//! without triggering segfaults.

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Minimal Senzing Test ===");

    // Initialize environment using the helper utility
    println!("1. Creating environment...");
    let env = ExampleEnvironment::initialize("minimal-test")?;
    println!("✅ Environment created successfully");

    // Get engine (this works reliably)
    println!("2. Getting engine...");
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    println!("✅ Engine obtained successfully");

    // Perform ONE simple search operation
    println!("3. Testing simple search...");
    let search_attrs = r#"{"NAME_LAST": "Test"}"#;
    match engine.search_by_attributes(search_attrs, None, None) {
        Ok(results) => {
            println!("✅ Search successful");
            println!("   Results: {}", results);
        }
        Err(e) => println!("⚠️  Search failed: {}", e),
    }

    println!("4. Test complete - exiting cleanly");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
