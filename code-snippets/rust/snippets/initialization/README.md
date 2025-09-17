# Initialization Examples

This directory contains examples demonstrating how to initialize and prepare the Senzing SDK environment for optimal performance.

## Examples

### [engine_priming](engine_priming/)
**Purpose**: Optimizes the Senzing engine for better search performance by pre-loading internal structures.

**What it demonstrates**:
- Environment initialization using the singleton pattern
- Engine priming for performance optimization
- Proper resource cleanup
- Performance timing measurement

**Key API calls**:
- `ExampleEnvironment::initialize()`
- `environment.get_engine()`
- `engine.prime_engine()`
- `ExampleEnvironment::cleanup()`

**Usage**:
```bash
cd engine_priming
cargo run
```

### [environment_and_hubs](environment_and_hubs/)
**Purpose**: Shows how to access all major SDK components (hubs) through the environment.

**What it demonstrates**:
- Accessing the Product component for version information
- Accessing the ConfigManager for configuration operations
- Accessing the Diagnostic component for system monitoring
- Accessing the Engine for entity resolution operations
- Component validation and error handling

**Key API calls**:
- `environment.get_product()`
- `environment.get_config_manager()`
- `environment.get_diagnostic()`
- `environment.get_engine()`

**Usage**:
```bash
cd environment_and_hubs
cargo run
```

### [purge_repository](purge_repository/)
**Purpose**: Demonstrates how to completely clear the Senzing repository of all data while preserving configuration.

**What it demonstrates**:
- Repository purging operations
- Configuration preservation during data clearing
- Confirmation of purge operations
- Safe repository reset procedures

**Key API calls**:
- `engine.purge_repository()`
- Repository state verification

**Usage**:
```bash
cd purge_repository
cargo run
```

## Common Patterns

All initialization examples follow these patterns:

### Environment Setup
```rust
let environment = ExampleEnvironment::initialize(env!("CARGO_PKG_NAME"))?;
```

### Component Access
```rust
let engine = environment.get_engine()?;
let product = environment.get_product()?;
let config_manager = environment.get_config_manager()?;
let diagnostic = environment.get_diagnostic()?;
```

### Cleanup
```rust
ExampleEnvironment::cleanup()?;
```

## Prerequisites

- Senzing SDK properly installed
- `SENZING_ENGINE_CONFIGURATION_JSON` environment variable configured
- Appropriate database connectivity

## Performance Considerations

- **Engine Priming**: Significantly improves search performance but requires initial processing time
- **Component Caching**: The environment singleton pattern ensures efficient component reuse
- **Resource Management**: Always call cleanup to prevent resource leaks