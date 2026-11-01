if($p.operation -eq 'apply'){Require-Admin}
$name=[string]$p.name
if($name -cnotmatch '^[A-Za-z0-9.-]{1,160}$'){Fail 'protected'}
$principal=[Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent())
$allUsers=$principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if($allUsers){
    $packages=@(Get-AppxPackage -AllUsers -Name $name -PackageTypeFilter Main,Bundle | Where-Object {$_.Name -ieq $name})
    $provisioned=@(Get-AppxProvisionedPackage -Online | Where-Object {$_.DisplayName -ieq $name})
}else{
    # Standard users can inspect their own packages. Full removal recaptures elevated.
    $packages=@(Get-AppxPackage -Name $name -PackageTypeFilter Main,Bundle | Where-Object {$_.Name -ieq $name})
    $provisioned=@()
}
if($packages.Count -eq 0 -and $provisioned.Count -eq 0){Fail 'not_found'}
if(@($packages | Where-Object {$_.NonRemovable -or $_.IsFramework -or $_.IsResourcePackage}).Count -gt 0){Fail 'protected'}
$families=@($packages | ForEach-Object {$_.PackageFamilyName})
# Provisioned package names encode Name_version_arch_resource_PublisherId.
foreach($package in $provisioned){$publisher=([string]$package.PackageName).Split('_')[-1];$families+=($name+'_'+$publisher)}
$snapshot=@{type='appx';packages=@($packages.PackageFullName | Sort-Object -Unique);provisioned=@($provisioned.PackageName | Sort-Object -Unique);families=@($families | Sort-Object -Unique)}
if($p.operation -eq 'capture'){Emit $snapshot;return}
if($p.operation -ne 'apply'){Fail 'manual_reinstall_required'}
if(-not $allUsers){Fail 'access_denied'}
if(($snapshot.packages -join '|') -ne (@($p.expected.packages) -join '|') -or ($snapshot.provisioned -join '|') -ne (@($p.expected.provisioned) -join '|') -or ($snapshot.families -join '|') -ne (@($p.expected.families) -join '|')){Fail 'conflict'}
# Only exact inventory returned for this catalog name is passed to removal cmdlets.
foreach($package in ($packages | Sort-Object IsBundle -Descending)){
    $remaining=@(Get-AppxPackage -AllUsers -Name $name -PackageTypeFilter Main,Bundle | Where-Object {$_.PackageFullName -ceq $package.PackageFullName})
    if($remaining.Count -gt 0){Remove-AppxPackage -Package $package.PackageFullName -AllUsers -Confirm:$false}
}
foreach($package in $provisioned){$null=Remove-AppxProvisionedPackage -Online -PackageName $package.PackageName -AllUsers}
if(@(Get-AppxPackage -AllUsers -Name $name -PackageTypeFilter Main,Bundle | Where-Object {$_.Name -ieq $name}).Count -gt 0 -or @(Get-AppxProvisionedPackage -Online | Where-Object {$_.DisplayName -ieq $name}).Count -gt 0){Fail 'failed' 'Some app packages remain installed or provisioned'}
Emit $true
