# A simple test that the launcher restarts a supervisor when it exits abruptly
# The one optional argument set the exit code of the supervisor (default: 1).
# By default this runs against the installed biome binaries. To override and
# test locally-built code, set overrides in the environment of the script.
# See https://github.com/biome-sh/biome/blob/master/BUILDING.md#testing-changes

Describe "Supervisor restarts on abrupt exit" {
    $exit_file = New-TemporaryFile
    $env:BIO_FEAT_TEST_EXIT = $exit_file
    $supLog = New-SupervisorLogFile("supervisor_restarts_on_abrupt_exit")
    $launcher_proc = Start-Supervisor -Timeout 45 -LogFile $supLog
    $supervisor_proc = Get-Process bio-sup
    Set-Content $exit_file -Value 1

    $testScript = { $supervisor_proc.HasExited }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to exit" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45

    $testScript = { Get-Process bio-sup -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to be restarted" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45

    $new_supervisor_proc = Get-Process bio-sup
    $new_launcher_proc = Get-Process bio-launch

    It "will not exit the launcher" {
        $launcher_proc.HasExited | Should -Be $false
    }
    It "will spawn a new supervisor" {
        $new_supervisor_proc.Id | Should -Not -Be $supervisor_proc.Id
    }
    It "will not spawn a new launcher" {
        $new_launcher_proc.Id | Should -Be $launcher_proc.Id
    }

    AfterAll {
        Stop-Supervisor
    }
}

Describe "Supervisor restarts on 'bio sup restart'" {
    $supLog = New-SupervisorLogFile("supervisor_restarts_on_bio_sup_restart")
    $launcher_proc = Start-Supervisor -Timeout 45 -LogFile $supLog
    $supervisor_proc = Get-Process bio-sup
    Invoke-NativeCommand bio sup restart

    $testScript = { $supervisor_proc.HasExited }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to exit" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45

    $testScript = { Get-Process bio-sup -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to be restarted" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45

    $new_supervisor_proc = Get-Process bio-sup
    $new_launcher_proc = Get-Process bio-launch

    It "will not exit the launcher" {
        $launcher_proc.HasExited | Should -Be $false
    }
    It "will spawn a new supervisor" {
        $new_supervisor_proc.Id | Should -Not -Be $supervisor_proc.Id
    }
    It "will not spawn a new launcher" {
        $new_launcher_proc.Id | Should -Be $launcher_proc.Id
    }

    AfterAll {
        Stop-Supervisor
    }
}
