FROM alpine
MAINTAINER Yauhen Artsiukhou <jsirex@gmail.com>
ARG BIO_VERSION=
ARG PACKAGE_TARGET
RUN set -ex \
    && apk add --no-cache ca-certificates curl bash \
    && curl https://raw.githubusercontent.com/biome-sh/biome/master/components/bio/install-linux.sh | bash \
    && rm /hab/cache/artifacts/*.hart
