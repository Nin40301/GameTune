param(
    [ValidateSet('interface','game_monitor')][string]$Mode='interface',
    [ValidateSet('full','reduced')][string]$Effects='full'
)
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskName=if($Mode -eq 'game_monitor'){'ametista-monitor'}else{'ametista'}
$taskExe=Join-Path $taskRoot ('target/release/'+$taskName+'.exe')
$taskProbe=Join-Path $taskRoot 'target/release/examples/budget.exe'
$taskBaseline=Join-Path $taskRoot 'docs/perf-baseline.json'
if(-not(Test-Path -LiteralPath $taskExe) -or -not(Test-Path -LiteralPath $taskProbe)){throw 'Build release executables and metrics budget example first'}
if($Mode -eq 'interface' -and -not(Test-Path -LiteralPath $taskBaseline)){throw 'Versioned performance baseline is required; never infer approval from collection alone'}
$taskLegacyFolder=Join-Path $taskRoot 'work/resource-budget-phase-6'
$taskFolder=Join-Path $taskLegacyFolder ($Mode+'-'+$Effects+'-'+[guid]::NewGuid().ToString('N'))
$null=New-Item -ItemType Directory -Path $taskFolder -Force
$taskFinalReport=Join-Path $taskLegacyFolder ($Mode+'.json')
@{passed=$false;status='collecting';mode=$Mode;effects=$Effects;evidenceDirectory=$taskFolder;startedUtc=(Get-Date).ToUniversalTime().ToString('o')} | ConvertTo-Json | Set-Content -LiteralPath $taskFinalReport -Encoding utf8
$taskRuns=if($Mode -eq 'interface'){5}else{1}
$taskReports=@()
$taskPreviousMode=$env:AMETISTA_RESOURCE_MODE
Write-Output ('Expected effects: '+$Effects+'. Choose them in Settings before running; close games and leave Home visible at the documented size/theme/DPI.')
try{
    for($taskRun=1;$taskRun -le $taskRuns;$taskRun++){
        $taskStart=@{FilePath=$taskExe;WorkingDirectory=(Split-Path -Parent $taskExe);PassThru=$true}
        if($Mode -eq 'game_monitor'){$taskStart.ArgumentList='--readonly';$taskStart.WindowStyle='Hidden'}
        $env:AMETISTA_RESOURCE_MODE='interface'
        $taskApp=Start-Process @taskStart
        $taskReport=Join-Path $taskFolder ('run-'+$taskRun+'.json')
        $taskCode=2
        try{
            Start-Sleep -Seconds 30
            if($taskApp.HasExited){throw 'App exited during warmup'}
            & $taskProbe $taskApp.Id $taskReport $Mode
            $taskCode=$LASTEXITCODE
        }finally{
            if(-not $taskApp.HasExited){
                if($Mode -eq 'game_monitor'){
                    $taskStateDir=Join-Path $env:LOCALAPPDATA 'io.github.nin40301.ametista/games'
                    $taskState=Get-Content -LiteralPath (Join-Path $taskStateDir 'monitor-status.json') -Raw | ConvertFrom-Json
                    if($taskState.pid -ne $taskApp.Id){throw 'Refuse to stop a monitor not created by this harness'}
                    ConvertTo-Json -InputObject $taskState.instance -Compress | Set-Content -LiteralPath (Join-Path $taskStateDir 'monitor-stop.json') -Encoding utf8
                }else{$null=$taskApp.CloseMainWindow()}
                if(-not $taskApp.WaitForExit(15000)){throw 'App did not stop gracefully; no process was terminated'}
            }
        }
        if(-not(Test-Path -LiteralPath $taskReport)){throw 'No fresh report; collection errors are not budget approval'}
        $taskReports+=$taskReport
        if($taskCode -ne 0){
            if($Mode -eq 'game_monitor'){Copy-Item -LiteralPath $taskReport -Destination $taskFinalReport}
            throw ('Collection or strict monitor gate failed; inspect '+$taskReport)
        }
    }
    if($Mode -eq 'interface'){
        $taskEvaluationReport=Join-Path $taskFolder 'evaluation.json'
        & $taskProbe evaluate $taskBaseline $Effects $taskEvaluationReport @taskReports
        $taskCode=$LASTEXITCODE
        if(-not(Test-Path -LiteralPath $taskEvaluationReport)){throw 'Cohort evaluation produced no fresh report'}
        Copy-Item -LiteralPath $taskEvaluationReport -Destination $taskFinalReport
        $taskEvaluation=Get-Content -LiteralPath $taskFinalReport -Raw|ConvertFrom-Json
        if($taskEvaluation.cpuP95Alert){Write-Warning 'Interface CPU p95 exceeded 2% in at least one run; warning only, not a peak-CPU veto'}
        if($taskEvaluation.informational -and -not $taskEvaluation.passed){Write-Warning 'Reduced mode is informational: numerical RAM criteria were not met. Nonblocking does not mean the numerical budget passed.'}
    }else{Copy-Item -LiteralPath $taskReports[0] -Destination $taskFinalReport}
    if($taskCode -ne 0){throw ('Resource regression gate failed; inspect '+$taskFinalReport)}
    Write-Output ('Resource gate passed; evidence: '+$taskFinalReport)
}finally{
    if($null -eq $taskPreviousMode){Remove-Item Env:AMETISTA_RESOURCE_MODE -ErrorAction SilentlyContinue}else{$env:AMETISTA_RESOURCE_MODE=$taskPreviousMode}
}
