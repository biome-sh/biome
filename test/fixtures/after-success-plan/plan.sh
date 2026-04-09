pkg_name="after_success_plan"
pkg_origin="biome-testing"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_version="0.0.0"

do_build() { :; }
do_install() { :; }

do_after_success() {
    echo "I am a success"
}

do_after_failure() {
    echo "I am a failure"
}
