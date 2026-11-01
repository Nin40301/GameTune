$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
[Console]::InputEncoding = [Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$p = [Console]::In.ReadToEnd() | ConvertFrom-Json
function Emit($value) { [Console]::Out.WriteLine((@{ok=$true;value=$value;error=$null} | ConvertTo-Json -Depth 32 -Compress)) }
function Fail([string]$code, [string]$detail='') {
    $e = @{code=$code}
    if ($code -eq 'failed') { $e.detail=$detail }
    [Console]::Out.WriteLine((@{ok=$false;value=$null;error=$e} | ConvertTo-Json -Depth 32 -Compress))
    exit 0
}
function Require-Admin {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    if (-not ([Security.Principal.WindowsPrincipal]::new($identity)).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { Fail 'access_denied' }
}
function Is-Guid([string]$value) { return $value -cmatch '^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$' }
