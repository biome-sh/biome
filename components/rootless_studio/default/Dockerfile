ARG PACKAGE_TARGET
FROM biomesh/bio-${PACKAGE_TARGET}:${BIO_VERSION} as bio
ENV PATH=${PATH}:/bio/bin
ARG BLDR_CHANNEL=stable
ARG BLDR_URL=https://bldr.biome.sh
ARG BIO_LICENSE=no-accept
RUN --mount=type=secret,id=bio_auth_token \
  export BIO_AUTH_TOKEN=$(cat /run/secrets/bio_auth_token) \
  && bio pkg install --url ${BLDR_URL} --channel ${BLDR_CHANNEL} biome/bio-backline \
  && bio pkg binlink core/bash -d /bio/bin \
  && bio pkg binlink biome/bio -d /bio/bin \
  && unset BIO_AUTH_TOKEN

FROM scratch
COPY --from=bio /bio /bio
COPY --from=bio /bio/bin /bin
COPY --from=bio /bin/bio /bin/
COPY ./etc/ /etc/
ADD ./entrypoint.sh /
ADD ./profile /etc/
ADD ./profile.enter /etc/
ADD ./build /bin/
ENTRYPOINT ["/entrypoint.sh"]
CMD ["enter"]
WORKDIR /src
