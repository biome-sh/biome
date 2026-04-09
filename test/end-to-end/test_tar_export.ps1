Remove-Item *.tar.gz

function Get-Ident($pkg, $tar) {
    $ident = tar --list --file $tar | Where-Object { $_ -like "bio/pkgs/$pkg/**/IDENT" }
    if ($null -ne $ident) {
        tar --extract --to-stdout --file $tar $ident
    }
}

Describe "bio pkg export tar core/nginx" {
    bio pkg export tar core/nginx --base-pkgs-channel $env:BIO_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    $version = ((((bio --version) -split " ")[1]) -split "/")[0]
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Includes bio" {
        Get-Ident biome/bio $tar | Should -BeLike "biome/bio/$version/*"
    }
    It "Includes supervisor" {
        Get-Ident biome/bio-sup $tar | Should -BeLike "biome/bio-sup/$version/*"
    }
    It "Includes launcher" {
        Get-Ident biome/bio-launcher $tar | Should -Not -Be $null
    }
}

Describe "bio pkg export tar core/nginx --no-bio-bin" {
    bio pkg export tar core/nginx --no-bio-bin --base-pkgs-channel $env:BIO_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Does not include bio binary directory" {
        $bioBinDir = tar --list --file $tar | Where-Object { $_ -like "bio/bin/*" }
        $bioBinDir | Should -Be $null
    }
    It "Includes supervisor" {
        Get-Ident biome/bio-sup $tar | Should -Not -Be $null
    }
    It "Includes launcher" {
        Get-Ident biome/bio-launcher $tar | Should -Not -Be $null
    }
}

Context "bio pkg export tar core/nginx --no-bio-sup" {
    bio pkg export tar core/nginx --no-bio-sup --base-pkgs-channel $env:BIO_INTERNAL_BLDR_CHANNEL
    $tar = Get-Item core-nginx-*.tar.gz
    It "Creates tarball" {
        $tar | Should -Not -Be $null
    }
    It "Includes nginx" {
        Get-Ident core/nginx $tar | Should -Not -Be $null
    }
    It "Includes bio binary directory" {
        $bioBinDir = tar --list --file $tar | Where-Object { $_ -like "bio/bin/*" }
        $bioBinDir | Should -Not -Be $null
    }
    It "Does not include supervisor" {
        Get-Ident biome/bio-sup $tar | Should -Be $null
    }
    It "Does not include launcher" {
        Get-Ident biome/bio-launcher $tar | Should -Be $null
    }
}
