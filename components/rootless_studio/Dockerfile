FROM alpine
ARG BIO_VERSION=
ARG PACKAGE_TARGET
RUN --mount=type=secret,id=bio_auth_token \
  set -ex \
  && apk add --no-cache --virtual .build-deps \
    ca-certificates \
    gnupg \
    wget \
    bash \
  \
  && cd /tmp \
  && wget https://raw.githubusercontent.com/biome-sh/biome/main/components/bio/install.sh \
  && export BIO_AUTH_TOKEN=$(cat /run/secrets/bio_auth_token) \
  && bash install.sh -t ${PACKAGE_TARGET} \
  && unset BIO_AUTH_TOKEN \
  && rm -rf install.sh /bio/cache /root/.wget-hsts /root/.gnupg \
  && apk del .build-deps
