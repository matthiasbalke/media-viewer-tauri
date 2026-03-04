#!/usr/bin/env bash
# update-macos-tauri-conf.sh
#
# Collects libheif and all its Homebrew transitive dylib dependencies,
# copies them into src-tauri/macOS-frameworks/, and generates
# src-tauri/tauri.macos.conf.json with bundle.macOS.frameworks pointing
# to relative paths (e.g. "./macOS-frameworks/libheif.1.dylib").
#
# Tauri requires LOCAL relative paths (relative to src-tauri/) so that it
# can correctly rewrite the binary's LC_LOAD_DYLIB entries from the absolute
# Homebrew path to @executable_path/../Frameworks/ at bundle time.
# See: https://tauri.app/distribute/macos-application-bundle/#including-macos-frameworks
#
# Run this script whenever you update libheif (brew upgrade libheif),
# then commit both src-tauri/macOS-frameworks/ and src-tauri/tauri.macos.conf.json.
#
# Usage:
#   ./scripts/update-macos-tauri-conf.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC_TAURI="$REPO_ROOT/src-tauri"
FRAMEWORKS_DIR="$SRC_TAURI/macOS-frameworks"
CONF_FILE="$SRC_TAURI/tauri.macos.conf.json"

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

echo "Resolving libheif dylib dependencies ..."

# Track visited libraries to avoid infinite loops (bash 3.2-compatible, no declare -A)
VISITED=""
ABSOLUTE_PATHS=()

# Recursively follow otool -L to collect all Homebrew dylib paths.
collect_deps() {
  local lib="$1"
  local name
  name=$(basename "$lib")
  case ":$VISITED:" in *":$name:"*) return ;; esac
  VISITED="$VISITED:$name:"
  echo "  + $lib"
  ABSOLUTE_PATHS+=("$lib")
  while IFS= read -r dep; do
    collect_deps "$dep"
  done < <(
    otool -L "$lib" 2>/dev/null \
      | awk '/^[[:space:]]+\// && /\/(opt\/homebrew|usr\/local)\// && !/\/usr\/lib/{print $1}'
  )
}

collect_deps "$(brew --prefix libheif)/lib/libheif.dylib"

echo ""
echo "Copying dylibs to $FRAMEWORKS_DIR ..."
rm -rf "$FRAMEWORKS_DIR"
mkdir -p "$FRAMEWORKS_DIR"

RELATIVE_PATHS=()
for abs in "${ABSOLUTE_PATHS[@]}"; do
  name=$(basename "$abs")
  cp "$abs" "$FRAMEWORKS_DIR/$name"
  echo "  copied $name"
  RELATIVE_PATHS+=("./macOS-frameworks/$name")
done

echo ""
echo "Generating $CONF_FILE ..."

# Write relative paths to a temp file (one per line) — avoids bash 4.4-only @Q expansion
_TMP_PATHS=$(mktemp)
printf '%s\n' "${RELATIVE_PATHS[@]}" > "$_TMP_PATHS"

python3 - "$_TMP_PATHS" "$CONF_FILE" <<'EOF'
import json, sys

paths_file, conf_file = sys.argv[1], sys.argv[2]
with open(paths_file) as f:
    framework_list = [l.rstrip("\n") for l in f if l.strip()]

config = {
    "bundle": {
        "macOS": {
            "frameworks": framework_list
        }
    }
}

with open(conf_file, "w") as fp:
    json.dump(config, fp, indent=2)
    fp.write("\n")

print(open(conf_file).read())
EOF

rm -f "$_TMP_PATHS"

echo "Done. Commit src-tauri/macOS-frameworks/ and src-tauri/tauri.macos.conf.json to version control."
