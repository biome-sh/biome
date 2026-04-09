$pkg_name="after_failure_plan"
$pkg_origin="biome-testing"
$pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
$pkg_version="0.0.0"

function Invoke-Build {
    throw "build is broken"
}

function Invoke-AfterSuccess {
    Write-Host "I am a success"
}

function Invoke-AfterFailure {
    Write-Host "I am a failure"
}
