//! Regression test for issue #25: a failed `Sz_init` must be observed by
//! *every* thread, not just the first one to attempt initialization.
//!
//! The original bug marked the environment initialized before `Sz_init` ran
//! and returned `Ok` regardless of the result, so only the first worker saw
//! the real error while the rest proceeded against an uninitialized native
//! engine. This test constructs an environment with a deliberately invalid
//! configuration (which makes `Sz_init` fail fast, with no network access),
//! then races several threads through `get_engine()` and asserts they all
//! receive an error.
//!
//! It uses `SzEnvironmentCore::new` directly rather than the global singleton
//! helper so it does not pollute shared state for other tests, and lives in
//! its own test binary so no earlier successful `Sz_init` runs in-process.

use std::sync::Arc;
use std::thread;

use sz_rust_sdk::prelude::*;

const WORKER_COUNT: usize = 8;

#[test]
fn test_failed_init_is_seen_by_all_threads() {
    // Valid JSON, but not a usable engine configuration: no PIPELINE paths and
    // a bogus SQL connection. `Sz_init` rejects this and returns a non-zero
    // code without attempting any network I/O.
    let bad_ini = r#"{"SQL":{"CONNECTION":"bogus://not-a-real-backend"}}"#;
    let env = Arc::new(
        SzEnvironmentCore::new("sz-rust-sdk-init-failure-test", bad_ini, false)
            .expect("constructing the environment struct must not fail"),
    );

    let handles: Vec<_> = (0..WORKER_COUNT)
        .map(|_| {
            let env = Arc::clone(&env);
            thread::spawn(move || env.get_engine().map(|_| ()))
        })
        .collect();

    let results: Vec<SzResult<()>> = handles
        .into_iter()
        .map(|h| h.join().expect("worker thread should not panic"))
        .collect();

    for (i, result) in results.iter().enumerate() {
        assert!(
            result.is_err(),
            "worker {i} must observe the Sz_init failure, but got Ok(()) \
             (regression of issue #25 — only the first thread saw the error)"
        );
    }

    // One worker ran Sz_init; the rest got the stored failure. Every copy must
    // keep the native code, component and category (not a code-less
    // Unrecoverable).
    let errors: Vec<&SzError> = results.iter().filter_map(|r| r.as_ref().err()).collect();
    let first = errors[0];
    eprintln!("Sz_init failure seen by all workers: {first}");
    assert!(
        first.error_code().is_some(),
        "native error code lost: {first}"
    );
    assert_eq!(first.component(), Some(SzComponent::Engine), "{first}");
    for (i, error) in errors.iter().enumerate() {
        assert_eq!(
            error.error_code(),
            first.error_code(),
            "worker {i}: {error}"
        );
        assert_eq!(error.component(), first.component(), "worker {i}: {error}");
        assert_eq!(error.hierarchy(), first.hierarchy(), "worker {i}: {error}");
    }
}

/// `get_diagnostic()` runs `SzDiagnostic_init`, so bad settings fail it with
/// the diagnostic's native error, kept for every later call.
#[test]
fn test_failed_diagnostic_init_keeps_native_error() {
    let bad_ini = r#"{"SQL":{"CONNECTION":"bogus://not-a-real-backend"}}"#;
    let env = SzEnvironmentCore::new("sz-rust-sdk-diag-init-failure-test", bad_ini, false)
        .expect("constructing the environment struct must not fail");

    let first = env
        .get_diagnostic()
        .err()
        .expect("SzDiagnostic_init must reject an unusable configuration");
    eprintln!("SzDiagnostic_init failure: {first}");
    assert!(
        first.error_code().is_some(),
        "native error code lost: {first}"
    );
    assert_eq!(first.component(), Some(SzComponent::Diagnostic), "{first}");

    let again = env
        .get_diagnostic()
        .err()
        .expect("the stored SzDiagnostic_init failure must be returned again");
    assert_eq!(again.error_code(), first.error_code());
    assert_eq!(again.hierarchy(), first.hierarchy());
}

/// Settings that cannot become a C string (interior NUL) are reported as
/// `StringConversion` by every call, not stored as an init failure.
#[test]
fn test_invalid_settings_string_is_string_conversion() {
    let env = SzEnvironmentCore::new("sz-rust-sdk-nul-settings-test", "{\0}", false)
        .expect("constructing the environment struct must not fail");
    for attempt in 0..2 {
        match env.get_engine() {
            Err(SzError::StringConversion(_)) => {}
            Err(other) => panic!("attempt {attempt}: expected StringConversion, got: {other}"),
            Ok(_) => panic!("attempt {attempt}: settings with a NUL must be rejected"),
        }
    }
}
