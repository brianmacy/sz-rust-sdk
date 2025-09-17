# Senzing C# SDK Test Analysis Report

This comprehensive analysis examines the test patterns, inputs, SDK method calls, and expected outputs from the Senzing C# SDK test suite to guide the implementation of equivalent Rust tests.

## Executive Summary

The C# SDK test suite demonstrates comprehensive testing across multiple dimensions:
- **19 core test files** covering engine operations, configuration, diagnostics
- **Standardized test patterns** with common setup/teardown procedures
- **Diverse test data** including fictional entities across multiple data sources
- **Comprehensive error handling** with detailed exception hierarchies
- **Multi-format data support** (JSON, CSV, JSON Lines)

## Main Test Files Analysis

### SzExceptionTest.cs
**Purpose**: Validates exception handling across the entire SDK

**Key Findings**:
- **15 exception types tested**: SzException, SzConfigurationException, SzDatabaseConnectionLostException, etc.
- **Constructor patterns**: Default, message-only, error code, inner exception, full constructor
- **Test inputs**: Error codes (10L, 20L, 30L, 40L), standard message ("Some Message")
- **Validation logic**: Message preservation, error code setting, inner exception chaining, ToString() formatting

**Rust Implementation Notes**:
```rust
// Equivalent error hierarchy needed
pub enum SzError {
    Configuration(SzConfigurationError),
    Database(SzDatabaseError),
    BadInput(SzBadInputError),
    // ... other variants
}

// Test pattern example
#[test]
fn test_error_with_code_and_message() {
    let error = SzError::new_with_code(10, "Some Message");
    assert_eq!(error.code(), 10);
    assert_eq!(error.message(), "Some Message");
}
```

### SzFlagsTest.cs
**Purpose**: Validates bitflag operations and metadata consistency

**Key Findings**:
- **Flag operations**: Combination (bitwise OR), group classification, string conversion
- **Test inputs**: SzSearchIncludeNameOnly, SzEntityIncludeEntityName, various flag combinations
- **Validation logic**: Flag-to-string conversion, group membership, metadata consistency

**Rust Implementation Notes**:
```rust
use bitflags::bitflags;

bitflags! {
    pub struct SzFlags: u64 {
        const INCLUDE_NAME_ONLY = 0x1;
        const INCLUDE_ENTITY_NAME = 0x2;
        // ... other flags
    }
}

#[test]
fn test_flag_combinations() {
    let combined = SzFlags::INCLUDE_NAME_ONLY | SzFlags::INCLUDE_ENTITY_NAME;
    assert!(combined.contains(SzFlags::INCLUDE_NAME_ONLY));
}
```

### UtilitiesTest.cs
**Purpose**: Tests utility functions for data transformation

**Key Findings**:
- **HexFormat**: Converts integers to hexadecimal with round-trip validation
- **JsonEscape**: Handles JSON string escaping with null safety
- **Test inputs**: Various integers (0, 1, 2, 20, 40, 80, 160, 3200, 64000, 128345789)
- **Edge cases**: Null inputs, escape characters (\n, \f, \b, \\, \t, \r), Unicode

## Core Test Files Analysis

### AbstractTest.cs - Test Infrastructure
**Purpose**: Provides common test infrastructure and utilities

**Key Components**:
- **Environment setup**: Repository creation, configuration management
- **Test lifecycle**: BeginTests(), EndTests(), InitializeTestEnvironment()
- **Utilities**: JSON validation, file preparation (CSV/JSON), data generation
- **Repository management**: Temporary repository creation, cleanup

**Rust Implementation Pattern**:
```rust
pub struct TestEnvironment {
    repository_path: PathBuf,
    config_id: i64,
}

impl TestEnvironment {
    pub fn new() -> Result<Self, SzError> {
        // Initialize test repository
        // Setup configuration
    }

    pub fn cleanup(&self) -> Result<(), SzError> {
        // Clean up test resources
    }
}

// Use in tests
#[test]
fn test_with_environment() {
    let env = TestEnvironment::new().unwrap();
    // ... test logic
    env.cleanup().unwrap();
}
```

### SzCoreEngineBasicsTest.cs
**Purpose**: Tests fundamental engine operations

**Key Operations**:
- Native API retrieval
- Engine priming
- Data encoding (sources, entity IDs, record keys)

**Test Data Patterns**:
- Empty sets, single items, multiple items
- Data sources: Customers, Employees, Watchlist

**SDK Methods**:
- `GetNativeApi()`
- `PrimeEngine()`
- `EncodeDataSources()`, `EncodeEntityIDs()`, `EncodeRecordKeys()`

### SzCoreEngineReadTest.cs
**Purpose**: Tests entity and record reading operations

**Test Data Sources**:
- **Passengers**: 4 records (Joe Schmoe, John Doe, etc.)
- **Employees**: 4 records
- **VIPs**: 2 records (Mark Hightower, Bruce Wayne)
- **Marriages**: 4 records

