. $PSScriptRoot\..\bin\shared.ps1

Describe "Get-BioPackagePath" {
    New-Item "TestDrive:\src" -ItemType Directory -Force | Out-Null
    $script:BIO_PKG_PATH = Join-Path (Get-PSDrive TestDrive).Root "bio\pkgs"
    New-Item -ItemType Directory $BIO_PKG_PATH
    $pkg_path = Join-Path $BIO_PKG_PATH "core\blah\0.1.0\111"
    $script:pkg_all_deps_resolved = @($pkg_path)

    It "finds path for origin/pkg" {
        Get-BioPackagePath "core/blah" | Should -Be $pkg_path
    }

    It "finds path for package name" {
        Get-BioPackagePath "blah" | Should -Be $pkg_path
    }

    It "finds path for package name/version" {
        Get-BioPackagePath "blah/0.1.0" | Should -Be $pkg_path
    }

    It "errors if there is no package found" {
        Get-BioPackagePath "blah/0.11.0" | Should -Be $null
    }
}
