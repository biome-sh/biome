BeforeAll {
    . $PSScriptRoot\..\bin\shared.ps1

    # Describe/Context bodies run during Pester's Discovery phase, before the built-in
    # TestDrive: PSDrive is created (that only exists during the Run phase). So we create
    # our own PSDrive rooted at a fresh temp directory here instead, mirroring what
    # TestDrive does, so it is available immediately.
    #
    # This, and all other setup below, lives in BeforeAll blocks (rather than directly
    # in the Describe body) because Pester v5 only runs Describe/Context bodies once,
    # during Discovery. Functions and plain variables set there are not visible to It
    # blocks, which run later, during the separate Run phase. BeforeAll, on the other
    # hand, is specifically designed to share its functions/variables with the It
    # blocks in the same (or nested) block.
    function New-PlanBuildTestDrive {
        if (Get-PSDrive -Name PlanBuildTestDrive -ErrorAction SilentlyContinue) {
            Remove-PSDrive -Name PlanBuildTestDrive -Force
        }
        $root = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid())
        New-Item $root -ItemType Directory -Force | Out-Null
        New-PSDrive -Name PlanBuildTestDrive -PSProvider FileSystem -Root $root -Scope Global | Out-Null
    }
}

Describe "Get-BioPackagePath" {
    BeforeAll {
        New-PlanBuildTestDrive
        New-Item "PlanBuildTestDrive:\src" -ItemType Directory -Force | Out-Null
        $script:BIO_PKG_PATH = Join-Path (Get-PSDrive PlanBuildTestDrive).Root "bio\pkgs"
        New-Item -ItemType Directory $BIO_PKG_PATH
        $pkg_path = Join-Path $BIO_PKG_PATH "core\blah\0.1.0\111"
        $script:pkg_all_deps_resolved = @($pkg_path)
    }

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
        {
            $ErrorActionPreference = "Stop"
            Get-BioPackagePath "blah/0.11.0"
        } | Should -Throw -ExpectedMessage "*Get-BioPackagePath 'blah/0.11.0' did not find a suitable installed package*"
    }
}
