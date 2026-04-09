# A simple test assertion that running `bio sup --help` will not
# attempt to install `biome/bio-sup` if that pkg is not present.

$env:FS_ROOT = (Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName()))
$env:BIO_SUP_BINARY = $null

Describe "bio sup --help" {
    bio sup --help | Out-Null

    It "runs successfully" {
        $LASTEXITCODE | Should -Be 0
    }
    It "does not install the supervisor package" {
        "$env:FS_ROOT/bio/pkgs/biome/bio-sup" | Should -Not -Exist
    }
}
