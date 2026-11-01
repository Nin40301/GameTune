$powercfg=Join-Path ([Environment]::SystemDirectory) 'powercfg.exe'
function Power([string[]]$arguments){$output=& $powercfg @arguments 2>&1;if($LASTEXITCODE -ne 0){Fail 'failed' 'Power scheme operation is unavailable or rejected by Windows'};return ($output -join "`n")}
function Inventory {return @([regex]::Matches((Power -arguments @('/list')),'[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}') | ForEach-Object {$_.Value.ToLowerInvariant()} | Sort-Object -Unique)}
$guids=@(Inventory)
$active=([regex]::Match((Power -arguments @('/getactivescheme')),'[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}')).Value.ToLowerInvariant()
if(-not (Is-Guid $active)){Fail 'failed' 'Cannot determine active power scheme'}
if($p.operation -eq 'capture'){Emit @{type='power';active_guid=$active;existing_guids=$guids};return}
Require-Admin
$target=[string]$p.guid
if(-not (Is-Guid $target)){Fail 'protected'}
if($p.expected -and (($active -ne $p.expected.active_guid) -or (($guids -join ',') -ne (@($p.expected.existing_guids) -join ',')))){Fail 'conflict'}
if($p.operation -eq 'apply'){
    if($target -notin $guids){
        $source='SCHEME_MIN'
        if($p.testSource){if(-not (Is-Guid $p.testSource)){Fail 'protected'};$source=[string]$p.testSource}
        $null=Power -arguments @('/duplicatescheme',$source,$target)
    }
    $null=Power -arguments @('/setactive',$target)
}elseif($p.operation -eq 'restore'){
    $original=[string]$p.original.active_guid
    if(-not (Is-Guid $original) -or $original -notin @($p.original.existing_guids)){Fail 'protected'}
    $null=Power -arguments @('/setactive',$original)
    if($target -notin @($p.original.existing_guids) -and $target -in @(Inventory)){$null=Power -arguments @('/delete',$target)}
}else{Fail 'protected'}
Emit $true
