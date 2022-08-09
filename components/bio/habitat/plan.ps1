$pkg_name = "bio"
$pkg_origin = "biome"
$pkg_maintainer = "The Biome Maintainers <humans@biome.sh>"
$pkg_license = @("Apache-2.0")
$pkg_deps=@(
    "core/visual-cpp-redist-2015"
)
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @(
    "core/visual-cpp-build-tools-2015",
    "core/rust/$(Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")",
    "core/cacerts",
    "core/protobuf"
)

function pkg_version {
    Get-Content "$SRC_PATH/../../VERSION"
}

function Invoke-Before {
    Invoke-DefaultBefore
    Set-PkgVersion
}

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = Join-Path -Path "$HAB_CACHE_SRC_PATH" -ChildPath "$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"

    # Used by the `build.rs` program to set the version of the binaries
    $env:PLAN_VERSION = "$pkg_version/$pkg_release"
    Write-BuildLine "Setting env:PLAN_VERSION=$env:PLAN_VERSION"

    # Used to set the active package target for the binaries at build time
    $env:PLAN_PACKAGE_TARGET = "$pkg_target"
    Write-BuildLine "Setting env:PLAN_PACKAGE_TARGET=$env:PLAN_PACKAGE_TARGET"

    $env:PROTOC_NO_VENDOR=1
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release --verbose
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Write-BuildLine "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    Copy-Item "$env:CARGO_TARGET_DIR/release/bio.exe" "$pkg_prefix/bin/bio.exe"
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2015")/bin/*.dll" "$pkg_prefix/bin"
}

function Invoke-Clean {
    if(!$env:HAB_SKIP_CLEAN) { Invoke-DefaultClean }
}
