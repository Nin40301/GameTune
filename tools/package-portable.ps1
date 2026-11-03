param([Parameter(Mandatory)][string]$Destination)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path -Parent $PSScriptRoot
$taskOutput = [IO.Path]::GetFullPath($Destination, $taskRoot)
$taskAllowed = [IO.Path]::GetFullPath((Join-Path $taskRoot 'outputs')) + [IO.Path]::DirectorySeparatorChar
if (-not $taskOutput.StartsWith($taskAllowed, [StringComparison]::OrdinalIgnoreCase)) { throw 'Destination must be inside repository outputs' }
if (Test-Path -LiteralPath $taskOutput) { throw 'Use a new destination; packaging never overwrites evidence' }
& node (Join-Path $taskRoot 'tools/verify-version.mjs')
if ($LASTEXITCODE -ne 0) { throw 'Version verification failed' }
$taskVersion = (Get-Content -LiteralPath (Join-Path $taskRoot 'apps/desktop/package.json') -Raw | ConvertFrom-Json).version
$taskCommit = & git -C $taskRoot rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw 'Cannot determine build commit' }
$null = New-Item -ItemType Directory -Path $taskOutput
$taskStage = Join-Path $taskRoot ('work/portable-' + [guid]::NewGuid().ToString('N'))
$null = New-Item -ItemType Directory -Path $taskStage
foreach ($taskKind in @('desktop', 'cli')) {
    $taskDir = Join-Path $taskStage $taskKind
    $null = New-Item -ItemType Directory -Path $taskDir
    $taskBinaries = if ($taskKind -eq 'desktop') { @('ametista.exe', 'ametista-helper.exe', 'ametista-monitor.exe') } else { @('ametista-cli.exe') }
    $taskHashes = @{}
    foreach ($taskName in $taskBinaries) {
        $taskSource = Join-Path $taskRoot ('target/release/' + $taskName)
        if (-not (Test-Path -LiteralPath $taskSource)) { throw "Missing release binary: $taskName" }
        Copy-Item -LiteralPath $taskSource -Destination $taskDir
        $taskHashes[$taskName] = (Get-FileHash -Algorithm SHA256 -LiteralPath $taskSource).Hash.ToLowerInvariant()
    }
    foreach ($taskDoc in @('LICENSE', 'NOTICE', 'docs/user/recovery.md', 'docs/user/recovery.en.md')) {
        Copy-Item -LiteralPath (Join-Path $taskRoot $taskDoc) -Destination $taskDir
    }
    $taskLicenseDir = Join-Path $taskDir 'LICENSES'
    $null = New-Item -ItemType Directory -Path $taskLicenseDir
    Copy-Item -LiteralPath (Join-Path $taskRoot 'LICENSES/PlusJakartaSans-OFL-1.1.txt') -Destination $taskLicenseDir
    $taskManifest = @{ product = 'Ametista'; version = $taskVersion; commit = $taskCommit; identifier = 'io.github.nin40301.ametista'; architecture = 'windows-x64'; signed = $false; vmFixtureFeatures = $false; files = $taskHashes; limitation = 'Alpha. Test only in a disposable VM. Portable writable folders cannot elevate; use the future protected installer.' }
    [IO.File]::WriteAllText((Join-Path $taskDir 'release-info.json'), ($taskManifest | ConvertTo-Json -Depth 5) + "`n", [Text.UTF8Encoding]::new($false))
    Compress-Archive -Path (Join-Path $taskDir '*') -DestinationPath (Join-Path $taskOutput "ametista-$taskVersion-windows-x64-$taskKind-portable.zip") -CompressionLevel Optimal
}
Write-Output $taskOutput
