//! FFI bindings to the native Senzing library (Internal)
//!
//! This module contains the low-level FFI declarations for interfacing
//! with the native Senzing C library. This module is internal to the SDK
//! and not part of the public API.
//!
//! Bindings are auto-generated from Senzing C headers using bindgen.
//! To regenerate: cargo run --example generate_bindings

// Auto-generated bindings from C headers
#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::upper_case_acronyms
)]
pub(crate) mod bindings_generated;

#[allow(dead_code)]
pub(crate) mod helpers;

// Re-export all generated bindings for internal use
pub(crate) use bindings_generated::*;
