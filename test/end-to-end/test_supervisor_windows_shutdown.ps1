bio pkg install core/nginx --channel stable
bio pkg install core/windows-service

Describe "Clean Biome Shutdown" {
    Start-Service Biome
    bio pkg install core/nginx
    Wait-Supervisor -Timeout 45
    bio svc load core/nginx
    Wait-SupervisorService nginx -Timeout 20
    It "Starts running nginx" {
        # This will error with a 403 because nginx is not running any sites
        try  { Invoke-WebRequest "http://localhost" }
        catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
        [string]$headers.Server | Should -BeLike "nginx/*"
    }
    It "Stops all processes" {
        Stop-Service Biome
        Get-Process bio-sup -ErrorAction SilentlyContinue | Should -Be $null
        Get-Process bio-launch -ErrorAction SilentlyContinue | Should -Be $null
        Get-Process nginx -ErrorAction SilentlyContinue | Should -Be $null
    }
}
