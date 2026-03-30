## Code Generation

### FFI Bindings and Error Mappings Are Checked In

Generated code is **checked into source control**, NOT auto-generated at build time. This avoids build-time dependencies on external tools and makes the build reproducible.

### Generator Scripts

Generator scripts live in `scripts/` and are registered as `[[example]]` targets in `Cargo.toml`. Run them with `cargo run --example <script_name>`.

### Error Mapping Generation

```bash
cargo run --example generate_error_mappings
```

Source: `~/dev/G2/dev/build/dist/sdk/szerrors.json`
Output: `src/error_mappings_generated.rs` (456 error codes)

### build.rs

`build.rs` only handles library linking — NO code generation. `serde_json` is in dev-dependencies (for generator scripts), not build-dependencies.

See `CODEGEN.md` for the full workflow.
