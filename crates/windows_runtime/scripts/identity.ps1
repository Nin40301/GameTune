# Fixed FFI: query only this desktop session. No game process handles or tokens.
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class GameTuneSession {
    [DllImport("wtsapi32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    static extern bool WTSQuerySessionInformation(IntPtr server, int session, int kind, out IntPtr buffer, out int size);
    [DllImport("wtsapi32.dll")] static extern void WTSFreeMemory(IntPtr pointer);
    public static string Query(int session, int kind) {
        IntPtr buffer; int size;
        if (!WTSQuerySessionInformation(IntPtr.Zero, session, kind, out buffer, out size))
            throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
        try { return Marshal.PtrToStringUni(buffer) ?? ""; } finally { WTSFreeMemory(buffer); }
    }
}
'@
$session = [Diagnostics.Process]::GetCurrentProcess().SessionId
if($p.testCurrentUser){
    # Only the cfg(test) Rust constructor issues this request. Never used by detect().
    $sid=[Security.Principal.WindowsIdentity]::GetCurrent().User.Value
}else{
    if ($session -eq 0) { Fail 'failed' 'No interactive desktop session' }
    $userName = [GameTuneSession]::Query($session,5)
    $domainName = [GameTuneSession]::Query($session,7)
    if ([string]::IsNullOrEmpty($userName)) { Fail 'failed' 'Interactive session has no user' }
    # Resolve locally without domain/name lookup (which could contact a server).
    # GetOwnerSid is called ONLY for the canonical Windows shell in this session.
    $expectedShell=Join-Path (Split-Path -Parent ([Environment]::SystemDirectory)) 'explorer.exe'
    $candidates=@()
    $shells=@(Get-CimInstance -Query ("SELECT ProcessId,ExecutablePath,SessionId,Name FROM Win32_Process WHERE Name='explorer.exe' AND SessionId="+[int]$session))
    foreach($shell in $shells){
        if([string]::Equals([string]$shell.ExecutablePath,$expectedShell,[StringComparison]::OrdinalIgnoreCase)){
            $owner=Invoke-CimMethod -InputObject $shell -MethodName GetOwnerSid
            if($owner.ReturnValue -eq 0){$candidates+=[string]$owner.Sid}
        }
    }
    if($candidates.Count -eq 0){
        # Custom-shell fallback: match the locally loaded interactive profile.
        $hives=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::Users,[Microsoft.Win32.RegistryView]::Registry64)
        try{
            foreach($profile in $hives.GetSubKeyNames()){
                if($profile -cnotmatch '^S-1-5-[0-9-]+$'){continue}
                $environment=$null
                try{$environment=$hives.OpenSubKey(($profile+'\Volatile Environment'))}catch [UnauthorizedAccessException]{continue}
                if($environment){try{
                    if([string]::Equals([string]$environment.GetValue('USERNAME'),$userName,[StringComparison]::OrdinalIgnoreCase) -and [string]::Equals([string]$environment.GetValue('USERDOMAIN'),$domainName,[StringComparison]::OrdinalIgnoreCase)){$candidates+=$profile}
                }finally{$environment.Dispose()}}
            }
        }finally{$hives.Dispose()}
    }
    $candidates=@($candidates | Sort-Object -Unique)
    if($candidates.Count -ne 1){Fail 'failed' 'Cannot uniquely resolve the local interactive SID'}
    $sid=$candidates[0]
}
$machine = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine,[Microsoft.Win32.RegistryView]::Registry64)
try {
    $crypto=$machine.OpenSubKey('SOFTWARE\Microsoft\Cryptography')
    $version=$machine.OpenSubKey('SOFTWARE\Microsoft\Windows NT\CurrentVersion')
    try { $guid=$crypto.GetValue('MachineGuid'); $build=[uint32]$version.GetValue('CurrentBuildNumber'); $sku=[string]$version.GetValue('EditionID') }
    finally { if($crypto){$crypto.Dispose()};if($version){$version.Dispose()} }
} finally { $machine.Dispose() }
$edition = switch -Regex ($sku) { '^Core' {'home';break}; '^Professional' {'pro';break}; '^Enterprise' {'enterprise';break}; '^Education' {'education';break}; default { Fail 'failed' 'Unsupported Windows edition' } }
$principal = [Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent())
Emit @{sid=$sid;machineGuid=$guid;sessionId=$session;elevated=$principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator);fileOwnerSid=[Security.Principal.WindowsIdentity]::GetCurrent().Owner.Value;build=$build;edition=$edition}
