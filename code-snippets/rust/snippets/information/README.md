# Information Examples

This directory contains examples demonstrating how to retrieve system information, version details, and performance metrics from the Senzing SDK.

## Examples

### [get_version](get_version/)
**Purpose**: Retrieves comprehensive version and license information from the Senzing SDK.

**What it demonstrates**:
- SDK version information retrieval
- License details and limitations
- Build information and compatibility
- System configuration validation
- Product component access

**Key API calls**:
- `environment.get_product()`
- `product.get_version()`
- `product.get_license()`
- JSON parsing for structured information display

**Information Retrieved**:
1. **Version Information**:
   - API version
   - Native API version
   - Build version and date
   - Build number
   - Compatibility version

2. **License Information**:
   - Customer details
   - Contract information
   - License type and level
   - Record limits
   - Expiration dates

**Usage**:
```bash
cd get_version
cargo run
```

### [database_demo](database_demo/)
**Purpose**: Demonstrates database performance monitoring and system diagnostics capabilities.

**What it demonstrates**:
- Database performance statistics retrieval
- Connection pool monitoring
- Operation timing analysis
- System health checking
- Configuration status reporting

**Key API calls**:
- `environment.get_diagnostic()`
- `diagnostic.get_datastore_performance()`
- `diagnostic.check_datastore_performance()`
- `diagnostic.get_feature()`
- `engine.get_active_config_id()`
- `engine.export_json_entity_report()`

**Metrics Provided**:
1. **Database Operations**:
   - Insert/Update/Delete/Select statistics
   - Average, minimum, maximum execution times
   - Total operation counts

2. **Connection Pool**:
   - Maximum connections
   - Active connections
   - Available connections

3. **Configuration**:
   - Active configuration ID
   - Data source statistics
   - Entity and record counts

**Usage**:
```bash
cd database_demo
cargo run
```

## Common Patterns

### Product Information Access
```rust
let product = environment.get_product()?;
let version_json = product.get_version()?;
let license_json = product.get_license()?;
```

### JSON Information Processing
```rust
let version_info: Value = serde_json::from_str(&version_json)?;

if let Some(api_version) = version_info.get("API_VERSION").and_then(|v| v.as_str()) {
    println!("API Version: {}", api_version);
}
```

### Performance Monitoring
```rust
let diagnostic = environment.get_diagnostic()?;
let performance_json = diagnostic.get_datastore_performance()?;
let performance_info: Value = serde_json::from_str(&performance_json)?;
```

### Health Checking
```rust
let check_result = diagnostic.check_datastore_performance(1)?;
println!("Datastore Health: {}", if check_result == 0 { "PASSED" } else { "ISSUES DETECTED" });
```

## Version Information Structure

### API Version Data
```rust
{
    "API_VERSION": "4.0.0",
    "NATIVE_API_VERSION": "4.0.0",
    "BUILD_VERSION": "4.0.0.24001",
    "BUILD_DATE": "2024-01-15",
    "BUILD_NUMBER": "24001",
    "COMPATIBILITY_VERSION": {
        "CONFIG_VERSION": "4.0.0"
    }
}
```

### License Information
```rust
{
    "customer": "Example Customer",
    "contract": "Contract-12345",
    "issueDate": "2024-01-01",
    "licenseType": "EVAL",
    "licenseLevel": "STANDARD",
    "billing": "MONTHLY",
    "expireDate": "2024-12-31",
    "recordLimit": 1000000
}
```

## Performance Metrics Structure

### Database Performance
```rust
{
    "datastore": {
        "type": "PostgreSQL",
        "database": "senzing",
        "schema": "public"
    },
    "performance": {
        "inserts": {
            "count": 1000,
            "averageTime": 15.5,
            "minTime": 5.2,
            "maxTime": 45.1,
            "totalTime": 15500.0
        },
        "selects": {...},
        "updates": {...},
        "deletes": {...}
    },
    "connectionPool": {
        "maxConnections": 20,
        "activeConnections": 5,
        "availableConnections": 15
    }
}
```

## System Diagnostics

### Health Checks
- **Database Connectivity**: Verifies database connection status
- **Performance Thresholds**: Checks if operations meet performance criteria
- **Resource Availability**: Monitors connection pool and system resources
- **Configuration Validity**: Validates active configuration integrity

### Performance Monitoring
- **Operation Timing**: Tracks database operation performance
- **Throughput Analysis**: Measures records processed per time unit
- **Resource Utilization**: Monitors memory and connection usage
- **Bottleneck Identification**: Identifies performance constraints

## Use Cases

### System Administration
- Monitor system health and performance
- Validate license compliance
- Track resource utilization
- Performance tuning and optimization

### Development and Testing
- Verify SDK version compatibility
- Monitor development environment performance
- Validate configuration changes
- Debug performance issues

### Production Monitoring
- Continuous health monitoring
- Performance baseline establishment
- Capacity planning
- Issue detection and alerting

## Monitoring Best Practices

### Regular Health Checks
```rust
// Check datastore performance regularly
let health_status = diagnostic.check_datastore_performance(1)?;
if health_status != 0 {
    println!("WARNING: Datastore performance issues detected");
}
```

### Performance Baseline Establishment
```rust
// Establish performance baselines
let performance = diagnostic.get_datastore_performance()?;
// Log and track performance metrics over time
```

### License Monitoring
```rust
// Monitor license expiration and limits
let license_info: Value = serde_json::from_str(&product.get_license()?)?;
if let Some(expire_date) = license_info.get("expireDate") {
    // Check if license is approaching expiration
}
```

## Integration with Monitoring Systems

The examples demonstrate patterns for integrating with external monitoring systems:
- Structured JSON output for log aggregation
- Numeric metrics for time-series databases
- Health check results for alerting systems
- Performance statistics for dashboard visualization