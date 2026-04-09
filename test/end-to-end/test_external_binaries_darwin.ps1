# macOS-specific external binaries tests.
# Tests that bio can find and execute binaries from installed packages.
# Container and Tar exporters are not compiled for macOS, so we test
# bio pkg exec instead, which is the underlying mechanism for running
# external package binaries.

$channel = "aarch64-darwin"

Describe "`bio` correctly executes external binaries" {
    BeforeAll {
        bio pkg install core/gzip --channel $channel
    }

    It "`bio pkg exec` runs gzip from the package" {
        # Cannot use --version/--help/-V because clap intercepts those flags
        # before they reach the actual binary via execvp().
        # Instead, compress a file and verify the output exists.
        "hello biome" | Set-Content -Path /tmp/bio_test_gzip_input.txt
        bio pkg exec core/gzip gzip /tmp/bio_test_gzip_input.txt
        $LASTEXITCODE | Should -Be 0
        "/tmp/bio_test_gzip_input.txt.gz" | Should -Exist
        Remove-Item -Force /tmp/bio_test_gzip_input.txt.gz
    }

    It "`bio pkg exec` can decompress data" {
        # Verify gzip decompression works via pipe (gzip -d reads stdin)
        "test data" | Set-Content -Path /tmp/bio_test_gzip2.txt
        bio pkg exec core/gzip gzip /tmp/bio_test_gzip2.txt
        bio pkg exec core/gzip gzip -d /tmp/bio_test_gzip2.txt.gz
        $LASTEXITCODE | Should -Be 0
        "/tmp/bio_test_gzip2.txt" | Should -Exist
        Get-Content /tmp/bio_test_gzip2.txt | Should -Be "test data"
        Remove-Item -Force /tmp/bio_test_gzip2.txt
    }

    It "`bio pkg exec` with nonexistent package fails" {
        bio pkg exec core/nonexistent-pkg-12345 some-binary 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "`bio pkg exec` with nonexistent binary fails" {
        bio pkg exec core/gzip nonexistent-binary-12345 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }

    It "`bio pkg export` with bad exporter fails gracefully" {
        bio pkg export a_bad_exporter --help 2>&1
        $LASTEXITCODE | Should -Not -Be 0
    }
}
