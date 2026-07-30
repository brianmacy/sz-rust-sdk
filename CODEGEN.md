# Code Generation

This SDK uses code generation for two components that depend on the Senzing SDK distribution:

1. **FFI Bindings** - bindgen C-ABI declarations, generated and maintained upstream in the shared
   [`sz-rust-sdk-ffi`](https://github.com/brianmacy/sz-rust-sdk-ffi) crate (also depended on by
   Senzing's in-tree Rust binding layer). Not generated in this repo at all anymore.
2. **Error Category Taxonomy** - `SzErrorCategory` + `classify(code)`, generated from
   `szerrors.json` via `sz_rust_sdk_ffi::codegen::generate_error_taxonomy` (the same generator
   function that upstream crate's consumers all call against their own `szerrors.json` copy).

Both are **checked into source control** and **NOT regenerated automatically** during normal
builds. This ensures:

- Fast builds without requiring Senzing SDK headers/data files
- Stable, reviewable generated code
- CI/CD works without full Senzing SDK installation

## FFI Bindings

This repo no longer generates or vendors FFI bindings at all -- `sz-rust-sdk-ffi`'s `bindings`
module (feature `ffi`, default-on) is a normal dependency. To pick up a new Senzing SDK version's
C-ABI surface, regenerate bindings in that upstream repo (see its
`examples/generate_bindings.rs`), then bump this repo's `rev` pin in `Cargo.toml`.

## Regenerating the Error Category Taxonomy

**When to regenerate:**

- Upgrading to a new Senzing SDK version (new error codes / a new error class)

**How to regenerate:**

```bash
cd tools/generate-error-mappings
cargo run
```

This is its own standalone Cargo project, NOT an `[[example]]` of the main `sz-rust-sdk` package --
see `tools/generate-error-mappings/README.md` for why (a Windows-only linker failure this
structure fixes).

**Requirements:**

- `szerrors.json` must be available in one of:
  - Project root: `./szerrors.json`
  - G2 dev build: `~/dev/G2/dev/build/dist/sdk/szerrors.json`
  - Homebrew cask / Scoop / Linux standard install locations

**Output:**

- `src/error_category_generated.rs` -- the generated `SzErrorCategory` enum + `classify(code)`.

**What it generates:**

Only the code→category *classification table* (`classify(code) -> SzErrorCategory`, 13 categories).
`map_error_code()`/`get_error_hierarchy()` are **hand-written** (NOT generated) in
`src/error_category_bridge.rs` -- a 13-arm exhaustive match over `SzErrorCategory` (one arm per
error *class*, not per code), which fails to compile if a new category ever appears until a human
decides where it belongs in `SzError`.

## Workflow

### Normal Development

```bash
git clone <repo>
cargo build    # Just works - no code generation needed
cargo test
```

### After Senzing SDK Upgrade

```bash
# 1. Copy new szerrors.json to project root
cp ~/dev/G2/dev/build/dist/sdk/szerrors.json .

# 2. Regenerate the error category taxonomy
(cd tools/generate-error-mappings && cargo run)

# 3. If bindgen-visible C headers changed, regenerate + bump the sz-rust-sdk-ffi pin instead
#    (see "FFI Bindings" above) -- there's nothing to regenerate in THIS repo for that.

# 4. Review changes
git diff src/error_category_generated.rs

# 5. If a NEW category appeared, update src/error_category_bridge.rs's exhaustive match (it
#    won't compile until you do)

# 6. Test
cargo test

# 7. Commit
git add src/error_category_generated.rs src/error_category_bridge.rs szerrors.json
git commit -m "Update error taxonomy for Senzing SDK vX.Y.Z"
```

## Files

### Checked Into Version Control

- `src/error_category_generated.rs` - Generated `SzErrorCategory` + `classify()`
- `src/error_category_bridge.rs` - Hand-written category->`SzError` adapter (NOT generated)
- `szerrors.json` - Senzing error definitions (copied from SDK)

### Generator

- `tools/generate-error-mappings/` - standalone Cargo project; see its `src/main.rs`

### NOT Generated

- `src/error.rs` - Hand-written error types and hierarchy logic
- `src/error_category_bridge.rs` - Hand-written category->`SzError` mapping (see above)
- `build.rs` - Only handles library linking, no code generation
