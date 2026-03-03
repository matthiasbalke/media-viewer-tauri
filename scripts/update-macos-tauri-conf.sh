#!/usr/bin/env bash
# update-macos-tauri-conf.sh
#
# Collects libheif and all its Homebrew transitive dylib dependencies into
# src-tauri/macos-frameworks/ and regenerates src-tauri/tauri.macos.conf.json
# so that Tauri bundles them into Contents/Frameworks/ during the macOS build.
#
# Run this script whenever you update libheif (brew upgrade libheif).
# The generated tauri.macos.conf.json should be committed to version control.
#
# Usage:
#   ./scripts/update-macos-tauri-conf.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FRAMEWORKS_DIR="$REPO_ROOT/src-tauri/macos-frameworks"
CONF_FILE="$REPO_ROOT/src-tauri/tauri.macos.conf.json"

if [[ "$(uname)" != "Darwin" ]]; then
  echo "This script is macOS-only." >&2
  exit 1
fi

if ! command -v brew &>/dev/null; then
  echo "Homebrew is required but not found." >&2
  exit 1
fi

if ! brew list libheif &>/dev/null; then
  echo "libheif is not installed. Run: brew install libheif" >&2
  exit 1
fi

echo "Collecting libheif frameworks into $FRAMEWORKS_DIR ..."
rm -rf "$FRAMEWORKS_DIR"
mkdir -p "$FRAMEWORKS_DIR"

# Recursively collect libheif and all Homebrew transitive dependencies.
collect_deps() {
  local lib="$1"
  local name
  name=$(basename "$lib")
  [[ -f "$FRAMEWORKS_DIR/$name" ]] && return
  echo "  + $name"
  cp "$lib" "$FRAMEWORKS_DIR/"
  otool -L "$lib" 2>/dev/null \
    | awk '/^[[:space:]]+\// && /\/(opt\/homebrew|usr\/local)\// && !/\/usr\/lib/{print $1}' \
    | while read -r dep; do
        collect_deps "$dep"
      done
}

collect_deps "$(brew --prefix libheif)/lib/libheif.dylib"

echo ""
echo "Generating $CONF_FILE ..."
python3 - <<EOF
import os, json

files = sorted(os.listdir("$FRAMEWORKS_DIR"))
config = {
    "bundle": {
        "macOS": {
            "frameworks": ["macos-frameworks/" + f for f in files]
        }
    }
}

with open("$CONF_FILE", "w") as fp:
    json.dump(config, fp, indent=2)
    fp.write("\n")

print(open("$CONF_FILE").read())
EOF

echo "Done. Commit src-tauri/tauri.macos.conf.json to version control."
