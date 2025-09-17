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

## Senzing v4 SDK Requirements

- **MANDATORY: 100% API Coverage** - The Rust SDK MUST achieve 100% functional parity with the C# Senzing v4 SDK
- **Reference Implementation** - Always use the C# SDK as the authoritative reference for API design, method signatures, and behavior
- **Missing Method Policy** - Any method available in C# SzEngine, SzConfig, SzConfigManager, SzDiagnostic, or SzProduct interfaces MUST be implemented in Rust
- **Flag Compatibility** - All SzFlag combinations available in C# must have equivalent SzFlags in Rust
- **Error Hierarchy** - Mirror the complete C# exception hierarchy with equivalent Rust error types
- **FFI Completeness** - Bind to ALL native Senzing functions that the C# SDK uses, not just a subset

### Critical Missing Functions (MUST IMPLEMENT)
- `reevaluate_entity` - Required for stewardship operations
- `reevaluate_record` - Required for record-level stewardship
- `why_record_in_entity` - Required for entity analysis
- `why_entities` (not `why_entity`) - Use correct v4 function names
- Any other method gaps identified through C# SDK comparison

### Quality Standards
- Senzing uses a synchronous design
- **MANDATORY: Use Thread Pools, NOT Async/Await** - Senzing SDK scales with real OS threads, use thread pools and futures for parallelization
- **Thread Pool Architecture** - Each thread gets its own engine instance, engines are thread-safe at C library level
- **No Async/Await** - Do not use tokio, async/await, or async runtimes. Use std::thread and mpsc channels for coordination
- **Thread Scaling** - Examples must demonstrate proper scaling with 4+ OS threads showing per-thread performance metrics
- No mock tests for this project.  Only ones that actually use the SDK.
- Senzing is installed at /opt/senzing/er/.
- Look to the C# code for guidance on which native functions to use for FFI.
- Do not create mock tests for use without the native library.  All tests should require the native library.
- All the examples must run successfully (target: 22/22 = 100%)
- If the Senzing SDK function returns -2, create the proper error from the appropriate *_getLastException function.
- Make sure all error checking and processing is happening in all the core functions
- Make sure the tests are aligned with the detailed C# test analysis.
- DEMAND that all native SDK errors are caught and mapped to SzErrors
- DEMAND code-snippets are minimal demonstrations of a single concept and should limit themselves to the minimal needed to know to accomplish the specific task.  Also, document the Senzing SDK usage well.