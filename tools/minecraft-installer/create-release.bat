@echo off
REM Minecraft Installer Release Script for Windows
REM Usage: create-release.bat [version]
REM Example: create-release.bat 0.1.0

setlocal enabledelayedexpansion

REM Get version from argument or prompt
if "%1"=="" (
    set /p VERSION="Enter version number (e.g., 0.1.0): "
) else (
    set VERSION=%1
)

echo 🚀 Creating release v%VERSION%...

REM Check if we're in a git repository
git rev-parse --git-dir >nul 2>&1
if errorlevel 1 (
    echo ❌ Not in a git repository
    exit /b 1
)

REM Check if there are uncommitted changes
git diff-index --quiet HEAD --
if errorlevel 1 (
    echo ❌ You have uncommitted changes. Please commit or stash them first.
    exit /b 1
)

REM Check if tag already exists
git rev-parse "v%VERSION%" >nul 2>&1
if not errorlevel 1 (
    echo ❌ Tag v%VERSION% already exists
    exit /b 1
)

REM Create and push tag
echo 📝 Creating tag v%VERSION%...
git tag -a "v%VERSION%" -m "Release v%VERSION%"

echo 📤 Pushing tag to remote...
git push origin "v%VERSION%"

echo ✅ Release v%VERSION% created successfully!
echo.
echo 🎉 GitHub Actions will now automatically:
echo    • Build for all platforms (Windows, Linux, macOS)
echo    • Run tests
echo    • Create a GitHub release with all binaries
echo    • Generate checksums for verification
echo.
echo 📋 Check the progress at:
echo    https://github.com/perlytiara/minecraft-installer/actions
echo.
echo 📦 Once complete, download from:
echo    https://github.com/perlytiara/minecraft-installer/releases/tag/v%VERSION%

pause








