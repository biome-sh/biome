[Diagnostics.CodeAnalysis.SuppressMessage("PSUseCorrectCasing", '')]
param ()


#  We assume that if the build succeeds (exits 0) we've passed this
# test, and leave more detailed testing to the build output to e2e tests for bio-plan-build
bio origin key generate $env:BIO_ORIGIN

Describe "Studio build" {
    It "builds package" {
        bio pkg build test/fixtures/plan-in-root -D
        $LASTEXITCODE | Should -Be 0
    }
}
