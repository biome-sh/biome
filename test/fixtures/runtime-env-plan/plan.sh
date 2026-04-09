pkg_name="runtime-env-plan"
pkg_origin="biome-testing"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_version="0.0.0"

do_setup_environment() {
    set_runtime_env "SOME_VAR" "SOME_VAL"
}
do_build() { :; }
do_install() { :; }
