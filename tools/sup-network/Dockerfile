FROM ubuntu:24.04

# Set non-interactive mode to prevent prompts during build
ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/bin:/usr/bin:${PATH}"
ENV BIO_LICENSE=accept

# Combine RUN commands to reduce layers and add cleanup
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libczmq-dev \
        libssl-dev \
        jq \
        curl \
        ca-certificates && \
    # Clean up apt cache to reduce image size
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# This useradd mirrors the one used in our on-prem-builder repo
RUN useradd --system --no-create-home bio

# install stable bio
RUN curl https://raw.githubusercontent.com/biome-sh/biome/main/components/bio/install.sh | bash
RUN bio pkg install core/bio-sup
RUN bio pkg install core/bio-launcher
