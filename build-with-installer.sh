#!/bin/bash

echo "Building NAHA MC Helper with minecraft-installer.exe integration..."

echo ""
echo "Step 1: Building minecraft-installer.exe..."
cd tools/minecraft-installer
./build.sh
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to build minecraft-installer.exe"
    exit 1
fi
cd ../..

echo ""
echo "Step 2: Building frontend..."
npm run build
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to build frontend"
    exit 1
fi

echo ""
echo "Step 3: Building Electron app..."
npm run dist
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to build Electron app"
    exit 1
fi

echo ""
echo "Build completed successfully!"
echo "Output: dist-electron/"
