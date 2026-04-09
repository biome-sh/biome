#!/bin/bash
#
set -eou pipefail

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

BIO_BINARY_URL=https://github.com/biome-sh/biome/releases/download/v2.0.107/bio-2.0.107-x86_64-linux
BIO_BINARY_SHA256="07c73ec812679660253db828b5008232023cbad109f87a82307d06f97cc52e6c"
BIO_BINARY_PATH=/usr/local/bio-2.0.107/bin/bio

if [ -f "$BIO_BINARY_PATH" ]; then
    echo "Biome bootstrap binary is already downloaded."
else
    echo "Downloading Biome bootstrap binary."
    mkdir -p "$(dirname "$BIO_BINARY_PATH")"
    curl -sSL "$BIO_BINARY_URL" > "$BIO_BINARY_PATH"
fi

if echo "$BIO_BINARY_SHA256 $BIO_BINARY_PATH" | sha256sum -c; then
    echo "Checksum validated."
    chmod +x "$BIO_BINARY_PATH"
else
    echo "Checksum failed. Removing executable flag"
    chmod -x "$BIO_BINARY_PATH"
    exit 1
fi

"$BIO_BINARY_PATH" pkg install -fb biome/bio
