#!/usr/bin/env bash
# This is a lightweight test to verify a studio can be created before merging a PR.
# This (hopefully) prevents us spending time building the first half of a release 
# only to hit a broken studio. 
# 
# Failure case: because this creates a studio from source, we don't exercise changes
# in our plan.sh, and could still end up with a bad studio build.


set -euo pipefail

export HAB_LICENSE="accept-no-persist"

sudo bio pkg install core/busybox-static biome/bio biome/bio-backline

# Current studio has the expectation that busybox and bio live in the libexec directroy
# These two lines should be removed at a later date to validate this is no longer a requirement
# While not explicily mentioned, resolving habitat-sh/habitat#6509 will likely remove this 
# explicit requirement.
cp "$(bio pkg path core/busybox-static)"/bin/busybox libexec/busybox
cp "$(bio pkg path biome/bio)"/bin/bio libexec/bio

HAB_STUDIO_BACKLINE_PKG="$(< "$(bio pkg path biome/bio-backline)"/IDENT)"

export HAB_STUDIO_BACKLINE_PKG

sudo --preserve-env bin/bio-studio-linux.sh new

rm libexec/bio
rm libexec/busybox
