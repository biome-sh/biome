Describe "ctl_gateway" {
    It "should NOT be able to issue a remote-sup call with the wrong secret key" {
        $env:BIO_CTL_SECRET=(bio sup secret generate)
        bio svc status --remote-sup "alpha.biome.dev"
        $LASTEXITCODE | Should -Not -Be 0
    }
}
