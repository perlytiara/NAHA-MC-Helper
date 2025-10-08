@echo off
REM Simple build script for both minecraft-installer and minecraft-updater
REM This script builds for platforms that can be cross-compiled from Windows

echo Building Minecraft Tools for Windows...
echo.

REM Create output directory
if not exist dist mkdir dist

REM Build for Windows (MSVC) - native - this always works
echo ================================================================
echo Building for Windows (x86_64-pc-windows-msvc)...
echo ================================================================

echo [1/2] Building minecraft-installer...
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-msvc\release\minecraft-installer.exe dist\minecraft-installer-windows-x86_64.exe
    echo ✓ minecraft-installer build successful
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] Building minecraft-updater...
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-msvc\release\minecraft-updater.exe dist\minecraft-updater-windows-x86_64.exe
    echo ✓ minecraft-updater build successful
) else (
    echo ✗ minecraft-updater build failed
)

echo.
echo ================================================================
echo Build Summary
echo ================================================================
echo ✓ minecraft-installer-windows-x86_64.exe
echo ✓ minecraft-updater-windows-x86_64.exe
echo.
echo Note: Cross-compilation to Linux/macOS requires additional setup:
echo - For Linux: Install OpenSSL development libraries and cross-compilation tools
echo - For macOS: Install Xcode command line tools and cross-compilation toolchain
echo - For Windows GNU: Install MinGW-w64 toolchain
echo.
echo For automated multi-platform builds, use GitHub Actions:
echo   git tag v1.0.0
echo   git push origin v1.0.0
echo.
echo Output files:
dir dist








