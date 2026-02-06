# Code Generation

This SDK uses code generation for two components that depend on the Senzing SDK distribution:

1. **FFI Bindings** - Generated from C headers
2. **Error Mappings** - Generated from szerrors.json

Both are **checked into source control** and **NOT regenerated automatically** during normal builds. This ensures:

- Fast builds without requiring Senzing SDK headers/data files
- Stable, reviewable generated code
- CI/CD works without full Senzing SDK installation

## Regenerating FFI Bindings

**When to regenerate:**

- Upgrading to a new Senzing SDK version
- Adding new native function bindings

**How to regenerate:**

```bash
cargo run --example generate_bindings
```

**Requirements:**

- Senzing SDK must be installed (headers needed)
- Set `SENZING_SDK_PATH` if not in default location

**Output:**

- `src/ffi/bindings_generated.rs`

## Regenerating Error Mappings

**When to regenerate:**

- Upgrading to a new Senzing SDK version (new error codes)
- szerrors.json changes

**How to regenerate:**

```bash
cargo run --example generate_error_mappings
```

**Requirements:**

- `szerrors.json` must be available in one of:
  - Project root: `./szerrors.json`
  - G2 dev build: `~/dev/G2/dev/build/dist/sdk/szerrors.json`
  - Homebrew: `/opt/homebrew/opt/senzing/runtime/sdk/szerrors.json`

**Output:**

- `src/error_mappings_generated.rs`

**What it generates:**

1. `map_error_code()` - Maps 456 error codes to SzError variants
2. `get_error_hierarchy()` - Returns error category hierarchy for each code

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

# 2. Regenerate error mappings
cargo run --example generate_error_mappings

# 3. Regenerate FFI bindings (if headers changed)
cargo run --example generate_bindings

# 4. Review changes
git diff src/error_mappings_generated.rs
git diff src/ffi/bindings_generated.rs

# 5. Test
cargo test

# 6. Commit
git add src/error_mappings_generated.rs src/ffi/bindings_generated.rs szerrors.json
git commit -m "Update for Senzing SDK vX.Y.Z"
```

## Files

### Checked Into Version Control

- `src/ffi/bindings_generated.rs` - Generated FFI bindings
- `src/error_mappings_generated.rs` - Generated error mappings
- `szerrors.json` - Senzing error definitions (copied from SDK)

### Generator Scripts

- `scripts/generate_bindings.rs` - FFI binding generator
- `scripts/generate_error_mappings.rs` - Error mapping generator

### NOT Generated

- `src/error.rs` - Hand-written error types and hierarchy logic
- `build.rs` - Only handles library linking, no code generation
