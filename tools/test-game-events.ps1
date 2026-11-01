$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskGame=Join-Path $taskRoot 'target/release/examples/synthetic_game.exe'
$taskProbe=Join-Path $taskRoot 'target/release/examples/wmi_probe.exe'
$taskFolder=Join-Path $taskRoot 'work/game-events'
$null=New-Item -ItemType Directory -Path $taskFolder -Force
$taskObserver=Start-Process -FilePath $taskProbe -ArgumentList ('"'+$taskGame+'"') -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $taskFolder 'events.txt') -RedirectStandardError (Join-Path $taskFolder 'errors.txt')
Start-Sleep -Seconds 3
$taskChild=Start-Process -FilePath $taskGame -WindowStyle Hidden -PassThru
$taskChild.WaitForExit()
$taskObserver.WaitForExit()
Get-Content -LiteralPath (Join-Path $taskFolder 'events.txt')
if($taskObserver.ExitCode -ne 0){Get-Content -LiteralPath (Join-Path $taskFolder 'errors.txt');throw 'Synthetic game event test failed'}
