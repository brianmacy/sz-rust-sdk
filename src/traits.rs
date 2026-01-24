//! Core traits defining the Senzing SDK interface
//!
//! This module contains the main trait definitions that mirror the C# SDK interfaces.
//! These traits define the contract for interacting with the Senzing engine.

use crate::{error::SzResult, flags::SzFlags, types::*};
use std::collections::HashSet;

/// Main entry point and factory for Senzing SDK components.
///
/// The `SzEnvironment` trait provides the primary interface for initializing
/// the Senzing SDK and obtaining instances of other SDK components. This is
/// the first interface you interact with when using the SDK.
///
/// # Example
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// // Initialize the environment
/// let settings = r#"{"PIPELINE": {"CONFIGPATH": "/etc/senzing", ...}}"#;
/// let env = SzEnvironmentCore::get_instance("my_app", settings, false)?;
///
/// // Get component interfaces
/// let engine = env.get_engine()?;
/// let product = env.get_product()?;
/// ```
///
/// # Singleton Pattern
///
/// `SzEnvironmentCore` implements a singleton pattern. Multiple calls to
/// `get_instance` with the same parameters return the same instance.
pub trait SzEnvironment: Send + Sync {
    /// Checks if the environment has been destroyed.
    ///
    /// # Returns
    ///
    /// `true` if `destroy()` has been called, `false` otherwise.
    fn is_destroyed(&self) -> bool;

    /// Reinitializes the environment with a different configuration.
    ///
    /// Switches to a different registered configuration without destroying
    /// the environment. This is thread-safe and can be called while other
    /// operations are in progress.
    ///
    /// # Arguments
    ///
    /// * `config_id` - ID of a registered configuration to activate
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Configuration ID does not exist
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn reinitialize(&self, config_id: ConfigId) -> SzResult<()>;

    /// Gets the currently active configuration ID.
    ///
    /// # Returns
    ///
    /// The configuration ID currently in use by the engine.
    ///
    /// # Errors
    ///
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn get_active_config_id(&self) -> SzResult<ConfigId>;

    /// Gets the product interface for version and license information.
    ///
    /// # Returns
    ///
    /// An [`SzProduct`] instance for querying product information.
    ///
    /// # Errors
    ///
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn get_product(&self) -> SzResult<Box<dyn SzProduct>>;

    /// Gets the engine interface for entity resolution operations.
    ///
    /// # Returns
    ///
    /// An [`SzEngine`] instance for adding records, searching, and analysis.
    ///
    /// # Errors
    ///
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn get_engine(&self) -> SzResult<Box<dyn SzEngine>>;

    /// Gets the configuration manager interface.
    ///
    /// # Returns
    ///
    /// An [`SzConfigManager`] instance for managing configuration versions.
    ///
    /// # Errors
    ///
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn get_config_manager(&self) -> SzResult<Box<dyn SzConfigManager>>;

    /// Gets the diagnostic interface for system monitoring.
    ///
    /// # Returns
    ///
    /// An [`SzDiagnostic`] instance for performance testing and repository info.
    ///
    /// # Errors
    ///
    /// * `SzError::EnvironmentDestroyed` - Environment was destroyed
    fn get_diagnostic(&self) -> SzResult<Box<dyn SzDiagnostic>>;
}

/// Core entity resolution engine operations.
///
/// The `SzEngine` trait provides methods for adding records, retrieving entities,
/// performing searches, and conducting various types of analysis. This is the
/// primary interface for entity resolution operations.
///
/// # Obtaining an Instance
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
/// let engine = env.get_engine()?;
/// ```
pub trait SzEngine: Send + Sync {
    /// Primes the engine for optimal performance.
    ///
    /// Loads internal caches and prepares the engine for high-throughput operations.
    /// Call this once after initialization when processing large batches of records.
    ///
    /// # Errors
    ///
    /// Returns `SzError::NotInitialized` if the environment is not initialized.
    fn prime_engine(&self) -> SzResult<()>;

