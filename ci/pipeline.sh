#!/bin/bash -e

# We're always trying to update packages if they already installed
bio pkg install -fb ya/bio-sdk
bio pkg install -fb core/git
bio pkg install -fb core/shellcheck
bio pkg install -fb ya/tomlcheck

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

# Package Lint
for pkg in "${PACKAGES[@]}"; do
    bio-plan-tomlcheck "$pkg"
    bio-plan-shellcheck "$pkg"
    bio-plan-rendercheck "$pkg"
done

# Building
for pkg in "${PACKAGES[@]}"; do
    bio-plan-build "$pkg"
    # preserve build results for each package
    cp results/last_build.env "results/$(basename "$pkg").env"
done

echo TODO: bio-plan-bats
