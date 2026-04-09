#!/bin/bash

# Print a line of output. Takes the rest of the line as its only
# argument.
#
# ```sh
# info "Running command"
# ```
info() {
  if [ -n "${QUIET:-}" ]; then
    return 0
  fi

  if [ "${BIO_NOCOLORING:-}" = "true" ]; then
    echo "   bio-studio: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "   \033[1;36m%s: \033[1;37m%s\033[0m\n" "bio-studio" "$1"
        ;;
      *)
        echo "   bio-studio: $1"
        ;;
    esac
  fi
  return 0
}
