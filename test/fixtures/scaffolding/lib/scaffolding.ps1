[Diagnostics.CodeAnalysis.SuppressMessage("PSUseApprovedVerbs", '', Scope="function")]
param()
function Load-Scaffolding {
    $pkg_deps += @("$env:BIO_ORIGIN/minimal_package")
    $pkg_build_deps += @("$env:BIO_ORIGIN/dep-pkg-1")
}
