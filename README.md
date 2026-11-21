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
- **Datastore Snapshot / Restore** - Persist an `internal://` in-memory datastore to a portable file and warm-start a later run without re-ingesting the source data
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
- Senzing v4.5+ (SzLogging needs `libSzLogging.h`, first shipped in 4.5.0; `ExampleEnvironment` uses the `internal://` in-memory database)

### Platform-Specific Installation

#### macOS (Homebrew)

Install the official Senzing SDK cask:

```bash
brew tap senzing/senzingsdk https://github.com/Senzing/homebrew-senzingsdk
brew install --cask senzingsdk
```

Senzing installs to `/opt/homebrew/opt/senzing/er/` (Apple Silicon) or `/usr/local/opt/senzing/er/` (Intel).

**Required environment variables (for library loading):**

```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/er/lib
```

The macOS 4.3 cask is missing rpath entries for its OpenSSL and SQLite dependencies.
The SDK's `build.rs` handles this automatically, but at runtime you may also need:

```bash
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/senzing/er/lib:/opt/homebrew/opt/sqlite/lib:/opt/homebrew/opt/openssl@3/lib"
```

Or source the provided setup script which handles this:

```bash
source "$(brew --prefix)/opt/senzing/er/setupEnv"
```

#### Windows (Scoop)

