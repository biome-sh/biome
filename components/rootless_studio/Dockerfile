FROM alpine
MAINTAINER Yauhen Artsiukhou <jsirex@gmail.com>
ARG BIO_VERSION
ARG PACKAGE_TARGET

# RUN set -ex \
#     && apk add --no-cache ca-certificates curl bash \
#     && curl https://raw.githubusercontent.com/biome-sh/biome/master/components/bio/install-linux.sh | bash \
#     && rm /hab/cache/artifacts/*.hart

# Offline bootstrap
RUN --mount=type=bind,source=bin,target=/bootstrap/bin \
    cd /bootstrap/bin \
    && mkdir -p /hab/cache/keys/ \
    && cp biome-20190503140510.pub /hab/cache/keys/ \
    && ./bio pkg install -fb biome-bio-2.0.107-20250416221941-x86_64-linux.hart \
    && bio --version
