param([ValidateSet('idle','monitor')][string]$Mode='monitor')
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$exe=Join-Path $root 'target/release/gametune.exe'
$probe=Join-Path $root 'target/release/examples/budget.exe'
if(-not (Test-Path -LiteralPath $exe) -or -not (Test-Path -LiteralPath $probe)){throw 'Build release UI and metrics budget example first'}
$folder=Join-Path $root 'work/resource-budget'
$null=New-Item -ItemType Directory -Path $folder -Force
$env:GAMETUNE_RESOURCE_MODE=$Mode
$app=Start-Process -FilePath $exe -WorkingDirectory (Split-Path -Parent $exe) -PassThru -WindowStyle Hidden
try {
    # Warm startup/renderer/JIT caches before steady-state measurement.
    Start-Sleep -Seconds 30
    if($app.HasExited){throw 'Desktop exited during warmup'}
    $report=Join-Path $folder ($Mode+'.json')
    & $probe $app.Id $report
    $code=$LASTEXITCODE
    if($code -ne 0){throw ('Resource budget failed; inspect '+$report)}
} finally {
    # Close only the window of the app created by this harness. Never kill games
    # or enumerate/open unrelated processes. The OS owns WebView child shutdown.
    if(-not $app.HasExited){$null=$app.CloseMainWindow();$null=$app.WaitForExit(10000)}
    Remove-Item Env:GAMETUNE_RESOURCE_MODE
}
