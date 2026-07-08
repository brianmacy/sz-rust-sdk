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
}
