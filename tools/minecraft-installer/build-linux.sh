#!/bin/bash
# Build script for Linux (run on Linux or WSL)

set -e

echo "Building minecraft-installer for Linux..."

# Install dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev

# Build
cargo build --release --target x86_64-unknown-linux-gnu

# Create output directory
mkdir -p dist

# Copy binary
cp target/x86_64-unknown-linux-gnu/release/minecraft-installer dist/minecraft-installer-linux-x86_64

echo "✓ Linux build complete: dist/minecraft-installer-linux-x86_64"








