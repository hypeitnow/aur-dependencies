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

echo "=== Building aur-dependency-tracker (release) ==="

# Compile the Rust project
cargo build --manifest-path "${RUST_DIR}/Cargo.toml" --release

echo "=== Build finished ==="

# Ensure the bin directory exists and copy the compiled binary there
mkdir -p "${BIN_DIR}"
cp "${RUST_DIR}/target/release/aur-dependency-tracker" "${BIN_DIR}/" || {
  echo "Error: Compiled binary not found. Did the build succeed?" >&2
  exit 1
}

echo "Binary copied to ${BIN_DIR}/aur-dependency-tracker"

echo "Done."