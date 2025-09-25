@echo off
echo Testing Minecraft Installer with Launcher Integration...
echo.

REM Build the project first
echo Building project...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ✗ Build failed!
    pause
    exit /b 1
)

echo ✓ Build successful!
echo.

REM Test launcher detection
echo Testing launcher detection...
target\release\minecraft-installer.exe --list-launchers
echo.

REM Test version listing
echo Testing version listing...
target\release\minecraft-installer.exe --list-versions --version-type release | head -10
echo.

REM Test basic installation (without actually installing)
echo Testing installation process (dry run)...
echo This would install Minecraft 1.20.1 with launcher instance creation:
echo target\release\minecraft-installer.exe --version 1.20.1 --create-instance --verbose
echo.

echo Test completed!
echo.
echo Available commands:
echo   --list-launchers          : List detected launchers
echo   --list-versions          : List available Minecraft versions
echo   --version 1.20.1         : Install Minecraft 1.20.1
echo   --create-instance        : Create instances in detected launchers
echo   --mrpack modpack.mrpack  : Install Modrinth modpack
echo   --target-launcher prism  : Target specific launcher
echo   --verbose                : Enable detailed logging
echo.

pause






