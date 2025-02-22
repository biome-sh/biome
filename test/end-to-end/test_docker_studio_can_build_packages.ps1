[Diagnostics.CodeAnalysis.SuppressMessage("PSUseCorrectCasing", '')]
param ()


#  We assume that if the build succeeds (exits 0) we've passed this
# test, and leave more detailed testing to the build output to e2e tests for bio-plan-build
bio origin key generate $env:HAB_ORIGIN

Describe "Studio build" {
    It "builds package" {
        bio pkg build test/fixtures/plan-in-root -D
        $LASTEXITCODE | Should -Be 0
    }

}

Describe "Targeting different refresh channels" {
    It "Can target non default refresh channel" {
        bio pkg build test/fixtures/breakable-refresh-downgrade --refresh-channel LTS-2024 -D
        Set-Content -Path "results/last_build.ps1" -Value ""
        Get-Content "results/last_build.env" | ForEach-Object { Add-Content "results/last_build.ps1" -Value "`$$($_.Replace("=", '="'))`"" }
        . ./results/last_build.ps1
        bio pkg install ./results/$pkg_artifact
        "/hab/pkgs/$pkg_ident/TDEPS" | Should -FileContentMatch "core/glibc/2.36"
    }
}