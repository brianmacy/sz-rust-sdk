# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/brianmacy/sz-rust-sdk/releases/tag/v0.1.0
