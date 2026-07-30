//! FFI bindings to the native Senzing library (Internal)
//!
//! This module contains the low-level FFI declarations for interfacing
//! with the native Senzing C library. This module is internal to the SDK
//! and not part of the public API.
//!
//! The bindgen-generated bindings + str<->C marshaling primitives now live in the shared
//! `sz-rust-sdk-ffi` crate (github.com/brianmacy/sz-rust-sdk-ffi), also depended on by Senzing's
//! in-tree Rust binding layer (senzing-sys in the G2 repo) -- replacing this crate's previously
//! independently-maintained copy. To regenerate the bindings: see that crate's
//! `examples/generate_bindings.rs`.

#[allow(dead_code)]
pub(crate) mod helpers;

// Re-export all shared bindings for internal use (unqualified `crate::ffi::Sz_*` call sites
// throughout src/core/*.rs are unchanged).
pub(crate) use sz_rust_sdk_ffi::bindings::*;
