# Fixed FFI only to identify the existing desktop window owner. No process handles.
function Get-GameTuneDesktop([string]$expectedSid) {
    Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class GameTuneDesktopOwner {
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint processId);
}
'@
    $shell=New-Object -ComObject Shell.Application
    $location=[object]0; $root=[object]0; $desktopHwnd=0
    $desktop=$shell.Windows().FindWindowSW([ref]$location,[ref]$root,8,[ref]$desktopHwnd,1)
    if($null -eq $desktop -or $desktopHwnd -eq 0){Fail 'unavailable'}
    [uint32]$ownerPid=0
    $null=[GameTuneDesktopOwner]::GetWindowThreadProcessId([IntPtr]$desktopHwnd,[ref]$ownerPid)
    $owner=Get-CimInstance Win32_Process -Filter "ProcessId=$ownerPid"
    $expected=Join-Path ([Environment]::GetFolderPath('Windows')) 'explorer.exe'
    if($null -eq $owner -or $owner.ExecutablePath -ine $expected -or $owner.SessionId -ne [Diagnostics.Process]::GetCurrentProcess().SessionId){Fail 'protected'}
    $sid=Invoke-CimMethod -InputObject $owner -MethodName GetOwnerSid
    if($sid.ReturnValue -ne 0 -or $sid.Sid -cne $expectedSid){Fail 'protected'}
    return $desktop.Document.Application
}
