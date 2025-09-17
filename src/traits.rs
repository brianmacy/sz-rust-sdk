//! Core traits defining the Senzing SDK interface
//!
//! This module contains the main trait definitions that mirror the C# SDK interfaces.
//! These traits define the contract for interacting with the Senzing engine.

use crate::{error::SzResult, flags::SzFlags, types::*};
use std::collections::HashSet;

/// Main entry point and factory for Senzing SDK components
///
/// The `SzEnvironment` trait provides the primary interface for initializing
/// the Senzing SDK and obtaining instances of other SDK components.
pub trait SzEnvironment {
    /// Destroys the environment and releases all resources
    fn destroy(&mut self) -> SzResult<()>;

    /// Checks if the environment has been destroyed
    fn is_destroyed(&self) -> bool;

    /// Reinitializes the environment with a specific configuration
    fn reinitialize(&mut self, config_id: ConfigId) -> SzResult<()>;

    /// Gets the currently active configuration ID
    fn get_active_config_id(&self) -> SzResult<ConfigId>;

    /// Gets the product interface instance
    fn get_product(&self) -> SzResult<Box<dyn SzProduct>>;

    /// Gets the engine interface instance
    fn get_engine(&self) -> SzResult<Box<dyn SzEngine>>;

    /// Gets the configuration manager interface instance
    fn get_config_manager(&self) -> SzResult<Box<dyn SzConfigManager>>;

    /// Gets the diagnostic interface instance
    fn get_diagnostic(&self) -> SzResult<Box<dyn SzDiagnostic>>;
}

/// Core entity resolution engine operations
///
/// The `SzEngine` trait provides methods for adding records, retrieving entities,
/// performing searches, and conducting various types of analysis.
pub trait SzEngine {
    /// Primes the engine for optimal performance
    fn prime_engine(&self) -> SzResult<()>;

    /// Gets engine statistics
    fn get_stats(&self) -> SzResult<JsonString>;

    /// Adds a record for entity resolution
    fn add_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets a preview of how a record would be processed
    fn get_record_preview(
        &self,
        record_definition: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Deletes a record from the entity repository
    fn delete_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Reevaluates a specific record
    fn reevaluate_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Reevaluates all records for a specific entity
    fn reevaluate_entity(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Searches for entities by attributes
    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why a search result was returned for an entity
    fn why_search(
        &self,
        attributes: &str,
        entity_id: EntityId,
        search_profile: Option<&str>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets entity information by entity ID
    fn get_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString>;

    /// Gets entity information by record identifier
    fn get_entity_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Gets record information
    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds interesting entities related to a given entity
    fn find_interesting_entities_by_entity_id(
        &self,
        entity_id: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds interesting entities related to a given record
    fn find_interesting_entities_by_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds a path between two entities
    fn find_path(
        &self,
        start_entity_id: EntityId,
        end_entity_id: EntityId,
        max_degrees: i64,
        avoid_entity_ids: Option<&HashSet<EntityId>>,
        required_data_sources: Option<&HashSet<String>>,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Finds a network of entities
    fn find_network(
        &self,
        entity_list: &[EntityId],
        max_degrees: i64,
        build_out_degree: i64,
        max_entities: i64,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why two entities resolved together
    fn why_entity(
        &self,
        entity_id1: EntityId,
        entity_id2: EntityId,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes why two records resolved together
    fn why_records(
        &self,
        data_source_code1: &str,
        record_id1: &str,
        data_source_code2: &str,
        record_id2: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Analyzes how an entity was resolved
    fn how_entity(&self, entity_id: EntityId, flags: Option<SzFlags>) -> SzResult<JsonString>;

    /// Gets a virtual entity based on record definitions
    fn get_virtual_entity(
        &self,
        record_definitions: &[&str],
        flags: Option<SzFlags>,
    ) -> SzResult<JsonString>;

    /// Processes the next redo record
    fn process_redo_record(&self) -> SzResult<JsonString>;

    /// Counts the number of redo records
    fn count_redo_records(&self) -> SzResult<i64>;

    /// Exports entity data
    fn export_json_entity_report(&self, flags: Option<SzFlags>) -> SzResult<ExportHandle>;

    /// Exports CSV entity report
    fn export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: Option<SzFlags>,
    ) -> SzResult<ExportHandle>;

    /// Fetches the next batch of export data
    fn fetch_next(&self, export_handle: ExportHandle) -> SzResult<JsonString>;

    /// Closes an export operation
    fn close_export(&self, export_handle: ExportHandle) -> SzResult<()>;

    /// Gets statistics about the current export
    fn get_export_stats(&self, export_handle: ExportHandle) -> SzResult<JsonString>;
}

/// Configuration management operations
///
/// The `SzConfig` trait provides methods for managing Senzing configuration data.
pub trait SzConfig {
    /// Exports the complete configuration as JSON
    fn export(&self) -> SzResult<JsonString>;

    /// Gets the data source registry
    fn get_data_source_registry(&self) -> SzResult<JsonString>;

    /// Registers a new data source
    fn register_data_source(&self, data_source_code: &str) -> SzResult<JsonString>;

    /// Removes a data source from the configuration
    fn unregister_data_source(&self, data_source_code: &str) -> SzResult<()>;
}

/// Configuration lifecycle management
///
/// The `SzConfigManager` trait provides methods for managing configuration
/// versions, registration, and deployment.
pub trait SzConfigManager {
    /// Creates a new configuration instance
    fn create_config(&self) -> SzResult<Box<dyn SzConfig>>;

    /// Creates a configuration from an existing configuration ID
    fn create_config_from_id(&self, config_id: ConfigId) -> SzResult<Box<dyn SzConfig>>;

    /// Creates a configuration from a JSON definition
    fn create_config_from_definition(&self, config_definition: &str)
    -> SzResult<Box<dyn SzConfig>>;

    /// Gets the configuration registry
    fn get_config_registry(&self) -> SzResult<JsonString>;

    /// Gets the default configuration ID
    fn get_default_config_id(&self) -> SzResult<ConfigId>;

    /// Registers a new configuration
    fn register_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId>;

    /// Replaces the default configuration ID
    fn replace_default_config_id(
        &self,
        current_default_config_id: ConfigId,
        new_default_config_id: ConfigId,
    ) -> SzResult<()>;

    /// Sets a new default configuration from definition
    fn set_default_config(
        &self,
        config_definition: &str,
        config_comment: Option<&str>,
    ) -> SzResult<ConfigId>;

    /// Sets the default configuration ID
    fn set_default_config_id(&self, config_id: ConfigId) -> SzResult<()>;
}

/// System diagnostics and monitoring
///
/// The `SzDiagnostic` trait provides methods for system health monitoring,
/// performance analysis, and repository maintenance.
pub trait SzDiagnostic {
    /// Runs a performance test for the specified duration
    fn check_repository_performance(&self, seconds_to_run: i64) -> SzResult<JsonString>;

    /// Gets feature information by feature ID
    fn get_feature(&self, feature_id: FeatureId) -> SzResult<JsonString>;

    /// Gets repository information and statistics
    fn get_repository_info(&self) -> SzResult<JsonString>;

    /// Purges all data from the repository (configuration remains)
    fn purge_repository(&self) -> SzResult<()>;
}

/// Product version and license information
///
/// The `SzProduct` trait provides methods for retrieving product version
/// and licensing information.
pub trait SzProduct {
    /// Gets the product license details
    fn get_license(&self) -> SzResult<JsonString>;

    /// Gets the product version information
    fn get_version(&self) -> SzResult<JsonString>;
}
