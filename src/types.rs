//! Common types and type aliases for the Senzing SDK

use serde::{Deserialize, Serialize};

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

/// Entity record data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRecord {
    pub data_source: DataSourceCode,
    pub record_id: RecordId,
    pub json_data: JsonString,
}

/// Search result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub entity_id: EntityId,
    pub match_score: Option<f64>,
    pub match_level: Option<i32>,
    pub entity_name: Option<String>,
}

/// Path finding result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResult {
    pub start_entity_id: EntityId,
    pub end_entity_id: EntityId,
    pub entities: Vec<EntityId>,
    pub path_length: i32,
}

/// Network analysis result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResult {
    pub entities: Vec<EntityId>,
    pub max_degree: i32,
    pub build_out_degree: i32,
    pub max_entities: i32,
}

/// Repository performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub duration_seconds: i32,
    pub records_processed: i64,
    pub entities_processed: i64,
    pub throughput_records_per_second: f64,
}

/// Version information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub product_name: String,
    pub version: String,
    pub build_version: String,
    pub build_date: String,
    pub compatibility_version: String,
}

/// License information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub customer: String,
    pub contract: String,
    pub issue_date: String,
    pub license_type: String,
    pub license_level: String,
    pub billing: String,
    pub expire_date: String,
    pub record_limit: i64,
}

/// Configuration definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDefinition {
    pub config_id: Option<ConfigId>,
    pub config_comments: Option<String>,
    pub config_definition: JsonString,
}

/// Data source definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceDefinition {
    pub data_source_code: DataSourceCode,
    pub data_source_id: i32,
}

/// Record preview result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPreview {
    pub data_source: DataSourceCode,
    pub record_definition: JsonString,
    pub entity_type: Option<String>,
}

/// Interesting entities result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestingEntitiesResult {
    pub interesting_entities: Vec<EntityId>,
}

/// Why analysis result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhyResult {
    pub why_key: String,
    pub entities: Vec<EntityId>,
    pub perspective_code: String,
}

/// How analysis result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HowResult {
    pub resolution_steps: Vec<String>,
    pub final_state: JsonString,
}

/// Repository information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub workspace_path: String,
    pub config_path: String,
    pub support_path: String,
    pub resource_path: String,
    pub data_path: String,
}

/// Feature definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDefinition {
    pub feature_id: FeatureId,
    pub feature_type: String,
    pub feature_value: String,
    pub usage_type: String,
}

/// Export statistics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStats {
    pub num_entities_exported: i64,
    pub num_records_exported: i64,
}

/// Redo record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedoRecord {
    pub data_source: DataSourceCode,
    pub record_id: RecordId,
    pub entity_type: String,
    pub redo_operation: String,
}
