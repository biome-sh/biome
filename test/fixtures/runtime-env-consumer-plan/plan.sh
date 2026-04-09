pkg_name="runtime-env-consumer-plan"
pkg_origin="biome-testing"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_version="0.0.0"
pkg_build_deps=("biome-testing/runtime-env-plan")

do_build() {
    if [[ "$SOME_VAR" != "SOME_VAL" ]]; then
        exit 1
    else
        exit 0
    fi
}
do_install() { :; }
