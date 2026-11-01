# One read-only inventory per scan, narrowed to embedded catalog names.
$names=@($p.names)
foreach($name in $names){if($name -cnotmatch '^[A-Za-z0-9.-]{1,160}$'){Fail 'protected'}}
$principal=[Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent())
$all=$principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
$packages=if($all){@(Get-AppxPackage -AllUsers -PackageTypeFilter Main,Bundle)}else{@(Get-AppxPackage -PackageTypeFilter Main,Bundle)}
$provisioned=if($all){@(Get-AppxProvisionedPackage -Online)}else{@()}
$result=@{}
foreach($name in $names){
    $installed=@($packages|Where-Object {$_.Name -ieq $name})
    $staged=@($provisioned|Where-Object {$_.DisplayName -ieq $name})
    if($installed.Count -eq 0 -and $staged.Count -eq 0){continue}
    $families=@($installed|ForEach-Object {$_.PackageFamilyName})
    foreach($package in $staged){$families+=($name+'_'+([string]$package.PackageName).Split('_')[-1])}
    $result[$name]=@{snapshot=@{type='appx';packages=@($installed.PackageFullName|Sort-Object -Unique);provisioned=@($staged.PackageName|Sort-Object -Unique);families=@($families|Sort-Object -Unique)};removable=(@($installed|Where-Object {$_.NonRemovable -or $_.IsFramework -or $_.IsResourcePackage}).Count -eq 0)}
}
Emit $result
