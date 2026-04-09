# 17.10 (artful) will be EOL July 2018; update FROM directive before then
FROM ubuntu:18.04
MAINTAINER The Biome Maintainers <humans@biome.sh>

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG BIO_BLDR_URL
ENV BIO_BLDR_URL ${BIO_BLDR_URL:-}

COPY components/bio/install.sh \
  support/linux/install_dev_0_ubuntu_latest.sh \
  support/linux/install_dev_9_linux.sh \
  /tmp/
COPY support/devshell_profile.sh /root/.bash_profile

RUN apt-get update \
  && apt-get install -y --no-install-recommends sudo \
  && sh /tmp/install_dev_0_ubuntu_latest.sh \
  && sh /tmp/install_dev_9_linux.sh \
  && useradd -m -s /bin/bash -G sudo jdoe && echo jdoe:1234 | chpasswd \
  && rm -rf \
    /tmp/install.sh \
    /tmp/install_dev_0_ubuntu_latest.sh \
    /tmp/install_dev_9_linux.sh \
    /bio/cache \
    /root/.cargo/registry \
    /var/lib/apt/lists/*

WORKDIR /src
CMD ["bash", "-l"]
