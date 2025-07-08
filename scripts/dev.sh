#!/bin/bash
# scripts/dev.sh

set -e

echo "Starting Timeless in development mode..."
echo "Watching for changes..."

# Install cargo-watch if not present
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
fi

cargo watch -x "run -- $@"