# This test is designed to catch the regression described in
# https://github.com/habitat-sh/habitat/issues/6771
#
# When a user runs `bio studio enter` for the first time after installing a
# new Biome release, the `biome/bio-studio` package won't be present on the
# system and the cli will automatically download and install the appropirate
# package. Since we always install the studio as part of our build process to
# ensure we're using the correct version, this behavior needs to be exercised
# as its own test.

# Ensure there are no studios installed
if(Test-Path /bio/pkgs/biome/bio-studio) {
    bio pkg uninstall biome/bio-studio
}

Describe "Studio install" {
    # 'studio enter' requires a signing key to be present for the current origin
    bio origin key generate "$env:BIO_ORIGIN"

    It "can create a new studio when no studio package is installed" {
        bio studio new
        $LASTEXITCODE | Should -Be 0
    }
}
