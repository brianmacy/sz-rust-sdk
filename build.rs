use std::env;
use std::path::Path;

fn main() {
    let lib_name = "Sz";

    // Priority: SENZING_LIB_PATH > SENZING_DIR > platform-specific auto-detection
    let senzing_lib_path = env::var("SENZING_LIB_PATH")
        .ok()
        .or_else(|| env::var("SENZING_DIR").ok().map(|d| format!("{d}/lib")))
        .or_else(detect_senzing_lib)
        .unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "C:\\Program Files\\Senzing\\er\\lib".to_string()
            } else {
                "/opt/senzing/er/lib".to_string()
            }
        });

    println!("cargo:rustc-link-search=native={senzing_lib_path}");
    println!("cargo:rustc-link-lib=dylib={lib_name}");

    // macOS 4.3 cask is missing rpath entries for openssl and sqlite3
    if cfg!(target_os = "macos")
        && let Some(prefix) = homebrew_prefix()
    {
        for formula in ["sqlite", "openssl@3"] {
            let lib_dir = format!("{prefix}/opt/{formula}/lib");
            if Path::new(&lib_dir).exists() {
                println!("cargo:rustc-link-search=native={lib_dir}");
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SENZING_LIB_PATH");
    println!("cargo:rerun-if-env-changed=SENZING_DIR");
}

fn detect_senzing_lib() -> Option<String> {
    // macOS: Homebrew official cask, then legacy unofficial tap
    for base in [
        "/opt/homebrew/opt/senzing/er/lib",
        "/usr/local/opt/senzing/er/lib",
        "/opt/homebrew/opt/senzing/runtime/er/lib",
        "/usr/local/opt/senzing/runtime/er/lib",
    ] {
        if Path::new(base).exists() {
            return Some(base.to_string());
        }
    }
    // Linux standard
    let linux = "/opt/senzing/er/lib";
    if Path::new(linux).exists() {
        return Some(linux.to_string());
    }
    None
}

fn homebrew_prefix() -> Option<String> {
    for prefix in ["/opt/homebrew", "/usr/local"] {
        if Path::new(prefix).join("bin/brew").exists() {
            return Some(prefix.to_string());
        }
    }
    None
}
