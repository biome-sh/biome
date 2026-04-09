# syntax=docker/dockerfile:1
FROM alpine as build_image

# All Supervisor-related packages will be downloaded from this channel
# on Builder.
ARG CHANNEL=stable

# Always accept the license when we run this image.
ENV BIO_LICENSE=accept-no-persist

# Bootstrap us enough to get a `bio` binary on the box, but then clear
# out the /bio directory. The `bio` binary isn't necessarily the one
# that we want in the final image.
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && apk add bash wget \
    && wget https://raw.githubusercontent.com/biome-sh/biome/main/components/bio/install.sh \
    && bash install.sh \
    && mv /bio/pkgs/biome/bio/*/*/bin/bio /bin/ \
    && rm -Rf /bio \
    && unset BIO_AUTH_TOKEN

# Install only what's needed to run the Supervisor
# TODO: need a way to parameterize busybox
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && bio pkg install --channel="${CHANNEL}" biome/bio \
    && bio pkg install --channel="${CHANNEL}" biome/bio-sup \
    && bio pkg install --channel="${CHANNEL}" biome/bio-launcher \
    && bio pkg install --channel="base-2025" core/busybox \
    && bio pkg binlink biome/bio -d /bio/bin \
    && unset BIO_AUTH_TOKEN

# Create enough of a filesystem for the Supervisor to operate. We
# can't do this in a `FROM scratch` image because there's no shell to
# run `mkdir`. Thus, we'll build it here, and then copy it into the
# final image later.
WORKDIR /tmp/rootfs
RUN mkdir tmp \
    && mkdir bin \
    && mkdir -p var/tmp \
    && mkdir root

# This is important because the Supervisor relies on busybox being
# present for some functionality. Without these present, some packages
# with auto-generated run hooks won't work.
RUN --mount=type=secret,id=bio_auth_token \
    export BIO_AUTH_TOKEN="$(cat /run/secrets/bio_auth_token)" \
    && bio pkg binlink core/busybox --dest=/tmp/rootfs/bin \
    && unset BIO_AUTH_TOKEN

########################################################################

FROM scratch

# Always accept the license when we run this image.
ENV BIO_LICENSE=accept-no-persist

# Copy everything we need from the build image
COPY --from=build_image /tmp/rootfs/ /
COPY --from=build_image /bio /bio
COPY --from=build_image /bio/bin /bin
COPY etc/ /etc
ENTRYPOINT ["bio", "sup"]
CMD ["run"]
