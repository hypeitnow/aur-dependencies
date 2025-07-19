#!/usr/bin/env bash
# A convenience wrapper around the aur-dependency-tracker Rust binary.
# If the binary is missing, this script will invoke the build script first.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

BINARY="$PROJECT_ROOT/bin/aur-dependency-tracker"

if [[ ! -x "$BINARY" ]]; then
  echo "[cli.sh] Binary not found or not executable. Building project first..."
  "$PROJECT_ROOT/scripts/build.sh"
fi

exec "$BINARY" "$@"