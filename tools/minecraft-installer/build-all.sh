#!/bin/bash

# Build script for both minecraft-installer and minecraft-updater cross-compilation
# This script builds for multiple platforms

set -e

echo "Building Minecraft Tools for multiple platforms..."
echo

# Create output directory
mkdir -p dist

# Build for Windows (MSVC) - native
echo "================================================================"
echo "Building for Windows (x86_64-pc-windows-msvc)..."
echo "================================================================"
echo "[1/2] minecraft-installer..."
if cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-installer; then
    cp target/x86_64-pc-windows-msvc/release/minecraft-installer.exe dist/minecraft-installer-windows-x86_64.exe
    echo "✓ minecraft-installer built"
else
    echo "✗ minecraft-installer build failed"
fi

echo "[2/2] minecraft-updater..."
if cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-updater; then
    cp target/x86_64-pc-windows-msvc/release/minecraft-updater.exe dist/minecraft-updater-windows-x86_64.exe
    echo "✓ minecraft-updater built"
else
    echo "✗ minecraft-updater build failed"
fi
echo

# Build for Windows (GNU) - alternative Windows build
echo "================================================================"
echo "Building for Windows GNU (x86_64-pc-windows-gnu)..."
echo "================================================================"
echo "[1/2] minecraft-installer..."
if cargo build --release --target x86_64-pc-windows-gnu --bin minecraft-installer; then
    cp target/x86_64-pc-windows-gnu/release/minecraft-installer.exe dist/minecraft-installer-windows-gnu-x86_64.exe
    echo "✓ minecraft-installer built"
else
    echo "✗ minecraft-installer build failed"
fi

echo "[2/2] minecraft-updater..."
if cargo build --release --target x86_64-pc-windows-gnu --bin minecraft-updater; then
    cp target/x86_64-pc-windows-gnu/release/minecraft-updater.exe dist/minecraft-updater-windows-gnu-x86_64.exe
    echo "✓ minecraft-updater built"
else
    echo "✗ minecraft-updater build failed"
fi
echo

# Build for Linux
echo "================================================================"
echo "Building for Linux (x86_64-unknown-linux-gnu)..."
echo "================================================================"
echo "[1/2] minecraft-installer..."
if cargo build --release --target x86_64-unknown-linux-gnu --bin minecraft-installer; then
    cp target/x86_64-unknown-linux-gnu/release/minecraft-installer dist/minecraft-installer-linux-x86_64
    chmod +x dist/minecraft-installer-linux-x86_64
    echo "✓ minecraft-installer built"
else
    echo "✗ minecraft-installer build failed"
fi

echo "[2/2] minecraft-updater..."
if cargo build --release --target x86_64-unknown-linux-gnu --bin minecraft-updater; then
    cp target/x86_64-unknown-linux-gnu/release/minecraft-updater dist/minecraft-updater-linux-x86_64
    chmod +x dist/minecraft-updater-linux-x86_64
    echo "✓ minecraft-updater built"
else
    echo "✗ minecraft-updater build failed"
fi
echo

# Build for macOS Intel
echo "================================================================"
echo "Building for macOS Intel (x86_64-apple-darwin)..."
echo "================================================================"
echo "[1/2] minecraft-installer..."
if cargo build --release --target x86_64-apple-darwin --bin minecraft-installer; then
    cp target/x86_64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-intel-x86_64
    chmod +x dist/minecraft-installer-macos-intel-x86_64
    echo "✓ minecraft-installer built"
else
    echo "✗ minecraft-installer build failed"
fi

echo "[2/2] minecraft-updater..."
if cargo build --release --target x86_64-apple-darwin --bin minecraft-updater; then
    cp target/x86_64-apple-darwin/release/minecraft-updater dist/minecraft-updater-macos-intel-x86_64
    chmod +x dist/minecraft-updater-macos-intel-x86_64
    echo "✓ minecraft-updater built"
else
    echo "✗ minecraft-updater build failed"
fi
echo

# Build for macOS Apple Silicon
echo "================================================================"
echo "Building for macOS Apple Silicon (aarch64-apple-darwin)..."
echo "================================================================"
echo "[1/2] minecraft-installer..."
if cargo build --release --target aarch64-apple-darwin --bin minecraft-installer; then
    cp target/aarch64-apple-darwin/release/minecraft-installer dist/minecraft-installer-macos-apple-silicon-aarch64
    chmod +x dist/minecraft-installer-macos-apple-silicon-aarch64
    echo "✓ minecraft-installer built"
else
    echo "✗ minecraft-installer build failed"
fi

echo "[2/2] minecraft-updater..."
if cargo build --release --target aarch64-apple-darwin --bin minecraft-updater; then
    cp target/aarch64-apple-darwin/release/minecraft-updater dist/minecraft-updater-macos-apple-silicon-aarch64
    chmod +x dist/minecraft-updater-macos-apple-silicon-aarch64
    echo "✓ minecraft-updater built"
else
    echo "✗ minecraft-updater build failed"
fi
echo

echo "================================================================"
echo "All builds completed!"
echo "================================================================"
echo "Output files:"
ls -lh dist/








