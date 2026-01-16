# Prepare for Commit

Run all validation checks before committing. All checks must pass.

## Required Environment Variables

Set these before running:
```bash
# Library paths (dev build)
export LIBRARY_PATH=/Users/brianmacy/dev/G2/dev/build/dist/lib:/Users/brianmacy/dev/GNR/build/dist/lib
export DYLD_LIBRARY_PATH=/Users/brianmacy/dev/G2/dev/build/dist/lib:/Users/brianmacy/dev/GNR/build/dist/lib

# Senzing resource paths (dev build)
export SENZING_CONFIGPATH=/Users/brianmacy/dev/G2/dev/build/dist/testdata
export SENZING_RESOURCEPATH=/Users/brianmacy/dev/G2/dev/build/dist/resources
export SENZING_SUPPORTPATH=/Users/brianmacy/dev/G2/dev/build/dist/data
export SENZING_TEMPLATE_DB=/Users/brianmacy/dev/G2/dev/apps/g2/python/sz-sdk-python/testdata/sqlite/G2C.db
```

## Checks to Run (in order)

1. **Format Check** - `cargo fmt -- --check`
   - If fails: run `cargo fmt` to fix

2. **Clippy** - `cargo clippy --all-targets --all-features -- -D warnings`
   - Must pass with zero warnings

3. **Build** - `cargo build --all-targets`
   - Full build including tests and examples

4. **Deny Check** - `cargo deny check`
   - License and dependency validation

5. **Audit** - `cargo audit`
   - Security vulnerability check

6. **Tests** - `cargo test`
   - All tests must pass (currently 223 tests)

7. **Examples** - Run all examples in `examples/` directory
   - Each example must complete successfully
   - Use: `cargo run --example <name>` for each example

8. **Documentation** - `cargo doc --no-deps`
   - Documentation must build without errors or warnings

## Execution

Run each check sequentially. Stop and report on first failure.
Report final status with checkmarks for each step:

```
✅ fmt
✅ clippy
✅ build
✅ deny
✅ audit
✅ tests (223 passed)
✅ examples (X/X passed)
✅ docs

Ready for commit!
```

Or on failure:
```
✅ fmt
✅ clippy
❌ build - [error details]

Fix required before commit.
```