**Key Operations**:
- `GetEntity()` - by record ID and entity ID
- `GetRecord()` - retrieve specific records
- `SearchByAttributes()` - phone, address, name searches
- `FindInterestingEntities()` - discover related entities
- `ExportCsvEntityReport()`, `ExportJsonEntityReport()` - bulk export

**Validation Patterns**:
- JSON structure validation
- Entity ID verification
- Record detail checking
- Search result filtering

### SzCoreEngineWriteTest.cs
**Purpose**: Tests record modification operations

**Write Operations**:
- `GetRecordPreview()` - preview changes before commit
- `AddRecord()` - insert new records
- `ReevaluateRecord()` - trigger re-analysis
- `DeleteRecord()` - remove records
- `ProcessRedoRecord()` - handle redo operations

**Test Data**: Multiple data sources with varied attributes (names, phones, addresses)

### SzCoreConfigTest.cs
**Purpose**: Tests configuration management

**Operations**:
- `CreateConfig()` - from template or JSON
- `Export()` - serialize configuration
- `RegisterDataSource()`, `UnregisterDataSource()` - manage data sources
- `GetDataSourceRegistry()` - retrieve data source info

**Configuration Patterns**:
- Default template-based creation
- JSON-based custom configuration
- Dynamic data source management

### SzCoreConfigManagerTest.cs
**Purpose**: Tests configuration lifecycle management

**Manager Operations**:
- `RegisterConfig()` - add new configurations
- `CreateConfig()` - generate configuration objects
- `GetConfigRegistry()` - retrieve configuration details
- `SetDefaultConfigID()`, `ReplaceDefaultConfigID()` - manage active config

**Lifecycle Patterns**:
- Configuration versioning
- Comment tracking
- Default configuration management

### SzCoreDiagnosticTest.cs
**Purpose**: Tests system diagnostics and monitoring

**Diagnostic Operations**:
- `GetRepositoryInfo()` - system information
- `CheckRepositoryPerformance(5)` - 5-second performance check
- `GetFeature(featureID)` - feature analysis
- `PurgeRepository()` - system cleanup

### SzCoreEngineWhyTest.cs
**Purpose**: Tests relationship analysis and explanation

**Why Operations**:
- `WhyEntities()` - compare entity relationships
- `WhySearch()` - search-based relationship discovery
- `WhyRecordInEntity()` - explain record membership
- `WhyRecords()` - analyze record relationships

**Relationship Patterns**:
- Employment connections
- Spousal relationships
- Shared contact information
- Geographic proximity

### SzCoreEngineHowTest.cs
**Purpose**: Tests entity resolution tracing

**How Operations**:
- `HowEntity(entityID, flags)` - trace entity formation
- Feature matching analysis
- Virtual entity creation
- Record consolidation logic

**Resolution Patterns**:
- Cross-data-source matching
- Feature scoring
- Identity consolidation paths

### SzCoreEngineGraphTest.cs
**Purpose**: Tests graph analysis and network discovery

**Graph Operations**:
- `FindPath()` - discover entity connections
- `FindNetwork()` - explore entity networks

**Parameters**:
- Maximum degrees of separation
- Entity avoidance constraints
- Required data source filtering
- Detailed relationship information

### SzCoreEnvironmentTest.cs
**Purpose**: Tests environment management and lifecycle

**Environment Operations**:
- Builder pattern initialization
- Singleton enforcement
- Component access (GetEngine(), GetConfig(), etc.)
- Lifecycle management (Initialize(), Destroy())

### SzCoreProductTest.cs
**Purpose**: Tests product information and metadata

**Product Operations**:
- `GetLicense()` - license information
- `GetVersion()` - version details
- Exception handling methods

**Validation**: JSON metadata with specific keys (customer, contract, issueDate, VERSION, BUILD_NUMBER)

## Specialized Test Components

### RecordReader.cs (IO Module)
**Purpose**: Multi-format test data reader

**Supported Formats**:
- JSON arrays
- JSON Lines (one object per line)
- CSV with headers

**Features**:
- Format auto-detection
- Data source normalization
- Error handling with line numbers
- Metadata augmentation

### AccessToken.cs (Utilities)
**Purpose**: Thread-safe access token management

**Features**:
- Thread-local storage
- Unique token generation per thread
- Referential equality
- Token rotation support

### TextUtilities.cs (Utilities)
**Purpose**: Text processing utilities

**Features**:
- Complex CSV parsing (quoted fields, escapes)
- Random text generation
- Secure random number generation

### InstallLocations.cs (Native API)
**Purpose**: Installation path discovery and validation

**Validation Logic**:
- Environment variable priority (SENZING_PATH, SENZING_DIR)
- Platform-specific defaults
- Directory structure verification
- Required file checking

## Common Test Data Patterns

### Entity Records
**Passengers Data Source**:
```json
{
  "RECORD_ID": "1001",
  "NAME_FIRST": "Joe",
  "NAME_LAST": "Schmoe",
  "PHONE_NUMBER": "702-919-1300",
  "ADDR_FULL": "101 Main Street, Anywhere, Texas 73227"
}
```

