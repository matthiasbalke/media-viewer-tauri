#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Construct the path to the LCOV file
LCOV_PATH="${SCRIPT_DIR}/../src-tauri/target/llvm-cov/html/index.html"

# Create the code coverage report
cd ${SCRIPT_DIR}/../src-tauri
cargo llvm-cov --all-features --workspace --html
echo "Created code coverage report: $LCOV_PATH"

# Open the code coverage report
open "$LCOV_PATH"
