# DESTRUCTIVE VM TEST ONLY. Never embedded in a default-feature release.
Require-Admin
if($env:GAMETUNE_DISPOSABLE_VM -cne 'I_HAVE_A_SNAPSHOT' -or $env:GAMETUNE_REAL_PROFILES -cne 'I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS'){Fail 'protected'}
$sid=[string]$p.sid
if($sid -cnotmatch '^S-1-5-[0-9-]{1,176}$' -or $p.operation -notin @('step','verify','recover')){Fail 'protected'}
$computer=Get-CimInstance Win32_ComputerSystem
if(($computer.Manufacturer+' '+$computer.Model) -notmatch 'Virtual|VMware|VirtualBox|KVM|QEMU|Xen|Parallels|Bochs'){Fail 'failed' 'A disposable virtual machine is required'}
$exe=[IO.Path]::GetFullPath([string]$p.exe)
if([IO.Path]::GetFileName($exe) -cne 'gametune-cli.exe' -or -not [IO.File]::Exists($exe)){Fail 'failed' 'Build the default-feature debug CLI first'}
$root=Join-Path ([Environment]::GetFolderPath('CommonApplicationData')) 'GameTuneRealProfileTests'
$directory=Join-Path $root $sid
foreach($path in @($root,$directory)){
    if([IO.Directory]::Exists($path) -and ([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::ReparsePoint)){Fail 'protected'}
    $null=[IO.Directory]::CreateDirectory($path)
    $acl=[Security.AccessControl.DirectorySecurity]::new()
    $acl.SetAccessRuleProtection($true,$false)
    foreach($identity in @('S-1-5-18','S-1-5-32-544')){$principal=[Security.Principal.SecurityIdentifier]::new($identity);$acl.AddAccessRule([Security.AccessControl.FileSystemAccessRule]::new($principal,'FullControl','ContainerInherit,ObjectInherit','None','Allow'))}
    $acl.SetOwner([Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    [IO.Directory]::SetAccessControl($path,$acl)
}
$statePath=Join-Path $directory 'state.json'
foreach($path in @($statePath,(Join-Path $directory 'state.tmp'))){if([IO.File]::Exists($path) -and ([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::ReparsePoint)){Fail 'protected'}}
$lease=[IO.File]::Open((Join-Path $directory 'exclusive.lock'),'OpenOrCreate','ReadWrite','None')
function Save-Json($path,$value){
    if([IO.File]::Exists($path) -and ([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::ReparsePoint)){throw 'Evidence reparse point refused'}
    $temporary=Join-Path $directory 'state.tmp'
    [IO.File]::WriteAllText($temporary,($value|ConvertTo-Json -Depth 64),[Text.UTF8Encoding]::new($false))
    $flush=[IO.File]::Open($temporary,'Open','ReadWrite','None');try{$flush.Flush($true)}finally{$flush.Dispose()}
    if([IO.File]::Exists($path)){[IO.File]::Replace($temporary,$path,$null)}else{[IO.File]::Move($temporary,$path)}
}
function Cli([string[]]$arguments){
    $text=(& $exe @arguments --json --lang en | Out-String)
    $code=$LASTEXITCODE
    $result=$text|ConvertFrom-Json
    if($code -notin @(0,3)){throw ('CLI rejected operation: '+$text)}
    return $result
}
function Stable-Json($value){
    if($null -eq $value){return 'null'}
    if($value -is [Collections.IDictionary]){$parts=@($value.Keys|Sort-Object|ForEach-Object {($_|ConvertTo-Json -Compress)+':'+(Stable-Json $value[$_])});return '{'+($parts -join ',')+'}'}
    if($value -is [pscustomobject]){$map=@{};foreach($property in $value.PSObject.Properties){$map[$property.Name]=$property.Value};return Stable-Json $map}
    if($value -is [Collections.IEnumerable] -and $value -isnot [string]){return '['+(@($value|ForEach-Object {Stable-Json $_}) -join ',')+']'}
    return ($value|ConvertTo-Json -Compress)
}
function Full-Key($key){
    if(-not $key){return $null};$values=@{};$children=@{}
    foreach($name in $key.GetValueNames()){$value=$key.GetValue($name,$null,'DoNotExpandEnvironmentNames');if($value -is [byte[]]){$value=[Convert]::ToBase64String($value)};$values[$name]=@{kind=[int]$key.GetValueKind($name);value=$value}}
    foreach($name in $key.GetSubKeyNames()){$child=$key.OpenSubKey($name);try{$children[$name]=Full-Key $child}finally{$child.Dispose()}}
    return @{values=$values;children=$children;security=$key.GetAccessControl().GetSecurityDescriptorSddlForm('Owner,Group,Access')}
}
$machine=[Microsoft.Win32.RegistryKey]::OpenBaseKey('LocalMachine','Registry64')
$users=[Microsoft.Win32.RegistryKey]::OpenBaseKey('Users','Registry64')
$interactive=$users.OpenSubKey($sid)
if(-not $interactive){Fail 'protected'}
$scheduler=New-Object -ComObject Schedule.Service;$scheduler.Connect()
$powercfg=Join-Path ([Environment]::SystemDirectory) 'powercfg.exe'
function Power([string[]]$arguments){$value=& $powercfg @arguments;if($LASTEXITCODE -ne 0){throw 'Power query failed'};return ($value -join "`n")}
function Snapshot($ids){
    $result=@{};$catalog=Cli -arguments @('--list')
    $analysis=Cli -arguments @('--profile','balanced','--dry-run')
    foreach($id in $ids){
        $item=@($catalog.result.items|Where-Object {$_.id -ceq $id})[0]
        $observation=@($analysis.items|Where-Object {$_.itemId -ceq $id})[0].observation
        if($observation.error){throw ('Cannot snapshot '+$id)}
        $target=$item.target;$extra=$null
        switch($item.type){
            'registry' {$base=if($target.hive -eq 'HKCU'){$interactive}else{$machine};$key=$base.OpenSubKey([string]$target.path);try{$extra=Full-Key $key}finally{if($key){$key.Dispose()}}}
            'service' {$key=$machine.OpenSubKey(('SYSTEM\CurrentControlSet\Services\'+$target.name));try{$registry=Full-Key $key}finally{if($key){$key.Dispose()}};$service=Get-CimInstance Win32_Service -Filter ("Name='"+$target.name+"'");$extra=@{registry=$registry;configuration=if($service){@{startMode=$service.StartMode;state=$service.State;startName=$service.StartName;pathName=$service.PathName;serviceType=$service.ServiceType;errorControl=$service.ErrorControl;desktopInteract=$service.DesktopInteract}}else{$null}}}
            'scheduled_task' {if($observation.exists){$task=$scheduler.GetFolder(([string]$target.path).TrimEnd('\')).GetTask([string]$target.name);$extra=@{xml=$task.Xml;enabled=$task.Enabled;security=$task.GetSecurityDescriptor(7)}}}
            'power' {$schemes=@{};foreach($match in [regex]::Matches((Power -arguments @('/list')),'[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}')){$schemes[$match.Value]=Power -arguments @('/query',$match.Value)};$extra=@{active=(Power -arguments @('/getactivescheme'));schemes=$schemes}}
            default {throw 'Unexpected automatic target type'}
        }
        $result[$id]=@{exists=$observation.exists;current=$observation.current;full=$extra}
    }
    return $result
}
function Smoke {
    # Only known Windows app windows are inspected. No OpenProcess or game handles.
    Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
public static class GameTuneVmWindows {
    delegate bool EnumProc(IntPtr hwnd,IntPtr param);
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc cb,IntPtr p);
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr w,EnumProc cb,IntPtr p);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr w);
    [DllImport("user32.dll")] static extern bool IsHungAppWindow(IntPtr w);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr w,out uint p);
    public static bool Responsive(uint pid) {
        bool found=false;
        EnumWindows((w,p)=>{if(!IsWindowVisible(w)||IsHungAppWindow(w))return true;
            uint owner;GetWindowThreadProcessId(w,out owner);if(owner==pid)found=true;
            EnumChildWindows(w,(c,x)=>{uint child;GetWindowThreadProcessId(c,out child);if(child==pid&&!IsHungAppWindow(c))found=true;return true;},IntPtr.Zero);return true;
        },IntPtr.Zero);return found;
    }
}
'@
    $desktop=Get-GameTuneDesktop $sid
    $edge=Join-Path ([Environment]::GetFolderPath('ProgramFilesX86')) 'Microsoft\Edge\Application\msedge.exe'
    if(-not [IO.File]::Exists($edge)){$edge=Join-Path ([Environment]::GetFolderPath('ProgramFiles')) 'Microsoft\Edge\Application\msedge.exe'}
    if(-not [IO.File]::Exists($edge)){throw 'Edge missing'}
    $specs=@(
        @{id='store';uri='ms-windows-store://home';name='WinStore.App.exe';pattern='\\WindowsApps\\Microsoft.WindowsStore_'},
        @{id='edge';uri=$edge;name='msedge.exe';pattern='\\Microsoft\\Edge\\Application\\msedge.exe$'},
        @{id='settings';uri='ms-settings:about';name='SystemSettings.exe';pattern='\\Windows\\ImmersiveControlPanel\\SystemSettings.exe$'},
        @{id='security';uri='windowsdefender:';name='SecHealthUI.exe';pattern='\\(SystemApps|WindowsApps)\\[^\\]*(SecHealthUI|SecurityHealth)[^\\]*\\'}
    )
    $evidence=@{}
    foreach($spec in $specs){
        $arguments=if($spec.id -eq 'edge'){'--new-window --no-first-run about:blank'}else{''}
        $desktop.ShellExecute($spec.uri,$arguments,'','open',1)
        $found=$null;$deadline=[datetime]::UtcNow.AddSeconds(40)
        while([datetime]::UtcNow -lt $deadline){
            foreach($process in @(Get-CimInstance Win32_Process -Filter ("Name='"+$spec.name+"'"))){
                if($process.ExecutablePath -notmatch $spec.pattern -or $process.SessionId -ne [Diagnostics.Process]::GetCurrentProcess().SessionId){continue}
                $owner=Invoke-CimMethod -InputObject $process -MethodName GetOwnerSid
                if($owner.Sid -ceq $sid -and [GameTuneVmWindows]::Responsive([uint32]$process.ProcessId)){$found=@{responsive=$true;path=$process.ExecutablePath;pid=$process.ProcessId};break}
            }
            if($found){break};Start-Sleep -Milliseconds 500
        }
        if(-not $found){throw ('No visible responsive '+$spec.id+' window')};$evidence[$spec.id]=$found
    }
    $defender=Get-MpComputerStatus
    if(-not $defender.AMServiceEnabled -or -not $defender.AntivirusEnabled -or -not $defender.RealTimeProtectionEnabled){throw 'Defender health check failed'}
    $evidence['defender']=@{service=$defender.AMServiceEnabled;antivirus=$defender.AntivirusEnabled;realTime=$defender.RealTimeProtectionEnabled}
    return $evidence
}
try {
    $boot=(Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToUniversalTime().ToString('o')
    $machineGuid=[string]$machine.OpenSubKey('SOFTWARE\Microsoft\Cryptography').GetValue('MachineGuid')
    $state=if([IO.File]::Exists($statePath)){[IO.File]::ReadAllText($statePath)|ConvertFrom-Json}else{$null}
    if($state -and ($state.sid -cne $sid -or $state.machine -cne $machineGuid)){throw 'Evidence belongs to another identity'}
    if($p.operation -eq 'recover'){$undo=Cli -arguments @('--undo','--apply','--confirm');if($undo.exitCode -ne 0){throw 'Undo incomplete'};if($state){$state.stage='aborted';Save-Json $statePath $state};Emit @{status='aborted';evidence=$directory};return}
    if($p.operation -eq 'verify'){
        if(-not $state -or $state.stage -ne 'complete' -or $state.completed.Count -ne 2){throw 'Run both reboot stages before acceptance'}
        foreach($profile in @('safe','balanced')){$record=@($state.completed|Where-Object {$_.profile -ceq $profile});if($record.Count -ne 1 -or $record[0].bootBefore -ceq $record[0].bootAfter -or -not $record[0].snapshotEqual){throw 'Incomplete reboot or undo evidence'};foreach($app in @('store','edge','settings','security')){if(-not $record[0].smoke.$app.responsive){throw 'Missing health evidence'}}}
        Emit @{status='complete';evidence=$directory};return
    }
    if(-not $state){
        $history=Cli -arguments @('--undo');if($history.result.needsRecovery -or @($history.result.entries|Where-Object {$_.status -ne 'reverted'}).Count -gt 0){throw 'Use a clean VM without active GameTune history'}
        $state=[pscustomobject]@{version=1;sid=$sid;machine=$machineGuid;stage='ready';profile='safe';bootBefore='';ids=@();before=$null;completed=@();baselineSmoke=(Smoke)}
    }
    if($state.stage -eq 'complete'){Emit @{status='complete';evidence=$directory};return}
    if($state.stage -eq 'await_reboot'){
        if($boot -ceq $state.bootBefore){Emit @{status='reboot_required';profile=$state.profile;evidence=$directory};return}
        $smoke=Smoke
        $afterReboot=Snapshot $state.ids
        Save-Json (Join-Path $directory ($state.profile+'-after-reboot.json')) $afterReboot
        $postBoot=Cli -arguments @('--profile',[string]$state.profile,'--dry-run')
        foreach($row in @($postBoot.items|Where-Object {$_.selected})){
            if($row.observation.error -or ($row.observation.exists -and -not $row.observation.alreadyApplied)){throw ('Adjustment did not survive reboot: '+$row.itemId)}
        }
        $undo=Cli -arguments @('--undo','--apply','--confirm');if($undo.exitCode -ne 0){throw 'Undo incomplete; preserve VM for recovery'}
        $restored=Snapshot $state.ids
        Save-Json (Join-Path $directory ($state.profile+'-restored.json')) $restored
        if((Stable-Json $restored) -cne (Stable-Json $state.before)){throw 'Full snapshot differs after undo'}
        $state.completed+=@{profile=$state.profile;bootBefore=$state.bootBefore;bootAfter=$boot;smoke=$smoke;snapshotEqual=$true}
        if($state.profile -eq 'balanced'){$state.stage='complete';Save-Json $statePath $state;Emit @{status='complete';evidence=$directory};return}
        $state.profile='balanced';$state.stage='ready';Save-Json $statePath $state
    }
    if($state.stage -ne 'ready'){throw 'Incomplete application; use recover or restore the hypervisor snapshot'}
    $preview=Cli -arguments @('--profile',[string]$state.profile,'--dry-run')
    $state.ids=@($preview.items|Where-Object {$_.selected}|ForEach-Object {$_.itemId})
    if($state.ids.Count -eq 0){throw 'Empty automatic profile'}
    $state.before=Snapshot $state.ids;$state.bootBefore=$boot;$state.stage='applying'
    Save-Json (Join-Path $directory ($state.profile+'-before.json')) $state.before
    Save-Json $statePath $state # Crash evidence BEFORE applying the real catalog.
    $apply=Cli -arguments @('--profile',[string]$state.profile,'--apply','--confirm')
    Save-Json (Join-Path $directory ($state.profile+'-apply.json')) $apply
    foreach($row in @($apply.result.report.items)){if($row.outcome -notin @('applied','already_applied') -and $row.error.code -ne 'not_found'){throw 'Apply incomplete'}}
    Save-Json (Join-Path $directory ($state.profile+'-after-apply.json')) (Snapshot $state.ids)
    $state.stage='await_reboot';Save-Json $statePath $state
    Emit @{status='reboot_required';profile=$state.profile;evidence=$directory}
} finally {$interactive.Dispose();$users.Dispose();$machine.Dispose();$lease.Dispose()}

