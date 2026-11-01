$name = [string]$p.name
if ($name -cnotmatch '^[A-Za-z0-9_.-]{1,160}$') { Fail 'protected' }
try{$service=Get-Service -Name $name -ErrorAction Stop}
catch{if($_.CategoryInfo.Category -eq 'ObjectNotFound'){Fail 'not_found'};throw}
$base=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine,[Microsoft.Win32.RegistryView]::Registry64)
try {
    $key=$base.OpenSubKey(('SYSTEM\CurrentControlSet\Services\'+$name),($p.operation -ne 'capture'))
    if(-not $key){Fail 'not_found'}
    try {
        $start=[uint32]$key.GetValue('Start')
        if($key.GetValueKind('Start') -ne [Microsoft.Win32.RegistryValueKind]::DWord){Fail 'failed' 'Unsupported service start value type'}
        if($start -notin @(2,3,4)){Fail 'protected'}
        $delayed=$key.GetValue('DelayedAutoStart',$null)
        if($null -ne $delayed -and $key.GetValueKind('DelayedAutoStart') -ne [Microsoft.Win32.RegistryValueKind]::DWord){Fail 'failed' 'Unsupported delayed auto-start value type'}
        if($null -ne $delayed -and $delayed -notin @(0,1)){Fail 'failed' 'Unsupported delayed auto-start value'}
        if($service.Status -notin @('Running','Stopped')){Fail 'conflict'}
        $snapshot=@{type='service';start_mode=$start;delayed_auto_start=$(if($null -eq $delayed){$null}else{[bool]$delayed});running=($service.Status -eq 'Running')}
        if($p.operation -eq 'capture'){Emit $snapshot;return}
        Require-Admin
        $expected=$p.expected
        if($expected -and (($start -ne $expected.start_mode) -or ($snapshot.running -ne $expected.running) -or ($snapshot.delayed_auto_start -cne $expected.delayed_auto_start))){Fail 'conflict'}
        if($p.operation -eq 'apply'){
            # Never cascade a stop to dependent services.
            if(@($service.DependentServices | Where-Object {$_.Status -ne 'Stopped'}).Count -gt 0){Fail 'conflict'}
            if($service.Status -eq 'Running'){$service.Stop();$service.WaitForStatus([ServiceProcess.ServiceControllerStatus]::Stopped,[TimeSpan]::FromSeconds(20))}
            $mode=if($p.mode -eq 'manual'){'Manual'}elseif($p.mode -eq 'disabled'){'Disabled'}else{Fail 'protected'}
            Set-Service -Name $name -StartupType $mode
        }elseif($p.operation -eq 'restore'){
            $original=$p.original
            $mode=switch([uint32]$original.start_mode){2{'Automatic'}3{'Manual'}4{'Disabled'}default{Fail 'protected'}}
            Set-Service -Name $name -StartupType $mode
            if($null -eq $original.delayed_auto_start){$key.DeleteValue('DelayedAutoStart',$false)}else{$key.SetValue('DelayedAutoStart',[int][bool]$original.delayed_auto_start,[Microsoft.Win32.RegistryValueKind]::DWord)}
            $key.Flush()
            $service.Refresh()
            if($original.running -and $service.Status -ne 'Running'){
                if(@($service.ServicesDependedOn | Where-Object {$_.Status -ne 'Running'}).Count -gt 0){Fail 'conflict'}
                if([uint32]$original.start_mode -eq 4){Set-Service -Name $name -StartupType Manual}
                try{$service.Start();$service.WaitForStatus([ServiceProcess.ServiceControllerStatus]::Running,[TimeSpan]::FromSeconds(20))}
                finally{if([uint32]$original.start_mode -eq 4){Set-Service -Name $name -StartupType Disabled}}
            }
            elseif(-not $original.running -and $service.Status -ne 'Stopped'){
                if(@($service.DependentServices | Where-Object {$_.Status -ne 'Stopped'}).Count -gt 0){Fail 'conflict'}
                $service.Stop();$service.WaitForStatus([ServiceProcess.ServiceControllerStatus]::Stopped,[TimeSpan]::FromSeconds(20))
            }
        }else{Fail 'protected'}
        Emit $true
    }finally{$key.Dispose()}
}finally{$base.Dispose();$service.Dispose()}
