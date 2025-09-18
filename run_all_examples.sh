#!/bin/bash

# Script to run all Senzing Rust SDK examples
# This helps verify that all examples work correctly and identify any memory errors

# Note: We don't use set -e here because we want to continue running examples even if some fail

echo "=== Running All Senzing Rust SDK Examples ==="
echo "Date: $(date)"
echo

# List of all examples (extracted from file paths)
examples=(
    "basic_usage"
    "complete_workflow"
    "manage_configuration"
    "register_data_sources"
    "database_demo"
    "delete_records"
    "engine_operations"
    "check_datastore_performance"
    "get_version"
    "engine_priming"
    "environment_and_hubs"
    "load_records"
    "load_with_info"
    "minimal_test"
    "search_records"
    "why_search"
    "simple_demo"
    "simple_working_demo"
    "successful_demo"
    "test_config_setup"
    "test_config_via_trait"
    "test_error_messages"
    "test_get_existing_instance"
    "test_senz7220"
    "test_singleton"
    "working_demo"
)

# Counters
total_examples=${#examples[@]}
passed=0
failed=0
timeout_examples=()
failed_examples=()

echo "Found $total_examples examples to run"
echo

# Function to run a single example
run_example() {
    local example_name="$1"
    local timeout_duration="30"  # 30 second timeout for each example

    echo "[$((passed + failed + 1))/$total_examples] Running: $example_name"

    # Run with timeout to prevent hanging
    if timeout "$timeout_duration" cargo run --example "$example_name" >/dev/null 2>&1; then
        echo "  ✅ PASSED: $example_name"
        ((passed++))
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "  ⏰ TIMEOUT: $example_name (exceeded ${timeout_duration}s)"
            timeout_examples+=("$example_name")
        else
            echo "  ❌ FAILED: $example_name (exit code: $exit_code)"
            failed_examples+=("$example_name")
        fi
        ((failed++))
    fi
    echo
}

# Run all examples
for example in "${examples[@]}"; do
    run_example "$example"
done

# Summary
echo "=== SUMMARY ==="
echo "Total examples: $total_examples"
echo "Passed: $passed"
echo "Failed: $failed"
echo

if [ ${#failed_examples[@]} -gt 0 ]; then
    echo "Failed examples:"
    for example in "${failed_examples[@]}"; do
        echo "  - $example"
    done
    echo
fi

if [ ${#timeout_examples[@]} -gt 0 ]; then
    echo "Timed out examples:"
    for example in "${timeout_examples[@]}"; do
        echo "  - $example"
    done
    echo
fi

# Exit with appropriate code
if [ $failed -eq 0 ]; then
    echo "🎉 All examples completed successfully!"
    exit 0
else
    echo "⚠️  Some examples failed. Check the output above for details."
    exit 1
fi