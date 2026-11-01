param([ValidateSet('interface','game_monitor')][string]$Mode='interface')
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskName=if($Mode -eq 'game_monitor'){'ametista-monitor'}else{'ametista'}
$taskExe=Join-Path $taskRoot ('target/release/'+$taskName+'.exe')
$taskProbe=Join-Path $taskRoot 'target/release/examples/budget.exe'
if(-not (Test-Path -LiteralPath $taskExe) -or -not (Test-Path -LiteralPath $taskProbe)){throw 'Build release executables and metrics budget example first'}
$taskFolder=Join-Path $taskRoot 'work/resource-budget-phase-6'
$null=New-Item -ItemType Directory -Path $taskFolder -Force
$taskStart=@{FilePath=$taskExe;WorkingDirectory=(Split-Path -Parent $taskExe);PassThru=$true}
if($Mode -eq 'game_monitor'){$taskStart.ArgumentList='--readonly';$taskStart.WindowStyle='Hidden'}
# Measure an OPEN visible interface, not a hidden/minimized WebView. This mode
# denies mutations and leaves the real dashboard sampler active.
$env:AMETISTA_RESOURCE_MODE='interface'
$taskApp=Start-Process @taskStart
$taskCode=2
try {
    Start-Sleep -Seconds 30
    if($taskApp.HasExited){throw 'App exited during warmup'}
    $taskReport=Join-Path $taskFolder ($Mode+'.json')
    & $taskProbe $taskApp.Id $taskReport $Mode
    $taskCode=$LASTEXITCODE
} finally {
    if(-not $taskApp.HasExited){
        if($Mode -eq 'game_monitor'){
            $taskStateDir=Join-Path $env:LOCALAPPDATA 'local.ametista.desktop/games'
            $taskState=Get-Content -LiteralPath (Join-Path $taskStateDir 'monitor-status.json') -Raw | ConvertFrom-Json
            if($taskState.pid -ne $taskApp.Id){throw 'Refuse to stop a monitor not created by this harness'}
            ConvertTo-Json -InputObject $taskState.instance -Compress | Set-Content -LiteralPath (Join-Path $taskStateDir 'monitor-stop.json') -Encoding utf8
        } else {$null=$taskApp.CloseMainWindow()}
        if(-not $taskApp.WaitForExit(15000)){throw 'App did not stop gracefully; no process was terminated'}
    }
    Remove-Item Env:AMETISTA_RESOURCE_MODE
}
if($taskCode -ne 0){throw ('Resource budget failed; inspect '+$taskReport)}
