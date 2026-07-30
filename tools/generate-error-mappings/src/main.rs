//! Senzing Error Category Generator
//!
//! Generates the `SzErrorCategory` enum + `classify(code)` taxonomy from `szerrors.json`, via the
//! shared `sz_rust_sdk_ffi::codegen` generator (github.com/brianmacy/sz-rust-sdk-ffi) -- the same
//! generator Senzing's in-tree Rust binding layer (senzing-sys in the G2 repo) uses against its
//! own szerrors.json copy, so the code->category classification logic has exactly one source
//! instead of being independently re-implemented per SDK.
//!
//! `map_error_code`/`get_error_hierarchy` (in `../../src/error_category_bridge.rs`) are now
//! HAND-WRITTEN (not generated) -- a 13-arm exhaustive match over `SzErrorCategory` is far more
//! stable than a 456-arm per-code match, and the compiler enforces that a newly-added category
//! can't be silently ignored (no wildcard arm).
//!
//! Run with: cd tools/generate-error-mappings && cargo run (see README.md for why this is its own
//! standalone Cargo project, not an example of the main sz-rust-sdk crate).
//!
//! The generated file is committed to version control since the Senzing v4 error codes are stable.
//! Re-run this after upgrading to a new Senzing SDK version.
//!
//! Input: szerrors.json (repo root, SENZING_DIR, or an installed Senzing SDK)
//! Output: ../../src/error_category_generated.rs

use std::env;
use std::path::PathBuf;

/// The main sz-rust-sdk repo root, two levels up from this standalone tool crate.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn find_szerrors_json() -> Option<PathBuf> {
    // Priority 1: main repo root
    let project_root = repo_root().join("szerrors.json");
    if project_root.exists() {
        println!("Found szerrors.json in project root");
        return Some(project_root);
    }

    // Priority 2: SENZING_DIR env var (set by Scoop on Windows, or manual override)
    if let Ok(senzing_dir) = env::var("SENZING_DIR") {
        let p = PathBuf::from(&senzing_dir).join("sdk/szerrors.json");
        if p.exists() {
            println!("Found szerrors.json via SENZING_DIR");
            return Some(p);
        }
    }

    // Priority 3: G2 dev build directory
    if let Ok(home) = env::var("HOME") {
        let g2_dev = PathBuf::from(home).join("dev/G2/dev/build/dist/sdk/szerrors.json");
        if g2_dev.exists() {
            println!("Found szerrors.json in G2 dev build directory");
            return Some(g2_dev);
        }
    }

    // Priority 4: Official Homebrew cask on macOS (ARM)
    let homebrew_arm = PathBuf::from("/opt/homebrew/opt/senzing/er/sdk/szerrors.json");
    if homebrew_arm.exists() {
        println!("Found szerrors.json in Homebrew cask (ARM)");
        return Some(homebrew_arm);
    }

    // Priority 5: Official Homebrew cask on macOS (Intel)
    let homebrew_intel = PathBuf::from("/usr/local/opt/senzing/er/sdk/szerrors.json");
    if homebrew_intel.exists() {
        println!("Found szerrors.json in Homebrew cask (Intel)");
        return Some(homebrew_intel);
    }

    // Priority 6: Legacy unofficial Homebrew tap (ARM)
    let legacy_arm = PathBuf::from("/opt/homebrew/opt/senzing/runtime/sdk/szerrors.json");
    if legacy_arm.exists() {
        println!("Found szerrors.json in legacy Homebrew (ARM)");
        return Some(legacy_arm);
    }

    // Priority 7: Legacy unofficial Homebrew tap (Intel)
    let legacy_intel = PathBuf::from("/usr/local/opt/senzing/runtime/sdk/szerrors.json");
    if legacy_intel.exists() {
        println!("Found szerrors.json in legacy Homebrew (Intel)");
        return Some(legacy_intel);
    }

    // Priority 8: Linux standard
    let linux = PathBuf::from("/opt/senzing/er/sdk/szerrors.json");
    if linux.exists() {
        println!("Found szerrors.json in Linux standard path");
        return Some(linux);
    }

    None
}

fn main() {
    let szerrors_path = find_szerrors_json().expect(
        "Could not find szerrors.json. \
         Copy it to the sz-rust-sdk repo root, or ensure the Senzing SDK is installed.",
    );

    println!(
        "Reading error definitions from: {}",
        szerrors_path.display()
    );

    let out = sz_rust_sdk_ffi::codegen::generate_error_taxonomy(&szerrors_path)
        .unwrap_or_else(|e| panic!("error-taxonomy codegen failed: {e}"));

    let out_path = repo_root().join("src").join("error_category_generated.rs");
    std::fs::write(&out_path, out).expect("write error_category_generated.rs");

    let status = std::process::Command::new("rustfmt")
        .arg(&out_path)
        .status()
        .expect("Failed to run rustfmt - is it installed?");
    if !status.success() {
        eprintln!("Warning: rustfmt exited with {status}");
    }

    println!("\n✅ Successfully generated error category taxonomy!");
    println!("   Output: {}", out_path.display());
    println!("\nNext steps:");
    println!("1. Review the generated file");
    println!("2. If a NEW category appeared, update src/error_category_bridge.rs's exhaustive");
    println!("   match (it will fail to compile until you do -- that's the point)");
    println!("3. Run cargo build (from the repo root) to verify");
}
