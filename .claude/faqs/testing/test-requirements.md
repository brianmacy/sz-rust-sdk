## Test Requirements

### No Mocks

All tests must use the real Senzing SDK. No mock implementations allowed. Tests that can't access the native library will SIGABRT.

### Doc Tests Must Run

All doc examples must actually execute — not `ignore`. Use `no_run` only for examples that destroy the singleton (destroy, cleanup, try_cleanup, SenzingGuard drop) since they poison the shared process in merged doctests. `no_run` still compile-checks the code.

Doc tests use `ExampleEnvironment` in hidden `#` lines for setup:

````rust
/// ```
/// # use sz_rust_sdk::helpers::ExampleEnvironment;
/// use sz_rust_sdk::prelude::*;
///
/// # let env = ExampleEnvironment::initialize("doctest_unique_name")?;
/// let engine = env.get_engine()?;
/// // ... visible API demonstration ...
/// # Ok::<(), SzError>(())
/// ```
````

### Test Naming

- `test_*` — Unit tests that MUST fail on any error
- `example_*` — Working examples that MUST succeed completely

### Running Tests

```bash
# Requires DYLD_LIBRARY_PATH on macOS
DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib cargo test

# Error unit tests (no SDK needed)
cargo test --lib test_error_mapping
```

### Data Source

Use "TEST" data source in tests — always available, no config setup needed. Use unique record IDs with method-specific prefixes to avoid conflicts between tests (e.g., "ADD_1001", "DEL_1001").
