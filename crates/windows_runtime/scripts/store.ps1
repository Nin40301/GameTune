$uri=[string]$p.uri
if($uri -cnotmatch '^ms-windows-store://pdp/\?PFN=[A-Za-z0-9.-]+_[a-z0-9]{13}$'){Fail 'protected'}
# Execute through the existing desktop shell, including when the CLI is elevated.
# Microsoft: shell/samples-execinexplorer and IShellWindows::FindWindowSW.
$desktop=Get-AmetistaDesktop ([string]$p.expectedSid)
$desktop.ShellExecute($uri,'','','open',1)
Emit $true
