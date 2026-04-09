bio pkg install core/windows-service

Describe "Terminate Launcher" {
    Start-Service Biome
    Wait-Supervisor -Timeout 45
    Get-Process bio-launch | Stop-Process -Force
    Start-Sleep -Seconds 2

    It "Stops service" {
        (Get-Service Biome).Status | Should -Be "Stopped"
    }
}

Describe "Bogus Launcher args" {
    Stop-Service Biome
    (Get-Content c:/bio/svc/windows-service/BioService.dll.config).replace('--no-color', '--poop') |
        Set-Content c:/bio/svc/windows-service/BioService.dll.config
    # Depending on random timing issues, starting the service will either
    # fail with an exception or succeed and immediately stop itself
    # either way, we just want to move on and validate that it is not running
    Start-Service Biome -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 2

    It "Stops service" {
        (Get-Service Biome).Status | Should -Be "Stopped"
    }
}
