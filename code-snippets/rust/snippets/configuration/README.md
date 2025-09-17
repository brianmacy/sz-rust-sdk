# Configuration Examples

This directory contains examples demonstrating Senzing configuration management, including setting up initial configurations and registering data sources.

## Examples

### [init_default_config](init_default_config/)
**Purpose**: Creates and initializes the default Senzing configuration required for entity resolution operations.

**What it demonstrates**:
- Configuration manager initialization
- Default configuration creation
- Configuration export and validation
- Configuration ID management
- Atomic configuration replacement

**Key API calls**:
- `environment.get_config_manager()`
- `config_manager.create_default_config()`
- `config_manager.add_config()`
- `config_manager.set_default_config_id()`
- `config.export_config()`

**Usage**:
```bash
cd init_default_config
cargo run
```

### [register_data_sources](register_data_sources/)
**Purpose**: Demonstrates how to register multiple data sources in the Senzing configuration with proper race condition handling.

**What it demonstrates**:
- Data source registration in configuration
- Race condition handling with retry logic
- Atomic configuration updates
- Configuration versioning and replacement
- Concurrent modification protection

**Key API calls**:
- `config_manager.get_default_config_id()`
- `config_manager.create_config_by_id()`
- `config.add_data_source()`
- `config_manager.replace_default_config_id()`
- Retry logic implementation

**Usage**:
```bash
cd register_data_sources
cargo run
```

## Common Patterns

All configuration examples follow these patterns:

### Configuration Manager Access
```rust
let config_manager = environment.get_config_manager()?;
```

### Atomic Configuration Updates
```rust
// Get current configuration
let config_id = config_manager.get_default_config_id()?;
let config = config_manager.create_config_by_id(config_id)?;

// Modify configuration
config.add_data_source("DATA_SOURCE_NAME")?;

// Export and save new configuration
let modified_config = config.export_config()?;
let new_config_id = config_manager.add_config(&modified_config, "Description")?;

// Replace default atomically
config_manager.replace_default_config_id(config_id, new_config_id)?;
```

### Race Condition Handling
```rust
let mut retry_count = 0;
const MAX_RETRIES: usize = 10;

while !config_replaced && retry_count < MAX_RETRIES {
    retry_count += 1;
    match attempt_configuration_update() {
        Ok(()) => config_replaced = true,
        Err(e) => {
            if retry_count >= MAX_RETRIES {
                return Err(e);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
```

## Data Sources

Common data sources you might register:
- `CUSTOMERS` - Customer records
- `EMPLOYEES` - Employee records
- `WATCHLIST` - Watch list entries
- `VENDORS` - Vendor information
- Custom data source names as needed

## Race Condition Handling

Configuration modifications can conflict when multiple processes attempt updates simultaneously. The examples demonstrate:

1. **Detection**: Catching configuration version conflicts
2. **Retry Logic**: Implementing exponential backoff
3. **Maximum Attempts**: Preventing infinite retry loops
4. **Atomic Operations**: Using `replace_default_config_id()` for safe updates

## Configuration Versioning

Senzing configurations are versioned and immutable:
- Each modification creates a new configuration version
- Old configurations remain accessible by ID
- Default configuration ID points to the active version
- Atomic replacement prevents configuration corruption

## Best Practices

1. **Always use atomic replacement** for configuration updates
2. **Implement retry logic** for concurrent modification scenarios
3. **Validate configurations** before making them default
4. **Use descriptive comments** when adding configurations
5. **Monitor configuration IDs** for tracking changes