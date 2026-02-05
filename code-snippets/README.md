# Senzing Rust SDK Code Snippets

This directory contains minimal, focused code examples demonstrating specific Senzing SDK operations.

## Structure

The snippets are organized by category for easy discovery:

```
snippets/
├── configuration/      # Configuration management examples
├── deleting/          # Record deletion examples
├── information/       # System information examples
├── initialization/    # Environment setup examples
├── loading/          # Record loading examples
├── redo/             # Redo processing examples
├── searching/        # Search and query examples
└── stewardship/      # Data stewardship examples
```

## Running Examples

Each snippet is a standalone `.rs` file that can be run directly:

```bash
# Run a specific snippet
cargo run --bin load_records
cargo run --bin delete_records
cargo run --bin search_records
cargo run --bin get_version
cargo run --bin register_data_sources

# List all available snippets
cargo run --bin <TAB><TAB>
```

## Snippet Design

Each snippet follows these principles:

- **Single Concept**: Demonstrates exactly one Senzing SDK operation
- **Minimal Code**: Only the essential code needed to accomplish the task
- **Clear Documentation**: Comments explain what each SDK method does
- **Self-Contained**: Uses simple `get_environment()` helper for setup
- **TEST Data Source**: Uses default "TEST" data source when possible

## Example Snippet Structure

```rust
//! Brief Description
//!
//! Key Senzing SDK concepts demonstrated:
//! - Specific SDK operations covered
//! - Method signatures shown

use sz_rust_sdk::prelude::*;
use serde_json::json;

fn main() -> SzResult<()> {
    // Step 1: Get environment
    let env = get_environment()?;

    // Step 2: Get SDK interface
    let engine = env.get_engine()?;

    // Step 3: Demonstrate SDK operation
    // method_name(parameters) - clear signature documentation
    engine.some_method("param1", "param2", None)?;

    Ok(())
}

fn get_environment() -> SzResult<std::sync::Arc<SzEnvironmentCore>> {
    sz_rust_sdk::helpers::ExampleEnvironment::initialize("example_name")
}
```

## Snippet Categories

### Configuration (2 snippets)

- `init_default_config.rs` - Initialize default configuration
- `register_data_sources.rs` - Register new data sources

### Deleting (2 snippets)

- `delete_records.rs` - Delete individual records
- `delete_via_loop.rs` - Bulk record deletion

### Information (3 snippets)

- `database_demo.rs` - Database information demo
- `get_license.rs` - Retrieve license information
- `get_version.rs` - Retrieve version information

### Initialization (3 snippets)

- `engine_priming.rs` - Prime engine for performance
- `environment_and_hubs.rs` - Access all SDK interfaces
- `purge_repository.rs` - Clean repository data

### Loading (3 snippets)

- `load_records.rs` - Load individual records
- `load_via_loop.rs` - Bulk record loading
- `load_via_loop_threadpool.rs` - Multi-threaded loading

### Redo (4 snippets)

- `load_with_redo_via_loop.rs` - Loading with redo processing
- `redo_continuous.rs` - Continuous redo processing
- `redo_continuous_via_futures.rs` - Async redo processing
- `redo_with_info_continuous.rs` - Redo with detailed info

### Searching (3 snippets)

- `search_records.rs` - Basic entity search
- `search_threadpool.rs` - Multi-threaded search
- `why_search.rs` - Search with explanations

### Stewardship (2 snippets)

- `force_resolve.rs` - Force entity resolution
- `force_unresolve.rs` - Force entity separation

## Total: 22 Code Snippets

All snippets demonstrate minimal, focused examples of Senzing SDK usage with clear documentation and simple execution.
