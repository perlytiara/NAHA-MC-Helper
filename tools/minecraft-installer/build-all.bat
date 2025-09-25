@echo off
REM Build script for minecraft-installer cross-compilation
REM This script builds for multiple platforms

echo Building minecraft-installer for multiple platforms...

REM Create output directories
if not exist dist mkdir dist
if not exist ..\minecraft-installer-releases mkdir ..\minecraft-installer-releases

REM Build for Windows (MSVC) - native
echo Building for Windows (x86_64-pc-windows-msvc)...
cargo build --release --target x86_64-pc-windows-msvc
copy target\x86_64-pc-windows-msvc\release\minecraft-installer.exe dist\minecraft-installer-windows-x86_64.exe
copy target\x86_64-pc-windows-msvc\release\minecraft-installer.exe ..\minecraft-installer-releases\minecraft-installer-windows-x86_64.exe

REM Build for Windows (GNU) - alternative Windows build
echo Building for Windows GNU (x86_64-pc-windows-gnu)...
cargo build --release --target x86_64-pc-windows-gnu
copy target\x86_64-pc-windows-gnu\release\minecraft-installer.exe dist\minecraft-installer-windows-gnu-x86_64.exe
copy target\x86_64-pc-windows-gnu\release\minecraft-installer.exe ..\minecraft-installer-releases\minecraft-installer-windows-gnu-x86_64.exe

REM Build for Linux
echo Building for Linux (x86_64-unknown-linux-gnu)...
cargo build --release --target x86_64-unknown-linux-gnu
copy target\x86_64-unknown-linux-gnu\release\minecraft-installer dist\minecraft-installer-linux-x86_64
copy target\x86_64-unknown-linux-gnu\release\minecraft-installer ..\minecraft-installer-releases\minecraft-installer-linux-x86_64

REM Build for macOS Intel
echo Building for macOS Intel (x86_64-apple-darwin)...
cargo build --release --target x86_64-apple-darwin
copy target\x86_64-apple-darwin\release\minecraft-installer dist\minecraft-installer-macos-intel-x86_64
copy target\x86_64-apple-darwin\release\minecraft-installer ..\minecraft-installer-releases\minecraft-installer-macos-intel-x86_64

REM Build for macOS Apple Silicon
echo Building for macOS Apple Silicon (aarch64-apple-darwin)...
cargo build --release --target aarch64-apple-darwin
copy target\aarch64-apple-darwin\release\minecraft-installer dist\minecraft-installer-macos-apple-silicon-aarch64
copy target\aarch64-apple-darwin\release\minecraft-installer ..\minecraft-installer-releases\minecraft-installer-macos-apple-silicon-aarch64

echo All builds completed successfully!
echo Output files in dist\:
dir dist
echo Output files in ..\minecraft-installer-releases\:
dir ..\minecraft-installer-releases

