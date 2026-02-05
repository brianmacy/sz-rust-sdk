# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0] - 2026-02-05

### Changed

- **BREAKING**: Renamed `why_entity` to `why_entities` to correctly reflect that it compares TWO entities
- **BREAKING**: Unified `get_entity` and `get_entity_by_record` into single `get_entity(EntityRef, flags)` function
- **BREAKING**: Unified `find_interesting_entities_by_entity_id` and `find_interesting_entities_by_record` into single `find_interesting_entities(EntityRef, flags)` function
- **BREAKING**: Error variants now use `ErrorContext` struct instead of individual fields (enables error code/component preservation)
- **BREAKING**: Error variant matching syntax changed from `SzError::BadInput { message, .. }` to `SzError::BadInput(ctx)`
- Fixed error code 10 to map to `RetryTimeoutExceeded` instead of `BadInput`
- Fixed error code 87 to map to `Unhandled` instead of `BadInput`
- Fixed error codes 1006-1007 to map to `DatabaseConnectionLost` instead of generic `Database`
- Fixed error code 1008 to map to `DatabaseTransient` instead of generic `Database`
- Narrowed `NotInitialized` error code range to only codes 48, 49, 50, 53 (was 47-63)

### Added

- New `EntityRef` enum to specify entity by ID or by record key (data source + record ID)
- `EntityRef::Id(EntityId)` variant for entity ID references
- `EntityRef::Record { data_source, record_id }` variant for record key references
- `From<EntityId>` trait implementation for automatic conversion to `EntityRef::Id`
- `ErrorContext` struct to reduce error code duplication and preserve error metadata
- `SzError::error_code()` method to retrieve native Senzing error codes
- `SzError::component()` method to identify which SDK component generated the error
- `SzError::message()` method to extract error message string
- `SzError::category()` method to get error classification ("database", "license", etc.)
- `SzError::severity()` method to get error severity level ("critical", "high", "medium", "low")
- `SzError::is_database()` method to catch all database-related errors
- `SzError::is_license()` method to catch all license-related errors
- `SzError::is_configuration()` method to catch all configuration-related errors
- `SzError::is_initialization()` method to catch all initialization-related errors
- `SzError::with_source()` builder method for adding error sources
- `SzResultExt` trait with `.or_retry()`, `.filter_retryable()`, and error checking methods
- `#[non_exhaustive]` attribute to `SzError` enum for future API stability

### Removed

- `get_entity_by_record()` - replaced by `get_entity(EntityRef::Record {...})`
- `find_interesting_entities_by_entity_id()` - replaced by `find_interesting_entities(EntityRef::Id(...))`
- `find_interesting_entities_by_record()` - replaced by `find_interesting_entities(EntityRef::Record {...})`

### Migration Guide

```rust
// Old API
engine.why_entity(id1, id2, flags)?;
engine.get_entity(entity_id, flags)?;
engine.get_entity_by_record("TEST", "123", flags)?;
engine.find_interesting_entities_by_entity_id(entity_id, flags)?;
engine.find_interesting_entities_by_record("TEST", "123", flags)?;

// New API
engine.why_entities(id1, id2, flags)?;  // Plural: compares TWO entities
engine.get_entity(entity_id.into(), flags)?;
engine.get_entity(EntityRef::Record { data_source: "TEST", record_id: "123" }, flags)?;
engine.find_interesting_entities(entity_id.into(), flags)?;
engine.find_interesting_entities(EntityRef::Record { data_source: "TEST", record_id: "123" }, flags)?;
```

## [0.9.1] - 2026-01-26

### Fixed

- **Critical**: ARM Linux (aarch64-unknown-linux-gnu) portability bug in error.rs
- Changed buffer type from hardcoded `i8` to platform-agnostic `c_char` type
- SDK now compiles on all supported platforms: x86_64/aarch64 on Linux/macOS/Windows

### Notes

