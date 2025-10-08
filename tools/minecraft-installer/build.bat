@echo off
echo Building Minecraft Tools...
echo.

REM Build both executables
echo [1/2] Building minecraft-installer...
cargo build --release --bin minecraft-installer

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ✗ minecraft-installer build failed!
    echo.
    pause
    exit /b 1
)

echo [2/2] Building minecraft-updater...
cargo build --release --bin minecraft-updater

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ✗ minecraft-updater build failed!
    echo.
    pause
    exit /b 1
)

echo.
echo ✅ Both builds successful!
echo.
echo Executables location:
echo   target\release\minecraft-installer.exe
echo   target\release\minecraft-updater.exe
echo.
echo Usage examples:
echo.
echo Installer:
echo   minecraft-installer.exe --download-neoforge --create-instance
echo   minecraft-installer.exe --mrpack "modpack.mrpack" --create-instance
echo.
echo Updater:
echo   minecraft-updater.exe scan --format compact
echo   minecraft-updater.exe interactive --modpack-type neoforge
echo   minecraft-updater.exe update --instance-path "C:\path\to\instance" --modpack-type neoforge
echo.

pause










