# Senzing Rust SDK

A Rust SDK for the Senzing Entity Resolution Engine, providing safe and idiomatic Rust bindings to the native Senzing library.

## Overview

This SDK provides Rust developers with access to Senzing's powerful entity resolution capabilities through a type-safe, memory-safe interface. It follows the same architectural patterns as the official C# SDK while leveraging Rust's ownership system and error handling.

## Features

- **Type-Safe FFI Bindings** - Safe Rust wrappers around native Senzing functions
- **Thread Pool Scaling** - Real OS thread scaling with per-thread engine instances
- **High Performance** - Thread-safe engine operations designed for parallel processing
- **Comprehensive Error Handling** - Structured error types with detailed error messages
- **Database Isolation** - Automatic test database isolation for concurrent testing
- **Entity Resolution** - Add, search, and resolve entities across data sources
- **Configuration Management** - Manage Senzing configurations and data sources
- **Performance Diagnostics** - Built-in performance testing and monitoring
- **Memory Safety** - No manual memory management required

## Architecture

The SDK is organized into several core components:

- **`SzEnvironment`** - Main entry point and factory for SDK components
- **`SzEngine`** - Core entity resolution operations (add, search, resolve)
- **`SzConfig`** - Configuration management and data source registration
- **`SzConfigManager`** - Configuration lifecycle management
- **`SzDiagnostic`** - System diagnostics and performance testing
- **`SzProduct`** - Version and license information

## Prerequisites

- Rust 2024 edition or later
- Senzing v4 installed at `/opt/senzing/er/`
- SQLite3 (for database operations)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sz-rust-sdk = "0.1.0"
```

## Quick Start

```rust
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Initialize the Senzing environment
    let env = ExampleEnvironment::initialize("my-app")?;

    // Get the engine for entity operations
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    // Add a record
    let record = r#"{"NAME_FULL": "John Smith", "EMAIL_ADDRESS": "john@example.com"}"#;
    let result = engine.add_record("CUSTOMERS", "CUST001", record, None)?;

    // Search for similar entities
    let search_attrs = r#"{"NAME_FULL": "Jon Smith"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;

    println!("Search results: {}", results);

    // Clean up
    ExampleEnvironment::cleanup()?;
    Ok(())
}
```

## Examples

The SDK includes comprehensive examples demonstrating various use cases:

### Basic Operations
- **`minimal_test`** - Basic SDK functionality verification
- **`basic_usage`** - Simple entity operations
- **`complete_workflow`** - End-to-end entity resolution workflow

### Entity Management
- **`load_records`** - Loading and managing entity records
- **`delete_records`** - Deleting records and observing impact
- **`search_records`** - Searching and finding entities

### Configuration
- **`register_data_sources`** - Adding new data sources
- **`manage_configuration`** - Configuration lifecycle management

### Performance & Diagnostics
- **`check_datastore_performance`** - Performance benchmarking
- **`engine_priming`** - Engine optimization

Run examples with:
```bash
cargo run --example minimal_test
cargo run --example register_data_sources
cargo run --example load_records
```

## Code Snippets

The SDK provides comprehensive code snippets organized by category, providing complete feature parity with C# v4 code snippets:

### Information Retrieval
- **`information/get_version`** - Retrieve SDK and engine version information
- **`information/get_license`** - Display licensing details and limits
- **`information/database_demo`** - Database operations demonstration

### Initialization & Configuration
- **`initialization/environment_and_hubs`** - Environment setup and management
- **`initialization/engine_priming`** - Engine performance optimization
- **`initialization/purge_repository`** - Repository cleanup operations
- **`configuration/init_default_config`** - Default configuration setup
- **`configuration/register_data_sources`** - Data source registration

### Loading Operations
- **`loading/load_records`** - Basic record loading
- **`loading/load_via_loop`** - Batch loading with monitoring
- **`loading/load_via_loop_threadpool`** - Thread pool batch loading

### Search Operations
- **`searching/search_records`** - Entity search by attributes
- **`searching/search_threadpool`** - Thread pool search operations
- **`searching/why_search`** - Understanding search results

### Deletion Operations
- **`deleting/delete_records`** - Basic record deletion
- **`deleting/delete_via_loop`** - Batch deletion with retry logic

### Redo Processing
- **`redo/load_with_redo_via_loop`** - Loading with redo processing phases
- **`redo/redo_continuous`** - Continuous redo monitoring
- **`redo/redo_continuous_via_futures`** - Thread pool redo processing
- **`redo/redo_with_info_continuous`** - Comprehensive redo tracking

### Stewardship Operations
- **`stewardship/force_resolve`** - Manual entity resolution forcing
- **`stewardship/force_unresolve`** - Manual entity separation

Run all code snippets with:
```bash
./run_all_code_snippets.sh
```

Run individual snippets with:
```bash
cd code-snippets/rust/snippets/[category]/[example]
cargo run
```

## Development

### Building
```bash
cargo build
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation
```bash
# Build and open documentation
cargo doc --open
```

## Error Handling

The SDK uses structured error types for comprehensive error handling:

```rust
use sz_rust_sdk::{SzResult, SzError};

fn example() -> SzResult<()> {
    match some_operation() {
        Ok(result) => println!("Success: {}", result),
        Err(SzError::Configuration { message, .. }) => {
            println!("Configuration error: {}", message);
        }
        Err(SzError::Engine { message, .. }) => {
            println!("Engine error: {}", message);
        }
        Err(e) => println!("Other error: {}", e),
    }
    Ok(())
}
```

## Database Isolation

The SDK automatically provides database isolation for testing:

- Each test uses a unique SQLite database in `/tmp/`
- Databases are automatically created and cleaned up
- No manual database management required
- Safe for concurrent test execution

## Configuration

Senzing configuration can be provided through:

1. **Environment Variable**:
   ```bash
   export SENZING_ENGINE_CONFIGURATION_JSON='{% raw %}{"PIPELINE":{"CONFIGPATH":"/etc/opt/senzing",...}}{% endraw %}'
   ```

2. **Automatic Setup**: The SDK will automatically configure appropriate settings for development and testing.

## Contributing

1. Ensure all tests pass: `cargo test`
2. Run code quality checks: `cargo clippy` and `cargo fmt`
3. All examples must run successfully
4. Follow Rust best practices and the existing code patterns

## License

[License information would go here]

## Support

For questions about Senzing entity resolution concepts, see the [Senzing documentation](https://senzing.zendesk.com/).

For issues with this Rust SDK, please file an issue in this repository.
