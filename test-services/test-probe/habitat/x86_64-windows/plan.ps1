$pkg_name = "test-probe"
$pkg_origin = "biome-testing"
$pkg_version = "0.1.0"
$pkg_maintainer = "The Biome Maintainers <humans@biome.sh>"
$pkg_license = @("Apache-2.0")
$pkg_bin_dirs = @("bin")
$pkg_deps=@(
    "core/visual-cpp-redist-2022"
)
$pkg_build_deps = @(
    "core/rust",
    "core/windows-11-sdk",
    "core/visual-build-tools-2022"
)

$pkg_binds_optional = @{
    thing_with_a_port = "port"
}

function Invoke-Prepare {
    $env:CARGO_TARGET_DIR = Join-Path -Path "$HAB_CACHE_SRC_PATH" -ChildPath "$pkg_dirname"
    Write-BuildLine "Setting env:CARGO_TARGET_DIR=$env:CARGO_TARGET_DIR"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --verbose
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item "$env:CARGO_TARGET_DIR/debug/test-probe.exe" "$pkg_prefix/bin/test-probe.exe"
    Copy-Item "$PLAN_CONTEXT/health_exit" "$pkg_prefix"
}
