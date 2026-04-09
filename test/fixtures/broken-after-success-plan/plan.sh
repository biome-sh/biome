pkg_name="broken_after_success_plan"
pkg_origin="biome-testing"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_version="0.0.0"

do_build() { :; }
do_install() { :; }

do_after_success() {
    exit 1
}

do_after_failure() {
    echo "I am a failure"
}
