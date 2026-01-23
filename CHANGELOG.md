# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brianmacy/sz-rust-sdk/releases/tag/v0.1.0
