@echo off
REM Build script for both minecraft-installer and minecraft-updater cross-compilation
REM This script builds for multiple platforms

echo Building Minecraft Tools for multiple platforms...
echo.

REM Create output directory
if not exist dist mkdir dist

REM Build for Windows (MSVC) - native
echo ================================================================
echo Building for Windows (x86_64-pc-windows-msvc)...
echo ================================================================
echo [1/2] minecraft-installer...
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-msvc\release\minecraft-installer.exe dist\minecraft-installer-windows-x86_64.exe
    echo ✓ minecraft-installer built
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] minecraft-updater...
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-msvc\release\minecraft-updater.exe dist\minecraft-updater-windows-x86_64.exe
    echo ✓ minecraft-updater built
) else (
    echo ✗ minecraft-updater build failed
)
echo.

REM Build for Windows (GNU) - alternative Windows build
echo ================================================================
echo Building for Windows GNU (x86_64-pc-windows-gnu)...
echo ================================================================
echo [1/2] minecraft-installer...
cargo build --release --target x86_64-pc-windows-gnu --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-gnu\release\minecraft-installer.exe dist\minecraft-installer-windows-gnu-x86_64.exe
    echo ✓ minecraft-installer built
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] minecraft-updater...
cargo build --release --target x86_64-pc-windows-gnu --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-pc-windows-gnu\release\minecraft-updater.exe dist\minecraft-updater-windows-gnu-x86_64.exe
    echo ✓ minecraft-updater built
) else (
    echo ✗ minecraft-updater build failed
)
echo.

REM Build for Linux
echo ================================================================
echo Building for Linux (x86_64-unknown-linux-gnu)...
echo ================================================================
echo [1/2] minecraft-installer...
cargo build --release --target x86_64-unknown-linux-gnu --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-unknown-linux-gnu\release\minecraft-installer dist\minecraft-installer-linux-x86_64
    echo ✓ minecraft-installer built
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] minecraft-updater...
cargo build --release --target x86_64-unknown-linux-gnu --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-unknown-linux-gnu\release\minecraft-updater dist\minecraft-updater-linux-x86_64
    echo ✓ minecraft-updater built
) else (
    echo ✗ minecraft-updater build failed
)
echo.

REM Build for macOS Intel
echo ================================================================
echo Building for macOS Intel (x86_64-apple-darwin)...
echo ================================================================
echo [1/2] minecraft-installer...
cargo build --release --target x86_64-apple-darwin --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-apple-darwin\release\minecraft-installer dist\minecraft-installer-macos-intel-x86_64
    echo ✓ minecraft-installer built
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] minecraft-updater...
cargo build --release --target x86_64-apple-darwin --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\x86_64-apple-darwin\release\minecraft-updater dist\minecraft-updater-macos-intel-x86_64
    echo ✓ minecraft-updater built
) else (
    echo ✗ minecraft-updater build failed
)
echo.

REM Build for macOS Apple Silicon
echo ================================================================
echo Building for macOS Apple Silicon (aarch64-apple-darwin)...
echo ================================================================
echo [1/2] minecraft-installer...
cargo build --release --target aarch64-apple-darwin --bin minecraft-installer
if %ERRORLEVEL% EQU 0 (
    copy target\aarch64-apple-darwin\release\minecraft-installer dist\minecraft-installer-macos-apple-silicon-aarch64
    echo ✓ minecraft-installer built
) else (
    echo ✗ minecraft-installer build failed
)

echo [2/2] minecraft-updater...
cargo build --release --target aarch64-apple-darwin --bin minecraft-updater
if %ERRORLEVEL% EQU 0 (
    copy target\aarch64-apple-darwin\release\minecraft-updater dist\minecraft-updater-macos-apple-silicon-aarch64
    echo ✓ minecraft-updater built
) else (
    echo ✗ minecraft-updater build failed
)
echo.

echo ================================================================
echo All builds completed!
echo ================================================================
echo Output files:
dir dist








