# generate-error-mappings

Regenerates `../../src/error_category_generated.rs` from `szerrors.json`, via the shared
`sz_rust_sdk_ffi::codegen` generator.

## Why this is a separate Cargo project, not a `sz-rust-sdk` example

It used to be `scripts/generate_error_mappings.rs`, an `[[example]]` of the main `sz-rust-sdk`
package. That broke on Windows: `sz-rust-sdk`'s main library depends on `sz-rust-sdk-ffi` with
its default `ffi` feature (bindings + helpers), and Cargo unifies a dependency's features across
*all* of a package's dependency edges that are active in the same build — normal and dev
dependencies are unified together (unlike build-dependencies, which resolver v2 keeps separate).
So even with `default-features = false, features = ["codegen"]` on the dev-dependency entry, the
example still ended up compiling the unconditional `ffi` module transitively through the main
crate — and MSVC does not reliably dead-strip its unreferenced `SzHelper_free` extern the way
macOS/Linux linkers do, causing an unresolved-external link failure.

Being a wholly independent Cargo project (own `Cargo.toml`, no dependency edge shared with
`sz-rust-sdk`'s main lib) means `sz-rust-sdk-ffi`'s `ffi` feature is never enabled here at all —
this crate compiles only the `codegen` module, so `SzHelper_free` never appears in its object code
in the first place.

## Usage

```sh
cd tools/generate-error-mappings
cargo run
```

Drop a `szerrors.json` in the main repo root, set `SENZING_DIR`, or install the Senzing SDK via
Homebrew/Scoop/apt — see `main.rs` for the full search order (unchanged from the original script).
