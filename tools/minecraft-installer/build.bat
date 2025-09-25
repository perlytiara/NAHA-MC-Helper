@echo off
echo Building Minecraft Installer...
echo.

REM Build release version
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✓ Build successful!
    echo.
    echo Executable location: target\release\minecraft-installer.exe
    echo.
    echo Usage examples:
    echo   minecraft-installer.exe --list-versions
    echo   minecraft-installer.exe --version 1.20.1
    echo   minecraft-installer.exe --version 1.19.4 --install-dir C:\Games\Minecraft
    echo.
) else (
    echo.
    echo ✗ Build failed!
    echo.
)

pause






