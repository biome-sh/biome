ARG PACKAGE_TARGET
FROM biomesh/bio-${PACKAGE_TARGET}:${BIO_VERSION} as bio
ENV PATH=${PATH}:/hab/bin
ARG BLDR_CHANNEL=stable
ARG BLDR_URL=https://bldr.biome.sh
ARG HAB_LICENSE=no-accept
RUN bio pkg install --url ${BLDR_URL} --channel ${BLDR_CHANNEL} biome/bio-backline \
  && bio pkg binlink core/bash -d /hab/bin \
  && bio pkg binlink biome/bio -d /hab/bin

FROM scratch
COPY --from=bio /hab /hab
COPY --from=bio /hab/bin /bin
COPY --from=bio /bin/bio /bin/
COPY ./etc/ /etc/
ADD ./entrypoint.sh /
ADD ./profile /etc/
ADD ./profile.enter /etc/
ADD ./build /bin/
ENTRYPOINT ["/entrypoint.sh"]
CMD ["enter"]
WORKDIR /src
