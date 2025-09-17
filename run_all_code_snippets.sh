#!/bin/bash

#
# Senzing Rust SDK Code Snippets Runner
#
# This script runs all Rust code snippet examples in order, providing
# detailed output and error handling for each example.
#

set -e  # Exit on error

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CODE_SNIPPETS_DIR="$SCRIPT_DIR/code-snippets/rust/snippets"
LOG_FILE="$SCRIPT_DIR/code_snippets_run.log"
RESULTS_FILE="$SCRIPT_DIR/code_snippets_results.txt"

# Statistics
TOTAL_COUNT=0
SUCCESS_COUNT=0
FAILED_COUNT=0
SKIPPED_COUNT=0

# Arrays to track results
declare -a SUCCESSFUL_EXAMPLES=()
declare -a FAILED_EXAMPLES=()
declare -a SKIPPED_EXAMPLES=()

#
# Utility Functions
#

print_header() {
    echo -e "${BLUE}${BOLD}"
    echo "=================================================================="
    echo "         Senzing Rust SDK Code Snippets Runner"
    echo "=================================================================="
    echo -e "${NC}"
    echo "Script: $(basename "$0")"
    echo "Date: $(date)"
    echo "Directory: $CODE_SNIPPETS_DIR"
    echo "Log file: $LOG_FILE"
    echo "Results file: $RESULTS_FILE"
    echo
}

print_section() {
    local title="$1"
    echo -e "${CYAN}${BOLD}"
    echo "--- $title ---"
    echo -e "${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

#
# Validation Functions
#

check_prerequisites() {
    print_section "Checking Prerequisites"

    # Check if running from correct directory
    if [[ ! -d "$CODE_SNIPPETS_DIR" ]]; then
        print_error "Code snippets directory not found: $CODE_SNIPPETS_DIR"
        print_info "Please run this script from the sz-rust-sdk root directory"
        exit 1
    fi

    # Check for required environment variable
    if [[ -z "$SENZING_ENGINE_CONFIGURATION_JSON" ]]; then
        print_warning "SENZING_ENGINE_CONFIGURATION_JSON not set"
        print_info "Setting default configuration for SQLite..."
        export SENZING_ENGINE_CONFIGURATION_JSON='{
            "PIPELINE": {
                "CONFIGPATH": "/etc/opt/senzing",
                "RESOURCEPATH": "/opt/senzing/er/resources",
                "SUPPORTPATH": "/opt/senzing/data"
            },
            "SQL": {
                "CONNECTION": "sqlite3://na:na@/tmp/G2C.db"
            }
        }'
        echo "Environment variable set to default SQLite configuration"
    else
        print_success "SENZING_ENGINE_CONFIGURATION_JSON is configured"
    fi

    # Check for Rust/Cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo (Rust package manager) not found"
        print_info "Please install Rust: https://rustup.rs/"
        exit 1
    fi
    print_success "Cargo found: $(cargo --version)"

    # Check for Senzing installation
    if [[ ! -d "/opt/senzing/er" ]]; then
        print_warning "Senzing SDK not found at /opt/senzing/er"
        print_info "Some examples may fail without proper Senzing installation"
    else
        print_success "Senzing SDK installation detected"
    fi

    echo
}

#
# Example Discovery and Organization
#

discover_examples() {
    print_section "Discovering Code Snippets"

    # Define the order to run examples (dependencies first)
    declare -a EXAMPLE_ORDER=(
        "information/get_version"
        "information/get_license"
        "initialization/environment_and_hubs"
        "initialization/engine_priming"
        "configuration/init_default_config"
        "configuration/register_data_sources"
        "loading/load_records"
        "loading/load_via_loop"
        "loading/load_via_loop_threadpool"
        "searching/search_records"
        "searching/search_threadpool"
        "searching/why_search"
        "information/database_demo"
        "deleting/delete_records"
        "deleting/delete_via_loop"
        "redo/load_with_redo_via_loop"
        "redo/redo_continuous"
        "redo/redo_continuous_via_futures"
        "redo/redo_with_info_continuous"
        "stewardship/force_resolve"
        "stewardship/force_unresolve"
        "initialization/purge_repository"
    )

    # Verify all examples exist
    for example in "${EXAMPLE_ORDER[@]}"; do
        local example_dir="$CODE_SNIPPETS_DIR/$example"
        if [[ -d "$example_dir" && -f "$example_dir/Cargo.toml" ]]; then
            TOTAL_COUNT=$((TOTAL_COUNT + 1))
            print_success "Found: $example"
        else
            print_warning "Missing: $example"
        fi
    done

    echo "Total examples found: $TOTAL_COUNT"
    echo
}

