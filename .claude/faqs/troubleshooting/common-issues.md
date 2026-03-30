## Common Issues

### SIGABRT on Test Run

**Symptom:** Tests crash with `dyld: Library not loaded: @rpath/libSz.dylib`

**Fix:** Set the library path before running:

```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib  # macOS
export LD_LIBRARY_PATH=/opt/senzing/er/lib                          # Linux
```

### "Unknown data source" Error

**Symptom:** `SzError::UnknownDataSource` when adding records

**Fix:** Use "TEST" data source (always available) or register your data source first:

```rust
let config_mgr = env.get_config_manager()?;
let config = config_mgr.create_config()?;
config.register_data_source("MY_SOURCE")?;
let def = config.export()?;
config_mgr.set_default_config(&def, Some("Added MY_SOURCE"))?;
```

### Singleton Conflicts in Tests

**Symptom:** Second `get_instance()` call fails or returns stale state

**Cause:** `SzEnvironmentCore` is a process-wide singleton. Only one environment can exist.

**Fix:** Use `ExampleEnvironment::initialize()` which handles singleton lifecycle. For integration tests, use `serial_test` crate to run tests sequentially.

### Clippy Failures

**Requirement:** Clippy MUST pass with strict flags:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Fix all warnings before committing. This is a non-negotiable project requirement.

### Doc Test Shows "ignored"

**Policy:** Zero ignored doc tests allowed. All examples must be runnable with `ExampleEnvironment` setup in hidden `#` lines. If a test can't run, restructure it — don't mark it `ignore`.
