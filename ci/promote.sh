#!/bin/bash -e

CHANNEL=stable

PACKAGES=(
    components/bio
    components/launcher
    components/sup

    components/plan-build
    components/backline
    components/studio

    components/pkg-export-docker
    components/pkg-cfize
    components/pkg-export-tar
    components/pkg-mesosize
)

for pkg in "${PACKAGES[@]}"; do
    source "results/$(basename "$pkg").env"

    # shellcheck disable=SC2154
    bio pkg promote "$pkg_ident" "$CHANNEL"
done