**Employees Data Source**:
```json
{
  "RECORD_ID": "1001",
  "NAME_FIRST": "John",
  "NAME_LAST": "Doe",
  "PHONE_NUMBER": "818-555-1313",
  "ADDR_FULL": "100 Main Street, Anytown, Texas 73227"
}
```

### Common Test Scenarios
1. **Single entity across multiple data sources** - same person in different systems
2. **Related entities** - family members, employees and companies
3. **Ambiguous matches** - similar but different people
4. **Missing/incomplete data** - testing partial information scenarios

## Error Handling Patterns

### Exception Hierarchy
```
SzException (base)
├── SzConfigurationException
├── SzDatabaseException
│   └── SzDatabaseConnectionLostException
├── SzBadInputException
├── SzLicenseException
├── SzNotFoundException
├── SzNotInitializedException
└── SzUnhandledException
```

### Error Code Patterns
- **-2 return code**: Indicates exception information available via `GetLastException()`
- **Specific error codes**: 10L, 20L, 30L, 40L used in tests
- **Message preservation**: All exceptions maintain original error messages
- **Inner exception chaining**: Supports exception cause tracking

## Test Infrastructure Requirements

### Environment Setup
1. **Repository initialization**: Temporary test repositories
2. **Configuration management**: Default and custom configurations
3. **Data loading**: CSV and JSON test data preparation
4. **Cleanup**: Automatic resource cleanup after tests

### Native Library Integration
- All tests require actual Senzing native library (no mocking)
- Installation at `/opt/senzing/er/` (Linux default)
- Environment variable support for custom paths
- Platform-specific installation detection

## Rust Implementation Recommendations

### Project Structure
```
tests/
├── common/
│   ├── mod.rs              # Common test utilities
│   ├── test_environment.rs # Environment setup/teardown
│   ├── test_data.rs        # Standard test data
│   └── record_reader.rs    # Multi-format data reader
├── unit/
│   ├── exceptions_test.rs  # Error handling tests
│   ├── flags_test.rs       # Bitflag operations
│   └── utilities_test.rs   # Utility functions
├── integration/
│   ├── engine_basics_test.rs
│   ├── engine_read_test.rs
│   ├── engine_write_test.rs
│   ├── config_test.rs
│   ├── diagnostic_test.rs
│   └── environment_test.rs
└── data/
    ├── passengers.json
    ├── employees.json
    └── test_config.json
```

### Key Implementation Patterns

1. **Environment Management**:
```rust
pub struct TestEnvironment {
    _temp_dir: TempDir,
    environment: SzEnvironment,
}

impl TestEnvironment {
    pub fn new() -> Result<Self, SzError> {
        let temp_dir = TempDir::new()?;
        let environment = SzEnvironment::builder()
            .instance_name("rust_test")
            .settings(&format!("{{\"PIPELINE\": {{\"CONFIGPATH\": \"{}\"}}}}",
                      temp_dir.path().display()))
            .build()?;
        Ok(Self { _temp_dir: temp_dir, environment })
    }
}
```

2. **Test Data Management**:
```rust
pub fn load_test_data() -> Vec<TestRecord> {
    vec![
        TestRecord {
            data_source: "PASSENGERS".to_string(),
            record_id: "1001".to_string(),
            data: json!({
                "NAME_FIRST": "Joe",
                "NAME_LAST": "Schmoe",
                "PHONE_NUMBER": "702-919-1300"
            }),
        },
        // ... more test records
    ]
}
```

3. **Error Testing**:
```rust
#[test]
fn test_error_hierarchy() {
    let error = SzError::Configuration {
        code: 10,
        message: "Test message".to_string(),
        source: None,
    };

    assert_eq!(error.code(), 10);
    assert_eq!(error.message(), "Test message");
}
```

4. **Flag Testing**:
```rust
#[test]
fn test_flag_combinations() {
    let flags = SzEntityFlags::INCLUDE_ENTITY_NAME | SzEntityFlags::INCLUDE_RECORD_DATA;
    assert!(flags.contains(SzEntityFlags::INCLUDE_ENTITY_NAME));
    assert!(flags.contains(SzEntityFlags::INCLUDE_RECORD_DATA));
}
```

## Critical Test Requirements

1. **100% Test Coverage**: All public API methods must have corresponding tests
2. **No Mock Tests**: All tests must use actual Senzing native library
3. **Data Validation**: Comprehensive JSON structure validation
4. **Error Scenarios**: Test both success and failure paths
5. **Resource Cleanup**: Automatic cleanup of test resources
6. **Thread Safety**: Tests for concurrent operations where applicable
7. **Performance**: Basic performance validation for key operations

## Conclusion

The C# SDK test suite provides a comprehensive blueprint for Rust implementation with:
- **Systematic coverage** of all SDK components
- **Realistic test data** representing common use cases
- **Robust error handling** across multiple failure modes
- **Standardized patterns** for setup, execution, and cleanup
- **Multi-format support** for flexible data input

The Rust implementation should mirror these patterns while leveraging Rust's type system and error handling capabilities for even more robust testing.