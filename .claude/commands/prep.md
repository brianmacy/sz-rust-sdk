# Prepare for Commit

Run all validation checks before committing. All checks must pass.

## Required Environment Variables

Set these before running:
```bash
# Homebrew Senzing installation (macOS)
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib

# Senzing resource paths (Homebrew)
export SENZING_CONFIGPATH=/opt/homebrew/opt/senzing/runtime/er/etc
export SENZING_RESOURCEPATH=/opt/homebrew/opt/senzing/runtime/er/resources
export SENZING_SUPPORTPATH=/opt/homebrew/opt/senzing/runtime/data
export SENZING_TEMPLATE_DB=/opt/homebrew/opt/senzing/runtime/er/resources/templates/G2C.db
```

## Checks to Run (in order)

1. **Stale Files Check** - Check for temporary/planning files in git staging
   - Run: `git status --porcelain` to get all staged and modified files
   - Flag any files matching these patterns:
     - `plan*.md`, `PLAN*.md`, `*_plan.md` - planning documents
     - `TODO.md`, `NOTES.md`, `SCRATCH.md` - personal notes
     - `commit_message*.txt` - saved commit messages
     - `*.tmp`, `*.temp`, `*.bak`, `*.backup` - temporary/backup files
     - `*.log` - log files (unless in a logs/ directory that's intentional)
     - `*.orig` - merge conflict originals
     - `*.swp`, `*.swo`, `*~` - editor swap/backup files
     - `.DS_Store` - macOS metadata
     - Files in `tmp/`, `temp/`, `.tmp/` directories
     - `debug_*.rs`, `test_scratch*.rs` - debug/scratch code files
   - If any flagged files are staged (lines starting with 'A' or 'M'):
     - List them and ask user: "These files appear to be temporary/planning docs. Remove from staging? (y/n)"
     - If yes: run `git reset HEAD <file>` for each
     - If no: continue with warning

2. **Format Check** - `cargo fmt -- --check`
   - If fails: run `cargo fmt` to fix

3. **Clippy** - `cargo clippy --all-targets --all-features -- -D warnings`
   - Must pass with zero warnings

4. **Build** - `cargo build --all-targets`
   - Full build including tests and examples

5. **Deny Check** - `cargo deny check`
   - License and dependency validation

6. **Audit** - `cargo audit`
   - Security vulnerability check

7. **Tests** - `cargo test`
   - All tests must pass (currently 223 tests)

8. **Examples** - Run all examples in `examples/` directory
   - Each example must complete successfully
   - Use: `cargo run --example <name>` for each example

9. **Documentation** - `cargo doc --no-deps`
   - Documentation must build without errors or warnings

## Execution

Run each check sequentially. Stop and report on first failure.
Report final status with checkmarks for each step:

```
✅ stale files (none found)
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

Or with stale files found:
```
⚠️ stale files - Found 2 potentially temporary files:
   - commit_message.txt
   - PLAN.md
   Remove from staging? [prompted user, removed]
✅ fmt
...
```

Or on failure:
```
✅ stale files (none found)
✅ fmt
✅ clippy
❌ build - [error details]

Fix required before commit.
```
