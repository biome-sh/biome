# Biome Studio

This is the code that creates a biome studio for linux and windows.

The key pieces here are:

* The studio hosting script - `bin/biome-studio`. The `.sh` sets up a linux studio and the `.ps1` sets up a windows studio.
* The studio environment "types" in `libexec`. These tweak the environment of a studio and target it for specific purposes. Most will only be interested in `bio-studio-type-default.sh`.
* The biome plans used to create the biome studio package.

## What determines the studio type and version used?

Different studios will run depending on OS platform, bio client version and environmental overrides.

### Linux

If you are run in a Linux host or VM, the biome client will install the `biome/bio-studio/<version>` if it is not already installed. `<version>` will match the exact version of the `bio` binary you are using. However do note that if you have a later version already installed, it will use that one instead. It will then invoke `bin/bio-studio.sh <bio studio command>` passing whatever studio command you have entered (`enter`, `build`. etc) to the bash script.

The `-t` argument will determine which studio `type` is sourced. The default is `default`.

### Experimental Windows Studio

If you have the `$env:BIO_WINDOWS_STUDIO` variable set and are running on a Windows host, the bio client will pull down the appropriately versioned `biome/bio-studio` package and invoke its `bin/bio-studio.ps1` script. Note that you must also be using a depot that hosts the windows biome packages.

## Building a studio

You will need to build the studio and bio client packages for the same version.

### Build the bio and bio-studio packages

Navigate to the root of the biome repo and build the bio client

```
cd /biome
bio pkg build components/bio
```

Next build the studio

```
bio pkg build components/studio
```

The most important thing here is that you end up with `hart` files at the same version for both `biome/bio` and `biome/bio-studio`.

On a linux OS, you can `bio pkg install` the `hart`s and then run the bio binary you installed with the studio command. For example:

```
/bio/pkgs/biome/bio/2.0.134/20250606091728/bin/bio studio enter
```
