$pkg_name="runtime-env-plan"
$pkg_origin="biome-testing"
$pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
$pkg_version="0.0.0"

function Invoke-SetupEnvironment {
    Set-RuntimeEnv "SOME_VAR" "SOME_VAL"
}
