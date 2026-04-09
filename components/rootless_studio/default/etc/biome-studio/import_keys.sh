#!/bin/bash

source /etc/biome-studio/logging.sh

: "${BIO_ORIGIN_KEYS:=${BIO_ORIGIN:-}}"

if [ -n "$BIO_ORIGIN_KEYS" ]; then
  # There's a method to this madness: `bio` is the raw path to `bio`
  # will use the outside cache key path, whereas the `bio` function has
  # the `$FS_ROOT` set for the inside of the Studio. We're copying from
  # the outside in, using `bio` twice. I love my job.
  for key in $(echo "$BIO_ORIGIN_KEYS" | bio pkg exec biome/bio-backline tr ',' ' '); do
    key_text=""
    # Import the secret origin key, required for signing packages
    info "Importing '$key' secret origin key"
    if key_text=$(bio origin key export --type secret "$key"); then
      printf -- "%s" "${key_text}" | bio origin key import
    else
      echo "Error exporting $key key"
      # key_text will contain an error message
      echo "${key_text}"
      echo "Biome was unable to export your secret signing key. Please"
      echo "verify that you have a signing key for $key present in either"
      # shellcheck disable=SC2088
      echo "~/.bio/cache/keys (if running via sudo) or /bio/cache/keys"
      echo "(if running as root). You can test this by running:"
      echo ""
      echo "    bio origin key export --type secret $key"
      echo ""
      echo "This test will print your signing key to the console or error"
      echo "if it cannot find the key. To create a signing key, you can run: "
      echo ""
      echo "    bio origin key generate $key"
      echo ""
      echo "You'll also be prompted to create an origin signing key when "
      echo "you run 'bio cli setup'."
      echo ""
      exit 1
    fi
    # Attempt to import the public origin key, which can be used for local
    # package installations where the key may not yet be uploaded.
    if key_text=$(bio origin key export --type public "$key" 2> /dev/null); then
      info "Importing '$key' public origin key"
      printf -- "%s" "${key_text}" | bio origin key import
    else
      info "Tried to import '$key' public origin key, but key was not found"
    fi
  done
else
  info "No secret keys imported! Did you mean to set BIO_ORIGIN?"
  echo "To specify a BIO_ORIGIN, either set the BIO_ORIGIN environment"
  echo "variable to your origin name or run 'bio cli setup' and specify a"
  echo "default origin."
fi