Install the Senzing SDK via [Scoop](https://scoop.sh/):

```pwsh
scoop bucket add senzingsdk https://github.com/Senzing/scoop-senzingsdk
scoop install senzingsdk/senzingsdk
```

Scoop automatically sets `SENZING_DIR` and adds the library directory to `PATH`.

#### Linux

Install Senzing following the [official instructions](https://senzing.com/). Senzing typically installs to `/opt/senzing/er/`.

**Required environment variable (for library loading):**

```bash
export LD_LIBRARY_PATH=/opt/senzing/er/lib
```

Senzing engine configuration is passed programmatically as a JSON string — see [Quick Start](#quick-start) and the [Senzing engine configuration tutorial](https://www.senzing.com/docs/tutorials/senzing_engine_config/).

### Build Configuration

The SDK's `build.rs` automatically detects Senzing in these locations (in order):

1. `SENZING_LIB_PATH` environment variable (if set)
2. Official Homebrew cask: `/opt/homebrew/opt/senzing/er/lib`
3. Official Homebrew cask (Intel): `/usr/local/opt/senzing/er/lib`
4. Legacy unofficial Homebrew tap: `.../senzing/runtime/er/lib`
5. Default Linux: `/opt/senzing/er/lib`

On macOS, `build.rs` also adds Homebrew's `sqlite` and `openssl@3` library paths
to resolve missing rpath entries in the Senzing 4.3 cask.

To override, set `SENZING_LIB_PATH` before building:

```bash
export SENZING_LIB_PATH=/custom/path/to/senzing/lib
cargo build
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sz-rust-sdk = { git = "https://github.com/brianmacy/sz-rust-sdk", tag = "v4.5.0" }
```

### Versioning

The SDK version tracks the minimum Senzing SDK version it supports: **major.minor** matches the Senzing SDK version, **patch** is Rust SDK-specific. For example, `4.3.1` requires Senzing SDK v4.3+ and is the first Rust SDK patch release for that series.

## Quick Start

```rust
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    // Build settings JSON (paths vary by platform - see Prerequisites above)
    let settings = r#"{
        "PIPELINE": {
            "CONFIGPATH": "/opt/homebrew/opt/senzing/er/resources/templates",
            "RESOURCEPATH": "/opt/homebrew/opt/senzing/er/resources",
            "SUPPORTPATH": "/opt/homebrew/opt/senzing/data"
        },
        "SQL": {"CONNECTION": "internal://"}
    }"#;

    // Initialize - SzEnvironmentCore is the only concrete type you use directly
    let env = SzEnvironmentCore::get_instance("my-app", settings, false)?;

    // All other interfaces are traits (Box<dyn SzEngine>, Box<dyn SzConfig>, etc.)
    let engine = env.get_engine()?;
    let config_mgr = env.get_config_manager()?;

    // Register a data source via config
    let config = config_mgr.create_config()?;
    config.register_data_source("CUSTOMERS")?;
    let config_json = config.export()?;
    config_mgr.set_default_config(&config_json, Some("Added CUSTOMERS"))?;

    // Add a record
    let record = r#"{"NAME_FULL": "John Smith", "EMAIL_ADDRESS": "john@example.com"}"#;
    engine.add_record("CUSTOMERS", "CUST001", record, None)?;

    // Search for similar entities
    let search_attrs = r#"{"NAME_FULL": "Jon Smith"}"#;
    let results = engine.search_by_attributes(search_attrs, None, None)?;

    println!("Search results: {}", results);
    Ok(())
}
```

## Usage Pattern

The SDK follows a "concrete entry point, trait interfaces" pattern:

| Component           | Type                               | Purpose                                          |
| ------------------- | ---------------------------------- | ------------------------------------------------ |
| `SzEnvironmentCore` | Concrete struct                    | Entry point - use `get_instance()` to initialize |
| `SzEngine`          | Trait (`Box<dyn SzEngine>`)        | Entity resolution operations                     |
| `SzConfig`          | Trait (`Box<dyn SzConfig>`)        | Configuration editing                            |
| `SzConfigManager`   | Trait (`Box<dyn SzConfigManager>`) | Configuration lifecycle                          |
| `SzDiagnostic`      | Trait (`Box<dyn SzDiagnostic>`)    | Diagnostics and performance                      |
| `SzProduct`         | Trait (`Box<dyn SzProduct>`)       | Version and license info                         |

This allows your code to depend on traits rather than concrete implementations, improving testability and flexibility.

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
- **`snapshot_restore`** - Snapshot an `internal://` datastore to a file and warm-start a fresh environment from it

### Configuration

- **`register_data_sources`** - Adding new data sources
- **`manage_configuration`** - Configuration lifecycle management

### Performance & Diagnostics

- **`check_datastore_performance`** - Performance benchmarking
- **`engine_priming`** - Engine optimization

### Logging

- **`verbose_toggle`** - Toggle verbosity at runtime from SIGUSR1/SIGUSR2 via a watcher thread
- **`module_stats`** - Count log messages per module and level in a callback
- **`stream_to_channel`** - Stream JSONL log lines to a dev console through a bounded channel
- **`default_output_file`** - Send the default output to a file (truncate; append on Senzing >= 4.5.1/4.6.0)
- **`jsonl_file`** - Write the default output to a JSONL file
- **`forward_to_log`** - Forward Senzing's records to the `log` crate (`cargo run --example forward_to_log --features log`)

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

### Error Handling

- **`error_handling/error_inspection`** - Inspect Senzing errors in mixed error chains with `SzErrorInspect`
- **`error_handling/retry_with_backoff`** - Retry retryable Senzing errors with exponential backoff

### Stewardship Operations

- **`stewardship/force_resolve`** - Manual entity resolution forcing
- **`stewardship/force_unresolve`** - Manual entity separation

### Logging

- **`logging/log_callback`** - Receive Senzing log messages in a closure callback

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

# Include the optional `log` feature (what CI runs)
cargo test --all-features

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

The SDK provides structured error types with a category hierarchy and
chain-walking inspection.

### Senzing-only functions

When a function only calls Senzing APIs, it returns `SzResult<T>` and you
can match on error variants or use classification methods directly:

```rust
use sz_rust_sdk::prelude::*;
use std::thread;
use std::time::Duration;

fn add_with_retry(engine: &dyn SzEngine, json: &str, max_retries: u32) -> SzResult<String> {
    let mut attempt = 0;
    loop {
        match engine.add_record("CUSTOMERS", "1", json, None) {
            Ok(info) => return Ok(info),
            Err(ref e) if e.is_retryable() && attempt < max_retries => {
                attempt += 1;
                eprintln!("Retryable (attempt {attempt}/{max_retries}): {e}");
                thread::sleep(Duration::from_millis(100 * 2u64.pow(attempt)));
            }
            Err(e) => return Err(e),
        }
    }
}
```

Every `SzError` belongs to a hierarchy. Use `is()` for polymorphic checks —
a `DatabaseTransient` error matches both its specific category and its
parent `Retryable` category:

```rust
use sz_rust_sdk::prelude::*;

let err = SzError::database_transient("Deadlock");
assert!(err.is(ErrorCategory::DatabaseTransient));
assert!(err.is(ErrorCategory::Retryable));
```

### Functions that mix Senzing with other error types

When a function calls both Senzing and non-Senzing operations, Rust's `?`
operator needs a single error type. The standard approach is
`Result<T, Box<dyn Error>>` (or `anyhow::Result<T>`, or a custom enum).
The `SzErrorInspect` trait — automatically implemented for all error
types — walks the error chain to find and inspect any embedded `SzError`:

```rust
use sz_rust_sdk::prelude::*;
use std::fs;

fn load_from_file(
    engine: &dyn SzEngine,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = fs::read_to_string(path)?;         // io::Error on failure
    engine.add_record("TEST", "1", &data, None)?;  // SzError on failure
    Ok(())
}
```

At the call site, `SzErrorInspect` methods find the `SzError` inside
any wrapper. They return `false` for non-Senzing errors like `io::Error`:

```rust
# use sz_rust_sdk::prelude::*;
# fn load_from_file(engine: &dyn SzEngine, path: &str)
#     -> Result<(), Box<dyn std::error::Error + Send + Sync>> { Ok(()) }
# fn example(engine: &dyn SzEngine) {
match load_from_file(engine, "data.json") {
    Ok(()) => println!("Done"),
    Err(ref e) if e.is_sz_retryable() => eprintln!("Retry: {e}"),
    Err(ref e) if e.is_sz(ErrorCategory::NotFound) => eprintln!("Not found: {e}"),
    Err(ref e) if e.is_sz_unrecoverable() => eprintln!("Fatal: {e}"),
    Err(e) => eprintln!("Error: {e}"),
}
# }
```

Use `sz_error()` to extract the underlying `SzError` for detailed inspection:

```rust
# use sz_rust_sdk::prelude::*;
fn log_error(err: &(dyn std::error::Error + 'static)) {
    match err.sz_error() {
        Some(sz) => {
            eprintln!("[{}] {}", sz.category(), sz.message());
            if let Some(code) = sz.error_code() {
                eprintln!("  native code: {code}");
            }
        }
        None => eprintln!("Non-Senzing error: {err}"),
    }
}
```

### `SzErrorInspect` reference

| Method                  | Returns            | Description                                              |
| ----------------------- | ------------------ | -------------------------------------------------------- |
| `sz_error()`            | `Option<&SzError>` | First `SzError` in the chain, or `None`                  |
| `is_sz_retryable()`     | `bool`             | Chain contains a retryable Senzing error                 |
| `is_sz_unrecoverable()` | `bool`             | Chain contains an unrecoverable Senzing error            |
| `is_sz_bad_input()`     | `bool`             | Chain contains a bad-input Senzing error                 |
| `is_sz(category)`       | `bool`             | Chain contains an `SzError` matching the `ErrorCategory` |

## Logging

`env.get_logging()?` returns an `SzLogging` over Senzing's native logger (libSzLogging). All
logging state is native and process-global: each registration is one native registration, and
the returned `SzLogRegistration` unregisters when dropped.

```rust
use sz_rust_sdk::prelude::*;

let env = ExampleEnvironment::initialize("my-app")?; // or SzEnvironmentCore::get_instance(..)
let logging = env.get_logging()?;
let registration = logging.register(SzLogLevel::Info, |record| {
    println!("[{}] {} {}", record.level(), record.module(), record); // Bare: message text only
})?;                                  // register BEFORE the engine is initialized (4.5.0)
let engine = env.get_engine()?;       // initialization is logged to the callback
// ... work ...
drop(engine);
env.destroy()?;
drop(registration);                   // unregister AFTER destroy (4.5.0)

logging.set_default_output(SzLogOutput::file("/var/log/senzing/sz.log"))?;
logging.set_default_format(SzLogFormat::Jsonl)?; // after the output on 4.5.0
logging.set_default_log_level(SzLogLevel::Warn)?;           // safe at any time
```

**Senzing 4.5.0 usage rule: change registrations, the default callback, output or format only
while no engine threads are running — before the first engine initialization (`get_engine()` /
`get_diagnostic()`) or after `destroy()`.** `set_default_log_level` / `get_effective_log_level`
are safe at any time. Joining your own threads is not enough (Senzing's lock-watcher thread logs
once the engine is initialized). The SDK does not enforce the rule; the full concurrency
guarantees need the GDEV-4960 native fix.

**Recipes**

| Goal | Call |
|---|---|
| Silence Senzing | `logging.set_default_output(SzLogOutput::Null)?` |
| Verbose for debugging | `logging.set_default_log_level(SzLogLevel::Trace)?` (any time) |
| JSONL to a file | `set_default_output(SzLogOutput::file(path))?` then `set_default_format(SzLogFormat::Jsonl)?` |
| Forward to the `log` crate | feature `log`: `let fwd = logging.forward_to_log(SzLogLevel::Info)?;` (target `senzing::<module>`) |
| Callbacks as the only sink | `set_default_output(SzLogOutput::Null)?`, then `register(..)` / `forward_to_log(..)` |

**Registrations:** bind them to a name — `let _ = logging.register(..)` drops (unregisters) it
immediately. `SzLogCallbackOptions::new(level)` (and a bare `SzLogLevel`) uses the `Bare` format:
the record carries level and module, so the message has no prefix. `set_default(level, closure)`
makes a closure the default output.

**Flushing:** a file default output is flushed when the output is replaced or at process exit. On
Senzing 4.5.0 `destroy()` does not flush it: call `set_default_output(..)` after `destroy()` before
reading the file (Senzing >= 4.5.1 flushes at destroy). An unopenable path returns `BadInput`.

**`verbose_logging`:** each engine initialization re-applies the environment's `verbose_logging`
level and the Legacy format to the default output unless you configured them first.
`set_default_log_level`, `set_default_format` and `set_default_callback` count as configuration;
`set_default_output` and `register*` do not.

**Level names in files:** native spells levels its own way. JSONL `severity` is `trace`, `debug`,
`info`, `warning`, `error`, `critical`; Legacy tags are `TRCE`, `DBUG`, `INFO`, `WARN`, `ERR`,
`CRIT`. `SzLogLevel::as_str()` gives `TRACE`/`DEBUG`/`WARN`/..., so parse with
`"warning".parse::<SzLogLevel>()` (case-insensitive) rather than comparing against `as_str()`.

**Threads:** on 4.5.0, group records by `record.thread_hash()` (the per-thread handle);
`thread_id()` needs Senzing >= 4.5.1. Thread-local registrations follow the OS thread.

**Callbacks** run on the logging thread, concurrently across threads: keep them fast and never
call engine methods from them. A panic in a callback is caught at the FFI boundary. On 4.5.0,
sink changes from inside a callback return an error, and dropping a registration inside a
callback deactivates and leaks its closure (native may still be running it).

**Needs Senzing >= 4.5.1 / 4.6.0** (returns `SzError::Configuration` naming the native function
until then; record fields return `None`): `SzLogRegistration::set_level` / `level`,
`get_default_output` / `get_default_format` / `get_default_log_level`,
`SzLogOutput::append_file`, thread-local registrations (`register_thread_local*`,
`capture_thread_logs`: 4.5.0 never delivers to them, GDEV-4960), record `timestamp()` /
`thread_id()`, and UTC JSONL timestamps. Each stub is marked `SZ-FUTURE(<C function>)` in the
source.

## Database Isolation

The SDK automatically provides database isolation for testing:

- Uses `internal://` in-memory database (v4.3+) — no temp files or schema setup
- Each environment gets its own ephemeral database
- No manual database management required
- Safe for concurrent test execution

## Configuration

Senzing engine configuration is a JSON string passed to the initialization function. See the [Senzing engine configuration tutorial](https://www.senzing.com/docs/tutorials/senzing_engine_config/) for full details.

The JSON contains `PIPELINE` paths and a `SQL` connection. For testing and examples,
use `internal://` (v4.3+) for a zero-setup in-memory database. For production, use
SQLite or a full database:

**macOS (Homebrew):**

```json
{
  "PIPELINE": {
    "CONFIGPATH": "/opt/homebrew/opt/senzing/er/resources/templates",
    "RESOURCEPATH": "/opt/homebrew/opt/senzing/er/resources",
    "SUPPORTPATH": "/opt/homebrew/opt/senzing/data"
  },
  "SQL": { "CONNECTION": "internal://" }
}
```

**Linux:**

```json
{
  "PIPELINE": {
    "CONFIGPATH": "/etc/opt/senzing",
    "RESOURCEPATH": "/opt/senzing/er/resources",
    "SUPPORTPATH": "/opt/senzing/data"
  },
  "SQL": { "CONNECTION": "internal://" }
}
```

You can also set the `SENZING_ENGINE_CONFIGURATION_JSON` environment variable with this JSON string. The SDK's `ExampleEnvironment` helper automatically builds appropriate settings for development and testing using `internal://`.

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
