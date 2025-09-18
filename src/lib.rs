//! # Senzing Rust SDK
//!
//! This crate provides a Rust interface to the Senzing entity resolution engine.
//! It follows the same patterns as other Senzing SDKs while leveraging Rust's
//! type safety and zero-cost abstractions.
//!
//! ## Architecture
//!
//! The SDK is organized around the following core traits:
//! - [`SzEnvironment`] - Main entry point and factory for other components
//! - [`SzEngine`] - Core entity resolution operations
//! - [`SzConfig`] - Configuration management
//! - [`SzConfigManager`] - Configuration lifecycle management
//! - [`SzDiagnostic`] - System diagnostics and monitoring
//! - [`SzProduct`] - Version and license information
//!
//! ## Usage
//!
//! ```rust,no_run
//! use sz_rust_sdk::prelude::*;
//!
//! // Initialize the Senzing environment with proper settings
//! let settings = r#"{
//!     "PIPELINE": {
//!         "CONFIGPATH": "/etc/opt/senzing",
//!         "RESOURCEPATH": "/opt/senzing/er/resources",
//!         "SUPPORTPATH": "/opt/senzing/data"
//!     },
//!     "SQL": {
//!         "CONNECTION": "sqlite3://na:na@/tmp/G2C.db"
//!     }
//! }"#;
//! let env = SzEnvironmentCore::new("my-app", settings, false)?;
//!
//! // Get the engine instance
//! let engine = env.get_engine()?;
//!
//! // Add a record for entity resolution
//! let result = engine.add_record(
//!     "CUSTOMERS",
//!     "CUST001",
//!     r#"{"NAME_FULL": "John Smith", "EMAIL": "john@example.com"}"#,
//!     None
//! )?;
//!
//! println!("Entity resolution result: {}", result);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod core;
pub mod error;
mod ffi; // Internal FFI module - not part of public API
pub mod flags;
pub mod helpers;
pub mod traits;
pub mod types;

pub use core::*;
pub use error::*;
pub use flags::*;
pub use helpers::*;
pub use traits::*;
pub use types::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::*;
    pub use crate::error::*;
    pub use crate::flags::*;
    pub use crate::helpers::*;
    pub use crate::traits::*;
    pub use crate::types::*;
}
