# Unblock Minecraft Binaries
# This script removes the Windows security block from downloaded executables

Write-Host "Unblocking Minecraft binaries..." -ForegroundColor Cyan

$binariesPath = Join-Path $PSScriptRoot "minecraft-installer-releases"

if (-not (Test-Path $binariesPath)) {
    Write-Host "ERROR: Binaries directory not found: $binariesPath" -ForegroundColor Red
    exit 1
}

$files = Get-ChildItem -Path $binariesPath -File

$unblocked = 0
$alreadyUnblocked = 0

foreach ($file in $files) {
    try {
        $stream = Get-Item -Path $file.FullName -Stream Zone.Identifier -ErrorAction SilentlyContinue
        if ($stream) {
            Unblock-File -Path $file.FullName
            Write-Host "SUCCESS: Unblocked $($file.Name)" -ForegroundColor Green
            $unblocked++
        } else {
            Write-Host "SKIP: Already unblocked $($file.Name)" -ForegroundColor Gray
            $alreadyUnblocked++
        }
    } catch {
        Write-Host "WARNING: Could not check $($file.Name) - $($_.Exception.Message)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Unblocked: $unblocked" -ForegroundColor Green
Write-Host "  Already unblocked: $alreadyUnblocked" -ForegroundColor Gray
Write-Host ""
Write-Host "Done!" -ForegroundColor Green
