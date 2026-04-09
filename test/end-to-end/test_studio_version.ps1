$cliVersion = ((bio --version) -split " ")[1]

if($env:DOCKER_STUDIO_TEST) {
    $bioVersionCmd = "bio studio version -D"
} else {
    $bioVersionCmd = "bio studio version"
}
bio origin key generate $env:BIO_ORIGIN

# call this first to download the studio
Invoke-Expression $bioVersionCmd

#Linux docker studio does not support version command
if($IsWindows -Or !($env:DOCKER_STUDIO_TEST)) {
    Describe "Studio version" {
        It "should match bio cli" {
            (Invoke-Expression $bioVersionCmd) | Should -Match "bio-studio $(($cliVersion -split '/')[0])*"
        }
    }
}

# bio studio run is not yet implemented on macOS (run_studio function
# missing in bio-studio-darwin.sh), so skip this test on macOS.
if (!$IsMacOS) {
    Describe "Studio cli version" {
        It "should match bio cli" {
            (Invoke-StudioRun "bio --version")[-1] | Should -Be "bio $cliVersion"
        }
    }
}
