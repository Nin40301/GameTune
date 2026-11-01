Require-Admin
# Check the real history; never change the Windows restore-point frequency policy.
$points=@(Get-ComputerRestorePoint)
if($points.Count -gt 0){
    $latest=$points | Sort-Object SequenceNumber -Descending | Select-Object -First 1
    $created=[Management.ManagementDateTimeConverter]::ToDateTime([string]$latest.CreationTime)
    if(((Get-Date)-$created).TotalHours -lt 24){Emit 'recent';return}
}
try{Checkpoint-Computer -Description 'Ametista before applying changes' -RestorePointType MODIFY_SETTINGS -WarningAction Stop}
catch{
    $raced=@(Get-ComputerRestorePoint) | Sort-Object SequenceNumber -Descending | Select-Object -First 1
    if($raced){$racedAt=[Management.ManagementDateTimeConverter]::ToDateTime([string]$raced.CreationTime);if(((Get-Date)-$racedAt).TotalHours -lt 24){Emit 'recent';return}}
    throw
}
# Verify creation instead of treating all warnings/errors as the daily limit.
$after=@(Get-ComputerRestorePoint)
if($after.Count -le $points.Count -and $points.Count -eq 0){Fail 'failed' 'System Protection did not create a restore point'}
$new=$after | Sort-Object SequenceNumber -Descending | Select-Object -First 1
if(-not $new -or ($points.Count -gt 0 -and $new.SequenceNumber -le $latest.SequenceNumber)){Fail 'failed' 'Restore point was not created'}
Emit 'created'
