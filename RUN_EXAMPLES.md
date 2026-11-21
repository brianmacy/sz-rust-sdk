# Running All Examples

This directory contains scripts to run all Senzing Rust SDK examples for testing and verification purposes.

## Scripts

### `run_all_examples.sh`

Runs all examples quietly with just pass/fail status. Good for quick verification.

```bash
./run_all_examples.sh
```

**Features:**

- ✅ Runs all 33 examples (with `--all-features`, so `forward_to_log` runs too)
- ⏰ 30-second timeout per example
- 📊 Summary report with pass/fail counts
- 🔇 Silent execution (no output from examples)
- 🚀 Fast execution for CI/testing

### `run_all_examples_verbose.sh`

Runs all examples with full output. Good for debugging issues.

```bash
./run_all_examples_verbose.sh
```

**Features:**

- ✅ Runs all 33 examples (with `--all-features`, so `forward_to_log` runs too)
- ⏰ 30-second timeout per example
- 📋 Full output from each example
- 🐛 Detailed error information
- 📊 Comprehensive summary report

## Example Output

### Silent Mode

```
=== Running All Senzing Rust SDK Examples ===
Found 33 examples to run

[1/33] Running: basic_usage
  ✅ PASSED: basic_usage

[2/33] Running: complete_workflow
  ✅ PASSED: complete_workflow

...

=== SUMMARY ===
Total examples: 33
Passed: 33
Failed: 0

🎉 All examples completed successfully!
```

### Verbose Mode

Shows full compilation and execution output for each example, making it easy to debug any issues.

## Use Cases

### Development Testing

```bash
# Quick check after code changes
./run_all_examples.sh

# Debug specific issues
./run_all_examples_verbose.sh
```

### Continuous Integration

```bash
# In CI pipeline
./run_all_examples.sh || exit 1
```

### Memory Error Detection

Both scripts help identify:

- Memory leaks
- Double free errors
- Segmentation faults
- Infinite loops (via timeout)
- Compilation errors

## Exit Codes

- `0` - All examples passed
- `1` - One or more examples failed or timed out

## Requirements

- Rust toolchain
- Senzing SDK installed (see README for platform-specific instructions)
- All project dependencies (`cargo build` should work)
