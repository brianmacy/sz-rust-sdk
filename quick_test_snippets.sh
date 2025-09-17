#!/bin/bash

#
# Quick Test Script for Senzing Rust SDK Code Snippets
#
# This is a simplified script for quickly testing all code snippets
# with minimal output and fast execution.
#

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Quick Testing All Rust Code Snippets...${NC}"
echo

# Set default environment if not set
if [[ -z "$SENZING_ENGINE_CONFIGURATION_JSON" ]]; then
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
fi

# Counters
total=0
success=0
failed=0

# Test each code snippet
for toml in $(find code-snippets -name "Cargo.toml" 2>/dev/null | sort); do
    dir=$(dirname "$toml")
    name=$(basename "$dir")
    category=$(basename "$(dirname "$dir")")

    total=$((total + 1))

    echo -n "Testing $category/$name... "

    cd "$dir"
    if timeout 15s cargo run >/dev/null 2>&1; then
        echo -e "${GREEN}✅${NC}"
        success=$((success + 1))
    else
        echo -e "${RED}❌${NC}"
        failed=$((failed + 1))
    fi
    cd - >/dev/null
done

echo
echo "Results: $success/$total passed"
if [[ $failed -eq 0 ]]; then
    echo -e "${GREEN}🎉 All code snippets passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ $failed code snippets failed${NC}"
    exit 1
fi