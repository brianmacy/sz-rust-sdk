## Error Handling Design

### Flat Enum Over Nested Hierarchy

The Rust SDK uses a **flat enum** (`SzError`) instead of nested enums. This was a deliberate design decision — it allows callers to match on specific error variants (e.g., `SzError::DatabaseTransient`) without knowing the hierarchy.

Category checking is done via `ErrorCategory` enum + `hierarchy()` + `is(ErrorCategory)` methods, giving polymorphic behavior without nesting.

### Return Codes vs Exception Codes

Native Senzing functions return **simple return codes** (0 = success, non-zero = error). These do NOT indicate the error type. You MUST call `getLastExceptionCode()` to get the actual Senzing error code, then map it to the correct `SzError` variant.

**Wrong:**

```rust
match return_code {
    -1 => Err(SzError::unknown(...)),  // WRONG!
}
```

**Correct:**

```rust
if return_code != 0 {
    let actual_code = unsafe { Sz_getLastExceptionCode() };
    Err(SzError::from_code_with_message(actual_code, SzComponent::Engine))
}
```

### Error Code Mappings — Now Category-Based, Not Per-Code (2026-07-30)

Previously the 456 error code→`SzError`-variant mappings were generated directly into
`src/error_mappings_generated.rs` (one match arm per code). As of the `sz-rust-sdk-ffi` extraction,
the code→category *classification* is generated (`src/error_category_generated.rs`, via the shared
`sz_rust_sdk_ffi::codegen` generator also used by Senzing's in-tree Rust binding layer), and
`map_error_code()`/`get_error_hierarchy()` are now **hand-written** in
`src/error_category_bridge.rs` — a 13-arm exhaustive match over `SzErrorCategory` (one per error
*class*, not per code). See `CODEGEN.md` for the full picture. Regenerate with:

```bash
cd tools/generate-error-mappings && cargo run
```

(a standalone Cargo project now, not a `sz-rust-sdk` example — see that directory's README.md).

Source file: `~/dev/G2/dev/build/dist/sdk/szerrors.json`

### Each FFI Component Has Its Own Error Functions

- Engine: `Sz_getLastException()` / `Sz_getLastExceptionCode()`
- Config: `SzConfig_getLastException()` / `SzConfig_getLastExceptionCode()`
- ConfigMgr: `SzConfigMgr_getLastException()` / `SzConfigMgr_getLastExceptionCode()`
- Diagnostic: `SzDiagnostic_getLastException()` / `SzDiagnostic_getLastExceptionCode()`
- Product: `SzProduct_getLastException()` / `SzProduct_getLastExceptionCode()`
