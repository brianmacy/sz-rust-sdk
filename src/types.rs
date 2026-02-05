//! Common types and type aliases for the Senzing SDK

/// Entity ID type
pub type EntityId = i64;

/// Configuration ID type
pub type ConfigId = i64;

/// Feature ID type
pub type FeatureId = i64;

/// Export handle type
pub type ExportHandle = i64;

/// Data source code type
pub type DataSourceCode = String;

/// Record ID type
pub type RecordId = String;

/// JSON string type for Senzing data exchange
pub type JsonString = String;

/// Reference to an entity - either by direct ID or by record key.
///
/// This enum allows functions to accept either an entity ID or a record
/// identifier (data source + record ID) to refer to an entity.
///
/// # Examples
///
/// ```rust
/// use sz_rust_sdk::prelude::*;
///
/// // Reference by entity ID (automatic conversion)
/// let entity_id: EntityId = 1001;
/// let ref1: EntityRef = entity_id.into();
///
/// // Reference by record key
/// let ref2 = EntityRef::Record {
///     data_source: "CUSTOMERS",
///     record_id: "CUST001",
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityRef<'a> {
    /// Entity identified by its entity ID
    Id(EntityId),
    /// Entity identified by a record's data source and record ID
    Record {
        /// The data source code (e.g., "CUSTOMERS")
        data_source: &'a str,
        /// The record ID within the data source
        record_id: &'a str,
    },
}

impl From<EntityId> for EntityRef<'_> {
    fn from(id: EntityId) -> Self {
        EntityRef::Id(id)
    }
}

impl<'a> EntityRef<'a> {
    /// Create an EntityRef from a record key
    pub fn from_record(data_source: &'a str, record_id: &'a str) -> Self {
        EntityRef::Record {
            data_source,
            record_id,
        }
    }
}
