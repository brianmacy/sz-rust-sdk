#!/bin/bash

# Verbose script to run all Senzing Rust SDK examples with full output
# This helps debug issues and shows detailed execution information

# Note: We don't use set -e here because we want to continue running examples even if some fail

echo "=== Running All Senzing Rust SDK Examples (VERBOSE) ==="
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

# Function to run a single example with full output
run_example_verbose() {
    local example_name="$1"
    local timeout_duration="30"  # 30 second timeout for each example

    echo "====================================================================="
    echo "[$((passed + failed + 1))/$total_examples] Running: $example_name"
    echo "====================================================================="

    # Run with timeout and show full output
    if timeout "$timeout_duration" cargo run --example "$example_name"; then
        echo
        echo "  ✅ PASSED: $example_name"
        ((passed++))
    else
        local exit_code=$?
        echo
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
    echo
}

# Run all examples
for example in "${examples[@]}"; do
    run_example_verbose "$example"
done

# Summary
echo "====================================================================="
echo "=== FINAL SUMMARY ==="
echo "====================================================================="
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