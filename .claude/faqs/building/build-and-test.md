## Build and Test Commands

### Environment Setup

Senzing SDK environment must be sourced first:

```bash
source ~/dev/G2/dev/setupEnv
```

On macOS with Homebrew installation, set:

```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib
```

On Linux:

```bash
export LD_LIBRARY_PATH=/opt/senzing/er/lib
```

### Build

```bash
cargo build
cargo build --release
```

### Tests

```bash
# All tests (requires Senzing SDK runtime)
DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib cargo test

# Doc tests only
DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib cargo test --doc

# Specific test
DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib cargo test test_name
```

### Code Quality

```bash
# Clippy (MUST pass — project requirement)
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt
cargo fmt -- --check
```

### Important Notes

- Tests without the SDK runtime will SIGABRT on dylib load failure
- No mock tests allowed — all tests use the real SDK
- `cargo-audit` is installed via `cargo install cargo-audit` (not `cargo add audit`)
- Error unit tests can run without SDK: `cargo test --lib test_error_mapping`
