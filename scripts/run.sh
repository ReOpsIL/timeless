#!/bin/bash
# scripts/run.sh

set -e

# Check if binary exists
if [ ! -f "target/release/timeless" ]; then
    echo "Binary not found. Building..."
    ./scripts/build.sh
fi

# Run with all arguments passed through
./target/release/timeless "$@"