@echo off
REM Project Structure Generator for Windows
REM Generates a comprehensive folder structure summary

echo.
echo 🚀 Project Structure Analyzer
echo ========================================
echo.

REM Check if Node.js is available
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Node.js is not installed or not in PATH
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)

REM Set default output file
set OUTPUT_FILE=project-structure.txt

REM Check if user wants console output
if "%1"=="--console" (
    node generate-structure.cjs . --console
) else if "%1"=="-c" (
    node generate-structure.cjs . --console
) else (
    REM Check if custom output file is specified
    if not "%1"=="" (
        set OUTPUT_FILE=%1
    )
    
    echo 🔍 Analyzing project structure...
    node generate-structure.cjs . "%OUTPUT_FILE%"
    
    if exist "%OUTPUT_FILE%" (
        echo.
        echo 📄 Would you like to view the report? (Y/N)
        set /p choice=
        if /i "%choice%"=="Y" (
            type "%OUTPUT_FILE%"
        )
    )
)

echo.
pause

