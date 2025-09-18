# Running Code Snippets

This document describes how to run the Rust code snippets for the Senzing SDK.

## Prerequisites

1. **Senzing SDK**: Ensure Senzing is installed at `/opt/senzing/er/`
2. **Rust**: Install Rust 2024 edition
3. **Dependencies**: All dependencies are managed through the central `Cargo.toml`

## Simplified Directory Structure

The code snippets are organized by category in a simple, flat structure:

```
code-snippets/
├── configuration/
│   ├── init_default_config.rs
│   └── register_data_sources.rs
├── deleting/
│   ├── delete_records.rs
│   └── delete_via_loop.rs
├── information/
│   ├── database_demo.rs
│   ├── get_license.rs
│   └── get_version.rs
├── initialization/
│   ├── engine_priming.rs
│   ├── environment_and_hubs.rs
│   └── purge_repository.rs
├── loading/
│   ├── load_records.rs
│   ├── load_via_loop.rs
│   └── load_via_loop_threadpool.rs
├── redo/
│   ├── load_with_redo_via_loop.rs
│   ├── redo_continuous.rs
│   ├── redo_continuous_via_futures.rs
│   └── redo_with_info_continuous.rs
├── searching/
│   ├── search_records.rs
│   ├── search_threadpool.rs
│   └── why_search.rs
├── stewardship/
│   ├── force_resolve.rs
│   └── force_unresolve.rs
├── Cargo.toml          # Central build configuration
└── README.md           # Detailed snippet documentation
```

Each `.rs` file is a standalone, complete example that can be run directly.

## Running Individual Snippets

Navigate to the code-snippets directory and run any snippet by name:

```bash
cd code-snippets

# Run specific snippets
cargo run --bin load_records
cargo run --bin delete_records
cargo run --bin search_records
cargo run --bin get_version
cargo run --bin register_data_sources

# List all available snippets
cargo run --bin <TAB><TAB>
```

## Running All Snippets

Use the provided script to run all snippets automatically:

```bash
./run_all_code_snippets.sh
```

This script will:
- Run all 22 code snippets in category order
- Execute each snippet with a 30-second timeout
- Provide real-time colored output (✓ success, ✗ failed, ⏰ timeout)
- Generate a summary report with success rate
- Save detailed logs to `code_snippets_run.log`
- Save summary results to `code_snippets_results.txt`

## Expected Results

All snippets should run successfully and demonstrate specific Senzing SDK capabilities:

### Configuration (2 snippets)
- **init_default_config**: Initialize default Senzing configuration
- **register_data_sources**: Register new data sources in configuration

### Deleting (2 snippets)
- **delete_records**: Delete individual records with verification
- **delete_via_loop**: Bulk record deletion with error handling

### Information (3 snippets)
- **database_demo**: Database information and statistics
- **get_license**: Retrieve Senzing license information
- **get_version**: Retrieve Senzing version information

### Initialization (3 snippets)
- **engine_priming**: Prime the Senzing engine for optimal performance
- **environment_and_hubs**: Initialize environment and access all interfaces
- **purge_repository**: Safely purge all data from repository

### Loading (3 snippets)
- **load_records**: Load individual records with simple error handling
- **load_via_loop**: Bulk record loading with comprehensive error handling
- **load_via_loop_threadpool**: Multi-threaded record loading

### Redo (4 snippets)
- **load_with_redo_via_loop**: Loading with redo processing
- **redo_continuous**: Continuous redo processing
- **redo_continuous_via_futures**: Future-based redo processing
- **redo_with_info_continuous**: Redo processing with detailed information

### Searching (3 snippets)
- **search_records**: Basic entity search functionality
- **search_threadpool**: Multi-threaded search operations
- **why_search**: Search with explanations and analysis

### Stewardship (2 snippets)
- **force_resolve**: Force entity resolution
- **force_unresolve**: Force entity separation

## Snippet Design Philosophy

Each snippet follows these principles:

- **Single Concept**: Demonstrates exactly one Senzing SDK operation
- **Minimal Code**: Only essential code needed (~50-60 lines vs previous ~120 lines)
- **Clear Documentation**: Method signatures and SDK concepts documented
- **Self-Contained**: Simple `get_environment()` helper handles setup complexity
- **Default Data Source**: Uses "TEST" data source when possible (no setup required)

## Troubleshooting

### Common Issues

1. **Senzing Not Found**: Ensure Senzing is installed at `/opt/senzing/er/`
2. **Permission Issues**: Make sure you have read access to Senzing installation
3. **Database Issues**: The examples use temporary SQLite databases automatically created
4. **Timeout Issues**: Some operations may take longer than 30 seconds on slower systems

### Environment Variables

The snippets use automatic database configuration, but you can override with:

```bash
export SENZING_ENGINE_CONFIGURATION_JSON='{"PIPELINE":{"CONFIGPATH":"/etc/opt/senzing","RESOURCEPATH":"/opt/senzing/er/resources","SUPPORTPATH":"/opt/senzing/data"},"SQL":{"CONNECTION":"sqlite3://na:na@/tmp/G2C.db"}}'
```

## Performance Notes

- Initial compilation may take a few seconds
- Subsequent runs are fast due to cached compilation
- Each snippet creates an isolated test database
- Threading examples demonstrate real OS thread scaling
- Database operations are optimized for demonstration purposes

## Target: 22/22 Success Rate

The goal is for all 22 code snippets to run successfully, demonstrating complete Senzing SDK functionality coverage.