//! Core implementation structs for Senzing SDK interfaces
//!
//! Public API types:
//! - [`SzEnvironmentCore`] - The main environment singleton
//! - [`SenzingGuard`] - RAII wrapper for automatic cleanup
//!
//! All other core types are internal implementation details accessed through
//! trait objects.

mod config;
mod config_manager;
mod diagnostic;
mod engine;
mod guard;
mod product;

pub mod environment;

// Public API: SzEnvironmentCore and SenzingGuard
pub use environment::SzEnvironmentCore;
pub use guard::SenzingGuard;
