info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36mBiome devshell: \033[1;37m%s\033[0m\n" "$1"
      ;;
    *)
      printf -- "   devshell: %s\n" "$1"
      ;;
  esac
  return 0
}

echo
info 'Plan for success!'

if [[ -n "$BIO_ORIGIN" ]]; then
  info "Exported: BIO_ORIGIN=$BIO_ORIGIN"
fi
if [[ -n "$BIO_BLDR_URL" ]]; then
  info "Exported: BIO_BLDR_URL=$BIO_BLDR_URL"
fi
# shellcheck disable=2154
if [[ -n "$http_proxy" ]]; then
  info "Exported: http_proxy=$http_proxy"
fi
# shellcheck disable=2154
if [[ -n "$https_proxy" ]]; then
  info "Exported: https_proxy=$https_proxy"
fi
echo
