# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Cargo.toml metadata for crates.io publishing (repository, homepage, documentation)
- CHANGELOG.md for version history tracking
- Homebrew installation path auto-detection in build.rs
- Stale files check in /prep command

### Changed
- Updated prep command environment variables for Homebrew Senzing installation

## [0.1.0] - 2025-01-16

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

[Unreleased]: https://github.com/brianmacy/sz-rust-sdk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/brianmacy/sz-rust-sdk/releases/tag/v0.1.0
