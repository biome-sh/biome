$pkg_name="runtime-env-consumer-plan"
$pkg_origin="biome-testing"
$pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
$pkg_version="0.0.0"
$pkg_build_deps=@("biome-testing/runtime-env-plan")

function Invoke-Build {
    if($env:SOME_VAR -ne "SOME_VAL") {
        Write-Error "build failed"
    }
}