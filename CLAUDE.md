# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Setup

This is a Rust SDK project that is currently in its initial state. To initialize the project:

```bash
cargo init --name sz-rust-sdk .
```

## Common Commands

Once the project is initialized with Cargo:

### Build Commands
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Check for compilation errors without building
cargo check
```

### Test Commands
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests and generate coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Code Quality Commands
```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run clippy linter
cargo clippy

# Run clippy with all targets and features
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation
```bash
# Build and open documentation
cargo doc --open

# Build documentation without opening
cargo doc
```

## Architecture

This is a Rust SDK for Senzing entity resolution engine following the same patterns as the C# SDK:

- `src/lib.rs` - Main library entry point and public API
- `src/traits.rs` - Core trait definitions (SzEngine, SzConfig, etc.)
- `src/core/` - Core implementation structs (SzEngineCore, SzConfigCore, etc.)
- `src/ffi/` - FFI bindings to native Senzing library
- `src/error.rs` - Error types and handling
- `src/flags.rs` - Bitflag definitions for operation control
- `src/types.rs` - Common type definitions
- `tests/` - Integration tests
- `examples/` - Usage examples
- `Cargo.toml` - Project configuration and dependencies

### Core Components

- **SzEnvironment/SzEnvironmentCore** - Main entry point and factory
- **SzEngine/SzEngineCore** - Core entity resolution operations
- **SzConfig/SzConfigCore** - Configuration management
- **SzConfigManager/SzConfigManagerCore** - Configuration lifecycle
- **SzDiagnostic/SzDiagnosticCore** - System diagnostics
- **SzProduct/SzProductCore** - Version and license info

## Development Workflow

1. Use `cargo check` for quick compilation feedback
2. Run `cargo clippy` before committing to catch common issues
3. Use `cargo fmt` to maintain consistent code style
4. Ensure all tests pass with `cargo test`
5. Consider adding examples for public API functions
- Senzing uses a synchronous design
- No mock tests for this project.  Only ones that actually use the SDK.
- Senzing is installed at /opt/senzing/er/.
- Look to the C# code for guidance on which native functions to use for FFI.
- Do not create mock tests for use without the native library.  All tests should require the native library.
- All the examples must run successfully
- Mirror the error hierarchy from the C# exceptions
- If the Senzing SDK function returns -2, create the proper error from the appropriate *_getLastException function.
- Make sure all error checking and processing is happening in all the core functions