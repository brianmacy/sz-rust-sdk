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

### Error Code Mappings Are Auto-Generated

The 456 error code mappings come from `szerrors.json` and are generated into `src/error_mappings_generated.rs`. Regenerate with:

```bash
cargo run --example generate_error_mappings
```

Source file: `~/dev/G2/dev/build/dist/sdk/szerrors.json`

### Each FFI Component Has Its Own Error Functions

- Engine: `Sz_getLastException()` / `Sz_getLastExceptionCode()`
- Config: `SzConfig_getLastException()` / `SzConfig_getLastExceptionCode()`
- ConfigMgr: `SzConfigMgr_getLastException()` / `SzConfigMgr_getLastExceptionCode()`
- Diagnostic: `SzDiagnostic_getLastException()` / `SzDiagnostic_getLastExceptionCode()`
- Product: `SzProduct_getLastException()` / `SzProduct_getLastExceptionCode()`
