#!/bin/bash
# scripts/test.sh

set -e

echo "Running Timeless tests..."

# Run unit tests
echo "Running unit tests..."
cargo test --lib

# Run integration tests
echo "Running integration tests..."
cargo test --test integration

# Run doc tests
echo "Running documentation tests..."
cargo test --doc

echo "All tests completed!"