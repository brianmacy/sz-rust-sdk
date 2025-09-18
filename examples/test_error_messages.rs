//! Test Error Messages
//!
//! This example tests error message handling to verify that
//! null bytes are properly excluded from error messages.

use sz_rust_sdk::prelude::*;
use sz_rust_sdk::helpers::ExampleEnvironment;

fn main() -> SzResult<()> {
    println!("=== Test Error Messages ===\n");

    // Try to create an environment that will fail to generate an error message
    let bad_settings = r#"{"INVALID": "JSON", "MALFORMED"}"#;

    println!("Testing with malformed JSON settings:");
    println!("Settings: {}", bad_settings);

    match SzEnvironmentCore::new("test-error", bad_settings, true) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Error occurred: {:?}", e);
            let error_string = format!("{}", e);
            println!("Error string: '{}'", error_string);

            // Check for null bytes in the error message
            if error_string.contains('\0') {
                println!("❌ ERROR: Found literal null byte in error message!");
                for (i, ch) in error_string.chars().enumerate() {
                    if ch == '\0' {
                        println!("   Null byte found at position {}", i);
                    }
                }
            } else {
                println!("✅ No null bytes found in error message");
            }
        }
    }

    println!("\n=== Testing with completely invalid config ===");

    let invalid_settings = "not json at all";

    match SzEnvironmentCore::new("test-error2", invalid_settings, true) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Error occurred: {:?}", e);
            let error_string = format!("{}", e);
            println!("Error string: '{}'", error_string);

            // Check for null bytes in the error message
            if error_string.contains('\0') {
                println!("❌ ERROR: Found literal null byte in error message!");
            } else {
                println!("✅ No null bytes found in error message");
            }
        }
    }

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
