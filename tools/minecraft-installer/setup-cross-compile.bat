@echo off
REM Setup script for cross-compilation on Windows
echo Setting up cross-compilation environment...

REM Install MinGW-w64 for Windows GNU builds
echo Installing MinGW-w64...
winget install -e --id MSYS2.MSYS2
echo Please restart your terminal and run: pacman -S mingw-w64-x86_64-toolchain

REM Install WSL2 for Linux builds
echo Installing WSL2...
wsl --install
echo Please restart your computer and set up Ubuntu in WSL2

REM For macOS builds, you'll need to use GitHub Actions or a Mac
echo For macOS builds, use GitHub Actions or build on a Mac

echo.
echo Setup complete! Next steps:
echo 1. Restart your terminal
echo 2. Install MinGW-w64: pacman -S mingw-w64-x86_64-toolchain
echo 3. Set up WSL2 with Ubuntu
echo 4. Run build-all.bat again








