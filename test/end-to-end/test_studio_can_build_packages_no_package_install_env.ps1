bio origin key generate $env:BIO_ORIGIN

Context "NO_INSTALL_DEPS env variable is set" {
    BeforeEach {
        bio pkg uninstall core/redis
    }

    Describe "NO_INSTALL_DEPS Environment is Set to True"  {
        $env:BIO_STUDIO_SECRET_NO_INSTALL_DEPS = "true"

        It "When Package Dependency is already installed" {
            bio pkg install core/redis

            bio pkg build test/fixtures/no-install-deps
            $LASTEXITCODE | Should -Be 0
        }

        It "When Package Dependency is not installed" {

            bio pkg build test/fixtures/no-install-deps
            $LASTEXITCODE | Should -Not -Be 0
        }
    }

    Describe "NO_INSTALL_DEPS Environment is set to False" {
        $env:BIO_STUDIO_SECRET_NO_INSTALL_DEPS = "false"

        It "When Package Dependency is already installed" {
            bio pkg install core/redis
            bio pkg build test/fixtures/no-install-deps
            $LASTEXITCODE | Should -Be 0
        }

        It "When Package Dependency is not installed" {
            bio pkg build test/fixtures/no-install-deps
            $LASTEXITCODE | Should -Be 0
        }
    }

    Describe "NO_INSTALL_DEPS Environment is not set" {
        $env:BIO_STUDIO_SECRET_NO_INSTALL_DEPS = ""

        It "When Package Dependency is not installed" {
            bio pkg build test/fixtures/no-install-deps
            $LASTEXITCODE | Should -Be 0
        }
    }
}
