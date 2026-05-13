# Given a fresh install of Biome we should be able to install packages from builder
# Ensure that we don't have any ssl certificates cached that might influence our ability
# to connect.
# Note: since we're testing pre-release this won't be a "pure" fresh install, as we
# have to curlbash install stable first, in order to get the pre-release version.

Describe "Clean bio installation" {
    It "has no root ssl cache" {
        Test-Path /bio/cache/ssl | Should -Be $false
    }
    It "has no user ssl cache" {
        su bio -c "test ! -d ~/.bio/cache/ssl"
        $LASTEXITCODE | Should -Be 0
    }
    It "can talk to builder" {
        if ($IsMacOS) {
            $pkgChannel = "base-2025"
            # core/redis may not exist for aarch64-darwin; use core/nginx instead
            bio pkg install core/nginx --channel $pkgChannel
        } else {
            bio pkg install core/redis --channel stable
        }
        $LASTEXITCODE | Should -Be 0
    }
}