- Apple Silicon uses signed char (i8) deviating from ARM standard
- ARM Linux uses unsigned char (u8) following ARM ABI specification
- Fix ensures portability across all architectures without platform-specific conditionals

## [0.8.0] - 2026-01-24

### Changed

- Test databases now created from SQL schema instead of copying template files
- Removed dependency on `SENZING_TEMPLATE_DB` environment variable
- Database schema read from `SENZING_RESOURCEPATH/schema/szcore-schema-sqlite-create.sql`

### Added

- `rusqlite` dependency for database creation from SQL schema
- `Zlib` license added to allowed licenses in `deny.toml`

### Notes

- Tests are now more self-contained and don't require pre-existing template databases
- Only `SENZING_RESOURCEPATH` environment variable needed for schema location

## [0.7.0] - 2026-01-24

### Changed

- Replaced static `destroy_global_instance()` with instance method `destroy(self: Arc<Self>)`
- Uses `Arc::try_unwrap` for safe ownership-based cleanup
- `destroy()` only succeeds when caller holds sole reference to the environment
- Removed `destroy(&mut self)` from `SzEnvironment` trait (incompatible with Arc pattern)

### Added

- `test_destroy_ownership_semantics` test validating Arc ownership cleanup behavior

### Notes

- Calling `destroy()` with other Arc references outstanding returns an error
- Environment is restored to singleton storage if destroy fails
- Proper Rust ownership semantics prevent destroying while others hold references

## [0.6.0] - 2026-01-23

### Added

- `Send + Sync` bounds on `SzEnvironment`, `SzEngine`, `SzProduct`, and `SzDiagnostic` traits
- Enables sharing engine instances across threads with `Arc<dyn SzEngine>`
- Supports rayon parallel workloads without per-item `get_engine()` calls

### Notes

- `SzConfig` and `SzConfigManager` intentionally do NOT have `Send + Sync` bounds
- Configuration operations should be coordinated, not parallelized
- `get_engine()` cost is negligible (~2-10ns) even when called repeatedly

## [0.5.0] - 2026-01-17

### Fixed

- **Critical**: 9 FFI signature mismatches with actual Senzing C headers causing undefined behavior
- `why_entities` now correctly passes flags (uses V2 helper)
- `why_records` now correctly passes flags (uses V2 helper)
- `why_record_in_entity` now correctly passes flags (uses V2 helper)
- `why_search` now correctly passes flags (uses V2 helper)
- `get_record` now correctly passes flags (uses V2 helper)
- `how_entity` now correctly passes flags (uses V2 helper)
- `find_path` now correctly passes flags (uses V2 helper)
- `process_redo_record` signature fixed (C API has no flags parameter)
- Export functions (`export_json_entity_report`, `export_csv_entity_report`, `fetch_next`, `close_export`) now use correct handle types
- `Sz_getActiveConfigID` now uses correct out-parameter pattern
- Buffer size types in error handling changed from `i64` to `usize`
- Rust 2024 unsafe block compliance in FFI helpers

### Changed

- FFI bindings now auto-generated from Senzing C headers using bindgen
- Removed manual `bindings.rs` in favor of `bindings_generated.rs`
- Memory free function changed from `Sz_free` to `SzHelper_free`

### Added

- `scripts/generate_bindings.rs` for regenerating FFI bindings
- Support for `SENZING_SDK_PATH` environment variable override
- Result processing macros for each component type

## [0.4.0] - 2026-01-16

### Fixed

- `search_by_attributes` now correctly applies flags when `search_profile` is None (uses V2 helper)
- `get_entity` now correctly applies flags parameter (was ignored, uses V2 helper)
- `get_entity_by_record` now correctly applies flags parameter (was ignored, uses V2 helper)
- `get_virtual_entity` now correctly applies flags parameter (was ignored, uses V2 helper)

### Changed

- `get_virtual_entity` now supports multiple record keys (previously limited to single record)

### Added

