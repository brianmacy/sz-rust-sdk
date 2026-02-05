# Senzing Rust SDK Examples

This directory contains comprehensive examples demonstrating various features and capabilities of the Senzing Rust SDK. These examples are modeled after the [C# code snippets](https://github.com/Senzing/code-snippets-v4/tree/main/csharp) and showcase the same functionality using Rust.

## Prerequisites

Before running these examples, ensure you have:

1. **Senzing SDK Installed**: The native Senzing libraries must be installed and accessible
2. **Environment Configuration**: Set the `SENZING_ENGINE_CONFIGURATION_JSON` environment variable with your Senzing configuration
3. **Rust Environment**: Rust 1.70+ with Cargo
4. **Dependencies**: All required dependencies will be automatically downloaded by Cargo

## Environment Setup

Set the required environment variable:

```bash
export SENZING_ENGINE_CONFIGURATION_JSON='{% raw %}{"PIPELINE": {"CONFIGPATH": "/path/to/config", "RESOURCEPATH": "/path/to/resources", "SUPPORTPATH": "/path/to/support"}}{% endraw %}'
```

Or create a `.env` file in the project root:

```env
SENZING_ENGINE_CONFIGURATION_JSON={% raw %}{"PIPELINE": {"CONFIGPATH": "/path/to/config", "RESOURCEPATH": "/path/to/resources", "SUPPORTPATH": "/path/to/support"}}{% endraw %}
```

## Running Examples

Run any example using Cargo:

```bash
# Run a specific example
cargo run --example environment_and_hubs

# Run with release optimizations
cargo run --release --example load_records

# List all available examples
cargo run --example 2>&1 | grep "available examples"
```

## Example Categories

### 🚀 Initialization

Examples demonstrating SDK initialization and environment setup.

| Example                | Description                              | Run Command                                |
| ---------------------- | ---------------------------------------- | ------------------------------------------ |
| `environment_and_hubs` | Initialize SDK and get component handles | `cargo run --example environment_and_hubs` |
| `engine_priming`       | Prime the engine for optimal performance | `cargo run --example engine_priming`       |

### 📥 Loading

Examples showing how to load records into the Senzing repository.

| Example          | Description                                    | Run Command                          |
| ---------------- | ---------------------------------------------- | ------------------------------------ |
| `load_records`   | Basic record loading with various data types   | `cargo run --example load_records`   |
| `load_with_info` | Load records with detailed resolution tracking | `cargo run --example load_with_info` |

### 🔍 Searching

Examples demonstrating entity search capabilities.

| Example          | Description                                          | Run Command                          |
| ---------------- | ---------------------------------------------------- | ------------------------------------ |
| `search_records` | Search entities by various attribute combinations    | `cargo run --example search_records` |
| `why_search`     | Analyze why entities were returned in search results | `cargo run --example why_search`     |

### ⚙️ Configuration

Examples showing configuration management and data source registration.

| Example                 | Description                                 | Run Command                                 |
| ----------------------- | ------------------------------------------- | ------------------------------------------- |
| `register_data_sources` | Add data sources to Senzing configuration   | `cargo run --example register_data_sources` |
| `manage_configuration`  | Comprehensive configuration management demo | `cargo run --example manage_configuration`  |

### ℹ️ Information

Examples for retrieving system and product information.

| Example                       | Description                                 | Run Command                                       |
| ----------------------------- | ------------------------------------------- | ------------------------------------------------- |
| `get_version`                 | Get product version and license information | `cargo run --example get_version`                 |
| `check_datastore_performance` | Run performance tests on the datastore      | `cargo run --example check_datastore_performance` |

### 🗑️ Deleting

Examples demonstrating record deletion and impact analysis.

| Example          | Description                                         | Run Command                          |
| ---------------- | --------------------------------------------------- | ------------------------------------ |
| `delete_records` | Delete records and observe entity resolution impact | `cargo run --example delete_records` |

### 🎯 Complete Workflow

A comprehensive example demonstrating all major SDK capabilities in sequence.

| Example             | Description                                                                      | Run Command                             |
| ------------------- | -------------------------------------------------------------------------------- | --------------------------------------- |
| `complete_workflow` | End-to-end demo: initialization → configuration → loading → searching → analysis | `cargo run --example complete_workflow` |

## Example Features

### Common Patterns

All examples demonstrate these common Rust SDK patterns:

- **Environment Management**: Proper initialization and cleanup
- **Error Handling**: Comprehensive error handling with `SzResult<T>`
- **Resource Cleanup**: Automatic resource management with RAII
- **JSON Processing**: Parsing and displaying Senzing JSON responses
- **Flag Usage**: Proper use of bitflags for operation control

### Error Handling

Examples show proper error handling:

```rust
use sz_rust_sdk::prelude::*;

fn example() -> SzResult<()> {
    let env = SzEnvironmentCore::new("example", &settings, false)?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    match engine.add_record("DS", "ID", "{}", None) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

### Configuration Patterns

Examples demonstrate configuration best practices:

```rust
// Race-condition safe configuration updates
let mut replaced = false;
while !replaced {
    let config_id = config_mgr.get_default_config_id()?;
    let config = config_mgr.create_config_from_id(config_id)?;

    // Make changes...
    config.register_data_source("NEW_DS")?;

    let new_config = config.export()?;
    let new_id = config_mgr.register_config(&new_config, Some("comment"))?;

    config_mgr.replace_default_config_id(config_id, new_id)?;
    replaced = true;
}
```

## Testing

Many examples include unit tests. Run them with:

```bash
# Run all example tests
cargo test --examples

# Run tests for a specific example
cargo test --example load_records

# Run tests with output
cargo test --examples -- --nocapture
```

## Troubleshooting

### Common Issues

1. **Missing Environment Variable**

   ```
   Error: Configuration error: Unable to get SENZING_ENGINE_CONFIGURATION_JSON environment variable
   ```

   Solution: Set the `SENZING_ENGINE_CONFIGURATION_JSON` environment variable

2. **Native Library Not Found**

   ```
   Error: FFI error: Failed to load native library
   ```

   Solution: Ensure Senzing native libraries are installed and in your library path

3. **Permission Errors**

   ```
   Error: Database error: Permission denied
   ```

   Solution: Check file permissions for Senzing data directories

4. **Configuration Conflicts**
   ```
   Error: Retryable error: Configuration conflict
   ```
   Solution: Examples handle this automatically with retry logic

### Debug Mode

Run examples with debug logging:

```bash
RUST_LOG=debug cargo run --example environment_and_hubs
```

## Advanced Usage

### Custom Data Sources

Modify examples to use your own data sources:

```rust
let data_sources = vec![
    "YOUR_CUSTOMERS",
    "YOUR_EMPLOYEES",
    "YOUR_PARTNERS"
];
```

### Performance Tuning

For production use, consider:

- Using release builds: `cargo run --release --example load_records`
- Adjusting engine priming duration based on your dataset size
- Monitoring performance with the datastore performance checker

### Integration

These examples can be integrated into larger applications:

```rust
use sz_rust_sdk::prelude::*;

pub struct MyApplication {
    env: SzEnvironmentCore,
}

impl MyApplication {
    pub fn new(settings: &str) -> SzResult<Self> {
        let env = SzEnvironmentCore::new("my-app", settings, false)?;
        Ok(Self { env })
    }

    pub fn load_customer(&mut self, customer_data: &str) -> SzResult<String> {
        let engine = ExampleEnvironment::get_engine_with_setup(&self.env)?;
        engine.add_record("CUSTOMERS", "ID", customer_data, None)
    }
}
```

## Contributing

When adding new examples:

1. Follow the existing naming convention
2. Include comprehensive documentation
3. Add unit tests where appropriate
4. Update this README with the new example
5. Ensure examples work with different data scenarios

## References

- [Senzing C# Code Snippets](https://github.com/Senzing/code-snippets-v4/tree/main/csharp) - Original examples these are based on
- [Senzing SDK Documentation](https://docs.senzing.com/) - Official Senzing documentation
- [Rust SDK API Documentation](../target/doc/sz_rust_sdk/index.html) - Generated API docs (run `cargo doc --open`)

## License

These examples are provided under the same license as the Senzing Rust SDK.