    /// Gets engine performance statistics.
    ///
    /// Returns a JSON object containing internal performance metrics useful for
    /// monitoring and debugging.
    ///
    /// # Returns
    ///
    /// JSON string with engine statistics including cache hit rates and timing data.
    fn get_stats(&self) -> SzResult<JsonString>;

    /// Adds a record for entity resolution.
    ///
    /// Inserts or updates a record in the entity repository. The record will be
    /// matched and potentially merged with existing entities based on configured rules.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier (must be registered)
    /// * `record_id` - Unique identifier for the record within the data source
    /// * `record_definition` - JSON object containing the record attributes
    /// * `flags` - Optional flags controlling what information is returned
    ///
    /// # Returns
    ///
    /// JSON string with information about affected entities (when flags request it).
    ///
    /// # Errors
    ///
    /// * `SzError::UnknownDataSource` - Data source is not registered
    /// * `SzError::BadInput` - Invalid JSON or missing required fields
    fn add_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets a preview of how a record would be processed without persisting it.
    ///
    /// Useful for testing record mappings and seeing how features would be extracted
    /// before committing the record to the repository.
    ///
    /// # Arguments
    ///
    /// * `record_definition` - JSON object containing the record attributes
    /// * `flags` - Optional flags controlling what information is returned
    ///
    /// # Returns
    ///
    /// JSON string showing extracted features and potential matches.
    fn get_record_preview(
        &self,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Deletes a record from the entity repository.
    ///
    /// Removes the record and re-resolves any affected entities. If the record
    /// was the only record in an entity, the entity is also removed.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier
    /// * `record_id` - The record identifier to delete
    /// * `flags` - Optional flags controlling what information is returned
    ///
    /// # Returns
    ///
    /// JSON string with information about affected entities.
    ///
    /// # Errors
    ///
    /// * `SzError::UnknownDataSource` - Data source is not registered
    /// * `SzError::NotFound` - Record does not exist
    fn delete_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Reevaluates a specific record against current rules.
    ///
    /// Forces re-resolution of a record using the current configuration. Useful
    /// after configuration changes to update entity assignments.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier
    /// * `record_id` - The record identifier to reevaluate
    /// * `flags` - Optional flags controlling what information is returned
    ///
    /// # Returns
    ///
    /// JSON string with reevaluation results.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Record does not exist
    fn reevaluate_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Reevaluates all records for a specific entity.
    ///
    /// Forces re-resolution of all records in an entity. The entity may split
    /// into multiple entities or merge with others based on current rules.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity identifier to reevaluate
    /// * `flags` - Optional flags controlling what information is returned
    ///
    /// # Returns
    ///
    /// JSON string with reevaluation results.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Entity does not exist
    fn reevaluate_entity(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Searches for entities by attributes.
    ///
    /// Finds entities that match the provided attributes. Returns scored results
    /// based on match quality.
    ///
    /// # Arguments
    ///
    /// * `attributes` - JSON object with search attributes (e.g., name, address)
    /// * `search_profile` - Optional search profile name for customized matching
    /// * `flags` - Optional flags controlling result detail level
    ///
    /// # Returns
    ///
    /// JSON string with matching entities and match scores.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - Invalid JSON attributes
    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why a search result was returned for an entity.
    ///
    /// Provides detailed explanation of why a particular entity matched the
    /// search criteria, including feature comparisons and match scores.
    ///
    /// # Arguments
    ///
    /// * `attributes` - JSON object with search attributes
    /// * `entity_id` - The entity to analyze
    /// * `search_profile` - Optional search profile name
    /// * `flags` - Optional flags controlling detail level
    ///
    /// # Returns
    ///
    /// JSON string with detailed match analysis.
    fn why_search(
        &self,
        attributes: &str,
        entity_id: EntityId,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets entity information by entity ID.
    ///
    /// Retrieves complete entity data including all constituent records and
    /// relationships.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity identifier
    /// * `flags` - Optional flags controlling what data is included
    ///
    /// # Returns
    ///
    /// JSON string with entity details.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Entity does not exist
    fn get_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString>;

    /// Gets entity information by record identifier.
    ///
    /// Retrieves the entity containing the specified record.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier
    /// * `record_id` - The record identifier
    /// * `flags` - Optional flags controlling what data is included
    ///
    /// # Returns
    ///
    /// JSON string with entity details.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Record does not exist
    fn get_entity_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets record information.
    ///
    /// Retrieves the original record data as stored in the repository.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier
    /// * `record_id` - The record identifier
    /// * `flags` - Optional flags controlling what data is included
    ///
    /// # Returns
    ///
    /// JSON string with record details.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Record does not exist
    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds interesting entities related to a given entity.
    ///
    /// Identifies entities with notable relationships to the specified entity,
    /// such as disclosed relationships or possible matches.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity to analyze
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with interesting entity relationships.
    fn find_interesting_entities_by_entity_id(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds interesting entities related to a given record.
    ///
    /// Identifies entities with notable relationships to the entity containing
    /// the specified record.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier
    /// * `record_id` - The record identifier
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with interesting entity relationships.
    fn find_interesting_entities_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds a relationship path between two entities.
    ///
    /// Discovers the shortest path connecting two entities through their
    /// relationships, useful for understanding indirect connections.
    ///
    /// # Arguments
    ///
    /// * `start_entity_id` - Starting entity
    /// * `end_entity_id` - Target entity
    /// * `max_degrees` - Maximum relationship hops to traverse
    /// * `avoid_entity_ids` - Optional entities to exclude from the path
    /// * `required_data_sources` - Optional data sources that must appear in path
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with path details and intermediate entities.
    fn find_path(
        &self,
        start_entity_id: EntityId,
        end_entity_id: EntityId,
        max_degrees: i64,
        avoid_entity_ids: Option<&HashSet<EntityId>>,
        required_data_sources: Option<&HashSet<String>>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds a network of related entities.
    ///
    /// Builds a network graph starting from one or more seed entities,
    /// expanding outward through relationships.
    ///
    /// # Arguments
    ///
    /// * `entity_list` - Seed entity IDs to start from
    /// * `max_degrees` - Maximum relationship hops from seed entities
    /// * `build_out_degree` - Degrees to expand for building connections
    /// * `max_entities` - Maximum entities to include in the network
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with network graph data.
    fn find_network(
        &self,
        entity_list: &[EntityId],
        max_degrees: i64,
        build_out_degree: i64,
        max_entities: i64,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why two entities are related.
    ///
    /// Provides detailed explanation of the relationship between two entities,
    /// including shared features and match scores.
    ///
    /// # Arguments
    ///
    /// * `entity_id1` - First entity
    /// * `entity_id2` - Second entity
    /// * `flags` - Optional flags controlling detail level
    ///
    /// # Returns
    ///
    /// JSON string with relationship analysis.
    fn why_entity(
        &self,
        entity_id1: EntityId,
        entity_id2: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why two records resolved together.
    ///
    /// Explains why two specific records were merged into the same entity,
    /// showing the matching features and rules that caused the merge.
    ///
    /// # Arguments
    ///
    /// * `data_source_code1` - First record's data source
    /// * `record_id1` - First record's identifier
    /// * `data_source_code2` - Second record's data source
    /// * `record_id2` - Second record's identifier
    /// * `flags` - Optional flags controlling detail level
    ///
    /// # Returns
    ///
    /// JSON string with merge analysis.
    fn why_records(
        &self,
        data_source_code1: &str,
        record_id1: &str,
        data_source_code2: &str,
        record_id2: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why a record belongs to its entity.
    ///
    /// Explains the chain of matches that connected a record to its current
    /// entity assignment.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The record's data source
    /// * `record_id` - The record identifier
    /// * `flags` - Optional flags controlling detail level
    ///
    /// # Returns
    ///
    /// JSON string with entity membership analysis.
    fn why_record_in_entity(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes how an entity was constructed.
    ///
    /// Provides a step-by-step explanation of how records were merged to form
    /// the current entity, useful for understanding complex resolution paths.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity to analyze
    /// * `flags` - Optional flags controlling detail level
    ///
    /// # Returns
    ///
    /// JSON string with entity construction history.
    fn how_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString>;

    /// Creates a virtual entity from record keys without persisting.
    ///
    /// Simulates what an entity would look like if the specified records were
    /// merged, without affecting the actual repository.
    ///
    /// # Arguments
    ///
    /// * `record_keys` - Pairs of (data_source_code, record_id)
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with virtual entity data.
    fn get_virtual_entity(
        &self,
        record_keys: &[(String, String)],
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Processes a redo record for deferred resolution.
    ///
    /// Handles records that were queued for later processing due to conflicts
    /// or resource constraints.
    ///
    /// # Arguments
    ///
    /// * `redo_record` - The redo record JSON from `get_redo_record`
    /// * `flags` - Optional flags controlling result detail
    ///
    /// # Returns
    ///
    /// JSON string with processing results.
    fn process_redo_record(
        &self,
        redo_record: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets the next pending redo record.
    ///
    /// Retrieves one record from the redo queue for processing.
    ///
    /// # Returns
    ///
    /// JSON string with redo record data, or empty string if queue is empty.
    fn get_redo_record(&self) -> SzResult<JsonString>;

    /// Counts pending redo records.
    ///
    /// # Returns
    ///
    /// Number of records waiting in the redo queue.
    fn count_redo_records(&self) -> SzResult<i64>;

    /// Starts a JSON entity export.
    ///
    /// Initiates an export operation returning a handle for fetching results.
    /// Use `fetch_next` to retrieve data and `close_export` when done.
    ///
    /// # Arguments
    ///
    /// * `flags` - Optional flags controlling what data is exported
    ///
    /// # Returns
    ///
    /// Handle for fetching export data.
    fn export_json_entity_report(&self, flags: Option<SzFlags>) -> SzResult<ExportHandle>;

    /// Starts a CSV entity export.
    ///
    /// Initiates a CSV export with specified columns.
    ///
    /// # Arguments
    ///
    /// * `csv_column_list` - Comma-separated list of columns to include
    /// * `flags` - Optional flags controlling what data is exported
    ///
    /// # Returns
    ///
    /// Handle for fetching export data.
    fn export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<ExportHandle>;

    /// Fetches the next batch of export data.
    ///
    /// Call repeatedly until empty string is returned to get all export data.
    ///
    /// # Arguments
    ///
    /// * `export_handle` - Handle from `export_json_entity_report` or `export_csv_entity_report`
    ///
    /// # Returns
    ///
    /// Next batch of export data, or empty string when complete.
    fn fetch_next(&self, export_handle: ExportHandle) -> SzResult<JsonString>;

    /// Closes an export operation and releases resources.
    ///
    /// Must be called when finished with an export to free the handle.
    ///
    /// # Arguments
    ///
    /// * `export_handle` - Handle to close
    fn close_export(&self, export_handle: ExportHandle) -> SzResult<()>;
}

/// Configuration management operations.
///
/// The `SzConfig` trait provides methods for managing Senzing configuration data,
/// including data source registration and configuration export.
///
/// # Obtaining an Instance
///
/// Configuration instances are obtained through [`SzConfigManager`]:
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
/// let config_mgr = env.get_config_manager()?;
/// let config = config_mgr.create_config()?;
/// ```
pub trait SzConfig {
    /// Exports the complete configuration as JSON.
    ///
    /// Returns the full configuration definition that can be saved, modified,
    /// or registered as a new configuration version.
    ///
    /// # Returns
    ///
    /// JSON string containing the complete configuration.
    fn export(&self) -> SzResult<JsonString>;

    /// Gets the data source registry.
    ///
    /// Returns information about all registered data sources in this configuration.
    ///
    /// # Returns
    ///
    /// JSON string with array of data source definitions including codes and IDs.
    fn get_data_source_registry(&self) -> SzResult<JsonString>;

    /// Registers a new data source.
    ///
    /// Adds a data source to the configuration. Data sources must be registered
    /// before records can be added from that source.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - Unique identifier for the data source (e.g., "CUSTOMERS", "WATCHLIST")
    ///
    /// # Returns
    ///
    /// JSON string with the registered data source details including assigned ID.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - Data source code is invalid or already exists
    fn register_data_source(&self, data_source_code: &str) -> SzResult<JsonString>;

    /// Removes a data source from the configuration.
    ///
    /// Unregisters a data source. This should only be done if no records exist
    /// from that data source.
    ///
    /// # Arguments
    ///
    /// * `data_source_code` - The data source identifier to remove
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - Data source does not exist
    fn unregister_data_source(&self, data_source_code: &str) -> SzResult<()>;
}

/// Configuration lifecycle management.
///
/// The `SzConfigManager` trait provides methods for managing configuration
/// versions, registration, and deployment. Use this to create, modify, and
/// activate configuration versions.
///
/// # Obtaining an Instance
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
/// let config_mgr = env.get_config_manager()?;
/// ```
pub trait SzConfigManager {
    /// Creates a new configuration instance from the default template.
    ///
    /// Returns a configuration object that can be modified (e.g., adding data sources)
    /// before being registered and activated.
    ///
    /// # Returns
    ///
    /// A new [`SzConfig`] instance based on the default configuration template.
    fn create_config(&self) -> SzResult<Box<dyn SzConfig>>;

    /// Creates a configuration from an existing registered configuration.
    ///
    /// Loads a previously registered configuration for viewing or modification.
    ///
    /// # Arguments
    ///
    /// * `config_id` - ID of a registered configuration
    ///
    /// # Returns
    ///
    /// An [`SzConfig`] instance with the specified configuration loaded.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Configuration ID does not exist
    fn create_config_from_id(&self, config_id: ConfigId) -> SzResult<Box<dyn SzConfig>>;

    /// Creates a configuration from a JSON definition string.
    ///
    /// Parses a configuration JSON (e.g., from a file or `SzConfig::export()`)
    /// into a configuration object.
    ///
    /// # Arguments
    ///
    /// * `config_definition` - JSON string containing the configuration
    ///
    /// # Returns
    ///
    /// An [`SzConfig`] instance with the parsed configuration.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - Invalid JSON or configuration format
    fn create_config_from_definition(&self, config_definition: &str)
    -> SzResult<Box<dyn SzConfig>>;

    /// Gets the configuration registry.
    ///
    /// Returns information about all registered configuration versions.
    ///
    /// # Returns
    ///
    /// JSON string with array of configuration metadata including IDs, comments, and timestamps.
    fn get_config_registry(&self) -> SzResult<JsonString>;

    /// Gets the currently active default configuration ID.
    ///
    /// # Returns
    ///
    /// The configuration ID that is currently active for entity resolution.
    fn get_default_config_id(&self) -> SzResult<ConfigId>;

    /// Registers a new configuration version.
    ///
    /// Saves a configuration to the repository, making it available for activation.
    /// Does not activate the configuration - use `set_default_config_id` for that.
    ///
    /// # Arguments
    ///
    /// * `config_definition` - JSON string from `SzConfig::export()`
    /// * `config_comment` - Optional description for this configuration version
    ///
    /// # Returns
    ///
    /// The assigned configuration ID for the newly registered configuration.
    fn register_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId>;

    /// Atomically replaces the default configuration ID.
    ///
    /// Updates the active configuration only if the current default matches
    /// the expected value. This prevents race conditions when multiple processes
    /// may be updating the configuration.
    ///
    /// # Arguments
    ///
    /// * `current_default_config_id` - Expected current default (for optimistic locking)
    /// * `new_default_config_id` - New configuration to activate
    ///
    /// # Errors
    ///
    /// * `SzError::ReplaceConflict` - Current default doesn't match expected value
    /// * `SzError::NotFound` - New configuration ID does not exist
    fn replace_default_config_id(
        &self,
        current_default_config_id: ConfigId,
        new_default_config_id: ConfigId,
    ) -> SzResult<()>;

    /// Registers and activates a configuration in one operation.
    ///
    /// Convenience method that combines `register_config` and `set_default_config_id`.
    ///
    /// # Arguments
    ///
    /// * `config_definition` - JSON string from `SzConfig::export()`
    /// * `config_comment` - Optional description for this configuration version
    ///
    /// # Returns
    ///
    /// The assigned configuration ID.
    fn set_default_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId>;

    /// Sets the active configuration by ID.
    ///
    /// Activates a previously registered configuration. The engine will use
    /// this configuration for all subsequent operations.
    ///
    /// # Arguments
    ///
    /// * `config_id` - ID of a registered configuration to activate
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Configuration ID does not exist
    fn set_default_config_id(&self, config_id: ConfigId) -> SzResult<()>;
}

/// System diagnostics and monitoring.
///
/// The `SzDiagnostic` trait provides methods for system health monitoring,
/// performance analysis, and repository maintenance.
///
/// # Obtaining an Instance
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
/// let diagnostic = env.get_diagnostic()?;
/// ```
pub trait SzDiagnostic: Send + Sync {
    /// Runs a performance benchmark on the repository.
    ///
    /// Executes read operations against the repository to measure performance
    /// characteristics. Useful for baseline testing and capacity planning.
    ///
    /// # Arguments
    ///
    /// * `seconds_to_run` - Duration of the benchmark in seconds
    ///
    /// # Returns
    ///
    /// JSON string with performance metrics including operations per second.
    ///
    /// # Errors
    ///
    /// * `SzError::BadInput` - Invalid duration (must be positive)
    fn check_repository_performance(&self, seconds_to_run: i64) -> SzResult<JsonString>;

    /// Gets detailed information about a specific feature.
    ///
    /// Retrieves internal feature data useful for debugging entity resolution
    /// decisions. Features are the normalized attributes extracted from records.
    ///
    /// # Arguments
    ///
    /// * `feature_id` - Internal feature identifier
    ///
    /// # Returns
    ///
    /// JSON string with feature details including type, value, and usage statistics.
    ///
    /// # Errors
    ///
    /// * `SzError::NotFound` - Feature ID does not exist
    fn get_feature(&self, feature_id: FeatureId) -> SzResult<JsonString>;

    /// Gets repository statistics and information.
    ///
    /// Returns aggregate information about the entity repository including
    /// record counts, entity counts, and data source statistics.
    ///
    /// # Returns
    ///
    /// JSON string with repository statistics.
    fn get_repository_info(&self) -> SzResult<JsonString>;

    /// Purges all entity data from the repository.
    ///
    /// Removes all records and entities while preserving configuration.
    /// Use with caution - this operation is irreversible.
    ///
    /// # Warning
    ///
    /// This permanently deletes all entity resolution data. Configuration
    /// and data source definitions are preserved.
    fn purge_repository(&self) -> SzResult<()>;
}

/// Product version and license information.
///
/// The `SzProduct` trait provides methods for retrieving product version
/// and licensing information.
///
/// # Obtaining an Instance
///
/// ```ignore
/// use sz_rust_sdk::prelude::*;
///
/// let env = SzEnvironmentCore::get_instance("my_app", &settings, false)?;
/// let product = env.get_product()?;
/// ```
pub trait SzProduct: Send + Sync {
    /// Gets the product license details.
    ///
    /// Returns information about the Senzing license including type,
    /// expiration, and feature entitlements.
    ///
    /// # Returns
    ///
    /// JSON string with license information.
    fn get_license(&self) -> SzResult<JsonString>;

    /// Gets the product version information.
    ///
    /// Returns version details for the Senzing engine and its components.
    ///
    /// # Returns
    ///
    /// JSON string with version information including build date and component versions.
    fn get_version(&self) -> SzResult<JsonString>;
}
