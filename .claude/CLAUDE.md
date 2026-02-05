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
- No mock tests for this project. Only ones that actually use the SDK.

### Senzing Installation Paths

#### macOS (Homebrew)

Installed via: `brew install senzingsdk-runtime-unofficial`

```
Base:       /opt/homebrew/opt/senzing/runtime
Library:    /opt/homebrew/opt/senzing/runtime/er/lib/libSz.dylib
Config:     /opt/homebrew/opt/senzing/runtime/er/resources/templates
Resources:  /opt/homebrew/opt/senzing/runtime/er/resources
Support:    /opt/homebrew/opt/senzing/runtime/data
```

Environment variables for macOS:

```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib
export SENZING_CONFIGPATH=/opt/homebrew/opt/senzing/runtime/er/resources/templates
export SENZING_RESOURCEPATH=/opt/homebrew/opt/senzing/runtime/er/resources
export SENZING_SUPPORTPATH=/opt/homebrew/opt/senzing/runtime/data
```

#### Linux (Standard Installation)

```
Base:       /opt/senzing/er
Library:    /opt/senzing/er/lib/libSz.so
Config:     /opt/senzing/er/resources/templates
Resources:  /opt/senzing/er/resources
Support:    /opt/senzing/data
```

Environment variables for Linux:

```bash
export LD_LIBRARY_PATH=/opt/senzing/er/lib
export SENZING_CONFIGPATH=/opt/senzing/er/resources/templates
export SENZING_RESOURCEPATH=/opt/senzing/er/resources
export SENZING_SUPPORTPATH=/opt/senzing/data
```

- Look to the C# code for guidance on which native functions to use for FFI.
- Do not create mock tests for use without the native library. All tests should require the native library.
- All the examples must run successfully (target: 22/22 = 100%)
- Code-snippets are located in simplified structure: `code-snippets/category/example.rs`
- Make sure all error checking and processing is happening in all the core functions
- Make sure the tests are aligned with the detailed C# test analysis.
- DEMAND that all native SDK errors are caught and mapped to SzErrors

## Error Handling Architecture

### CRITICAL: Return Codes vs Exception Codes

Native Senzing functions return **simple return codes** (0, -1, -2, etc.) that only indicate whether an error occurred, NOT the actual error type. The return codes are:

- `0` = Success
- Non-zero = Error occurred (check getLastExceptionCode for details)

**WRONG approach:**

```rust
// DO NOT map simple return codes directly to error types!
match return_code {
    -1 => Err(SzError::unknown(...)),    // WRONG!
    -2 => Err(SzError::configuration(...)), // WRONG!
}
```

**CORRECT approach:**

```rust
if return_code != 0 {
    // Call getLastExceptionCode() to get the ACTUAL Senzing error code
    let actual_error_code = unsafe { Sz_getLastExceptionCode() };
    // Map the actual code (e.g., 7220, 999, 2000) to the correct error type
    Err(SzError::from_code_with_message(actual_error_code, SzComponent::Engine))
}
```

### Required FFI Bindings for Error Handling

Each component MUST have these error functions bound:

- `Sz_getLastException()` / `Sz_getLastExceptionCode()` - Engine
- `SzConfig_getLastException()` / `SzConfig_getLastExceptionCode()` - Config
- `SzConfigMgr_getLastException()` / `SzConfigMgr_getLastExceptionCode()` - ConfigMgr
- `SzDiagnostic_getLastException()` / `SzDiagnostic_getLastExceptionCode()` - Diagnostic
- `SzProduct_getLastException()` / `SzProduct_getLastExceptionCode()` - Product

### Error Hierarchy (Matching C# SDK)

The Rust SDK error types MUST mirror the C# SDK exception hierarchy:

```
SzError (base)
├── BadInput                        // SzBadInputException
│   ├── NotFound                    // SzNotFoundException (extends BadInput)
│   └── UnknownDataSource           // SzUnknownDataSourceException (extends BadInput)
├── Configuration                   // SzConfigurationException
├── Retryable                       // SzRetryableException
│   ├── DatabaseConnectionLost      // SzDatabaseConnectionLostException (extends Retryable)
│   ├── DatabaseTransient           // SzDatabaseTransientException (extends Retryable)
│   └── RetryTimeoutExceeded        // SzRetryTimeoutExceededException (extends Retryable)
├── Unrecoverable                   // SzUnrecoverableException
│   ├── Database                    // SzDatabaseException (extends Unrecoverable)
│   ├── License                     // SzLicenseException (extends Unrecoverable)
│   ├── NotInitialized              // SzNotInitializedException (extends Unrecoverable)
│   └── Unhandled                   // SzUnhandledException (extends Unrecoverable)
├── ReplaceConflict                 // SzReplaceConflictException
├── EnvironmentDestroyed            // SzEnvironmentDestroyedException
└── Unknown                         // Catch-all for unmapped errors
```

### Error Code Ranges (from getLastExceptionCode)

Map actual Senzing error codes to error types:

- `0-46, 64-100` → BadInput
- `47-63` → NotInitialized
- `999` → License
- `1000-1020` → Database
- `2000-2300` → Configuration
- `7200-7299` → Configuration
- `7301-7400` → BadInput
- `8500-8600` → Database
- `9000-9099, 9201-9999` → License
- `9100-9200` → Configuration

### Helper Methods Required

Each error type should support:

- `is_retryable()` - Returns true for Retryable and its subtypes
- `is_unrecoverable()` - Returns true for Unrecoverable and its subtypes
- `is_bad_input()` - Returns true for BadInput and its subtypes
- DEMAND code-snippets are minimal demonstrations of a single concept and should limit themselves to the minimal needed to know to accomplish the specific task. Also, document the Senzing SDK usage well.
- DEMAND all SDK methods must be implemented and tested
- DEMAND read both user and project requirements
