$pkg_name = "bio-launcher"
$pkg_origin = "biome"
$pkg_maintainer = "The Biome Maintainers <humans@biome.sh>"
$pkg_license = @("Apache-2.0")
$pkg_deps=@()
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @(
    "core/visual-cpp-redist-2022",
    "core/visual-build-tools-2022",
    "core/rust/$((ConvertFrom-StringData (Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")[1]).channel.Replace('"', ''))",
    "core/cacerts",
    "core/git",
    "core/windows-11-sdk",
    "core/protobuf"
)

function Invoke-Prepare {
    if($env:BIO_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:BIO_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = Join-Path -Path "$BIO_CACHE_SRC_PATH" -ChildPath "$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-BioPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:PLAN_VERSION               = "$pkg_version"
    Write-BuildLine "Setting env:PLAN_VERSION=$env:PLAN_VERSION"
    $env:LIB                        += ";$BIO_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$BIO_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:PROTOC_NO_VENDOR           = 1
}

function pkg_version {
    git rev-list (git rev-parse HEAD) --count
    if($LASTEXITCODE -ne 0) {
        Write-Error "Unable to deterine version from git!"
    }
}

function Invoke-Before {
    Set-PkgVersion
    $script:pkg_dirname = "${pkg_name}-${pkg_version}"
    $script:pkg_prefix = "$BIO_PKG_PATH\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"
    $script:pkg_artifact="$BIO_CACHE_ARTIFACT_PATH\${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT/.."
    try {
        cargo build --release
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item -Path "$env:CARGO_TARGET_DIR/release/bio-launch.exe" -Destination "$pkg_prefix/bin/bio-launch.exe"
    Copy-Item -Path "$(Get-BioPackagePath "visual-cpp-redist-2022")/bin/*" -Destination "$pkg_prefix/bin"
    Copy-Item -Path "$SRC_PATH/../../NOTICES.txt" -Destination "$pkg_prefix/NOTICES.txt"
}

function Invoke-Clean {
    if(!$env:BIO_SKIP_CLEAN) { Invoke-DefaultClean }
}
