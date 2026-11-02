Require-Admin
$programData=[Environment]::GetFolderPath([Environment+SpecialFolder]::CommonApplicationData)
if([string]::IsNullOrEmpty($programData)){Fail 'failed' 'CommonApplicationData is unavailable'}
# Check again immediately before creating or verifying the protected state directory.
$legacy=Join-Path $programData ('Game'+'Tune')
if(Test-Path -LiteralPath $legacy -ErrorAction Stop){Fail 'failed' 'legacy_data'}
function No-Reparse([string]$path){
    $current=[IO.DirectoryInfo]::new($path)
    while($null -ne $current){if($current.Exists -and ($current.Attributes -band [IO.FileAttributes]::ReparsePoint)){Fail 'protected'};$current=$current.Parent}
}
function Secure-Directory([string]$path){
    No-Reparse $path
    $acl=[Security.AccessControl.DirectorySecurity]::new()
    $acl.SetAccessRuleProtection($true,$false)
    $admin=[Security.Principal.SecurityIdentifier]::new('S-1-5-32-544')
    $system=[Security.Principal.SecurityIdentifier]::new('S-1-5-18')
    $acl.SetOwner($admin)
    foreach($sid in @($admin,$system)){$acl.AddAccessRule([Security.AccessControl.FileSystemAccessRule]::new($sid,'FullControl','ContainerInherit,ObjectInherit','None','Allow'))}
    $directory=[IO.DirectoryInfo]::new($path)
    if(-not $directory.Exists){$directory.Create($acl)}
    No-Reparse $path
    $actual=$directory.GetAccessControl()
    if(-not $actual.AreAccessRulesProtected){Fail 'protected'}
    if($actual.GetOwner([Security.Principal.SecurityIdentifier]).Value -notin @('S-1-5-32-544','S-1-5-18')){Fail 'protected'}
    $rules=@($actual.GetAccessRules($true,$true,[Security.Principal.SecurityIdentifier]))
    if($rules.Count -ne 2){Fail 'protected'}
    foreach($rule in $rules){if($rule.IdentityReference.Value -notin @('S-1-5-32-544','S-1-5-18') -or $rule.AccessControlType -ne 'Allow' -or $rule.FileSystemRights -ne 'FullControl'){Fail 'protected'}}
}
$sid=[string]$p.sid
if($sid -cnotmatch '^S-1-5-[0-9-]{1,176}$'){Fail 'protected'}
$rootName=if($p.testRoot){'AmetistaTestStore'}else{'Ametista'}
$root=Join-Path $programData $rootName
Secure-Directory $root
$directory=Join-Path $root $sid
Secure-Directory $directory
if($p.cleanupTest -and $p.testRoot){
    if([IO.Directory]::GetFileSystemEntries($directory).Count -ne 0){Fail 'conflict'}
    [IO.Directory]::Delete($directory)
    if([IO.Directory]::GetFileSystemEntries($root).Count -eq 0){[IO.Directory]::Delete($root)}
    Emit $true;return
}
# No traversal, junctions, symlinks or hardlinks in existing state files.
foreach($file in Get-ChildItem -LiteralPath $directory -Force){
    if($file.PSIsContainer -or ($file.Attributes -band [IO.FileAttributes]::ReparsePoint) -or $file.LinkType){Fail 'protected'}
    $acl=Get-Acl -LiteralPath $file.FullName
    if($acl.GetOwner([Security.Principal.SecurityIdentifier]).Value -notin @('S-1-5-32-544','S-1-5-18')){Fail 'protected'}
    foreach($rule in $acl.GetAccessRules($true,$true,[Security.Principal.SecurityIdentifier])){if($rule.IdentityReference.Value -notin @('S-1-5-32-544','S-1-5-18') -or $rule.AccessControlType -ne 'Allow'){Fail 'protected'}}
}
if(-not $p.testRoot){Secure-Directory (Join-Path $root 'logs')}
Emit $directory
