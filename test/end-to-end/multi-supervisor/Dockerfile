# syntax=docker/dockerfile:1
FROM ubuntu

# Channel from which to install Biome-related packages 
ARG CHANNEL=stable

# Bootstrap the installation of Biome
RUN apt-get update && apt-get -y install curl ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://raw.githubusercontent.com/biome-sh/biome/main/components/bio/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && ./install.sh \
    && unset BIO_AUTH_TOKEN

# Always accept the license when we run this image.
ENV BIO_LICENSE=accept-no-persist

# Install core Biome tools
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && bio pkg install biome/bio --binlink --force --channel="${CHANNEL}" \
    && unset BIO_AUTH_TOKEN

# Install Docker CLI for tests (via bio if available)
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && bio pkg install core/docker --binlink --force -c base-2025 \
    && unset BIO_AUTH_TOKEN

RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && bio pkg install core/powershell -c base-2025 \
    && bio pkg binlink core/powershell pwsh --force \
    && bio pkg install core/pester -c base-2025 \
    && unset BIO_AUTH_TOKEN
