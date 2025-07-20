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
RUST_SRC_DIR="${ROOT_DIR}/rust/src"

# Find the newest Rust source file
NEWEST_SRC=$(find "${RUST_SRC_DIR}" -type f -name '*.rs' -print0 | xargs -0 stat -c '%Y' | sort -nr | head -n1)
BINARY_MTIME=0
if [[ -f "${BINARY}" ]]; then
  BINARY_MTIME=$(stat -c '%Y' "${BINARY}")
fi

# If binary is missing or older than any Rust source file, rebuild.
if [[ ! -x "${BINARY}" || "${NEWEST_SRC}" -gt "${BINARY_MTIME}" ]]; then
  echo "Binary missing or out of date. Building project first…"
  echo "+ ${ROOT_DIR}/build.sh"
  "${ROOT_DIR}/build.sh"
fi

# Print the command being run
echo "+ ${BINARY} $*"
# Execute the compiled binary, forwarding all CLI arguments
exec "${BINARY}" "$@"
