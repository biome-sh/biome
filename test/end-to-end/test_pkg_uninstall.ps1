$pkg = "core/redis"
$pkgs = @(
    "$pkg/3.2.3/20161102201135",
    "$pkg/3.2.4/20161104175435",
    "$pkg/3.2.4/20161210004233",
    "$pkg/3.2.4/20161215055911",
    "$pkg/3.2.4/20170103160441",
    "$pkg/3.2.4/20170106011058",
    "$pkg/4.0.10/20180801003001",
    "$pkg/4.0.10/20190116005049",
    "$pkg/4.0.14/20200319184753",
    "$pkg/4.0.14/20200319200053"
)
$nginxPkg = "core/nginx"
$env:BIO_NOCOLORING="true"

bio origin key generate $env:BIO_ORIGIN

# Start the supervisor and load nginx
$job = Start-Job { bio sup run }
Wait-Supervisor -Timeout 120

Describe "pkg uninstall" {
    It "installs core/redis" {
        foreach ($p in $pkgs) {
            Write-Host "Installing package $p"
            Write-Host (bio pkg install "$p" | Out-String)
        }
        bio pkg list "$pkg" | Should -BeExactly $pkgs
    }

    It "uninstall a single package" {
        bio pkg uninstall "$pkg"
        bio pkg list "$pkg" | Should -BeExactly $pkgs[0..8]
    }

    It "uninstall all but the two latest of version 3.2.4" {
        Write-Host (bio pkg uninstall --keep-latest=2 "$pkg/3.2.4" | Out-String)
        bio pkg list "$pkg" | Should -BeExactly $pkgs[,0+4..8]
    }

    It "dry run should do nothing" {
        bio pkg uninstall -d "$pkg"
        bio pkg uninstall -d --keep-latest=1 "$pkg"
        bio pkg list "$pkg" | Should -BeExactly $pkgs[,0+4..8]
    }

    It "uninstall with a fully qualified ident" {
        bio pkg uninstall --keep-latest=3 "$pkg/3.2.3/20161102201135" | Should -Contain "… Skipping Only 1 packages installed"
        bio pkg uninstall --keep-latest=0 "$pkg/3.2.3/20161102201135"
        bio pkg list "$pkg" | Should -BeExactly $pkgs[4..8]
    }

    It "uninstall all but the three latest" {
        bio pkg uninstall --keep-latest=3 "$pkg"
        bio pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "uninstall does nothing if keeping all" {
        bio pkg uninstall --keep-latest=10 "$pkg" | Should -Contain "… Skipping Only 3 packages installed"
        bio pkg list "$pkg" | Should -BeExactly $pkgs[6..8]
    }

    It "cannot uninstall a package loaded by the supervisor or any of its dependencies" {
        # Install nginx
        bio pkg install $nginxPkg --channel stable

        # Get list of nginx dependencies before loading
        $initialDeps = @(bio pkg dependencies $nginxPkg --transitive)

        # Load nginx service
        bio svc load $nginxPkg
        Wait-SupervisorService nginx -Timeout 20

        # Attempt to uninstall nginx
        Write-Host (bio pkg uninstall $nginxPkg | Out-String)

        # Verify nginx is still installed
        bio pkg list $nginxPkg | Should -Not -BeNullOrEmpty

        # Verify all nginx dependencies are still installed
        foreach($dep in $initialDeps) {
            Write-Host "Checking dependency $dep"
            bio pkg list $dep | Should -Not -BeNullOrEmpty
        }
    }

    It "uninstall all" {
        bio pkg uninstall --keep-latest=0 "$pkg"
        bio pkg list "$pkg" | Should -BeExactly @()
        bio svc unload $nginxPkg
        Wait-SupervisorServiceUnload nginx -Timeout 20
        bio pkg uninstall --keep-latest=0 "$nginxPkg"
        bio pkg list "$nginxPkg" | Should -BeExactly @()
        # we know this is a dep of nginx and no other reverse dependencies
        bio pkg list "core/libedit" | Should -BeExactly @()
    }

    It "uninstalls all package deps and transitive deps" {
        Invoke-BuildAndInstall dep-pkg-1
        Invoke-BuildAndInstall dep-pkg-2
        Invoke-BuildAndInstall dep-pkg-4

        bio pkg uninstall $env:BIO_ORIGIN/dep-pkg-4

        bio pkg list "$env:BIO_ORIGIN/dep-pkg-4" | Should -BeExactly @()
        bio pkg list "$env:BIO_ORIGIN/dep-pkg-2" | Should -BeExactly @()
        bio pkg list "$env:BIO_ORIGIN/dep-pkg-1" | Should -BeExactly @()
    }

    It "Leaves package with reverse deps that are not being uninstalled" {
        Invoke-BuildAndInstall dep-pkg-1
        Invoke-BuildAndInstall dep-pkg-2
        Invoke-BuildAndInstall dep-pkg-3
        Invoke-BuildAndInstall dep-pkg-4

        bio pkg uninstall $env:BIO_ORIGIN/dep-pkg-4

        bio pkg list "$env:BIO_ORIGIN/dep-pkg-4" | Should -BeExactly @()
        bio pkg list "$env:BIO_ORIGIN/dep-pkg-3" | Should -Not -BeNullOrEmpty
        bio pkg list "$env:BIO_ORIGIN/dep-pkg-2" | Should -Not -BeNullOrEmpty
        bio pkg list "$env:BIO_ORIGIN/dep-pkg-1" | Should -Not -BeNullOrEmpty
    }

    AfterAll {
        Stop-Job -Job $job
        Remove-Job -Job $job

        bio pkg uninstall --keep-latest=0 "$pkg"
        bio pkg uninstall --keep-latest=0 "$nginxPkg"
    }
}
