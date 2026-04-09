$env:BIO_NOCOLORING="true"

Describe "pkg uninstall with uninstall hook" {
    $log = Join-Path -Path $env:SystemDrive -ChildPath bio -AdditionalChildPath @("svc", "uninstall-hook", "logs", "uninstall.stdout.log")
    BeforeEach {
        bio origin key generate $env:BIO_ORIGIN
        Invoke-BuildAndInstall uninstall-hook1
        Invoke-BuildAndInstall uninstall-hook2
    }

    It "does not run uninstall hook when only removing latest package" {
        bio pkg uninstall $env:BIO_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -Not -Exist
    }

    It "does not run uninstall hook when ignoring uninstall hook" {
        bio pkg uninstall --keep-latest=0 --ignore-uninstall-hook $env:BIO_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -Not -Exist
    }

    It "runs latest uninstall hook when uninstalling all packages" {
        bio pkg uninstall --keep-latest=0 $env:BIO_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 0
        $log | Should -FileContentMatchExactly "uninstalling 0.2.0"
    }

    It "exits with exit code in hook" {
        Invoke-BuildAndInstall uninstall-hook3
        bio pkg uninstall --keep-latest=0 $env:BIO_ORIGIN/uninstall-hook
        $LASTEXITCODE | Should -Be 1
        $log | Should -FileContentMatchExactly "uninstalling 0.3.0"
    }

    It "does not run uninstall hook of uninstalled dependency when there are newer versions on disk" {
        Invoke-BuildAndInstall dep-uninstall-hook
        $result = bio pkg list $env:BIO_ORIGIN/uninstall-hook/0.1.0
        $result.Length | Should -BeGreaterThan 0
        bio pkg uninstall $env:BIO_ORIGIN/dep-uninstall-hook
        $log | Should -Not -Exist
        bio pkg list $env:BIO_ORIGIN/uninstall-hook/0.1.0 | Should -BeExactly @()
    }

    AfterEach {
        bio pkg uninstall --keep-latest=0 $env:BIO_ORIGIN/uninstall-hook
        Remove-Item $log -ErrorAction SilentlyContinue
    }
}
