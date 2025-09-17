//! Debug Config JSON Example
//!
//! This example examines the JSON format differences between what
//! SzConfigMgr_getConfig_helper returns and what SzConfig_load_helper expects.

use sz_rust_sdk::ffi::{bindings, helpers};
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize environment
    let env = ExampleEnvironment::initialize("debug-config-json")?;
    let _engine = ExampleEnvironment::get_engine_with_setup(&env)?;
    let config_mgr = env.get_config_manager()?;

    // Get the default config ID
    let config_id = config_mgr.get_default_config_id()?;
    println!("Config ID: {}", config_id);

    // Try to get the config JSON directly
    let result = unsafe { bindings::SzConfigMgr_getConfig_helper(config_id) };

    if result.return_code != 0 {
        println!("Error getting config: {}", result.return_code);
        return Ok(());
    }

    let config_json = unsafe { helpers::c_str_to_string(result.response) }?;

    println!("=== JSON from SzConfigMgr_getConfig_helper ===");
    println!("Length: {}", config_json.len());
    println!(
        "First 500 chars: {}",
        &config_json[..config_json.len().min(500)]
    );
    println!(
        "Last 200 chars: {}",
        &config_json[config_json.len().saturating_sub(200)..]
    );

    // Check if it's valid JSON
    match serde_json::from_str::<serde_json::Value>(&config_json) {
        Ok(parsed) => {
            println!("✅ Valid JSON structure");
            if let Some(obj) = parsed.as_object() {
                println!("Top-level keys: {:?}", obj.keys().collect::<Vec<_>>());
            }
        }
        Err(e) => {
            println!("❌ Invalid JSON: {}", e);
        }
    }

    // Now try to create a new config and load this JSON
    println!("\n=== Testing SzConfig_load_helper ===");

    // Create empty config
    let create_result = unsafe { bindings::SzConfig_create_helper() };
    if create_result.return_code != 0 {
        println!("Error creating config: {}", create_result.return_code);
        return Ok(());
    }

    println!("Created empty config handle");

    // Try to load the JSON directly (new API)
    let config_def_c = helpers::str_to_c_string(&config_json)?;
    let load_result = unsafe { bindings::SzConfig_load_helper(config_def_c.as_ptr()) };

    if load_result.return_code == 0 {
        println!("✅ SzConfig_load_helper succeeded");
    } else {
        println!(
            "❌ SzConfig_load_helper failed with code: {}",
            load_result.return_code
        );

        // Get the actual error message
        let mut error_buffer = vec![0u8; 1024];
        let error_len = unsafe {
            bindings::SzConfig_getLastException(error_buffer.as_mut_ptr() as *mut i8, 1024)
        };

        if error_len > 0 {
            let error_msg = String::from_utf8_lossy(&error_buffer[..error_len as usize]);
            println!("Error details: {}", error_msg);
        }
    }

    // Compare with what export returns
    println!("\n=== Comparing with working config export ===");
    let working_config = config_mgr.create_config()?;
    let export_json = working_config.export()?;

    println!("Export JSON length: {}", export_json.len());
    println!(
        "First 500 chars: {}",
        &export_json[..export_json.len().min(500)]
    );

    // Clean up
    unsafe {
        bindings::SzConfig_close_helper(create_result.response);
    }

    ExampleEnvironment::cleanup()?;
    Ok(())
}
