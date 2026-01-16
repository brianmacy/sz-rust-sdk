//! Core implementation structs for Senzing SDK interfaces
//!
//! Only [`SzEnvironmentCore`] is part of the public API. All other core types
//! are internal implementation details accessed through trait objects.

mod config;
mod config_manager;
mod diagnostic;
mod engine;
mod product;

pub mod environment;

// Only SzEnvironmentCore is public - users access everything else through traits
pub use environment::SzEnvironmentCore;
