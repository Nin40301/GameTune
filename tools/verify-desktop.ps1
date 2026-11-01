$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$evidence=Join-Path $root 'work/desktop-verification'
$null=New-Item -ItemType Directory -Path $evidence -Force
$kits=Join-Path ${env:ProgramFiles(x86)} 'Windows Kits/10/bin'
$mt=Get-ChildItem -LiteralPath $kits -Filter mt.exe -Recurse -File | Where-Object {$_.Directory.Name -eq 'x64'} | Sort-Object FullName | Select-Object -Last 1
if(-not $mt){throw 'Windows SDK mt.exe is required'}
foreach($spec in @(@('gametune','asInvoker'),@('gametune-helper','requireAdministrator'))){
    $exe=Join-Path $root ('target/release/'+$spec[0]+'.exe')
    $manifest=Join-Path $evidence ($spec[0]+'.manifest.xml')
    & $mt.FullName -nologo ("-inputresource:"+$exe+';#1') ("-out:"+$manifest)
    if($LASTEXITCODE -ne 0){throw 'Manifest extraction failed'}
    [xml]$xml=Get-Content -LiteralPath $manifest -Raw
    $node=$xml.SelectSingleNode('//*[local-name()="requestedExecutionLevel"]')
    if($node.level -cne $spec[1] -or $node.uiAccess -cne 'false'){throw ('Invalid execution level for '+$spec[0])}
}
$assets=Join-Path $root 'apps/desktop/dist/assets'
foreach($asset in Get-ChildItem -LiteralPath $assets -Filter '*.js' -File){
    if((Get-Content -LiteralPath $asset.FullName -Raw) -match '__gametuneTest|createMockBackend|test-backend'){throw 'Mock backend leaked into production assets'}
}
$cap=Get-Content -LiteralPath (Join-Path $root 'apps/desktop/src-tauri/capabilities/main.json') -Raw | ConvertFrom-Json
$expected=@('allow-bootstrap','allow-analysis-start','allow-analysis-state','allow-analysis-cancel','allow-prepare-review','allow-execute-review')
if(@(Compare-Object ($cap.permissions|Sort-Object) ($expected|Sort-Object)).Count -ne 0){throw 'Unexpected Tauri permission'}
if($cap.windows.Count -ne 1 -or $cap.windows[0] -cne 'main'){throw 'Unexpected window capability'}
$config=Get-Content -LiteralPath (Join-Path $root 'apps/desktop/src-tauri/tauri.conf.json') -Raw | ConvertFrom-Json
if($config.app.security.csp -match 'unsafe-eval|unsafe-inline|https:|\*'){throw 'Unexpected CSP source'}
Write-Output 'Verified: asInvoker, requireAdministrator, six permissions, restrictive CSP, no mock assets'
