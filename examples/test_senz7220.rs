//! Test SENZ7220 Error
//!
//! This example specifically tests for the SENZ7220 error to verify
//! that null bytes are properly handled in this error message.

use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Test SENZ7220 Error ===\n");

    // Use an isolated empty database that should connect but have no config
    // Generate a unique empty database path to avoid conflicts
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let empty_db_path = format!("/tmp/G2C_empty_test_{}.db", timestamp);

    let settings = format!(
        r#"{{"PIPELINE":{{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
        empty_db_path
    );

    println!("Testing with settings that should trigger SENZ7220...");

    match SzEnvironmentCore::new("test-senz7220", &settings, false) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Error occurred: {:?}", e);
            let error_string = format!("{}", e);
            println!("Error string: '{}'", error_string);
            println!("Error string length: {}", error_string.len());

            // Check for null bytes in the error message
            if error_string.contains('\0') {
                println!("❌ ERROR: Found literal null byte in error message!");
                for (i, ch) in error_string.chars().enumerate() {
                    if ch == '\0' {
                        println!("   Null byte found at position {}", i);
                    }
                }

                // Show raw bytes
                println!("Raw error message bytes: {:?}", error_string.as_bytes());
            } else {
                println!("✅ No null bytes found in error message");
            }

            // Check if this is the specific SENZ7220 error
            if error_string.contains("SENZ7220") {
                println!("✅ This is the SENZ7220 error we're looking for");
                if error_string.ends_with('\0') {
                    println!("❌ ERROR: String ends with null byte!");
                } else {
                    println!("✅ String does not end with null byte");
                }
            }
        }
    }

    // Clean up the empty database file if it was created
    if std::path::Path::new(&empty_db_path).exists() {
        let _ = std::fs::remove_file(&empty_db_path);
        println!("Cleaned up empty test database: {}", empty_db_path);
    }

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
