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

- Rust 1.88+ (2024 edition)
- Senzing v4 SDK
- SQLite3 (for database operations)

### Platform-Specific Installation

#### macOS (Homebrew)

```bash
brew install senzing/tap/senzing
```

Senzing installs to `/opt/homebrew/opt/senzing/runtime/er/` (Apple Silicon) or `/usr/local/opt/senzing/runtime/er/` (Intel).

**Required environment variables:**
```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib
export SENZING_CONFIGPATH=/opt/homebrew/opt/senzing/runtime/er/etc
export SENZING_RESOURCEPATH=/opt/homebrew/opt/senzing/runtime/er/resources
export SENZING_SUPPORTPATH=/opt/homebrew/opt/senzing/runtime/data
```

#### Linux

Install Senzing following the [official instructions](https://senzing.com/). Senzing typically installs to `/opt/senzing/er/`.

**Required environment variables:**
```bash
export LD_LIBRARY_PATH=/opt/senzing/er/lib
export SENZING_CONFIGPATH=/opt/senzing/er/etc
export SENZING_RESOURCEPATH=/opt/senzing/er/resources
export SENZING_SUPPORTPATH=/opt/senzing/er/data
```

### Build Configuration

The SDK's `build.rs` automatically detects Senzing in these locations (in order):
1. `SENZING_LIB_PATH` environment variable (if set)
2. Homebrew location: `/opt/homebrew/opt/senzing/runtime/er/lib`
3. Intel Homebrew: `/usr/local/opt/senzing/runtime/er/lib`
4. Default Linux: `/opt/senzing/er/lib`

To override, set `SENZING_LIB_PATH` before building:
```bash
export SENZING_LIB_PATH=/custom/path/to/senzing/lib
cargo build
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sz-rust-sdk = { git = "https://github.com/brianmacy/sz-rust-sdk", tag = "v0.1.0" }
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

1. **Environment Variables** (recommended for development):

   **macOS (Homebrew):**
   ```bash
   export SENZING_ENGINE_CONFIGURATION_JSON='{
     "PIPELINE": {
       "CONFIGPATH": "/opt/homebrew/opt/senzing/runtime/er/etc",
       "RESOURCEPATH": "/opt/homebrew/opt/senzing/runtime/er/resources",
       "SUPPORTPATH": "/opt/homebrew/opt/senzing/runtime/data"
     },
     "SQL": {"CONNECTION": "sqlite3://na:na@/tmp/senzing.db"}
   }'
   ```

   **Linux:**
   ```bash
   export SENZING_ENGINE_CONFIGURATION_JSON='{
     "PIPELINE": {
       "CONFIGPATH": "/opt/senzing/er/etc",
       "RESOURCEPATH": "/opt/senzing/er/resources",
       "SUPPORTPATH": "/opt/senzing/er/data"
     },
     "SQL": {"CONNECTION": "sqlite3://na:na@/tmp/senzing.db"}
   }'
   ```

2. **Automatic Setup**: The SDK's `ExampleEnvironment` helper automatically configures appropriate settings for development and testing, detecting the platform and paths.

## Contributing

1. Ensure all tests pass: `cargo test`
2. Run code quality checks: `cargo clippy` and `cargo fmt`
3. All examples must run successfully
4. Follow Rust best practices and the existing code patterns

## License

Apache-2.0. See [LICENSE](LICENSE) for details.

## Support

For questions about Senzing entity resolution concepts, see the [Senzing documentation](https://senzing.zendesk.com/).

For issues with this Rust SDK, please file an issue in this repository.
