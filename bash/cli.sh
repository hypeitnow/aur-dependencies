#!/usr/bin/env bash
# Wrapper CLI for the aur-dependency-tracker project
#
# This script ensures the project is built (using ./build.sh) and then executes
# the resulting binary, forwarding any provided arguments.

set -euo pipefail

# Determine project root (one directory up from this script's location)
ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"

BIN_DIR="${ROOT_DIR}/bin"
BINARY="${BIN_DIR}/aur-dependency-tracker"

# If binary is missing or older than any Rust source file, rebuild.
if [[ ! -x "${BINARY}" ]]; then
  echo "Binary not found. Building project first…"
  "${ROOT_DIR}/build.sh"
fi

# Execute the compiled binary, forwarding all CLI arguments
exec "${BINARY}" "$@"