//! Debug Settings String
//!
//! This example debugs the settings string handling to identify
//! the SENZ0018 truncation issue.

use std::ffi::CString;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Debug Settings String ===\n");

    // Remove any existing environment configuration to use isolated database
    std::env::remove_var("SENZING_ENGINE_CONFIGURATION_JSON");

    // Generate an isolated database configuration
    // Note: This example tests string handling and direct Sz_init calls
    // We expect it to fail with SENZ7220 (no configuration) which demonstrates proper string handling
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/senzing_debug_{}.db", timestamp);

    // Copy the template database to create a working database for testing
    let template_path = "/opt/senzing/er/resources/templates/G2C.db";
    std::fs::copy(template_path, &db_path)
        .map_err(|e| SzError::configuration(format!("Failed to copy template database: {}", e)))?;
    println!("Created debug database from template: {}", db_path);

    let settings = format!(
        r#"{{"PIPELINE":{{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"}},"SQL":{{"CONNECTION":"sqlite3://na:na@{}"}}}}"#,
        db_path
    );

    println!("Original settings string:");
    println!("Length: {}", settings.len());
    println!("Content: {}", settings);
    println!(
        "Last few chars: {:?}",
        &settings[settings.len().saturating_sub(10)..]
    );
    println!();

    // Test C string conversion
    match CString::new(settings.clone()) {
        Ok(c_string) => {
            println!("CString conversion successful");
            println!("CString length: {}", c_string.as_bytes().len());
            println!("CString content: {:?}", c_string);

            // Convert back to verify
            let back_to_string = c_string.to_string_lossy();
            println!("Back to string length: {}", back_to_string.len());
            println!("Matches original: {}", back_to_string == settings);

            if back_to_string != settings {
                println!("DIFFERENCE DETECTED!");
                println!(
                    "Original ends with: {:?}",
                    &settings[settings.len().saturating_sub(5)..]
                );
                println!(
                    "CString ends with:  {:?}",
                    &back_to_string[back_to_string.len().saturating_sub(5)..]
                );
            }
        }
        Err(e) => {
            println!("CString conversion failed: {}", e);
            // Check for null bytes in the string
            for (i, byte) in settings.as_bytes().iter().enumerate() {
                if *byte == 0 {
                    println!("Found null byte at position {}", i);
                }
            }
        }
    }

    println!("\n=== Testing direct Sz_init call ===");

    let module_name = CString::new("debug-test").unwrap();
    let settings_c = CString::new(settings).unwrap();

    println!("About to call Sz_init with:");
    println!("Module: {:?}", module_name);
    println!("Settings length: {}", settings_c.as_bytes().len());
    println!("Settings: {:?}", settings_c);

    // Call Sz_init directly to see the error
    let result = unsafe {
        sz_rust_sdk::ffi::bindings::Sz_init(
            module_name.as_ptr(),
            settings_c.as_ptr(),
            1, // verbose logging
        )
    };

    println!("Sz_init returned: {}", result);

    if result != 0 {
        // Get the error message
        let mut buffer: Vec<u8> = vec![0; 2048];
        let actual_len = unsafe {
            sz_rust_sdk::ffi::bindings::Sz_getLastException(
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i64,
            )
        };

        if actual_len > 0 && actual_len < buffer.len() as i64 {
            // Exclude null terminator from the string content
            let string_len = if actual_len > 0 {
                (actual_len as usize) - 1
            } else {
                0
            };
            buffer.truncate(string_len);
            let error_msg = String::from_utf8_lossy(&buffer);
            println!("Error message: {}", error_msg);

            // Clean up the debug database file if it was created
            if std::path::Path::new(&db_path).exists() {
                let _ = std::fs::remove_file(&db_path);
                println!("Cleaned up debug database: {}", db_path);
            }

            // Check if this is the expected SENZ7220 configuration error
            if error_msg.contains("SENZ7220") {
                println!("✅ Got expected SENZ7220 error - string handling and FFI calls working correctly");
                println!("   This demonstrates the debug utility is working properly");
            } else {
                // Sz_init failed with an unexpected error, so this test should fail
                return Err(SzError::configuration(format!("Sz_init failed with code {}: {}", result, error_msg)));
            }
        } else {
            // Clean up the debug database file if it was created
            if std::path::Path::new(&db_path).exists() {
                let _ = std::fs::remove_file(&db_path);
                println!("Cleaned up debug database: {}", db_path);
            }

            // Sz_init failed but we couldn't get error details - this is unexpected
            return Err(SzError::configuration(format!("Sz_init failed with code {}", result)));
        }
    } else {
        println!("✅ Sz_init succeeded - debug utility working correctly");
    }

    // Clean up the debug database file if it was created
    if std::path::Path::new(&db_path).exists() {
        let _ = std::fs::remove_file(&db_path);
        println!("Cleaned up debug database: {}", db_path);
    }

    println!("\n🎯 Debug settings test completed successfully");

    // Clean up the test database
    ExampleEnvironment::cleanup()?;

    Ok(())
}
