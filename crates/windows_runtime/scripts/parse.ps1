# TEST ONLY: parse constants as data, without executing any Windows action script.
foreach($source in $p.sources){
    $tokens=$null;$errors=$null
    $null=[Management.Automation.Language.Parser]::ParseInput([string]$source,[ref]$tokens,[ref]$errors)
    if($errors.Count -gt 0){Fail 'failed' ([string]$errors[0].Message)}
}
# Compile the fixed fixture service in memory; never invoke it or register a service.
Add-Type -TypeDefinition ([string]$p.serviceHelper) -ReferencedAssemblies System.dll,System.ServiceProcess.dll
Emit $true
