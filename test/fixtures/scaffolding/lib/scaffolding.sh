scaffolding_load() {
    pkg_deps=("$BIO_ORIGIN/minimal_package" "${pkg_deps[@]}")
    pkg_build_deps=("$BIO_ORIGIN/dep-pkg-1" "${pkg_build_deps[@]}")
}