- FFI bindings for `Sz_searchByAttributes_V2_helper`, `Sz_getEntityByEntityID_V2_helper`, `Sz_getEntityByRecordID_V2_helper`, `Sz_getVirtualEntityByRecordID_V2_helper`

## [0.3.0] - 2026-01-16

### Changed

- Rewrite flags.rs to match C# SDK v4 exactly with correct bit positions
- Update examples and tests for new flag constant names (`*_DEFAULT` → `*_DEFAULT_FLAGS`)

### Added

- 18+ missing flags: `WITH_INFO`, `INCLUDE_FEATURE_SCORES`, `FIND_PATH_STRICT_AVOID`, `SEARCH_INCLUDE_STATS`, `ENTITY_INCLUDE_RECORD_TYPES`, `ENTITY_INCLUDE_INTERNAL_FEATURES`, `ENTITY_INCLUDE_FEATURE_STATS`, and more
- `Once` guards for thread-safe ConfigMgr and Product initialization
- Concurrent initialization tests in sz_environment_test.rs
- Homebrew vs Linux installation paths documented in CLAUDE.md

### Fixed

- Incorrect composite flag definitions and bit positions
- Race condition in component initialization

## [0.2.0] - 2026-01-16

### Added

- Cargo.toml metadata for crates.io publishing (repository, homepage, documentation)
- CHANGELOG.md for version history tracking
- Homebrew installation path auto-detection in build.rs
- Stale files check in /prep command
- `simple_usage` example demonstrating the enforced trait pattern
- Platform-specific installation documentation (Linux vs macOS/Homebrew)

### Changed

- **Breaking**: All Core types except `SzEnvironmentCore` are now private
- Users must access SDK components through traits (`Box<dyn SzEngine>`, etc.)
- `get_config_manager()` no longer requires `Sz_init`, enabling config setup before engine initialization
- Updated prep command environment variables for Homebrew Senzing installation
- Fixed template database path to use `resources/templates/G2C.db`

### Removed

- Unused `SzConfigManagerCore::new()` method (use `get_config_manager()` instead)
- Unused `SzProductCore::new()` method (use `get_product()` instead)

## [0.1.0] - 2026-01-16

### Added

- Initial Rust SDK release with 100% API parity with C# Senzing v4 SDK
- Core trait definitions: `SzEngine`, `SzConfig`, `SzConfigManager`, `SzDiagnostic`, `SzProduct`, `SzEnvironment`
- Core implementations: `SzEngineCore`, `SzConfigCore`, `SzConfigManagerCore`, `SzDiagnosticCore`, `SzProductCore`, `SzEnvironmentCore`
- FFI bindings to native Senzing library (internalized, not public API)
- Comprehensive error hierarchy matching C# SDK exceptions:
  - `SzError::BadInput` with `NotFound` and `UnknownDataSource` variants
  - `SzError::Configuration`
  - `SzError::Retryable` with `DatabaseConnectionLost`, `DatabaseTransient`, `RetryTimeoutExceeded` variants
  - `SzError::Unrecoverable` with `Database`, `License`, `NotInitialized`, `Unhandled` variants
  - `SzError::ReplaceConflict`
  - `SzError::EnvironmentDestroyed`
- `SzFlags` bitflags for operation control with all flag groups from C# SDK
- Prelude module for convenient imports
- Database isolation helpers (`ExampleEnvironment`) for safe testing
- 223 tests covering all SDK functionality
- 16 runnable examples demonstrating SDK usage
- Code snippets organized by category
- GitHub Pages documentation deployment
- Singleton pattern for `SzEnvironmentCore` with thread-safe access

### Security

- Proper error code retrieval using `getLastExceptionCode()` instead of mapping return codes directly
- No exposure of internal FFI bindings to public API

[Unreleased]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.9.1...HEAD
[0.9.1]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brianmacy/sz-rust-sdk/releases/tag/v0.1.0
