# shellcheck disable=2034
pkg_name=bio-studio
pkg_origin=biome
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(
  biome/bio-backline
)
pkg_build_deps=(core/coreutils
                core/tar
                core/xz
                core/wget
                core/busybox-static
                biome/bio)
pkg_bin_dirs=(bin)

pkg_version() {
  cat "$SRC_PATH/../../VERSION"
}

do_before() {
  do_default_before
  update_pkg_version
}

do_prepare() {
  set_runtime_env "HAB_STUDIO_BACKLINE_PKG" "$(< "$(pkg_path_for biome/bio-backline)"/IDENT)"
}

do_build() {
  # shellcheck disable=2154
  kernel="${pkg_target#*-}"
  kernel="${kernel%%-*}"  # Remove 'kernel2' part if exists
  cp -v "$SRC_PATH"/bin/bio-studio-"${kernel}".sh bio-studio
  cp -v "$SRC_PATH"/libexec/bio-studio-profile.sh .
  cp -v "$SRC_PATH"/libexec/bio-studio-type-*.sh .

  # Embed the release version and author information of the program.
  # shellcheck disable=2154
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i bio-studio
}

do_install() {
  # shellcheck disable=2154
  install -v -D bio-studio "$pkg_prefix"/bin/bio-studio
  install -v -D bio-studio-profile.sh "$pkg_prefix"/libexec/bio-studio-profile.sh
  for f in bio-studio-type-*.sh; do
    [[ -e $f ]] || break # see http://mywiki.wooledge.org/BashPitfalls#pf1
    install -v -D "$f" "$pkg_prefix"/libexec/"$f"
  done

  lbb="$pkg_prefix/libexec/busybox"

  # Install a copy of a statically built busybox under `libexec/`
  install -v -D "$(pkg_path_for busybox-static)"/bin/busybox "$lbb"

  bio_dir=$(tr '/' '-' < "$(pkg_path_for bio)"/IDENT)
  install -v -D "$(pkg_path_for bio)"/bin/bio \
    "$pkg_prefix"/libexec/"$bio_dir"/bin/bio
  ln -sv "$bio_dir"/bin/bio "$pkg_prefix"/libexec/bio

  cp -rv "${SRC_PATH}/defaults" "${pkg_prefix}"
}
