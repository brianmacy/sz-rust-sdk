#!/bin/bash

# Run all code snippets and capture results
# This script runs all code snippets from the simplified structure

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Statistics
TOTAL_SNIPPETS=0
SUCCESSFUL_SNIPPETS=0
FAILED_SNIPPETS=0
TIMEOUT_SNIPPETS=0

# Output file
OUTPUT_FILE="code_snippets_results.txt"
LOG_FILE="code_snippets_run.log"

echo "=== Senzing Rust SDK Code Snippets Test Run ===" > "$OUTPUT_FILE"
echo "Started: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "=== Senzing Rust SDK Code Snippets Test Run ==="
echo "Started: $(date)"
echo ""

# Function to run a single snippet
run_snippet() {
    local bin_name="$1"
    local category="$2"
    local snippet_name="$3"

    echo -e "${YELLOW}Testing: $category/$snippet_name${NC}"
    echo "Testing: $category/$snippet_name" >> "$OUTPUT_FILE"

    TOTAL_SNIPPETS=$((TOTAL_SNIPPETS + 1))

    # Change to code-snippets directory and run the specific binary
    pushd code-snippets > /dev/null 2>&1

    # Run with timeout
    if timeout 30 cargo run --bin "$bin_name" >> "../$LOG_FILE" 2>&1; then
        echo -e "  ${GREEN}✓ SUCCESS${NC}"
        echo "  ✓ SUCCESS" >> "../$OUTPUT_FILE"
        SUCCESSFUL_SNIPPETS=$((SUCCESSFUL_SNIPPETS + 1))
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo -e "  ${YELLOW}⏰ TIMEOUT (30s)${NC}"
            echo "  ⏰ TIMEOUT (30s)" >> "../$OUTPUT_FILE"
            TIMEOUT_SNIPPETS=$((TIMEOUT_SNIPPETS + 1))
        else
            echo -e "  ${RED}✗ FAILED${NC}"
            echo "  ✗ FAILED" >> "../$OUTPUT_FILE"
            FAILED_SNIPPETS=$((FAILED_SNIPPETS + 1))
        fi
    fi

    popd > /dev/null 2>&1
    echo "" >> "$OUTPUT_FILE"
    echo ""
}

# Clear previous logs
> "$LOG_FILE"

echo "Scanning for code snippets..." >> "$OUTPUT_FILE"
echo "Scanning for code snippets..."

# Run all snippets by category in alphabetical order
# Configuration snippets
run_snippet "init_default_config" "configuration" "init_default_config.rs"
run_snippet "register_data_sources" "configuration" "register_data_sources.rs"

# Deleting snippets
run_snippet "delete_records" "deleting" "delete_records.rs"
run_snippet "delete_via_loop" "deleting" "delete_via_loop.rs"

# Information snippets
run_snippet "database_demo" "information" "database_demo.rs"
run_snippet "get_license" "information" "get_license.rs"
run_snippet "get_version" "information" "get_version.rs"

# Initialization snippets
run_snippet "engine_priming" "initialization" "engine_priming.rs"
run_snippet "environment_and_hubs" "initialization" "environment_and_hubs.rs"
run_snippet "purge_repository" "initialization" "purge_repository.rs"

# Loading snippets
run_snippet "load_records" "loading" "load_records.rs"
run_snippet "load_via_loop" "loading" "load_via_loop.rs"
run_snippet "load_via_loop_threadpool" "loading" "load_via_loop_threadpool.rs"

# Redo snippets
run_snippet "load_with_redo_via_loop" "redo" "load_with_redo_via_loop.rs"
run_snippet "redo_continuous" "redo" "redo_continuous.rs"
run_snippet "redo_continuous_via_futures" "redo" "redo_continuous_via_futures.rs"
run_snippet "redo_with_info_continuous" "redo" "redo_with_info_continuous.rs"

# Searching snippets
run_snippet "search_records" "searching" "search_records.rs"
run_snippet "search_threadpool" "searching" "search_threadpool.rs"
run_snippet "why_search" "searching" "why_search.rs"

# Stewardship snippets
run_snippet "force_resolve" "stewardship" "force_resolve.rs"
run_snippet "force_unresolve" "stewardship" "force_unresolve.rs"

# Summary
echo "=== SUMMARY ===" >> "$OUTPUT_FILE"
echo "=== SUMMARY ==="
echo "Total snippets: $TOTAL_SNIPPETS" >> "$OUTPUT_FILE"
echo "Successful: $SUCCESSFUL_SNIPPETS" >> "$OUTPUT_FILE"
echo "Failed: $FAILED_SNIPPETS" >> "$OUTPUT_FILE"
echo "Timeouts: $TIMEOUT_SNIPPETS" >> "$OUTPUT_FILE"
echo "Completed: $(date)" >> "$OUTPUT_FILE"

echo "Total snippets: $TOTAL_SNIPPETS"
echo -e "Successful: ${GREEN}$SUCCESSFUL_SNIPPETS${NC}"
echo -e "Failed: ${RED}$FAILED_SNIPPETS${NC}"
echo -e "Timeouts: ${YELLOW}$TIMEOUT_SNIPPETS${NC}"
echo "Completed: $(date)"

# Success rate
if [ $TOTAL_SNIPPETS -gt 0 ]; then
    success_rate=$((SUCCESSFUL_SNIPPETS * 100 / TOTAL_SNIPPETS))
    echo "Success rate: ${success_rate}%" >> "$OUTPUT_FILE"
    echo "Success rate: ${success_rate}%"
fi

echo ""
echo "Results saved to: $OUTPUT_FILE"
echo "Detailed logs saved to: $LOG_FILE"

# Exit with appropriate code
if [ $FAILED_SNIPPETS -gt 0 ]; then
    exit 1
elif [ $TIMEOUT_SNIPPETS -gt 0 ]; then
    exit 2
else
    exit 0
fi