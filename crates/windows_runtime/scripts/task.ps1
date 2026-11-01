$scheduler=New-Object -ComObject Schedule.Service
$scheduler.Connect()
$folderPath=([string]$p.path).TrimEnd('\')
if($folderPath.Length -eq 0){$folderPath='\'}
try {$folder=$scheduler.GetFolder($folderPath);$task=$folder.GetTask([string]$p.name)}
catch {$exception=$_.Exception;while($exception.InnerException){$exception=$exception.InnerException};if(($exception.HResult -band 0xffff) -in @(2,3)){Fail 'not_found'};throw}
if($p.operation -eq 'capture'){Emit @{type='scheduled_task';enabled=[bool]$task.Enabled;definition=[string]$task.Xml};return}
Require-Admin
if($null -ne $p.expected -and [bool]$task.Enabled -ne [bool]$p.expected.enabled){Fail 'conflict'}
if($p.expected -and $p.expected.definition -and [string]$task.Xml -cne [string]$p.expected.definition){Fail 'conflict'}
if($p.operation -eq 'restore' -and $p.original.definition){
    function Comparable-Task([string]$text){
        $xml=[Xml.XmlDocument]::new()
        $xml.XmlResolver=$null
        $xml.LoadXml($text)
        foreach($node in @($xml.SelectNodes('/*[local-name()="Task"]/*[local-name()="Settings"]/*[local-name()="Enabled"]'))){$null=$node.ParentNode.RemoveChild($node)}
        return $xml.OuterXml
    }
    if((Comparable-Task ([string]$task.Xml)) -cne (Comparable-Task ([string]$p.original.definition))){Fail 'conflict'}
}
if($p.operation -eq 'apply'){$task.Enabled=$false}
elseif($p.operation -eq 'restore'){$task.Enabled=[bool]$p.original.enabled}
else{Fail 'protected'}
Emit $true
