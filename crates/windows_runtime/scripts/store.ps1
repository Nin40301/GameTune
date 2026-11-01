$uri=[string]$p.uri
if($uri -cnotmatch '^ms-windows-store://pdp/\?PFN=[A-Za-z0-9.-]+_[a-z0-9]{13}$'){Fail 'protected'}
# Only an explicit Reinstall action calls this script. No automatic Store launch.
# Store is interactive: the user explicitly asked to see its product page.
Start-Process -FilePath $uri
Emit $true
