# Senzing SDK Rust Code Snippets

This directory contains Rust code examples that demonstrate how to use the Senzing SDK for entity resolution. These examples are direct Rust equivalents of the C# v4 code snippets, showcasing the same functionality using idiomatic Rust patterns.

## Directory Structure

The examples are organized into functional categories:

- **[initialization/](initialization/)** - Environment setup and engine initialization
- **[configuration/](configuration/)** - Configuration management and data source registration
- **[loading/](loading/)** - Record loading patterns and batch processing
- **[searching/](searching/)** - Entity search and analysis operations
- **[information/](information/)** - System information and diagnostics
- **[deleting/](deleting/)** - Record deletion operations

## Prerequisites

- Rust 2024 edition or later
- Senzing SDK installed at `/opt/senzing/er/`
- Environment variable `SENZING_ENGINE_CONFIGURATION_JSON` configured

## Running Examples

Each example is a self-contained Rust project. To run any example:

```bash
cd <category>/<example_name>
cargo run
```

For example:
```bash
cd initialization/engine_priming
cargo run
```

## Common Patterns

All examples follow these Rust patterns:

### Environment Management
```rust
// Initialize Senzing environment
let environment = ExampleEnvironment::initialize(env!("CARGO_PKG_NAME"))?;

// Get components
let engine = environment.get_engine()?;
let config_manager = environment.get_config_manager()?;

// Always cleanup at the end
ExampleEnvironment::cleanup()?;
```

### Error Handling
```rust
// All operations return SzResult<T>
fn main() -> SzResult<()> {
    // Operations that can fail
    engine.add_record("DATA_SOURCE", "RECORD_ID", &json_data, None)?;
    Ok(())
}
```

### Flag Usage
```rust
// Use None for no flags
engine.add_record(data_source, record_id, json_data, None)?;

// Use Some(flags) for specific behavior
engine.search_by_attributes(
    search_json,
    None,
    Some(SzFlags::SEARCH_BY_ATTRIBUTES_ALL)
)?;
```

## Key Differences from C# Examples

1. **Memory Management**: Rust uses RAII instead of explicit disposal
2. **Error Handling**: `Result<T, E>` types instead of exceptions
3. **Null Safety**: `Option<T>` types instead of nullable references
4. **Resource Cleanup**: Automatic with `Drop` trait, explicit with `cleanup()`

## Example Categories

### Initialization
- **engine_priming**: Optimizes engine performance for better search speeds
- **environment_and_hubs**: Demonstrates accessing all SDK components
- **purge_repository**: Shows how to clear repository data

### Configuration
- **init_default_config**: Creates initial Senzing configuration
- **register_data_sources**: Adds data sources with race condition handling

### Loading
- **load_records**: Basic record loading with simple error handling
- **load_via_loop**: Advanced batch loading with retry logic and statistics

### Searching
- **search_records**: Entity search with result processing
- **why_search**: Advanced analysis explaining search results and entity matching

### Information
- **get_version**: Retrieves SDK version and license information
- **database_demo**: Shows database performance metrics and statistics

### Deleting
- **delete_records**: Record deletion with verification

## Building All Examples

To verify all examples compile:

```bash
find . -name "Cargo.toml" -execdir cargo check \;
```

## Contributing

When adding new examples:
1. Follow the existing directory structure
2. Use Rust 2024 edition
3. Include comprehensive error handling
4. Add documentation comments explaining the purpose
5. Ensure examples compile and run successfully