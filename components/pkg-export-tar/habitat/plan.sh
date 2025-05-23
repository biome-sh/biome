# shellcheck disable=2154
pkg_name=bio-pkg-export-tar
_pkg_distname=$pkg_name
pkg_origin=biome
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_license=('Apache-2.0')
pkg_deps=()
pkg_build_deps=(core/musl
                core/perl # Needed for vendored openssl-sys
                core/coreutils
                core/rust/"$(tail -n 1 "$SRC_PATH/../../rust-toolchain"  | cut -d'"' -f 2)"
                core/gcc
                core/make)
pkg_bin_dirs=(bin)

bin=$_pkg_distname

_common_prepare() {
  do_default_prepare

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used to set the active package target for the binaries at build time
  export PLAN_PACKAGE_TARGET="$pkg_target"
  build_line "Setting PLAN_PACKAGE_TARGET=$PLAN_PACKAGE_TARGET"

  if [ -z "$HAB_CARGO_TARGET_DIR" ]; then
    # Used by Cargo to use a pristine, isolated directory for all compilation
    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  else
    export CARGO_TARGET_DIR="$HAB_CARGO_TARGET_DIR"
  fi
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

pkg_version() {
  cat "$SRC_PATH/../../VERSION"
}

do_before() {
  do_default_before
  update_pkg_version
}

# shellcheck disable=2155
do_prepare() {
  _common_prepare

  # With the musl target, the ring crate is looking for aarch64-linux-musl-gcc,
  # but the core/musl package provides musl-gcc. This workaround is necessary until the appropriate changes are made to core/musl for aarch64.
  if [[ "${pkg_target%%-*}" == "aarch64" ]]; then
    if [[ ! -r "$(pkg_path_for musl)/bin/aarch64-linux-musl-gcc" ]]; then
      ln -sv "$(pkg_path_for musl)/bin/musl-gcc" "$(pkg_path_for musl)/bin/aarch64-linux-musl-gcc"
    fi
  fi

  export rustc_target="${pkg_target%%-*}-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

  PLAN_TAR_PKG_IDENT=$(pkg_path_for tar | sed "s,^$HAB_PKG_PATH/,,")
  export PLAN_TAR_PKG_IDENT
  build_line "Setting PLAN_TAR_PKG_IDENT=$PLAN_TAR_PKG_IDENT"

  # rust 1.46.0 enabled Position Independent Executables(PIE) for x86_64-unknown-linux-musl.
  # This causes the compiled binary to segfault when building with GCC versions that
  # support it. While we should investigate if there is something in the way we compile
  # GCC which causes this. Setting relocation-model to static suppresses PIE.
  export RUSTFLAGS='-C relocation-model=static'
}

do_build() {
  pushd "$PLAN_CONTEXT" || exit
  cargo build "${build_type#--debug}" --target="$rustc_target" --verbose
  popd || exit
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/"${build_type#--}"/$bin \
    "$pkg_prefix"/bin/$bin
}