#
# Example Execution
#

run_single_example() {
    local example_path="$1"
    local example_name="$(basename "$example_path")"
    local category="$(basename "$(dirname "$example_path")")"
    local full_name="$category/$example_name"

    print_info "Running: $full_name"
    echo "----------------------------------------"

    # Change to example directory
    local example_dir="$CODE_SNIPPETS_DIR/$example_path"
    if [[ ! -d "$example_dir" ]]; then
        print_error "Directory not found: $example_dir"
        FAILED_EXAMPLES+=("$full_name (directory not found)")
        FAILED_COUNT=$((FAILED_COUNT + 1))
        return 1
    fi

    cd "$example_dir"

    # Check if Cargo.toml exists
    if [[ ! -f "Cargo.toml" ]]; then
        print_error "Cargo.toml not found in $example_dir"
        FAILED_EXAMPLES+=("$full_name (no Cargo.toml)")
        FAILED_COUNT=$((FAILED_COUNT + 1))
        return 1
    fi

    # Compile the example first
    echo "Compiling $full_name..."
    if ! cargo check &>> "$LOG_FILE"; then
        print_error "Compilation failed for $full_name"
        FAILED_EXAMPLES+=("$full_name (compilation failed)")
        FAILED_COUNT=$((FAILED_COUNT + 1))
        return 1
    fi

    # Run the example with timeout
    echo "Executing $full_name..."
    local start_time=$(date +%s)

    # Determine timeout based on example type
    local timeout_seconds=30
    case "$full_name" in
        */redo_continuous|*/redo_continuous_via_futures|*/redo_with_info_continuous)
            # Continuous processing examples are designed to run for ~30 seconds
            # Give them extra time to avoid race conditions
            timeout_seconds=35
            ;;
    esac

    # Determine run arguments based on example type
    local run_args=""
    case "$full_name" in
        */purge_repository)
            # Purge repository requires confirmation - auto-confirm for testing
            run_args="-- --auto-confirm"
            ;;
    esac

    # Use timeout to prevent hanging
    if timeout ${timeout_seconds}s cargo run $run_args &>> "$LOG_FILE"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_success "$full_name completed successfully (${duration}s)"
        SUCCESSFUL_EXAMPLES+=("$full_name (${duration}s)")
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        return 0
    else
        local exit_code=$?
        if [[ $exit_code == 124 ]]; then
            print_error "$full_name timed out (>${timeout_seconds}s)"
            FAILED_EXAMPLES+=("$full_name (timeout >${timeout_seconds}s)")
        else
            print_error "$full_name failed with exit code $exit_code"
            FAILED_EXAMPLES+=("$full_name (exit code $exit_code)")
        fi
        FAILED_COUNT=$((FAILED_COUNT + 1))
        return 1
    fi
}

run_all_examples() {
    print_section "Running All Code Snippets"

    # Initialize log file
    echo "Code Snippets Execution Log - $(date)" > "$LOG_FILE"
    echo "================================================" >> "$LOG_FILE"

    # Define execution order
    local examples=(
        "information/get_version"
        "information/get_license"
        "initialization/environment_and_hubs"
        "initialization/engine_priming"
        "configuration/init_default_config"
        "configuration/register_data_sources"
        "loading/load_records"
        "loading/load_via_loop"
        "loading/load_via_loop_threadpool"
        "searching/search_records"
        "searching/search_threadpool"
        "searching/why_search"
        "information/database_demo"
        "deleting/delete_records"
        "deleting/delete_via_loop"
        "redo/load_with_redo_via_loop"
        "redo/redo_continuous"
        "redo/redo_continuous_via_futures"
        "redo/redo_with_info_continuous"
        "stewardship/force_resolve"
        "stewardship/force_unresolve"
        "initialization/purge_repository"
    )

    local current=1
    for example in "${examples[@]}"; do
        echo
        echo "[$current/$TOTAL_COUNT] Processing: $example"
        echo "[$current/$TOTAL_COUNT] Processing: $example" >> "$LOG_FILE"
        echo "======================================" >> "$LOG_FILE"

        run_single_example "$example"

        # Brief pause between examples
        sleep 1
        current=$((current + 1))
    done

    # Return to original directory
    cd "$SCRIPT_DIR"
}

