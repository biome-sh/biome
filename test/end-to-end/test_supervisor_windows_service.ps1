bio pkg install core/windows-service

Describe "Biome Windows Service" {
    It "Starts the service" {
        Start-Service Biome
        Wait-Supervisor -Timeout 45
        (Invoke-WebRequest "http://localhost:9631/butterfly" -UseBasicParsing).StatusCode | Should -Be 200
    }
    It "Stops the service" {
        Stop-Service Biome
        Get-Process bio-sup -ErrorAction SilentlyContinue | Should -Be $null
        Get-Process bio-launch -ErrorAction SilentlyContinue | Should -Be $null
    }

    Context "Running a service with current launcher" {
        BeforeAll {
            Stop-Service Biome
            Remove-Item "c:\bio\svc\windows-service\logs\biome.log"
            Start-Service Biome
            Wait-Supervisor -Timeout 45
            bio svc load core/nginx --channel stable
            Wait-SupervisorService nginx
        }

        It "Runs nginx" {
            # This will error with a 403 because nginx is not running any sites
            try  { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "nginx/*"
        }

        It "Runs service as system" {
            "c:\bio\svc\windows-service\logs\biome.log" | Should -FileContentMatch "Starting service as user=system"
        }

        AfterAll {
            bio svc unload core/nginx
            Start-Sleep -Seconds 3 # tears
            Stop-Service Biome
        }
    }

    Context "Running a service with older launcher" {
        BeforeAll {
            Stop-Service Biome
            Remove-Item "c:\bio\svc\windows-service\logs\biome.log"
            # This was the last stable Windows Launcher prior to the Launcher
            # being able to provide version numbers to the Supervisor directly.
            bio pkg install core/bio-launcher/13927/20200618210540
            $launcherPath = "$(bio pkg path core/bio-launcher/13927/20200618210540)\bin\bio-launch.exe"
            $configPath = "c:\bio\svc\windows-service\BioService.dll.config"
            [xml]$configXml = Get-Content $configPath
            $launcherPathNode = ($configXml.configuration.appSettings.SelectNodes("add[@key='launcherPath']"))[0]
            $launcherPathNode.SetAttribute("value", $launcherPath) | Out-Null
            $configXml.Save($configPath)
            Start-Service Biome
            Wait-Supervisor -Timeout 45
            bio svc load core/nginx --channel stable
            Wait-SupervisorService nginx
        }

        It "Runs nginx" {
            # This will error with a 403 because nginx is not running any sites
            try  { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "nginx/*"
        }

        It "Runs service as machine user" {
            $lowerComputerName = $env:COMPUTERNAME.ToLower()
            "c:\bio\svc\windows-service\logs\biome.log" | Should -FileContentMatch "Starting service as user=$lowerComputerName\`$"
        }

        AfterAll {
            bio svc unload core/nginx
            Stop-Service Biome
        }
    }
}
