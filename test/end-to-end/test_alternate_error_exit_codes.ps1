$PkgName = "custom-hook-exit-code"
$PkgOrigin = "biome-testing"
$PkgIdent = "$PkgOrigin/$PkgName"
$Env:BIO_ORIGIN = $PkgOrigin
bio origin key generate $PkgOrigin
Invoke-Build $PkgName
. ./results/last_build.ps1

Describe "install and uninstall hook error codes are propogated" {
    It "`bio pkg install` exits with install hook exit code" {
        $Env:CUSTOM_HOOK_EXIT_CODE = 42
        bio pkg install ./results/$pkg_artifact
        $LastExitCode | Should -Be $Env:CUSTOM_HOOK_EXIT_CODE
    }

    It "`bio pkg install` succeeds" {
        $Env:CUSTOM_HOOK_EXIT_CODE = ""
        bio pkg install ./results/$pkg_artifact
        $LastExitCode | Should -Be 0
    }

    It "`bio pkg uninstall` exits with uninstall hook exit code" {
        $Env:CUSTOM_HOOK_EXIT_CODE = 97
        bio pkg uninstall $PkgIdent
        $LastExitCode | Should -Be $Env:CUSTOM_HOOK_EXIT_CODE
    }

    It "`bio pkg uninstall` succeeds" {
        $Env:CUSTOM_HOOK_EXIT_CODE = ""
        bio pkg uninstall $PkgIdent
        $LastExitCode | Should -Be 0
    }

    AfterAll {
        bio pkg uninstall --keep-latest 0 $PkgIdent
    }
}
