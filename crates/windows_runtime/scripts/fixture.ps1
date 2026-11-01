# TEST ONLY. Compiled into the Rust runner solely with feature vm-tests.
Require-Admin
if($env:GAMETUNE_DISPOSABLE_VM -cne 'I_HAVE_A_SNAPSHOT'){Fail 'protected'}
$nonce=[string]$p.nonce
$sid=[string]$p.sid
if($nonce -cnotmatch '^V[0-9]+_[0-9]+$' -or $sid -cnotmatch '^S-1-5-[0-9-]{1,176}$'){Fail 'protected'}
$name='GameTuneTest_'+$nonce
$fixtureRoot=Join-Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::CommonApplicationData)) 'GameTuneTestFixtures'
$directory=Join-Path $fixtureRoot $nonce
$marker=Join-Path $directory 'owner.json'
$exe=Join-Path $directory 'GameTuneTestService.exe'
$run='Software\Microsoft\Windows\CurrentVersion\Run'
$approved='Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run'
$registryPath='Software\GameTuneTest\'+$nonce
$users=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::Users,[Microsoft.Win32.RegistryView]::Registry64)
$machine=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine,[Microsoft.Win32.RegistryView]::Registry64)
$interactive=$users.OpenSubKey($sid,$true)
if(-not $interactive){Fail 'failed' 'Interactive user hive is not loaded'}
$scheduler=New-Object -ComObject Schedule.Service
$scheduler.Connect()
$powercfg=Join-Path ([Environment]::SystemDirectory) 'powercfg.exe'
function Power([string[]]$arguments){$output=& $powercfg @arguments 2>&1;if($LASTEXITCODE -ne 0){Fail 'failed' 'Fixture power query failed'};return ($output -join "`n")}
function Full-Registry($key){
    if(-not $key){return $null}
    $values=@{}
    foreach($valueName in ($key.GetValueNames() | Sort-Object)){
        $value=$key.GetValue($valueName,$null,[Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
        if($value -is [byte[]]){$value=[Convert]::ToBase64String($value)}
        $values[$valueName]=@{kind=[int]$key.GetValueKind($valueName);value=$value}
    }
    $children=@{}
    foreach($childName in ($key.GetSubKeyNames() | Sort-Object)){$child=$key.OpenSubKey($childName);try{$children[$childName]=Full-Registry $child}finally{$child.Dispose()}}
    return @{values=$values;children=$children;security=$key.GetAccessControl().GetSecurityDescriptorSddlForm('Owner,Group,Access')}
}
function One-Value($path,$valueName){
    $key=$interactive.OpenSubKey($path)
    if(-not $key){return @{exists=$false;parentExists=$false}}
    try{
        if($valueName -notin $key.GetValueNames()){return @{exists=$false;parentExists=$true}}
        $value=$key.GetValue($valueName,$null,[Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
        if($value -is [byte[]]){$value=[Convert]::ToBase64String($value)}
        return @{exists=$true;parentExists=$true;kind=[int]$key.GetValueKind($valueName);value=$value;security=$key.GetAccessControl().GetSecurityDescriptorSddlForm('Owner,Group,Access')}
    }finally{$key.Dispose()}
}
try{
    if($p.operation -eq 'setup'){
        if([IO.Directory]::Exists($directory) -or (Get-Service -Name $name -ErrorAction SilentlyContinue)){Fail 'conflict'}
        $null=[IO.Directory]::CreateDirectory($directory)
        foreach($path in @($fixtureRoot,$directory)){if(([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::ReparsePoint)){Fail 'protected'}}
        $folderCreated=$false
        try{$folder=$scheduler.GetFolder('\GameTuneTest')}
        catch{$folder=$scheduler.GetFolder('\').CreateFolder('GameTuneTest');$folderCreated=$true}
        $runKey=$interactive.OpenSubKey($run)
        $approvedKey=$interactive.OpenSubKey($approved)
        $testKey=$interactive.OpenSubKey('Software\GameTuneTest')
        $initialPower=([regex]::Match((Power -arguments @('/getactivescheme')),'[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}')).Value
        $sha=[Security.Cryptography.SHA256]::Create()
        try{$hash=([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($nonce)))).Replace('-','').ToLowerInvariant()}finally{$sha.Dispose()}
        $testGuid='47544e54-'+$hash.Substring(0,4)+'-'+$hash.Substring(4,4)+'-'+$hash.Substring(8,4)+'-'+$hash.Substring(12,12)
        $state=@{nonce=$nonce;initialPower=$initialPower;testGuid=$testGuid;taskFolderCreated=$folderCreated;runParentExisted=($null -ne $runKey);approvedParentExisted=($null -ne $approvedKey);testParentExisted=($null -ne $testKey)}
        foreach($key in @($runKey,$approvedKey,$testKey)){if($key){$key.Dispose()}}
        [IO.File]::WriteAllText($marker,($state | ConvertTo-Json),[Text.UTF8Encoding]::new($false))
        # Fixed test-service source. No user text is interpolated into C#.
        Add-Type -TypeDefinition @'
using System;
using System.ServiceProcess;
public sealed class GameTuneTestService : ServiceBase {
    GameTuneTestService(string name) { ServiceName=name; CanStop=true; AutoLog=false; }
    protected override void OnStart(string[] args) { }
    protected override void OnStop() { }
    public static void Main(string[] args) {
        if(args.Length==1 && args[0].StartsWith("GameTuneTest_",StringComparison.Ordinal))
            ServiceBase.Run(new GameTuneTestService(args[0]));
    }
}
'@ -ReferencedAssemblies System.dll,System.ServiceProcess.dll -OutputAssembly $exe -OutputType WindowsApplication
        $null=New-Service -Name $name -BinaryPathName ('"'+$exe+'" '+$name) -StartupType Automatic -DisplayName $name
        $service=Get-Service -Name $name
        $service.Start();$service.WaitForStatus([ServiceProcess.ServiceControllerStatus]::Running,[TimeSpan]::FromSeconds(15));$service.Dispose()
        $serviceKey=$machine.OpenSubKey(('SYSTEM\CurrentControlSet\Services\'+$name),$true)
        try{$serviceKey.SetValue('DelayedAutoStart',1,[Microsoft.Win32.RegistryValueKind]::DWord)}finally{$serviceKey.Dispose()}
        $action=New-ScheduledTaskAction -Execute $exe -Argument '--task-probe'
        $trigger=New-ScheduledTaskTrigger -Once -At ([datetime]'2100-01-01T12:00:00')
        $principal=New-ScheduledTaskPrincipal -UserId 'SYSTEM' -LogonType ServiceAccount -RunLevel Highest
        $null=Register-ScheduledTask -TaskName $name -TaskPath '\GameTuneTest\' -Action $action -Trigger $trigger -Principal $principal
        $key=$interactive.CreateSubKey(($registryPath+'\Settings'))
        try{$key.SetValue('OnlyTest',[byte[]]@(0,255,128,42),[Microsoft.Win32.RegistryValueKind]::Binary);$key.SetValue('Untouched','keep',[Microsoft.Win32.RegistryValueKind]::String)}finally{$key.Dispose()}
        $key=$interactive.CreateSubKey($run)
        try{$key.SetValue($name,('"'+$exe+'" --startup-probe'),[Microsoft.Win32.RegistryValueKind]::String)}finally{$key.Dispose()}
        # Leave StartupApproved absent for this dedicated entry; undo must remove it again.
        Emit $directory;return
    }
    if(-not [IO.File]::Exists($marker)){Fail 'protected'}
    $state=[IO.File]::ReadAllText($marker) | ConvertFrom-Json
    if($state.nonce -cne $nonce){Fail 'protected'}
    foreach($path in @($fixtureRoot,$directory,$marker)){if(([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::ReparsePoint)){Fail 'protected'}}
    if($p.operation -eq 'snapshot'){
        $key=$interactive.OpenSubKey($registryPath)
        try{$registry=Full-Registry $key}finally{if($key){$key.Dispose()}}
        $key=$machine.OpenSubKey(('SYSTEM\CurrentControlSet\Services\'+$name))
        try{$serviceRegistry=Full-Registry $key}finally{if($key){$key.Dispose()}}
        $service=Get-CimInstance Win32_Service -Filter ("Name='"+$name+"'")
        $task=$scheduler.GetFolder('\GameTuneTest').GetTask($name)
        $schemes=@{}
        $list=Power -arguments @('/list')
        foreach($match in [regex]::Matches($list,'[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}')){
            $guid=$match.Value.ToLowerInvariant();$schemes[$guid]=Power -arguments @('/query',$guid)
        }
        Emit @{
            registry=$registry
            service=@{registry=$serviceRegistry;startMode=$service.StartMode;state=$service.State;startName=$service.StartName;pathName=$service.PathName;serviceType=$service.ServiceType;errorControl=$service.ErrorControl;desktopInteract=$service.DesktopInteract}
            task=@{xml=$task.Xml;enabled=[bool]$task.Enabled;security=$task.GetSecurityDescriptor(7)}
            startup=@{run=(One-Value $run $name);approved=(One-Value $approved $name)}
            power=@{list=$list;active=(Power -arguments @('/getactivescheme'));schemes=$schemes}
        };return
    }
    if($p.operation -eq 'cleanup'){
        if(-not (Is-Guid $state.initialPower) -or $state.testGuid -cnotmatch '^47544e54-[0-9a-f-]{27}$'){Fail 'protected'}
        $active=Power -arguments @('/getactivescheme')
        if($active -match [regex]::Escape($state.testGuid)){$null=Power -arguments @('/setactive',[string]$state.initialPower)}
        $plans=Power -arguments @('/list')
        if($plans -match [regex]::Escape($state.testGuid)){$null=Power -arguments @('/delete',[string]$state.testGuid)}
        # Remove only names created by setup, never real catalog targets.
        $service=Get-Service -Name $name -ErrorAction SilentlyContinue
        if($service){if($service.Status -ne 'Stopped'){$service.Stop();$service.WaitForStatus([ServiceProcess.ServiceControllerStatus]::Stopped,[TimeSpan]::FromSeconds(15))};$service.Dispose();$sc=Join-Path ([Environment]::SystemDirectory) 'sc.exe';$null=& $sc delete $name;if($LASTEXITCODE -ne 0){Fail 'failed' 'Test service deletion failed'}}
        $folder=$scheduler.GetFolder('\GameTuneTest')
        $folder.DeleteTask($name,0)
        if($state.taskFolderCreated -and $folder.GetTasks(1).Count -eq 0){$scheduler.GetFolder('\').DeleteFolder('GameTuneTest',0)}
        foreach($path in @($run,$approved)){$key=$interactive.OpenSubKey($path,$true);if($key){try{$key.DeleteValue($name,$false)}finally{$key.Dispose()}}}
        if($registryPath -cnotmatch '^Software\\GameTuneTest\\V[0-9]+_[0-9]+$'){Fail 'protected'}
        $interactive.DeleteSubKeyTree($registryPath,$false)
        foreach($pair in @(@($run,$state.runParentExisted),@($approved,$state.approvedParentExisted),@('Software\GameTuneTest',$state.testParentExisted))){
            if(-not $pair[1]){$key=$interactive.OpenSubKey($pair[0]);if($key){try{$empty=($key.ValueCount -eq 0 -and $key.SubKeyCount -eq 0)}finally{$key.Dispose()};if($empty){$interactive.DeleteSubKey($pair[0],$false)}}}
        }
        # No recursive filesystem deletion. Delete only the known helper and marker.
        [IO.File]::Delete($exe);[IO.File]::Delete($marker)
        if([IO.Directory]::GetFileSystemEntries($directory).Count -eq 0){[IO.Directory]::Delete($directory)}
        if([IO.Directory]::GetFileSystemEntries($fixtureRoot).Count -eq 0){[IO.Directory]::Delete($fixtureRoot)}
        Emit $true;return
    }
    Fail 'protected'
}finally{$interactive.Dispose();$users.Dispose();$machine.Dispose()}
