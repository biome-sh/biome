Describe "`bio` correctly executes external binaries" {
    It "container exporter help" {
        $le = "`n"
        if ($IsWindows) {
            $le = "`r`n"
        }
        $out = (bio pkg export container --help | Out-String)
        $LastExitCode | Should -Be 0
        $out | Should -BeLike "*Creates a container image from a set of Biome packages (and optionally pushes to a remote${le}repository)*"

        $out = (bio pkg export docker --help | Out-String)
        $LastExitCode | Should -Be 0
        $out | Should -BeLike "*Creates a container image from a set of Biome packages (and optionally pushes to a remote${le}repository)*"
    }

    It "tar exporter help" {
        $out = bio pkg export tar --help
        $LastExitCode | Should -Be 0
        "Creates a tar package from a Biome package" | Should -BeIn $out
    }

    It "`bio pkg export` with bad exporter" {
        bio pkg export a_bad_exporter --help
        $LastExitCode | Should -Be 1
    }

    It "`bio sup --version` correctly reports version" {
        # Install an use an old supervisor to ensure version match
        Invoke-NativeCommand bio pkg install "biome/bio-sup/1.6.56"
        $env:HAB_SUP_BINARY = "$(bio pkg path biome/bio-sup/1.6.56)/bin/bio-sup"
        $out = bio sup --version | Join-String
        $out | Should -BeLike "*1.6.56*"
        $env:HAB_SUP_BINARY = ""
    }
}

