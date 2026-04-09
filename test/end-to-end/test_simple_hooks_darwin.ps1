# macOS-specific hooks tests.
# Tests hook extension stripping, hook execution during package install,
# and bad hook extension detection.
# BIO_INTERPRETER_IDENT is set to core/coreutils in setup_environment_darwin.sh
# since core/busybox-static is not available for aarch64-darwin.

Describe "Hook extension stripping and execution" {
    BeforeAll {
        bio origin key generate $env:BIO_ORIGIN
        Invoke-Build hook-extension-plan
        . ./results/last_build.ps1
        $script:pkgIdent = $pkg_ident
        $script:pkgName = ($pkg_ident -split "/")[1]
        $script:pkgPrefix = "/bio/pkgs/$pkg_ident"
    }

    It "strips .sh extension from hook files" {
        "$script:pkgPrefix/hooks/install" | Should -Exist
    }

    It "does not keep the original .sh file" {
        Test-Path "$script:pkgPrefix/hooks/install.sh" | Should -Be $false
    }

    It "runs the install hook successfully on package install" {
        # Install the built package without --ignore-install-hook so the hook runs
        bio pkg install ./results/$pkg_artifact
        "$script:pkgPrefix/INSTALL_HOOK_STATUS" | Should -Exist
        Get-Content "$script:pkgPrefix/INSTALL_HOOK_STATUS" | Should -Be "0"
    }

    It "install hook renders templates and produces stdout" {
        $logPath = "/bio/svc/$($script:pkgName)/logs/install.stdout.log"
        $logPath | Should -Exist
        Get-Content $logPath | Should -Contain "install hook $($script:pkgName)"
    }
}

Describe "Bad hook extension plan" {
    It "fails when there are multiple extensions for the same hook" {
        bio pkg build test/fixtures/bad-hook-extension-plan
        $LASTEXITCODE | Should -Not -Be 0
    }
}
