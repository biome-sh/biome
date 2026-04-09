#!/bin/bash

source /etc/biome-studio/logging.sh


# Removes any potential malicious secrets
sanitize_secrets() {
  for x in BIO_BINLINK_DIR BIO_ORIGIN HOME LC_ALL PATH PWD STUDIO_TYPE TERM TERMINFO; do
    unset "BIO_STUDIO_SECRET_$x"
  done
}

# Builds up a secret environment based on the prefix `BIO_STUDIO_SECRET_`
# to pass into the studio
load_secrets() {
  sanitize_secrets
  bio pkg exec biome/bio-backline env | bio pkg exec biome/bio-backline awk -F '=' '/^BIO_STUDIO_SECRET_/ {gsub(/BIO_STUDIO_SECRET_/, ""); print}'
}

if [ -n "${BIO_CONFIG_EXCLUDE:-}" ]; then
info "Exported: BIO_CONFIG_EXCLUDE=$BIO_CONFIG_EXCLUDE"
fi
if [ -n "${BIO_AUTH_TOKEN:-}" ]; then
info "Exported: BIO_AUTH_TOKEN=[redacted]"
fi
if [ -n "${BIO_LICENSE:-}" ]; then
info "Exported: BIO_LICENSE=$BIO_LICENSE"
fi
if [ -n "${BIO_ORIGIN:-}" ]; then
info "Exported: BIO_ORIGIN=$BIO_ORIGIN"
fi
if [ -n "${BIO_BLDR_URL:-}" ]; then
info "Exported: BIO_BLDR_URL=$BIO_BLDR_URL"
fi
if [ -n "${BIO_BLDR_CHANNEL:-}" ]; then
info "Exported: BIO_BLDR_CHANNEL=$BIO_BLDR_CHANNEL"
fi
if [ -n "${BIO_NOCOLORING:-}" ]; then
info "Exported: BIO_NOCOLORING=$BIO_NOCOLORING"
fi
if [ -n "${BIO_NONINTERACTIVE:-}" ]; then
info "Exported: BIO_NONINTERACTIVE=$BIO_NONINTERACTIVE"
fi
if [ -n "${BIO_STUDIO_NOSTUDIORC:-}" ]; then
info "Exported: BIO_STUDIO_NOSTUDIORC=$BIO_STUDIO_NOSTUDIORC"
fi
if [ -n "${BIO_STUDIO_SUP:-}" ]; then
info "Exported: BIO_STUDIO_SUP=$BIO_STUDIO_SUP"
fi
if [ -n "${BIO_UPDATE_STRATEGY_PERIOD_MS:-}" ]; then
info "Exported: BIO_UPDATE_STRATEGY_PERIOD_MS=$BIO_UPDATE_STRATEGY_PERIOD_MS"
fi
if [ -n "${http_proxy:-}" ]; then
info "Exported: http_proxy=$http_proxy"
fi
if [ -n "${https_proxy:-}" ]; then
info "Exported: https_proxy=$https_proxy"
fi
if [ -n "${no_proxy:-}" ]; then
info "Exported: no_proxy=$no_proxy"
fi

for secret_name in $(load_secrets | bio pkg exec core/coreutils cut -d = -f 1); do
info "Exported: $secret_name=[redacted]"
done
