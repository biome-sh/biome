# macOS-specific pkg uninstall tests.

$pkg = "core/sqlite"
$nginxPkg = "core/nginx"
$channel = "aarch64-darwin"
$env:BIO_NOCOLORING = "true"

Describe "pkg uninstall (macOS)" {
    BeforeAll {
        bio pkg install "$pkg" --channel $channel
        bio pkg install "$nginxPkg" --channel $channel
    }

    It "lists installed package" {
        $list = bio pkg list "$pkg"
        $list | Should -Not -BeNullOrEmpty
    }

    It "uninstalls the package" {
        bio pkg uninstall "$pkg"
        $LASTEXITCODE | Should -Be 0
    }

    It "package is removed after uninstall" {
        bio pkg list "$pkg" | Should -BeExactly @()
    }

    It "dry run does not remove the package" {
        bio pkg install "$nginxPkg" --channel $channel
        bio pkg uninstall -d "$nginxPkg"
        bio pkg list "$nginxPkg" | Should -Not -BeNullOrEmpty
    }

    It "uninstalls with keep-latest=0" {
        bio pkg uninstall --keep-latest=0 "$nginxPkg"
        $LASTEXITCODE | Should -Be 0
        bio pkg list "$nginxPkg" | Should -BeExactly @()
    }

    AfterAll {
        bio pkg uninstall --keep-latest=0 "$pkg" 2>$null
        bio pkg uninstall --keep-latest=0 "$nginxPkg" 2>$null
    }
}
