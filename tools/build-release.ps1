param([switch]$SkipFrontend)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path -Parent $PSScriptRoot
$taskPreviousHelperHash = $env:AMETISTA_HELPER_SHA256
Push-Location $taskRoot
try {
    if (-not $SkipFrontend) {
        & npm ci --prefix apps/desktop --no-audit --no-fund
        if ($LASTEXITCODE -ne 0) { throw 'npm ci failed' }
        & npm run build --prefix apps/desktop
        if ($LASTEXITCODE -ne 0) { throw 'Frontend build failed' }
    }
    & cargo build --release --locked -p ametista-helper -p ametista-monitor -p ametista-cli
    if ($LASTEXITCODE -ne 0) { throw 'Native auxiliary build failed' }
    $env:AMETISTA_HELPER_SHA256 = (Get-FileHash -Algorithm SHA256 -LiteralPath 'target/release/ametista-helper.exe').Hash.ToLowerInvariant()
    & cargo build --release --locked -p ametista-desktop
    if ($LASTEXITCODE -ne 0) { throw 'Desktop build failed' }
} finally {
    if ($null -eq $taskPreviousHelperHash) { Remove-Item Env:AMETISTA_HELPER_SHA256 -ErrorAction SilentlyContinue }
    else { $env:AMETISTA_HELPER_SHA256 = $taskPreviousHelperHash }
    Pop-Location
}
