## Code Generation

### FFI Bindings Live Upstream Now (2026-07-30)

FFI bindings (`bindings`) and marshaling helpers (`helpers`) are no longer generated or vendored in
this repo at all -- they're a normal dependency on the shared
[`sz-rust-sdk-ffi`](https://github.com/brianmacy/sz-rust-sdk-ffi) crate (git dep pinned to a full
commit SHA), also depended on by Senzing's in-tree Rust binding layer (`senzing-sys` in the G2
repo). To pick up a Senzing SDK version's C-ABI changes, regenerate bindings in that upstream repo,
then bump this repo's `rev` pin.

### The Error Category Taxonomy Is Checked In

The generated `SzErrorCategory`/`classify()` (`src/error_category_generated.rs`) is **checked into
source control**, NOT auto-generated at build time. This avoids build-time dependencies on external
tools and makes the build reproducible.

### Generator Is a Standalone Cargo Project, Not an Example

`tools/generate-error-mappings/` is its own Cargo project (own `Cargo.toml`), NOT an `[[example]]`
of the main `sz-rust-sdk` package anymore. Cargo unifies a dependency's features across a package's
normal + dev dependency edges (dev-deps aren't isolated the way build-deps are under resolver v2),
so an in-package `codegen`-only example silently re-enabled `sz-rust-sdk-ffi`'s `ffi` feature via
the main lib's own dependency on it -- fine on macOS/Linux (dead-stripped), but MSVC doesn't
dead-strip the resulting unreferenced `SzHelper_free` extern, causing a Windows-only link failure.
Being a fully separate Cargo project (no shared dependency edge with the main lib) avoids this
category of bug entirely. Run with:

```bash
cd tools/generate-error-mappings && cargo run
```

Source: `szerrors.json` (repo root, `SENZING_DIR`, or an installed Senzing SDK)
Output: `src/error_category_generated.rs` (13 categories; `map_error_code`/`get_error_hierarchy`
are hand-written in `src/error_category_bridge.rs`, not generated)

### build.rs

`build.rs` only handles library linking — NO code generation.

See `CODEGEN.md` for the full workflow.
