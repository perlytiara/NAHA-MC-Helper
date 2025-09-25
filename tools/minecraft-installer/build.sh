#!/bin/bash

echo "Building Minecraft Installer..."
echo

# Build release version
cargo build --release

if [ $? -eq 0 ]; then
    echo
    echo "✓ Build successful!"
    echo
    echo "Executable location: target/release/minecraft-installer"
    echo
    echo "Usage examples:"
    echo "  ./target/release/minecraft-installer --list-versions"
    echo "  ./target/release/minecraft-installer --version 1.20.1"
    echo "  ./target/release/minecraft-installer --version 1.19.4 --install-dir ~/Games/Minecraft"
    echo
else
    echo
    echo "✗ Build failed!"
    echo
fi






