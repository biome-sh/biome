#!/bin/bash
#
set -eou pipefail

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

BIO_BINARY_URL=https://github.com/biome-sh/biome/releases/download/bio-1.6.42/bio-1.6.42-x86_64-linux
BIO_BINARY_SHA256="2f18fc905330783f70f52d6c9a7f103b4b4c3122159e6c4e3e26507c842016c1"
BIO_BINARY_PATH=/usr/local/bio-1.6.42/bin/bio

if echo "$BIO_BINARY_SHA256 $BIO_BINARY_PATH" | sha256sum -c; then
    echo "Biome bootstrap binary is already installed."
else
    echo "Downloading Biome bootstrap binary."
    mkdir -p "$(dirname "$BIO_BINARY_PATH")"
    curl -sSL "$BIO_BINARY_URL" > "$BIO_BINARY_PATH"
    chmod +x "$BIO_BINARY_PATH"
fi

"$BIO_BINARY_PATH" pkg install -fb biome/bio
