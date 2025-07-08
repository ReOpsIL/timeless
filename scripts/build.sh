#!/bin/bash
# scripts/build.sh

set -e

echo "Building Timeless in release mode..."
cargo build --release

echo "Build complete! Binary available at: target/release/timeless"
echo "Run with: ./target/release/timeless --help"