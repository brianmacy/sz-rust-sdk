use std::env;
use std::path::Path;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // Priority: SENZING_LIB_PATH env var > Homebrew location > default location
    let senzing_lib_path = env::var("SENZING_LIB_PATH")
        .ok()
        .or_else(|| {
            // Check Homebrew location (macOS)
            let homebrew_path = "/opt/homebrew/opt/senzing/runtime/er/lib";
            if Path::new(homebrew_path).join("libSz.dylib").exists() {
                return Some(homebrew_path.to_string());
            }
            // Check Intel Homebrew location
            let intel_homebrew_path = "/usr/local/opt/senzing/runtime/er/lib";
            if Path::new(intel_homebrew_path).join("libSz.dylib").exists() {
                return Some(intel_homebrew_path.to_string());
            }
            None
        })
        .unwrap_or_else(|| "/opt/senzing/er/lib".to_string());

    println!("cargo:rustc-link-search=native={}", senzing_lib_path);

    // Link against the Senzing library
    println!("cargo:rustc-link-lib=dylib=Sz");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SENZING_LIB_PATH");
}
