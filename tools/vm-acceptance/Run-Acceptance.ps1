[CmdletBinding()]
param([switch]$Resume, [switch]$Recover, [switch]$CheckOnly)
$ErrorActionPreference = 'Stop'
# Windows PowerShell 5.1, inbox only. No download, toolchain or policy bypass.
$taskVm = Get-CimInstance Win32_ComputerSystem
if (($taskVm.Manufacturer + ' ' + $taskVm.Model) -notmatch 'Virtual|VMware|VirtualBox|KVM|QEMU|Xen|Parallels|Bochs') {
    throw 'RECUSADO: use somente VM descartavel. Nenhuma alteracao foi iniciada.'
}
$taskIdentity = [Security.Principal.WindowsIdentity]::GetCurrent()
$taskPrincipal = [Security.Principal.WindowsPrincipal]::new($taskIdentity)
if (-not $taskPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'Abra Windows PowerShell como administrador DENTRO da VM, na mesma conta interativa'
}
if ([Diagnostics.Process]::GetCurrentProcess().SessionId -eq 0) { throw 'Desktop interativo obrigatorio' }
$taskSid = $taskIdentity.User.Value
$taskRoot = Join-Path ([Environment]::GetFolderPath('CommonApplicationData')) 'AmetistaVmAcceptance'
$taskPackage = Join-Path $taskRoot 'package'
$taskResults = Join-Path $taskRoot 'results'
$taskStatePath = Join-Path $taskResults 'runner-state.json'
$taskBoot = (Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToUniversalTime().ToString('o')
$taskMachine = (Get-CimInstance Win32_ComputerSystemProduct).UUID
function Refuse-Reparse([string]$Path) {
    $taskNode = [IO.Path]::GetFullPath($Path)
    while ($taskNode) {
        if ((Test-Path -LiteralPath $taskNode) -and ((Get-Item -LiteralPath $taskNode -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Reparse point recusado' }
        $taskNode = [IO.Path]::GetDirectoryName($taskNode)
    }
}
function Verify-Package([string]$Directory) {
    Refuse-Reparse $Directory
    $taskManifest = Get-Content -LiteralPath (Join-Path $Directory 'package.json') -Raw | ConvertFrom-Json
    $taskNames = @('bin/ametista-cli.exe','bin/vm-tests.exe','bin/vm-unit-tests.exe','bin/real-profile-tests.exe','bin/real-profile-runner.exe','Run-Acceptance.ps1','README.md','LICENSE','NOTICE')
    if ($taskManifest.version -ne 1 -or $taskManifest.staticCrt -ne $true -or @($taskManifest.files.PSObject.Properties).Count -ne $taskNames.Count) { throw 'Manifesto invalido' }
    foreach ($taskName in $taskNames) {
        $taskHash = $taskManifest.files.PSObject.Properties[$taskName].Value
        if ($taskHash -cnotmatch '^[a-f0-9]{64}$') { throw 'Hash ausente' }
        $taskFile = Join-Path $Directory $taskName
        Refuse-Reparse $taskFile
        if ((Get-FileHash -LiteralPath $taskFile -Algorithm SHA256).Hash.ToLowerInvariant() -cne $taskHash) { throw "Hash divergente: $taskName" }
    }
}
function Protect-Folder([string]$Path) {
    $taskAcl = [Security.AccessControl.DirectorySecurity]::new()
    $taskAcl.SetAccessRuleProtection($true,$false)
    $taskAcl.SetOwner([Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    foreach ($taskOwner in @('S-1-5-18','S-1-5-32-544')) {
        $taskAcl.AddAccessRule([Security.AccessControl.FileSystemAccessRule]::new([Security.Principal.SecurityIdentifier]::new($taskOwner),'FullControl','ContainerInherit,ObjectInherit','None','Allow'))
    }
    [IO.Directory]::SetAccessControl($Path,$taskAcl)
}
function Assert-Protected([string]$Path) {
    Refuse-Reparse $Path
    $taskAcl = Get-Acl -LiteralPath $Path
    if (-not $taskAcl.AreAccessRulesProtected) { throw 'ACL herdada recusada na retomada' }
    foreach ($taskRule in $taskAcl.GetAccessRules($true,$true,[Security.Principal.SecurityIdentifier])) {
        if ($taskRule.AccessControlType -eq 'Allow' -and $taskRule.IdentityReference.Value -notin @('S-1-5-18','S-1-5-32-544')) { throw 'ACL nao protegida' }
    }
    if ($taskAcl.GetOwner([Security.Principal.SecurityIdentifier]).Value -notin @('S-1-5-18','S-1-5-32-544')) { throw 'Dono nao protegido' }
}
Verify-Package $PSScriptRoot
if ($CheckOnly) { Write-Output 'VM, admin e hashes conferidos; nenhum teste ou tarefa iniciado'; exit 0 }
if (-not $Resume -and -not $Recover) {
    if ($env:AMETISTA_DISPOSABLE_VM -cne 'I_HAVE_A_SNAPSHOT' -or $env:AMETISTA_REAL_PROFILES -cne 'I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS') { throw 'RECUSADO: defina os DOIS opt-ins do README apenas dentro da VM com snapshot' }
    Write-Warning 'ESTE ENSAIO ALTERA PERFIS REAIS. Snapshot do hipervisor obrigatorio. Dois reinicios manuais. Restaure o snapshot ao terminar.'
    if ((Read-Host 'Digite SNAPSHOT para confirmar a VM descartavel e o snapshot existente') -cne 'SNAPSHOT') { throw 'Cancelado sem iniciar testes' }
    if (Test-Path -LiteralPath $taskRoot) { throw 'Ensaio anterior encontrado. Preserve resultados; use -Resume ou -Recover da copia protegida ou restaure o snapshot' }
    Refuse-Reparse $taskRoot
    $null = New-Item -ItemType Directory -Path $taskRoot
    Protect-Folder $taskRoot
    $null = New-Item -ItemType Directory -Path $taskPackage,$taskResults
    Protect-Folder $taskPackage
    Protect-Folder $taskResults
    foreach ($taskName in @('bin','Run-Acceptance.ps1','README.md','LICENSE','NOTICE','package.json')) {
        Copy-Item -LiteralPath (Join-Path $PSScriptRoot $taskName) -Destination $taskPackage -Recurse
    }
    Verify-Package $taskPackage
    $taskTaskName = 'AmetistaVmAcceptance-' + [guid]::NewGuid().ToString('N')
    $taskState = [pscustomobject]@{version=1;sid=$taskSid;machine=$taskMachine;phase='fixtures';boot=$taskBoot;task=$taskTaskName;snapshotConfirmed=$true;realProfilesConsent=$true;attempt=0}
} else {
    if ([IO.Path]::GetFullPath($PSScriptRoot) -cne [IO.Path]::GetFullPath($taskPackage)) { throw 'Retome pela copia protegida indicada no README' }
    foreach ($taskPath in @($taskRoot,$taskPackage,$taskResults)) { Assert-Protected $taskPath }
    Refuse-Reparse $taskStatePath
    $taskState = Get-Content -LiteralPath $taskStatePath -Raw | ConvertFrom-Json
    if ($taskState.version -ne 1 -or $taskState.sid -cne $taskSid -or $taskState.machine -cne $taskMachine -or -not $taskState.snapshotConfirmed -or -not $taskState.realProfilesConsent -or $taskState.task -cnotmatch '^AmetistaVmAcceptance-[a-f0-9]{32}$') { throw 'Retomada de outra conta/VM ou sem consentimento recusada' }
    if (-not $Recover -and $taskState.phase -in @('running','failed','aborted')) { throw 'Ensaio interrompido: preserve evidencias. Use -Recover explicitamente ou restaure o snapshot; nunca reaplicar cegamente' }
}
# Both values are process-only, restored from explicit consent in protected state.
$env:AMETISTA_DISPOSABLE_VM = 'I_HAVE_A_SNAPSHOT'
$env:AMETISTA_REAL_PROFILES = 'I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS'
function Save-State {
    Refuse-Reparse $taskStatePath
    $taskTemporary = Join-Path $taskResults 'runner-state.tmp'
    Refuse-Reparse $taskTemporary
    [IO.File]::WriteAllText($taskTemporary,($taskState | ConvertTo-Json -Depth 8),[Text.UTF8Encoding]::new($false))
    $taskFlush = [IO.File]::Open($taskTemporary,'Open','ReadWrite','None')
    try { $taskFlush.Flush($true) } finally { $taskFlush.Dispose() }
    if (Test-Path -LiteralPath $taskStatePath) { [IO.File]::Replace($taskTemporary,$taskStatePath,$null) } else { [IO.File]::Move($taskTemporary,$taskStatePath) }
}
function Copy-Evidence {
    $taskEvidence = Join-Path ([Environment]::GetFolderPath('CommonApplicationData')) ('AmetistaRealProfileTests/' + $taskSid)
    if (Test-Path -LiteralPath $taskEvidence) {
        Refuse-Reparse $taskEvidence
        foreach ($taskFile in Get-ChildItem -LiteralPath $taskEvidence -File -Filter '*.json') {
            Refuse-Reparse $taskFile.FullName
            Copy-Item -LiteralPath $taskFile.FullName -Destination (Join-Path $taskResults ('real-' + $taskFile.Name))
        }
    }
}
function Run-Test([string]$Name,[string[]]$Arguments) {
    Verify-Package $taskPackage
    $taskLog = Join-Path $taskResults ('run-' + $taskState.attempt + '-' + $Name + '.log')
    Refuse-Reparse $taskLog
    & (Join-Path $taskPackage ('bin/' + $Name + '.exe')) @Arguments 2>&1 | Tee-Object -FilePath $taskLog | Out-Host
    $taskCode = $LASTEXITCODE
    Copy-Evidence
    return $taskCode
}
Refuse-Reparse (Join-Path $taskResults 'runner.lock')
$taskLease = [IO.File]::Open((Join-Path $taskResults 'runner.lock'),'OpenOrCreate','ReadWrite','None')
Push-Location $taskResults
try {
    if ($Recover) {
        $taskState.phase='running'; Save-State
        $taskCode = Run-Test 'real-profile-runner' @('recover')
        if ($taskCode -ne 0) { throw 'Recuperacao incompleta; preserve a VM e restaure snapshot' }
        $taskState.phase='aborted';Save-State
        Unregister-ScheduledTask -TaskName $taskState.task -Confirm:$false -ErrorAction SilentlyContinue
        Write-Warning 'Desfazer tentado; ensaio abortado, NAO aprovado. Restaure o snapshot.'
        exit 0
    }
    if ($taskState.phase -eq 'complete') { Write-Output "Ensaio ja concluido. Resultados: $taskResults"; exit 0 }
    if ($taskState.phase -eq 'await_reboot' -and $taskState.boot -ceq $taskBoot) { Write-Warning 'Reinicie manualmente pelo Windows; nada foi reaplicado'; exit 10 }
    $taskFixtures = $taskState.phase -eq 'fixtures'
    if (-not $taskFixtures -and $taskState.phase -ne 'await_reboot') { throw 'Etapa desconhecida' }
    $taskState.phase='running'; $taskState.attempt++; Save-State
    if ($taskFixtures) {
        if ((Run-Test 'vm-tests' @('--ignored','--test-threads=1','--nocapture')) -ne 0) { throw 'Fixtures falharam' }
        if ((Run-Test 'vm-unit-tests' @('dedicated_hkcu','--ignored','--test-threads=1','--nocapture')) -ne 0) { throw 'Roundtrip HKCU falhou' }
    }
    $taskCode = Run-Test 'real-profile-runner' @('step')
    if ($taskCode -eq 10) {
        $taskState.phase='await_reboot';$taskState.boot=$taskBoot;Save-State
        # Fixed protected script, interactive current account, no password/autologon.
        $taskPowerShell = Join-Path ([Environment]::SystemDirectory) 'WindowsPowerShell/v1.0/powershell.exe'
        $taskAction = New-ScheduledTaskAction -Execute $taskPowerShell -Argument ('-NoProfile -File "' + (Join-Path $taskPackage 'Run-Acceptance.ps1') + '" -Resume') -WorkingDirectory $taskResults
        $taskTrigger = New-ScheduledTaskTrigger -AtLogOn -User $taskSid
        $taskUser = New-ScheduledTaskPrincipal -UserId $taskSid -LogonType Interactive -RunLevel Highest
        try {
            $null = Register-ScheduledTask -TaskName $taskState.task -Action $taskAction -Trigger $taskTrigger -Principal $taskUser -Force
        } catch {
            Write-Warning 'Nao foi possivel registrar retomada automatica. Apos reiniciar, use -Resume da copia protegida, como documentado. Nao altere a seguranca.'
        }
        Write-Warning "REINICIO NECESSARIO: reinicie pelo Windows e entre na mesma conta. A tarefa de teste retoma. Resultados: $taskResults"
        exit 10
    }
    if ($taskCode -ne 0) { throw 'Perfis reais falharam; preservar WAL/snapshot e resultados' }
    if ((Run-Test 'real-profile-tests' @('--ignored','--test-threads=1','--nocapture')) -ne 0) { throw 'Verificacao final falhou' }
    $taskState.phase='complete';Save-State
    Unregister-ScheduledTask -TaskName $taskState.task -Confirm:$false -ErrorAction SilentlyContinue
    Write-Output "Comparacoes automatizadas concluidas. Confira apps/jogos visualmente. Resultados: $taskResults. Restaure o snapshot."
} catch {
    $taskState.phase='failed'; Save-State
    Copy-Evidence
    Unregister-ScheduledTask -TaskName $taskState.task -Confirm:$false -ErrorAction SilentlyContinue
    throw
} finally { Pop-Location; $taskLease.Dispose() }
