use std::env;
#[cfg(target_os = "macos")]
use std::path::Path;

fn main() {
    // Platform-specific library name and default path
    let (lib_name, default_path) = if cfg!(target_os = "windows") {
        ("Sz", "C:\\Program Files\\Senzing\\er\\lib")
    } else {
        ("Sz", "/opt/senzing/er/lib")
    };

    // Tell cargo to look for shared libraries in the specified directory
    // Priority: SENZING_LIB_PATH env var > platform-specific defaults
    #[cfg(target_os = "macos")]
    let senzing_lib_path = env::var("SENZING_LIB_PATH")
        .ok()
        .or_else(|| {
            // Official cask (senzing/senzingsdk) — no "runtime" subdirectory
            let homebrew_path = "/opt/homebrew/opt/senzing/er/lib";
            if Path::new(homebrew_path).join("libSz.dylib").exists() {
                return Some(homebrew_path.to_string());
            }
            let intel_homebrew_path = "/usr/local/opt/senzing/er/lib";
            if Path::new(intel_homebrew_path).join("libSz.dylib").exists() {
                return Some(intel_homebrew_path.to_string());
            }
            // Legacy unofficial tap — "runtime" subdirectory
            let legacy_homebrew = "/opt/homebrew/opt/senzing/runtime/er/lib";
            if Path::new(legacy_homebrew).join("libSz.dylib").exists() {
                return Some(legacy_homebrew.to_string());
            }
            let legacy_intel = "/usr/local/opt/senzing/runtime/er/lib";
            if Path::new(legacy_intel).join("libSz.dylib").exists() {
                return Some(legacy_intel.to_string());
            }
            None
        })
        .unwrap_or_else(|| default_path.to_string());

    #[cfg(not(target_os = "macos"))]
    let senzing_lib_path = env::var("SENZING_LIB_PATH")
        .ok()
        .unwrap_or_else(|| default_path.to_string());

    println!("cargo:rustc-link-search=native={senzing_lib_path}");

    // Link against the Senzing library
    println!("cargo:rustc-link-lib=dylib={lib_name}");

    // macOS 4.3 cask is missing rpath entries for openssl and sqlite3 —
    // add Homebrew lib paths so the linker can resolve them at build time.
    #[cfg(target_os = "macos")]
    {
        if let Some(prefix) = homebrew_prefix() {
            for formula in ["sqlite", "openssl@3"] {
                let lib_dir = format!("{prefix}/opt/{formula}/lib");
                if Path::new(&lib_dir).exists() {
                    println!("cargo:rustc-link-search=native={lib_dir}");
                }
            }
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SENZING_LIB_PATH");
}

#[cfg(target_os = "macos")]
fn homebrew_prefix() -> Option<String> {
    for prefix in ["/opt/homebrew", "/usr/local"] {
        if Path::new(prefix).join("bin/brew").exists() {
            return Some(prefix.to_string());
        }
    }
    None
}
