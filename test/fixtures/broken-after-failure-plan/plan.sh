pkg_name="broken_after_failure_plan"
pkg_origin="biome-testing"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_version="0.0.0"

do_build() {
    exit 1
}
do_install() { :; }

do_after_success() {
    echo "I am a success"
}

do_after_failure() {
    exit 1
}
