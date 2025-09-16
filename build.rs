use std::env;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let senzing_lib_path =
        env::var("SENZING_LIB_PATH").unwrap_or_else(|_| "/opt/senzing/er/lib".to_string());

    println!("cargo:rustc-link-search=native={}", senzing_lib_path);

    // Link against the Senzing library
    println!("cargo:rustc-link-lib=dylib=Sz");

    // Set the library path for runtime
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", senzing_lib_path);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SENZING_LIB_PATH");
}
