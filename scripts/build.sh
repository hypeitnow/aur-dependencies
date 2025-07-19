#!/usr/bin/env bash
# Build the Rust project and place the resulting binary in the project-root/bin directory.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Ensure we are inside the Rust crate directory
cd "$PROJECT_ROOT/rust"

echo "[build.sh] Building aur-dependency-tracker in release mode..."

cargo build --release

BINARY_PATH="target/release/aur-dependency-tracker"
OUTPUT_DIR="$PROJECT_ROOT/bin"

mkdir -p "$OUTPUT_DIR"
cp "$BINARY_PATH" "$OUTPUT_DIR/"

echo "[build.sh] Build complete. Binary available at $OUTPUT_DIR/aur-dependency-tracker"