#!/usr/bin/env bash
# Build script for the aur-dependency-tracker project
#
# This script compiles the Rust code in release mode and puts the resulting
# binary in ./bin for easy access.

set -euo pipefail

# Determine the absolute path of the project root (the directory containing this script)
ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Paths
RUST_DIR="${ROOT_DIR}/rust"
BIN_DIR="${ROOT_DIR}/bin"
BINARY="${BIN_DIR}/aur-dependency-tracker"
RUST_SRC_DIR="${RUST_DIR}/src"

# Find the newest Rust source file
NEWEST_SRC=$(find "${RUST_SRC_DIR}" -type f -name '*.rs' -print0 | xargs -0 stat -c '%Y' | sort -nr | head -n1)
BINARY_MTIME=0
if [[ -f "${BINARY}" ]]; then
  BINARY_MTIME=$(stat -c '%Y' "${BINARY}")
fi

if [[ ! -x "${BINARY}" || "${NEWEST_SRC}" -gt "${BINARY_MTIME}" ]]; then
  echo "=== Building aur-dependency-tracker (release) ==="
  echo "+ cargo build --manifest-path \"${RUST_DIR}/Cargo.toml\" --release"
  cargo build --manifest-path "${RUST_DIR}/Cargo.toml" --release
  echo "=== Build finished ==="
  # Ensure the bin directory exists and copy the compiled binary there
  mkdir -p "${BIN_DIR}"
  echo "+ cp \"${RUST_DIR}/target/release/aur-dependency-tracker\" \"${BIN_DIR}/\""
  cp "${RUST_DIR}/target/release/aur-dependency-tracker" "${BIN_DIR}/" || {
    echo "Error: Compiled binary not found. Did the build succeed?" >&2
    exit 1
  }
  echo "Binary copied to ${BIN_DIR}/aur-dependency-tracker"
else
  echo "Binary is up to date. Skipping build."
fi

echo "Done."