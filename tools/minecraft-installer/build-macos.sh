#!/bin/bash
# Build script for macOS (run on macOS)

set -e

echo "Building minecraft-installer for macOS..."

# Install dependencies
brew install openssl

# Set environment variables
export OPENSSL_ROOT_DIR=$(brew --prefix openssl)
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include

# Build for Intel Macs
echo "Building for Intel Macs..."
cargo build --release --target x86_64-apple-darwin

# Build for Apple Silicon Macs
echo "Building for Apple Silicon Macs..."
cargo build --release --target aarch64-apple-darwin

# Create output directory
mkdir -p dist

# Copy binaries
cp target/x86_64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-intel-x86_64
cp target/aarch64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-apple-silicon-aarch64

echo "✓ macOS builds complete:"
echo "  - dist/minecraft-installer-macos-intel-x86_64"
echo "  - dist/minecraft-installer-macos-apple-silicon-aarch64"

