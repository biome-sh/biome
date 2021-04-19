+++
title = "Origin Keys"

date = 2020-10-12T13:59:46-07:00
draft = false
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Origin Keys"
    identifier = "habitat/origins/origin-keys Origin Keys"
    parent = "habitat/origins"
    weight = 20
+++

Prerequisites:

- [Get Biome]({{< relref "install_habitat.md" >}})
- [Create a Biome Builder account]({{< relref "builder_account#builder-account" >}})
- [Generate a personal access token]({{< relref "builder_profile#create-a-personal-access-token" >}})
- [Create an origin]({{< relref "origins" >}}) or join an origin by [invitation]({{< relref "origin_rbac#manage-origin-membership-with-bio-origin-invitations" >}})

When you create an origin, Biome Builder automatically generates _origin keys_.
Origin key cryptography is asymmetric: it has a public origin key that you can distribute freely, and a private origin key (also called a "signing key") that you should distribute only to users of the origin.
All Biome Builder users with access to the origin can view the public origin key revisions in the origin key tab (Builder > Origin > Keys) and download the public origin key, but only users with the origin 'administrator' or 'owner' roles can view or download the private origin key, or change the origin key pair.

| Keys Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |

Biome uses origin keys:

- When you build an artifact in your local environment, Biome signs the artifact with the private origin key
- When you upload an artifact to Biome Builder or Builder on-prem, Biome uses the public origin key to verify that the artifact was signed with the private origin key
- When you install any package onto a Biome Supervisor, Biome uses the public origin key to verify the package's integrity before it starts the installation
- When you download an artifact to your local Biome Studio, Biome uses the public origin key to verify the artifact's integrity before it starts the installation

Biome Builder origin key names follow the format:

```bio
<origin>-<datetime>.pub (public key)
<origin>-<datetime>.sig.key (private key, also called a "signing key")
```

For example, in:

```bio
testorigin-20190416223046.pub
testorigin-20190416223046.sig.key
```

- "testorigin" is the origin's name
- "20190416223046" is the date and time of the key's creation, which was 2019-04-16 22:30:46.
- `.pub` is the file extension for the public key
- `.sig.key` is the file extension for the private key, which is also called a "signing key"

## The Keys Tab

When you create an origin, Biome Builder automatically generates an origin key pair and saves both keys. To view your origin keys on Biome Builder, navigate to your origin and select the **Keys** tab. (Builder > Origins > Keys) You will always be able to view and download public origin keys, but you will only see the private keys for origins in which you are an "administrator" or "owner".

![Viewing your origin keys](/images/habitat/origin-keys.png)

### Download Origin Keys from the Keys Tab

Download your private or public origin key by selecting the **download** icon from the right end of the key details, under the _Actions_ heading.

![Detail of the download icon](/images/habitat/origin-key-download.png)

### Upload Origin Keys from the Keys Tab

You can upload origin keys that you generate on the command line to Biome Builder by selecting either the **Upload a private key** or **Upload a public key** icon, and copy your key into the form that appears.

![Example form content for uploading an origin key in Builder](/images/habitat/builder-key-upload.png)

## Managing Origin Keys with the CLI

Run Biome CLI commands from your local environment or from within the Biome Studio.

See the CLI documentation for more information on the [`bio origin key`]({{< relref "biome_cli.md/#bio-origin-key" >}}) commands.

### Find Your Origin Keys

Biome stores your public and private origin keys at `~/.hab/cache/keys` on Linux systems, `C:\hab\cache\keys` on Windows, and at `/hab/cache/keys` inside of the Biome Studio environment.

#### Find Origin Keys in a Local Environment

On Windows:

```PowerShell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la ~/.hab/cache/keys
```

#### Find Origin Keys in the Biome Studio

On Windows:

```powershell
Get-ChildItem C:\hab\cache\keys
```

On Linux or macOS:

```bash
ls -la /hab/cache/keys
```

### Generate Origin Keys

When you create an origin through the site, Biome Builder automatically generates an origin key pair.

The Biome CLI creates origin key pairs through two different commands, for two different uses:

- Use [`bio setup`]({{< relref "install_habitat.md" >}}) to generate your first origin key pair as part of setting up the `bio` CLI
- Use the `bio origin key generate <ORIGIN>` command to create a key pair for an origin

Create origin keys with the `bio` command:

```bio
bio origin key generate <ORIGIN>
```

### Download Origin Keys

To get your public origin key using the command line, use:

```bio
bio origin key download <ORIGIN>
```

### Upload Origin Keys

Creating an origin with the `bio origin create` command registers the origin on Biome Builder without creating an origin key pair. The `bio origin key generate` command creates the key pair and saves them in your local environment, but it does not upload either origin key to Biome Builder.

- Only "administrators" and "owners" can upload new keys to an origin.
- Builder requires the public origin key to upload artifacts for that origin, so you'll need to upload it.
- Builder requires the private origin key to enable new artifact builds from packages with plans linked to that origin.

Upload origin keys with the `bio` command:

```bio
bio origin key upload <ORIGIN>
```

Upload the private origin key:

```bio
bio origin key upload --secret <ORIGIN>
```

Upload both origin keys at the same time:

```bio
bio origin key upload  --secfile <PATH_TO_PRIVATE_KEY> --pubfile <PATH_TO_PUBLIC_KEY>
```

### Import Origin Keys

Use `bio origin key import` to read the key from a standard input stream into Biome Builder:

```bio
bio origin key import <enter or paste key>
bio origin key import <PATH_TO_KEY>
cat <PATH_TO_KEY> | bio origin key import
```

#### Troubleshoot Origin Key Import

On a macOS, you may encounter an upload failure.
To remediate this failure:

- Check that your `HAB_AUTH_TOKEN` environment variable is properly set and initialized
- Add your `SSL_CERT_FILE` to the environment variables in your interactive shell configuration file, such as your `.bashrc`.

```bash
  export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

Initialize the setting from the command line with:

```bash
 source ~/.bashrc
```
