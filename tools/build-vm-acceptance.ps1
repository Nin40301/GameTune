param([Parameter(Mandatory)][string]$Destination)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path -Parent $PSScriptRoot
$taskOutput = [IO.Path]::GetFullPath($Destination, $taskRoot)
$taskAllowed = [IO.Path]::GetFullPath((Join-Path $taskRoot 'outputs')) + [IO.Path]::DirectorySeparatorChar
if (-not $taskOutput.StartsWith($taskAllowed, [StringComparison]::OrdinalIgnoreCase)) { throw 'Use a new folder within outputs' }
if (Test-Path -LiteralPath $taskOutput) { throw 'Never overwrite an acceptance package' }
$taskPreviousTarget = $env:CARGO_TARGET_DIR
$taskPreviousFlags = $env:RUSTFLAGS
Push-Location $taskRoot
try {
    # Keep fixture features and the static CRT out of production artifacts/caches.
    $env:CARGO_TARGET_DIR = Join-Path $taskRoot 'work/vm-acceptance-build'
    $env:RUSTFLAGS = '-C target-feature=+crt-static'
    $null = New-Item -ItemType Directory -Path (Join-Path $taskOutput 'bin')
    function Build-Test([string]$Feature, [string]$Target, [switch]$Library) {
        $taskArguments = @('test','-p','ametista-windows-runtime','--release','--locked','--offline','--features',$Feature,'--no-run','--message-format=json')
        if ($Library) { $taskArguments += '--lib' } else { $taskArguments += @('--test',$Target) }
        $taskLines = & cargo @taskArguments
        if ($LASTEXITCODE -ne 0) { throw "Failed to compile $Target" }
        $taskArtifacts = @($taskLines | ForEach-Object { $_ | ConvertFrom-Json } | Where-Object { $_.reason -eq 'compiler-artifact' -and $_.executable -and $_.profile.test -and ($Library -or $_.target.name -eq $Target) })
        if ($taskArtifacts.Count -ne 1) { throw "Ambiguous test artifact: $Target" }
        return $taskArtifacts[0].executable
    }
    $taskVm = Build-Test 'vm-tests' 'windows_vm'
    Copy-Item -LiteralPath $taskVm -Destination (Join-Path $taskOutput 'bin/vm-tests.exe')
    $taskUnit = Build-Test 'vm-tests' 'dedicated_hkcu' -Library
    Copy-Item -LiteralPath $taskUnit -Destination (Join-Path $taskOutput 'bin/vm-unit-tests.exe')
    $taskReal = Build-Test 'real-profile-tests' 'real_profiles_vm'
    Copy-Item -LiteralPath $taskReal -Destination (Join-Path $taskOutput 'bin/real-profile-tests.exe')
    & cargo build -p ametista-windows-runtime --example real_profile_e2e --features real-profile-tests --release --locked --offline
    if ($LASTEXITCODE -ne 0) { throw 'Real profile runner build failed' }
    Copy-Item -LiteralPath (Join-Path $env:CARGO_TARGET_DIR 'release/examples/real_profile_e2e.exe') -Destination (Join-Path $taskOutput 'bin/real-profile-runner.exe')
    # Separate invocation without features: never distribute a fixture-enabled CLI.
    & cargo build -p ametista-cli --release --locked --offline
    if ($LASTEXITCODE -ne 0) { throw 'Production CLI build failed' }
    Copy-Item -LiteralPath (Join-Path $env:CARGO_TARGET_DIR 'release/ametista-cli.exe') -Destination (Join-Path $taskOutput 'bin/ametista-cli.exe')
    Copy-Item -LiteralPath tools/vm-acceptance/Run-Acceptance.ps1,tools/vm-acceptance/README.md,LICENSE,NOTICE -Destination $taskOutput
    $taskFiles = @{}
    foreach ($taskFile in Get-ChildItem -LiteralPath $taskOutput -Recurse -File) {
        $taskFiles[[IO.Path]::GetRelativePath($taskOutput,$taskFile.FullName).Replace('\','/')] = (Get-FileHash -LiteralPath $taskFile.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    $taskManifest = @{ version=1; product='Ametista'; package='VM ACCEPTANCE ONLY - NOT A RELEASE'; commit=(& git rev-parse HEAD); staticCrt=$true; cliFeatures='default only'; files=$taskFiles }
    [IO.File]::WriteAllText((Join-Path $taskOutput 'package.json'),($taskManifest | ConvertTo-Json -Depth 8),[Text.UTF8Encoding]::new($false))
    Compress-Archive -Path (Join-Path $taskOutput '*') -DestinationPath ($taskOutput + '.zip')
    Get-FileHash -LiteralPath ($taskOutput + '.zip') -Algorithm SHA256
} finally {
    if ($null -eq $taskPreviousTarget) { Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue } else { $env:CARGO_TARGET_DIR = $taskPreviousTarget }
    if ($null -eq $taskPreviousFlags) { Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue } else { $env:RUSTFLAGS = $taskPreviousFlags }
    Pop-Location
}
