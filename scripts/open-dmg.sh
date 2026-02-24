#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Construct the path to the DMG file
DMG_PATH="${SCRIPT_DIR}/src-tauri/target/release/bundle/dmg/media-viewer_0.1.0_x64.dmg"

# Check if the DMG file exists
if [ -f "$DMG_PATH" ]; then
    # Open the DMG file
    open "$DMG_PATH"
    echo "Opened: $DMG_PATH"
else
    echo "Error: DMG file not found at $DMG_PATH"
fi
