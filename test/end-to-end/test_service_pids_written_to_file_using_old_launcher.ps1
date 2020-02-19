# When using an old Launcher that cannot provide service PIDs to the
# Supervisor, we should still be using PID files for individual
# services.

Describe "Using a Launcher that cannot provide service PIDs" {
    # This was the last stable Linux Launcher prior to the Launcher
    # being able to provide service PIDs to the Supervisor directly.
    bio pkg install biome/bio-launcher/12605/20191112144831

    Start-Supervisor -Timeout 20
    Load-SupervisorService -PackageName "core/redis" -Timeout 20
    Wait-Process redis-server -Timeout 10

    It "should create PID file" {
        Test-Path "/hab/svc/redis/PID" | Should -Be $true
    }

    Context "Supervisor is restarted" {
        $supProc = Get-Process bio-sup
        $redisProc = Get-Process redis-server
        Restart-Supervisor
        Wait-Process redis-server -Timeout 10
        $newSupProc = Get-Process bio-sup
        $newRedisProc = Get-Process redis-server

        It "starts a new supervisor process" {
            $supProc.Id | Should -Not -Be $newSupProc.Id
        }
        It "runs the same redis process" {
            $redisProc.Id | Should -Be $newRedisProc.Id
        }
    }
}