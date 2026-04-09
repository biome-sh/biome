# Building a `bio` macOS Binary

As Biome currently does not have first class support for the Mac platform, a pragmatic approach has been taken to build a `bio` binary for macOS. This details the steps to build a release on macOS. It is also currently codified in [.buildkite/scripts/build_mac_release.sh](biome-sh/biome/.buildkite/scripts/build_mac_release.sh)

## Prerequisites

### Reference
The following script is what runs in CI and installs all the required prerequisites *except* xcode. Please use this as a reference for the following steps.
```sh
./.buildkite/scripts/build_mac_release.sh
```

### Install XCode Command Line Developer Tools
This installs basic developer tooling needed to compile and build software on a Mac.

```sh
xcode-select --install
```

### Install Omnibus Bootstrap Toolchain package

Since there is not yet a complete Biome build toolchain available for macOS, we provide the minimal set of binaries and static libraries needed to compile a `bio` binary using Biome's Omnibus tooling platform. This effectively takes the place of the packages we would add to a `pkg_build_deps` entry in a Biome plan file.

This currently exists in at `https://s3-us-west-2.amazonaws.com/shain-bk-test/mac-bootstrapper-1.0.0-latest.pkg`

```sh
sudo installer \
     -pkg <PATH_TO_BOOTSTRAP_TOOLCHAIN_PACKAGE> \
     -target /
```

This will install the toolchain in `/opt/mac-bootstrapper`; this is where the build program is expecting to find binaries and libraries it needs.

### Install Homebrew
Follow the instructions at https://brew.sh.

### Install Rust toolchain
Follow the instructions at https://rustup.rs/

### Install Homebrew prerequisites
The only extra homebrew dependency we have current is wget, which can be installed by running the following:
```sh
brew install wget
```

### Install `bio`
We install bio via the `curl|bash` method:

``` sh
curl https://raw.githubusercontent.com/biome-sh/biome/master/components/bio/install.sh | sudo bash
```

### Install Builder Origin Keys

To build a Biome package, you must have the secret `core` origin key for signing that package. Obtaining the secret key requires a personal authentication token from Builder.

``` sh
sudo bio origin key download core
sudo bio origin key download --secret --auth="${BIO_AUTH_TOKEN}" core
```
Here, we use `sudo` to install keys in system-wide `/bio/cache/keys/` directory in which the build program expects to find them.

## Building

Ensure that the necessary tools are on your path, and then build.

``` sh
PATH="/opt/mac-toolchain/embedded/bin:${PATH}"
PATH="~/.cargo/bin:${PATH}"
export PATH
sudo -E bash components/plan-build/bin/bio-plan-build.sh components/bio
```

Assuming success, this will produce a local `./results` directory with the artifact.
