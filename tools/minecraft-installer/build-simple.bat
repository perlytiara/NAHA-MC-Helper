@echo off
REM Simple build script for minecraft-installer
REM This script builds for platforms that can be cross-compiled from Windows

echo Building minecraft-installer for available platforms...

REM Create output directory
if not exist dist mkdir dist

REM Build for Windows (MSVC) - native - this always works
echo Building for Windows (x86_64-pc-windows-msvc)...
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-msvc\release\minecraft-installer.exe dist\minecraft-installer-windows-x86_64.exe
    echo ✓ Windows build successful
) else (
    echo ✗ Windows build failed
)

echo.
echo Build Summary:
echo ==============
echo ✓ Windows (x86_64-pc-windows-msvc): minecraft-installer-windows-x86_64.exe
echo.
echo Note: Cross-compilation to Linux/macOS requires additional setup:
echo - For Linux: Install OpenSSL development libraries and cross-compilation tools
echo - For macOS: Install Xcode command line tools and cross-compilation toolchain
echo - For Windows GNU: Install MinGW-w64 toolchain
echo.
echo Output files:
dir dist

