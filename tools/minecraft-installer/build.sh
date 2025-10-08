#!/bin/bash

echo "Building Minecraft Tools..."
echo

# Build both executables
echo "[1/2] Building minecraft-installer..."
cargo build --release --bin minecraft-installer

if [ $? -ne 0 ]; then
    echo
    echo "✗ minecraft-installer build failed!"
    echo
    exit 1
fi

echo "[2/2] Building minecraft-updater..."
cargo build --release --bin minecraft-updater

if [ $? -ne 0 ]; then
    echo
    echo "✗ minecraft-updater build failed!"
    echo
    exit 1
fi

echo
echo "✅ Both builds successful!"
echo
echo "Executables location:"
echo "  target/release/minecraft-installer"
echo "  target/release/minecraft-updater"
echo
echo "Usage examples:"
echo
echo "Installer:"
echo "  ./target/release/minecraft-installer --download-neoforge --create-instance"
echo "  ./target/release/minecraft-installer --mrpack modpack.mrpack --create-instance"
echo
echo "Updater:"
echo "  ./target/release/minecraft-updater scan --format compact"
echo "  ./target/release/minecraft-updater interactive --modpack-type neoforge"
echo "  ./target/release/minecraft-updater update --instance-path /path/to/instance --modpack-type neoforge"
echo










