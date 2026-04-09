$pkg_name="pkg_version"
$pkg_origin="biome-testing"
$pkg_maintainer="The Biome Maintainers <humans@biome.sh>"

function Invoke-SetupEnvironment {
    Push-RuntimeEnv "SomeVar" "SomeValue"
}

function pkg_version {
    "5.5.5"
}

function Invoke-Before {
    Invoke-DefaultBefore
    Set-PkgVersion
}