#
# Results and Cleanup
#

generate_results_report() {
    print_section "Generating Results Report"

    # Create detailed results file
    cat > "$RESULTS_FILE" << EOF
Senzing Rust SDK Code Snippets Execution Report
===============================================
Date: $(date)
Script: $(basename "$0")
Directory: $CODE_SNIPPETS_DIR

SUMMARY
-------
Total Examples: $TOTAL_COUNT
Successful: $SUCCESS_COUNT
Failed: $FAILED_COUNT
Skipped: $SKIPPED_COUNT
Success Rate: $(( SUCCESS_COUNT * 100 / TOTAL_COUNT ))%

SUCCESSFUL EXAMPLES ($SUCCESS_COUNT)
$(printf '%s\n' "${SUCCESSFUL_EXAMPLES[@]}" | sed 's/^/  ✅ /')

FAILED EXAMPLES ($FAILED_COUNT)
$(printf '%s\n' "${FAILED_EXAMPLES[@]}" | sed 's/^/  ❌ /')

DETAILED LOG
============
See: $LOG_FILE

EOF

    # Display summary
    echo
    echo -e "${BOLD}EXECUTION SUMMARY${NC}"
    echo "=================================="
    echo "Total Examples: $TOTAL_COUNT"
    echo -e "Successful: ${GREEN}$SUCCESS_COUNT${NC}"
    echo -e "Failed: ${RED}$FAILED_COUNT${NC}"
    echo -e "Success Rate: ${CYAN}$(( SUCCESS_COUNT * 100 / TOTAL_COUNT ))%${NC}"
    echo

    if [[ $SUCCESS_COUNT -gt 0 ]]; then
        echo -e "${GREEN}${BOLD}Successful Examples:${NC}"
        printf '%s\n' "${SUCCESSFUL_EXAMPLES[@]}" | sed 's/^/  ✅ /'
        echo
    fi

    if [[ $FAILED_COUNT -gt 0 ]]; then
        echo -e "${RED}${BOLD}Failed Examples:${NC}"
        printf '%s\n' "${FAILED_EXAMPLES[@]}" | sed 's/^/  ❌ /'
        echo
    fi

    print_info "Detailed results written to: $RESULTS_FILE"
    print_info "Execution log written to: $LOG_FILE"
}

show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --verbose  Enable verbose output"
    echo "  -l, --list     List all available examples without running"
    echo "  -c, --check    Only check compilation, don't run examples"
    echo
    echo "Examples:"
    echo "  $0                    # Run all code snippets"
    echo "  $0 --list            # List all available examples"
    echo "  $0 --check           # Only compile, don't execute"
    echo
}

#
# Main Execution
#

main() {
    local action="run"
    local verbose=false

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verbose)
                verbose=true
                shift
                ;;
            -l|--list)
                action="list"
                shift
                ;;
            -c|--check)
                action="check"
                shift
                ;;
            *)
                echo "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    # Always show header
    print_header

    # Check prerequisites
    check_prerequisites

    # Discover examples
    discover_examples

    case $action in
        "list")
            print_section "Available Code Snippets"
            find "$CODE_SNIPPETS_DIR" -name "Cargo.toml" -exec dirname {} \; | \
                sed "s|$CODE_SNIPPETS_DIR/||" | sort
            ;;
        "check")
            print_section "Checking Compilation Only"
            # Implement compilation check logic here
            ;;
        "run")
            run_all_examples
            generate_results_report

            # Exit with error code if any examples failed
            if [[ $FAILED_COUNT -gt 0 ]]; then
                exit 1
            fi
            ;;
    esac
}

# Execute main function with all arguments
main "$@"