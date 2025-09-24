@echo off
echo Building NAHA MC Helper with minecraft-installer.exe integration...

echo.
echo Step 1: Building minecraft-installer.exe...
cd tools\minecraft-installer
call build.bat
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build minecraft-installer.exe
    pause
    exit /b 1
)
cd ..\..

echo.
echo Step 2: Building frontend...
call npm run build
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build frontend
    pause
    exit /b 1
)

echo.
echo Step 3: Building Electron app...
call npm run dist
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build Electron app
    pause
    exit /b 1
)

echo.
echo Build completed successfully!
echo Output: dist-electron\
pause
