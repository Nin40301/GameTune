$exception = $_.Exception
while($exception.InnerException){$exception=$exception.InnerException}
if ($exception -is [UnauthorizedAccessException] -or (($exception.HResult -band 0xffff) -eq 5)) { Fail 'access_denied' }
Fail 'failed' ('Windows operation failed: ' + $exception.Message)
