//! Senzing FFI Bindings Generator
//!
//! This script generates Rust FFI bindings from Senzing C headers using bindgen.
//! Run with: cargo run --example generate_bindings
//!
//! The generated file is committed to version control since the Senzing v4 API is stable.
//! Re-run this script when upgrading to a new Senzing SDK version.
//!
//! Environment variables:
//! - SENZING_SDK_PATH: Override the Senzing SDK root path

use std::env;
use std::path::{Path, PathBuf};

fn find_senzing_sdk_path() -> Option<PathBuf> {
    // Priority 1: Environment variable override
    if let Ok(path) = env::var("SENZING_SDK_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            println!("Using SENZING_SDK_PATH: {path}");
            return Some(p);
        }
        eprintln!("Warning: SENZING_SDK_PATH set but path doesn't exist: {path}");
    }

    // Priority 2: macOS Homebrew ARM
    let homebrew_arm = PathBuf::from("/opt/homebrew/opt/senzing/runtime/er");
    if homebrew_arm.exists() {
        println!("Found Senzing SDK at: {}", homebrew_arm.display());
        return Some(homebrew_arm);
    }

    // Priority 3: macOS Homebrew Intel
    let homebrew_intel = PathBuf::from("/usr/local/opt/senzing/runtime/er");
    if homebrew_intel.exists() {
        println!("Found Senzing SDK at: {}", homebrew_intel.display());
        return Some(homebrew_intel);
    }

    // Priority 4: Linux standard
    let linux = PathBuf::from("/opt/senzing/er");
    if linux.exists() {
        println!("Found Senzing SDK at: {}", linux.display());
        return Some(linux);
    }

    None
}

fn main() {
    let sdk_path = find_senzing_sdk_path().expect(
        "Could not find Senzing SDK. Set SENZING_SDK_PATH environment variable or install the SDK.",
    );

    let include_path = sdk_path.join("sdk/c");
    let helpers_path = include_path.join("szhelpers");

    // Verify headers exist
    let headers = [
        include_path.join("libSz.h"),
        helpers_path.join("SzLang_helpers.h"),
        include_path.join("libSzConfig.h"),
        include_path.join("libSzConfigMgr.h"),
        include_path.join("libSzDiagnostic.h"),
        include_path.join("libSzProduct.h"),
    ];

    for header in &headers {
        if !header.exists() {
            panic!("Header not found: {}", header.display());
        }
    }

    println!(
        "Generating bindings from headers in: {}",
        include_path.display()
    );

    // Create a wrapper header that includes all Senzing headers
    let wrapper_content = format!(
        r#"
#include "{}/libSz.h"
#include "{}/SzLang_helpers.h"
#include "{}/libSzConfig.h"
#include "{}/libSzConfigMgr.h"
#include "{}/libSzDiagnostic.h"
#include "{}/libSzProduct.h"
"#,
        include_path.display(),
        helpers_path.display(),
        include_path.display(),
        include_path.display(),
        include_path.display(),
        include_path.display(),
    );

    // Write wrapper to temp file
    let wrapper_path = std::env::temp_dir().join("senzing_wrapper.h");
    std::fs::write(&wrapper_path, &wrapper_content).expect("Failed to write wrapper header");

    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .clang_arg(format!("-I{}", include_path.display()))
        .clang_arg(format!("-I{}", helpers_path.display()))
        // Parse all functions
        .allowlist_function("Sz_.*")
        .allowlist_function("SzConfig_.*")
        .allowlist_function("SzConfigMgr_.*")
        .allowlist_function("SzDiagnostic_.*")
        .allowlist_function("SzProduct_.*")
        .allowlist_function("SzHelper_.*")
        // Parse all types/structs
        .allowlist_type("Sz.*")
        .allowlist_type("ExportHandle")
        // Generate derives
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        // Use core types
        .use_core()
        .ctypes_prefix("libc")
        // Layout tests can be noisy, skip them
        .layout_tests(false)
        // Generate the bindings
        .generate()
        .expect("Unable to generate bindings");

    // Determine output path
    let out_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("ffi")
        .join("bindings_generated.rs");

    println!("Writing bindings to: {}", out_path.display());

    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    // Clean up wrapper
    let _ = std::fs::remove_file(&wrapper_path);

    println!("Done! Generated bindings at: {}", out_path.display());
    println!("\nNext steps:");
    println!("1. Review the generated file");
    println!("2. Update src/ffi/mod.rs to include bindings_generated");
    println!("3. Run cargo build to verify");
}
