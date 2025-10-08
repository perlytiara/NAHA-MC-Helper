@echo off
REM GitHub Release Management Script for Minecraft Updater (Windows)
REM This script helps delete all tags and releases, then create a new one

setlocal enabledelayedexpansion

set REPO_OWNER=perlytiara
set REPO_NAME=minecraft-updater
set GITHUB_REPO=%REPO_OWNER%/%REPO_NAME%

echo.
echo 🚀 GitHub Release Management for %GITHUB_REPO%
echo ==============================================

if "%1"=="" goto :usage
if "%1"=="delete-releases" goto :delete_releases
if "%1"=="delete-tags" goto :delete_tags
if "%1"=="delete-all" goto :delete_all
if "%1"=="create" goto :create_release
if "%1"=="reset-and-create" goto :reset_and_create
goto :usage

:delete_releases
echo.
echo 🗑️  Deleting all releases...
gh release list --repo %GITHUB_REPO% --limit 1000 --json id --jq ".[].id" > temp_releases.txt
if %ERRORLEVEL% NEQ 0 (
    echo ✅ No releases found to delete.
    goto :eof
)

for /f "delims=" %%i in (temp_releases.txt) do (
    echo   Deleting release ID: %%i
    gh api -X DELETE "/repos/%GITHUB_REPO%/releases/%%i" 2>nul
)
del temp_releases.txt 2>nul
echo ✅ All releases deleted!
goto :eof

:delete_tags
echo.
echo 🗑️  Deleting all tags...
git tag > temp_tags.txt
if %ERRORLEVEL% NEQ 0 (
    echo ✅ No tags found to delete.
    goto :eof
)

for /f "delims=" %%t in (temp_tags.txt) do (
    if not "%%t"=="" (
        echo   Deleting tag: %%t
        git tag -d %%t 2>nul
        git push origin --delete %%t 2>nul
    )
)
del temp_tags.txt 2>nul
echo ✅ All tags deleted!
goto :eof

:delete_all
call :delete_releases
call :delete_tags
goto :eof

:create_release
if "%2"=="" (
    echo Error: Version required
    goto :usage
)
set VERSION=%2
set TITLE=%3
if "%TITLE%"=="" set TITLE=Minecraft Updater v%VERSION%
set NOTES_FILE=%4

echo.
echo 📦 Creating new release v%VERSION%...
echo   Creating tag v%VERSION%...
git tag -a "v%VERSION%" -m "%TITLE%"
git push origin "v%VERSION%"

echo   Creating GitHub release...
if "%NOTES_FILE%"=="" (
    gh release create "v%VERSION%" --title "%TITLE%" --notes "Release v%VERSION%" --repo %GITHUB_REPO%
) else (
    gh release create "v%VERSION%" --title "%TITLE%" --notes-file "%NOTES_FILE%" --repo %GITHUB_REPO%
)

echo ✅ Release v%VERSION% created successfully!
goto :eof

:reset_and_create
if "%2"=="" (
    echo Error: Version required
    goto :usage
)
set VERSION=%2
set TITLE=%3
if "%TITLE%"=="" set TITLE=Minecraft Updater v%VERSION%
set NOTES_FILE=%4

echo.
echo ⚠️  WARNING: This will delete ALL existing releases and tags!
echo Then create a new release v%VERSION%
set /p confirm="Continue? (yes/no): "
if not "!confirm!"=="yes" (
    echo Cancelled.
    exit /b 0
)

call :delete_all
call :create_release %VERSION% "%TITLE%" "%NOTES_FILE%"
goto :eof

:usage
echo Usage: %~nx0 {delete-releases^|delete-tags^|delete-all^|create^|reset-and-create} [args]
echo.
echo Commands:
echo   delete-releases              Delete all GitHub releases
echo   delete-tags                  Delete all Git tags (local and remote)
echo   delete-all                   Delete all releases and tags
echo   create ^<version^> [title] [notes-file]
echo                               Create a new release
echo   reset-and-create ^<version^> [title] [notes-file]
echo                               Delete everything and create new release
echo.
echo Examples:
echo   %~nx0 delete-all
echo   %~nx0 create 1.0.0 "Minecraft Updater v1.0.0" notes\updater\RELEASE_NOTES_UPDATER_v1.0.0.md
echo   %~nx0 reset-and-create 1.0.0
exit /b 1

:eof
echo.
echo ✅ Done!
endlocal

