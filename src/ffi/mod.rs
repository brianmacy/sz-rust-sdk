//! FFI bindings to the native Senzing library (Internal)
//!
//! This module contains the low-level FFI declarations for interfacing
//! with the native Senzing C library. This module is internal to the SDK
//! and not part of the public API.

#[allow(unused_imports, dead_code)]
pub(crate) mod bindings;
#[allow(unused_imports, dead_code)]
pub(crate) mod helpers;

// Re-export for internal crate use only
#[allow(unused_imports)]
pub(crate) use bindings::*;
#[allow(unused_imports)]
pub(crate) use helpers::*;
