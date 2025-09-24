@echo off
echo Creating NAHA MC Helper package...

REM Create package directory
mkdir naha-mc-helper-package 2>nul

REM Copy essential files
echo Copying files...
copy main.js naha-mc-helper-package\
copy preload.js naha-mc-helper-package\
copy minecraft-installer.exe naha-mc-helper-package\
copy package.json naha-mc-helper-package\

REM Copy dist folder
xcopy dist naha-mc-helper-package\dist\ /E /I /Q

REM Copy node_modules (only essential ones)
mkdir naha-mc-helper-package\node_modules 2>nul
xcopy node_modules\adm-zip naha-mc-helper-package\node_modules\adm-zip\ /E /I /Q
xcopy node_modules\yauzl naha-mc-helper-package\node_modules\yauzl\ /E /I /Q

REM Create a simple launcher script
echo @echo off > naha-mc-helper-package\run.bat
echo cd /d "%%~dp0" >> naha-mc-helper-package\run.bat
echo start "" "%%~dp0node_modules\.bin\electron.cmd" . >> naha-mc-helper-package\run.bat

echo.
echo Package created in 'naha-mc-helper-package' folder!
echo To run the app, double-click 'run.bat' in that folder.
echo.
pause



