function Stop-ProcessTree($Id) {
    Get-CimInstance Win32_Process | Where-Object { $_.ParentProcessId -eq $Id } | ForEach-Object { Stop-ProcessTree $_.ProcessId }
    Stop-Process -Id $Id -ErrorAction SilentlyContinue
}

Describe "bio studio enter with studio_profile.ps1" {
    BeforeAll {
        Set-Content studio_profile.ps1 -Value "write-host 'hohoho';kill `$PID"
        bio origin key generate $env:HAB_ORIGIN
    }
    Context "No HAB_STUDIO_NOPROFILE set" {
        $env:HAB_STUDIO_NOPROFILE = $null

        It "sources studio_profile.ps1" {
            if($env:DOCKER_STUDIO_TEST) {
                $bioEnterCmd = "bio studio enter -D"
            } else {
                $bioEnterCmd = "bio studio enter"
            }
            $result = Invoke-Expression $bioEnterCmd
            $result[-1] | Should -Be "hohoho"
        }
    }
    Context "HAB_STUDIO_NOPROFILE is set" {
        $env:HAB_STUDIO_NOPROFILE = $true

        It "does not source studio_profile.ps1" {
            $studioArgs = @("studio", "enter")
            if($env:DOCKER_STUDIO_TEST) {
                $env:HAB_DOCKER_OPTS = "-l buildkitejob=$env:BUILDKITE_JOB_ID"
                $studioArgs += "-D"
            }
            $procArgs = @{
                FilePath               = "bio"
                ArgumentList           = $studioArgs
                RedirectStandardOutput = "out.log"
                PassThru               = $true
            }
            $proc = Start-Process @procArgs
            Wait-PathIncludesContent -Path out.log -Content "[HAB-STUDIO] Biome:\src>"
            Stop-ProcessTree $proc.Id
        }
    }
    AfterAll {
        Remove-Item studio_profile.ps1
        # For some reason buildkite runs complain that the log file is in use by another
        # process even though we have just stopped the entire process tree. Local
        # runs do not have this problem. Because buildkite runs occur in a container,
        # its not a big deal if the file is not deleted.
        Remove-Item out.log -Force -ErrorAction SilentlyContinue
        if($env:DOCKER_STUDIO_TEST) {
            docker ps -q --filter "label=buildkitejob=$env:BUILDKITE_JOB_ID" | ForEach-Object { docker stop $_ }
        }
    }
}
