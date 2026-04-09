+++
title = "Environment Variables"
description = "Customize and configure your Biome Studio and Supervisor environments"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Environment Variables"
    identifier = "biome/reference/environment-variables"
    parent = "biome/reference"

+++

This is a list of all environment variables that can be used to modify the operation of the Biome Studio and Supervisor.

| Variable | Context | Default | Description |
|----------|----------|----------|-----------|
| `BIO_AUTH_TOKEN` | build system | no default | Authorization token used to perform privileged operations against the depot, e.g. uploading packages or keys. Can also be configured in `~/.bio/etc/cli.toml` as `auth_token`.
| `BIO_BINLINK_DIR` | build system | `/bio/bin` | Allows you to change the target directory for the symlink created when you run `bio pkg binlink`. The default value is already included in the `$PATH` variable inside the Studio. |
| `BIO_CACHE_KEY_PATH` | build system, Supervisor | `/bio/cache/keys` if running as root; `$HOME/.bio/cache/keys` if running as non-root | Cache directory for origin signing keys |
| `BIO_CTL_SECRET` | Supervisor | no default | Shared secret used for [communicating with a Supervisor]({{< relref "sup_remote_control" >}}). |
| `BIO_BLDR_CHANNEL` | build system, Supervisor | `stable` | Set the Biome Builder channel you are subscribing to, to a specific channel. Defaults to `stable`.
| `BIO_BLDR_URL` | build system, Supervisor | `https://bldr.biome.sh` | Sets an alternate default endpoint for communicating with Builder. Used by the Biome build system and the Supervisor |
| `BIO_DOCKER_OPTS` | build system | no default | When running a Studio on a platform that uses Docker (macOS), additional command line options to pass to the `docker` command. |
| `BIO_INTERNAL_BLDR_CHANNEL` | build system, Supervisor, exporters | `stable` | Channel from which Biome-specific packages (e.g., `biome/bio-sup`, `biome/bio-launcher`, etc.) are downloaded on-demand when first called. Generally of use only for those developing Biome. Only applies to Biome-specific packages, and nothing else. |
| `BIO_LICENSE` | build system, Supervisor, exporters | no default | Used to accept the [Biome EULA]({{< relref "biome_license#biome-eula" >}}). See [Accepting the Biome License]({{< relref "biome_license_accept#biome" >}}) for valid values. |
| `BIO_LISTEN_CTL` | Supervisor | 127.0.0.1:9632 | The listen address for the Control Gateway. This also affects `bio` commands that interact with the Supervisor via the Control Gateway, for example: `bio sup status`. |
| `BIO_LISTEN_GOSSIP` | Supervisor | 0.0.0.0:9638 | The listen address for the Gossip System Gateway |
| `BIO_LISTEN_HTTP` | Supervisor | 0.0.0.0:9631 | The listen address for the HTTP Gateway |
| `BIO_NOCOLORING` | build system | no default | If set to the lowercase string `"true"` this environment variable will unconditionally disable text coloring where possible |
| `BIO_NONINTERACTIVE` | build system | no default | If set to the lowercase string `"true"` this environment variable will unconditionally disable interactive progress bars (i.e. "spinners") where possible |
| `BIO_ORG` | Supervisor | no default | Organization to use when running with [service group encryption]({{< relref "sup_secure" >}})
| `BIO_ORIGIN` | build system | no default | Origin used to build packages. The signing key for this origin is passed to the build system. |
| `BIO_ORIGIN_KEYS` | build system | no default | Comma-separated list of origin keys to automatically share with the build system |
| `BIO_REFRESH_CHANNEL` | build system | `base` | Channel used to retrieve plan dependencies for Biome supported origins. Can also be configured in `~/.bio/etc/cli.toml` as `refresh_channel`. |
| `BIO_RING` | Supervisor | no default | The name of the ring used by the Supervisor when running with [wire encryption]({{< relref "sup_secure" >}}) |
| `BIO_RING_KEY` | Supervisor | no default | The contents of the ring key when running with [wire encryption]({{< relref "sup_secure" >}}). Useful when running in a container. |
| `BIO_STUDIO_SECRET_<VARIABLE>` | build system | no default | Prefix to allow environment variables into the Studio. The prefix will be removed and your variable will be passed into the Studio at build time. |
| `BIO_STUDIOS_HOME` | build system | `/bio/studios` | Directory in which to create build Studios |
| `BIO_STUDIO_BACKLINE_PKG` | build system | `biome/bio-backline/{{studio_version}}` | Overrides the default package identifier for the "backline" package which installs the Studio baseline package set. |
| `BIO_STUDIO_ROOT` | build system | no default | Root of the current Studio under `$BIO_STUDIOS_HOME`. Infrequently overridden. |
| `BIO_STUDIO_NOSTUDIORC` | build system | no default | When set to a non-empty value, a `.studiorc` will not be sourced when entering an interactive Studio via `bio studio enter`. |
| `BIO_STUDIO_SUP` | build system | no default | Used to customize the arguments passed to an automatically launched Supervisor, or to disable the automatic launching by setting it to `false`, `no`, or `0`. |
| `BIO_GLYPH_STYLE` | build system | `full` (`limited` on Windows) | Used to customize the rendering of unicode glyphs in UI messages. Valid values are `full`, `limited`, or `ascii`. |
| `BIO_SUP_UPDATE_MS` | Supervisor | 60000 | Interval in milliseconds governing how often to check for Supervisor updates when running with the [--auto-update]({{< relref "biome_cli/#bio-sup-run" >}}) flag. Note: This variable has been deprecated. Users should instead use the [--auto-update-period]({{< relref "biome_cli/#bio-sup-run" >}}) flag. |
| `BIO_UPDATE_STRATEGY_FREQUENCY_MS` | Supervisor | 60000 | Interval in milliseconds governing how often to check for service updates when running with an [update strategy]({{< relref "service_updates" >}}). Note: This variable has been deprecated. Users should instead use the [--service-update-period]({{< relref "biome_cli/#bio-sup-run" >}}) flag. |
| `BIO_USER` | Supervisor | no default | User key to use when running with [service group encryption]({{< relref "sup_secure" >}}) |
| `http_proxy` | build system, Supervisor | no default | A URL for a local HTTP proxy server optionally supporting basic authentication |
| `https_proxy` | build system, Supervisor | no default | A URL for a local HTTPS proxy server optionally supporting basic authentication |
| `NO_INSTALL_DEPS` | build system | no default | Set this variable to prevent dependencies install during build |
| `no_proxy` | build system, Supervisor | no default | A comma-separated list of domain exclusions for the `http_proxy` and `https_proxy` environment variables |
| `SSL_CERT_FILE` | system | no default | Standard OpenSSL environment variable to override the system certificate file. This is particularly important for the secure HTTPS connection with a Builder instance. Can be used to help you navigate corporate firewalls. |
