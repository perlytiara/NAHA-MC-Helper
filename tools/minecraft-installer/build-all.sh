#!/bin/bash

# Build script for minecraft-installer cross-compilation
# This script builds for multiple platforms

set -e

echo "Building minecraft-installer for multiple platforms..."

# Create output directories
mkdir -p dist
mkdir -p ../minecraft-installer-releases

# Build for Windows (MSVC) - native
echo "Building for Windows (x86_64-pc-windows-msvc)..."
cargo build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/minecraft-installer.exe dist/minecraft-installer-windows-x86_64.exe
cp target/x86_64-pc-windows-msvc/release/minecraft-installer.exe ../minecraft-installer-releases/minecraft-installer-windows-x86_64.exe

# Build for Windows (GNU) - alternative Windows build
echo "Building for Windows GNU (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/minecraft-installer.exe dist/minecraft-installer-windows-gnu-x86_64.exe
cp target/x86_64-pc-windows-gnu/release/minecraft-installer.exe ../minecraft-installer-releases/minecraft-installer-windows-gnu-x86_64.exe

# Build for Linux
echo "Building for Linux (x86_64-unknown-linux-gnu)..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/minecraft-installer dist/minecraft-installer-linux-x86_64
cp target/x86_64-unknown-linux-gnu/release/minecraft-installer ../minecraft-installer-releases/minecraft-installer-linux-x86_64

# Build for macOS Intel
echo "Building for macOS Intel (x86_64-apple-darwin)..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-intel-x86_64
cp target/x86_64-apple-darwin/release/minecraft-installer ../minecraft-installer-releases/minecraft-installer-macos-intel-x86_64

# Build for macOS Apple Silicon
echo "Building for macOS Apple Silicon (aarch64-apple-darwin)..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-apple-silicon-aarch64
cp target/aarch64-apple-darwin/release/minecraft-installer ../minecraft-installer-releases/minecraft-installer-macos-apple-silicon-aarch64

echo "All builds completed successfully!"
echo "Output files in dist/:"
ls -la dist/
echo "Output files in ../minecraft-installer-releases/:"
ls -la ../minecraft-installer-releases/

