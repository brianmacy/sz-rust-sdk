//! Senzing Error Mappings Generator
//!
//! This script generates Rust error code mappings and hierarchy from szerrors.json.
//! Run with: cargo run --example generate_error_mappings
//!
//! The generated file is committed to version control since the Senzing v4 error codes are stable.
//! Re-run this script when upgrading to a new Senzing SDK version.
//!
//! Input: szerrors.json (from Senzing SDK distribution or ~/dev/G2/dev/build/dist/sdk/)
//! Output: src/error/mappings_generated.rs

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn find_szerrors_json() -> Option<PathBuf> {
    // Priority 1: Project root
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("szerrors.json");
    if project_root.exists() {
        println!("Found szerrors.json in project root");
        return Some(project_root);
    }

    // Priority 2: G2 dev build directory
    let g2_dev =
        PathBuf::from(env::var("HOME").unwrap()).join("dev/G2/dev/build/dist/sdk/szerrors.json");
    if g2_dev.exists() {
        println!("Found szerrors.json in G2 dev build directory");
        return Some(g2_dev);
    }

    // Priority 3: Homebrew on macOS (ARM)
    let homebrew_arm = PathBuf::from("/opt/homebrew/opt/senzing/runtime/sdk/szerrors.json");
    if homebrew_arm.exists() {
        println!("Found szerrors.json in Homebrew (ARM)");
        return Some(homebrew_arm);
    }

    // Priority 4: Homebrew on macOS (Intel)
    let homebrew_intel = PathBuf::from("/usr/local/opt/senzing/runtime/sdk/szerrors.json");
    if homebrew_intel.exists() {
        println!("Found szerrors.json in Homebrew (Intel)");
        return Some(homebrew_intel);
    }

    None
}

