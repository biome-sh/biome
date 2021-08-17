+++
title = "Keys"
description = "Biome Security"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Keys"
    identifier = "habitat/reference/keys"
    parent = "habitat/reference"

+++

Biome has strong cryptography built into Biome Builder, the Supervisor, and the `bio` CLI commands. This means there are several different kinds of keys.

## Origin Key Pairs

Every Biome artifact belongs to an [origin]({{< relref "pkg_ids" >}}) and is cryptographically signed with that origin's private key. Biome requires the private key for producing artifacts and requires the public key for verification of artifacts before installation. If it is present on Builder, Biome will automatically download the public key for an origin when necessary.

Origin key cryptography is asymmetric: it has a public key that you can distribute freely, and a private key that you should keep safe.

Biome uses the public origin key to verify the integrity of downloaded artifacts before installing them.
Biome will only install artifacts for which it has the public origin key.

You can provide a public origin key to Biome by pointing it to a Builder site that has the origin key with the `--url` argument to `bio pkg install` or using the `bio origin key import` command.
Use `bio origin key upload` to upload origin keys to Builder.
Use `bio origin key download` to download your origin keys from Builder to your environment.
Use `bio origin key import` to read the key from a standard input stream or local file:

```bash
bio origin key import <enter or paste key>
bio origin key import < <PATH_TO_KEY>
curl <URL_THAT_RETURNS_KEY> | bio origin key import
```

See the [bio origin key]({{< relref "biome_cli/#bio-origin-key" >}}) command documentation for more information about working with origin keys from the command line.

## User and Service Group Keys

User and service group keys are used to set up trust relationships between these two entities. Service groups can be set up to reject communication (e.g. applying new configuration via `bio config apply`) from untrusted users.

By default, service groups will trust *any* communication, so for a production deployment of Biome, setting up these relationships is essential.

User and service group keys also use asymmetric cryptography. To apply configuration changes to service groups when running in this mode, a user uses their own private key to encrypt configuration information for a service group, using that service group's public key. The service group then uses its private key to decrypt the configuration information, and the user's public key to verify.

## Ring Encryption Key

A Supervisor network can be optionally set up to encrypt *all* supervisor-to-supervisor communication.
This requires the use of a symmetric, pre-shared key.
