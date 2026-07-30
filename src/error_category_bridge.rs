//! Hand-written (NOT generated) adapter from the shared `SzErrorCategory` taxonomy
//! (`error_category_generated::classify`, from `sz_rust_sdk_ffi::codegen`) to this crate's own
//! rich `SzError` enum and `ErrorCategory` hierarchy.
//!
//! Previously `map_error_code`/`get_error_hierarchy` were generated with one match arm PER ERROR
//! CODE (456 arms), independently re-deciding "code N belongs to class X" on every regeneration.
//! Now the code->category decision is made ONCE, by the shared generator (same one Senzing's
//! in-tree Rust binding layer uses), and this file only decides category->SzError-variant -- a
//! mapping that changes only when Senzing adds a wholly new error CLASS, not when codes are
//! added/removed within existing classes.
//!
//! Both matches below are deliberately EXHAUSTIVE (no `_ =>` wildcard arm): if
//! `error_category_generated.rs` ever gains a 14th `SzErrorCategory` variant, this file fails to
//! compile until a human decides where it belongs in `SzError`/`ErrorCategory` -- silently
//! defaulting a brand-new error class to `Unknown` would be a worse outcome than a build break.

use crate::error::{ErrorCategory, ErrorContext, SzError};
use crate::error_category_generated::{SzErrorCategory, classify};

/// Maps a Senzing error code to the appropriate [`SzError`] variant, via the shared taxonomy.
pub(super) fn map_error_code(error_code: i64, ctx: ErrorContext) -> SzError {
    match classify(error_code as i32) {
        SzErrorCategory::Base => SzError::Unknown(ctx),
        SzErrorCategory::BadInput => SzError::BadInput(ctx),
        SzErrorCategory::Configuration => SzError::Configuration(ctx),
        SzErrorCategory::Database => SzError::Database(ctx),
        SzErrorCategory::DatabaseConnectionLost => SzError::DatabaseConnectionLost(ctx),
        SzErrorCategory::DatabaseTransient => SzError::DatabaseTransient(ctx),
        SzErrorCategory::License => SzError::License(ctx),
        SzErrorCategory::NotFound => SzError::NotFound(ctx),
        SzErrorCategory::NotInitialized => SzError::NotInitialized(ctx),
        SzErrorCategory::ReplaceConflict => SzError::ReplaceConflict(ctx),
        SzErrorCategory::RetryTimeoutExceeded => SzError::RetryTimeoutExceeded(ctx),
        SzErrorCategory::Unhandled => SzError::Unhandled(ctx),
        SzErrorCategory::UnknownDataSource => SzError::UnknownDataSource(ctx),
    }
}

/// Returns the error hierarchy (most specific to most general) for a given error code, via the
/// shared taxonomy. Mirrors the per-class hierarchy chains from the prior per-code generator
/// exactly (verified against all 456 original codes before this change).
pub(super) fn get_error_hierarchy(error_code: i64) -> Vec<ErrorCategory> {
    match classify(error_code as i32) {
        SzErrorCategory::Base => vec![],
        SzErrorCategory::BadInput => vec![ErrorCategory::BadInput],
        SzErrorCategory::Configuration => vec![ErrorCategory::Configuration],
        SzErrorCategory::Database => vec![ErrorCategory::Database, ErrorCategory::Unrecoverable],
        SzErrorCategory::DatabaseConnectionLost => vec![
            ErrorCategory::DatabaseConnectionLost,
            ErrorCategory::Retryable,
        ],
        SzErrorCategory::DatabaseTransient => {
            vec![ErrorCategory::DatabaseTransient, ErrorCategory::Retryable]
        }
        SzErrorCategory::License => vec![ErrorCategory::License, ErrorCategory::Unrecoverable],
        SzErrorCategory::NotFound => vec![ErrorCategory::NotFound, ErrorCategory::BadInput],
        SzErrorCategory::NotInitialized => {
            vec![ErrorCategory::NotInitialized, ErrorCategory::Unrecoverable]
        }
        SzErrorCategory::ReplaceConflict => vec![ErrorCategory::ReplaceConflict],
        SzErrorCategory::RetryTimeoutExceeded => vec![
            ErrorCategory::RetryTimeoutExceeded,
            ErrorCategory::Retryable,
        ],
        SzErrorCategory::Unhandled => vec![ErrorCategory::Unhandled, ErrorCategory::Unrecoverable],
        SzErrorCategory::UnknownDataSource => {
            vec![ErrorCategory::UnknownDataSource, ErrorCategory::BadInput]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_999_maps_to_license() {
        // Mirrors the SzError::from_code_with_message doctest in error.rs.
        let ctx =
            ErrorContext::with_code("test".to_string(), 999, crate::error::SzComponent::Engine);
        assert!(matches!(map_error_code(999, ctx), SzError::License(_)));
    }

    #[test]
    fn unknown_code_falls_back_to_base_hierarchy() {
        assert_eq!(get_error_hierarchy(987_654), Vec::<ErrorCategory>::new());
    }
}