fn main() {
    let szerrors_path = find_szerrors_json().expect(
        "Could not find szerrors.json. \
         Copy it from ~/dev/G2/dev/build/dist/sdk/szerrors.json to the project root, \
         or ensure Senzing SDK is installed.",
    );

    println!(
        "Reading error definitions from: {}",
        szerrors_path.display()
    );

    let json_content = fs::read_to_string(&szerrors_path).expect("Failed to read szerrors.json");

    let errors: HashMap<String, serde_json::Value> =
        serde_json::from_str(&json_content).expect("Failed to parse szerrors.json");

    println!("Parsed {} error definitions", errors.len());

    // Collect and sort errors by code number
    let mut code_map: Vec<(i64, String, String)> = errors
        .iter()
        .filter_map(|(code_str, value)| {
            let code = code_str.parse::<i64>().ok()?;
            let class = value.get("class")?.as_str()?.to_string();
            let comment = value.get("comment")?.as_str().unwrap_or("").to_string();
            Some((code, class, comment))
        })
        .collect();

    code_map.sort_by_key(|(code, _, _)| *code);

    // Output path (sibling to error.rs)
    let out_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error_mappings_generated.rs");

    let mut file = fs::File::create(&out_path).expect("Failed to create output file");

    // Write file header
    writeln!(file, "// Auto-generated from szerrors.json").unwrap();
    writeln!(file, "// DO NOT EDIT MANUALLY").unwrap();
    writeln!(file, "//").unwrap();
    writeln!(
        file,
        "// Regenerate with: cargo run --example generate_error_mappings"
    )
    .unwrap();
    writeln!(file, "//").unwrap();
    writeln!(
        file,
        "// This file contains {} Senzing error code mappings",
        code_map.len()
    )
    .unwrap();
    writeln!(file).unwrap();
    writeln!(
        file,
        "use crate::error::{{ErrorCategory, ErrorContext, SzError}};"
    )
    .unwrap();
    writeln!(file).unwrap();

    // Generate map_error_code function
    writeln!(file, "/// Maps Senzing error codes to SzError types").unwrap();
    writeln!(file, "///").unwrap();
    writeln!(
        file,
        "/// This function is auto-generated from szerrors.json and maps each"
    )
    .unwrap();
    writeln!(
        file,
        "/// native Senzing error code to the appropriate Rust error variant."
    )
    .unwrap();
    writeln!(file, "#[allow(clippy::too_many_lines)]").unwrap();
    writeln!(
        file,
        "pub(super) fn map_error_code(error_code: i64, ctx: ErrorContext) -> SzError {{"
    )
    .unwrap();
    writeln!(file, "    match error_code {{").unwrap();

    for (code, class, _comment) in &code_map {
        let error_variant = match class.as_str() {
            "SzBadInputError" => "SzError::BadInput",
            "SzNotFoundError" => "SzError::NotFound",
            "SzUnknownDataSourceError" => "SzError::UnknownDataSource",
            "SzConfigurationError" => "SzError::Configuration",
            "SzDatabaseConnectionLostError" => "SzError::DatabaseConnectionLost",
            "SzDatabaseTransientError" => "SzError::DatabaseTransient",
            "SzRetryTimeoutExceededError" => "SzError::RetryTimeoutExceeded",
            "SzDatabaseError" => "SzError::Database",
            "SzLicenseError" => "SzError::License",
            "SzNotInitializedError" => "SzError::NotInitialized",
            "SzUnhandledError" => "SzError::Unhandled",
            "SzReplaceConflictError" => "SzError::ReplaceConflict",
            "SzError" => "SzError::Unknown",
            _ => "SzError::Unknown",
        };
        writeln!(file, "        {} => {}(ctx),", code, error_variant).unwrap();
    }

    writeln!(file, "        _ => SzError::Unknown(ctx),").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    // Generate get_error_hierarchy function
    writeln!(
        file,
        "/// Returns the error hierarchy for a given error code"
    )
    .unwrap();
    writeln!(file, "///").unwrap();
    writeln!(
        file,
        "/// This function returns the category chain for an error code,"
    )
    .unwrap();
    writeln!(
        file,
        "/// from most specific to most general (e.g., [DatabaseTransient, Retryable])."
    )
    .unwrap();
    writeln!(file, "#[allow(clippy::too_many_lines)]").unwrap();
    writeln!(
        file,
        "pub(super) fn get_error_hierarchy(error_code: i64) -> Vec<ErrorCategory> {{"
    )
    .unwrap();
    writeln!(file, "    match error_code {{").unwrap();

    for (code, class, _comment) in &code_map {
        let hierarchy = match class.as_str() {
            "SzNotFoundError" => "vec![ErrorCategory::NotFound, ErrorCategory::BadInput]",
            "SzUnknownDataSourceError" => {
                "vec![ErrorCategory::UnknownDataSource, ErrorCategory::BadInput]"
            }
            "SzBadInputError" => "vec![ErrorCategory::BadInput]",
            "SzDatabaseConnectionLostError" => {
                "vec![ErrorCategory::DatabaseConnectionLost, ErrorCategory::Retryable]"
            }
            "SzDatabaseTransientError" => {
                "vec![ErrorCategory::DatabaseTransient, ErrorCategory::Retryable]"
            }
            "SzRetryTimeoutExceededError" => {
                "vec![ErrorCategory::RetryTimeoutExceeded, ErrorCategory::Retryable]"
            }
            "SzDatabaseError" => "vec![ErrorCategory::Database, ErrorCategory::Unrecoverable]",
            "SzLicenseError" => "vec![ErrorCategory::License, ErrorCategory::Unrecoverable]",
            "SzNotInitializedError" => {
                "vec![ErrorCategory::NotInitialized, ErrorCategory::Unrecoverable]"
            }
            "SzUnhandledError" => "vec![ErrorCategory::Unhandled, ErrorCategory::Unrecoverable]",
            "SzConfigurationError" => "vec![ErrorCategory::Configuration]",
            "SzReplaceConflictError" => "vec![ErrorCategory::ReplaceConflict]",
            _ => "vec![]",
        };
        writeln!(file, "        {} => {},", code, hierarchy).unwrap();
    }

    writeln!(file, "        _ => vec![],").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    println!("\n✅ Successfully generated error mappings!");
    println!("   Output: {}", out_path.display());
    println!("   Mapped {} error codes", code_map.len());
    println!("\nNext steps:");
    println!("1. Review the generated file");
    println!("2. Ensure src/error/mod.rs includes: mod mappings_generated;");
    println!("3. Update from_code_with_message() to use map_error_code()");
    println!("4. Run cargo build to verify");
}
