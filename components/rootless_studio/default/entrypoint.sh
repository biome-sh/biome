#!/bin/bash

bio pkg exec biome/bio-backline mkdir -m 1777 -p /tmp
bio pkg exec biome/bio-backline mkdir -m 0750 -p /root
bio pkg exec biome/bio-backline mkdir -m 0755 -p /usr/bin

source /etc/biome-studio/import_keys.sh
source /etc/biome-studio/environment.sh

declare -a secrets
readarray -t secrets < <(load_secrets)

case "$1" in
  enter)
    bio pkg exec biome/bio-backline env STUDIO_ENTER=true "${secrets[@]}" bash --login +h;;
  build)
    shift
    bio pkg exec biome/bio-backline env "${secrets[@]}" /bin/build "$@";;
  run)
    shift
    bio pkg exec biome/bio-backline env "${secrets[@]}" bash --login -c "$@";;
  *)
    echo "Unknown Studio Command: ${1}" && exit 1;;
esac
