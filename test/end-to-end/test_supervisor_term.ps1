$tempFile = Join-Path ([System.IO.Path]::GetTempPath()) "testpkgstophook.out"
$supLog = New-SupervisorLogFile("test_supervisor_term")
$launcherProc = Start-Supervisor -LogFile $supLog -Timeout 45

Describe "bio sup term" {
    BeforeAll {
        bio origin key generate $env:BIO_ORIGIN
        Invoke-BuildAndInstall testpkgstophook
        Load-SupervisorService "$env:BIO_ORIGIN/testpkgstophook"
        bio sup term
        $supProc = Get-Process "bio-sup"
        $supProc.WaitForExit(5000)
        $launcherProc.WaitForExit(10000)
    }

    It "should terminate launcher" {
        $launcherProc.HasExited | Should -Be $true
    }
    It "should gracefully terminate service run hook" {
        $tempFile | Should -FileContentMatch "run hook is terminating"
    }
    It "should fire stop hook" {
        $tempFile | Should -FileContentMatch "post-stop hook has fired"
    }
}
