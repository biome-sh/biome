FROM ubuntu:latest

# Update package lists and upgrade existing packages
RUN echo "--- Boostsrap env for automate install" && \
    apt-get update && \
    apt-get install -y \
        curl

# Download and install Cinc Automate
RUN curl https://packages.biome.sh/files/current/latest/biome-automate-cli/biome-automate_linux_amd64.zip | gunzip - > biome-automate && chmod +x biome-automate
# Configure automate. Make sure its fqdn is set to localhost and not the build containers
# IP or hostname which will only work in the build environment
RUN ./biome-automate init-config --fqdn localhost

# Docker environment may not have a systemd so hint to automate not to use it
ENV BIOME_AUTOMATE_SKIP_SYSTEMD="true"

ENV BIO_LICENSE=accept-no-persist

# Get all the automate services deployed and running
RUN mkdir -p /bio/sup/default
RUN ./biome-automate deploy config.toml --skip-preflight --accept-terms-and-mlsa

# This is how containers will start automate
CMD bio sup run