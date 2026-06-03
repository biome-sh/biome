+++
title = "Create an Origin on Builder"
description = "Create an Origin on Builder"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Origins"
    identifier = "biome/builder/origins"
    parent = "biome/builder"
    weight = 30
+++

{{< readfile file="/biome/reusable/md/builder_origins.md" >}}

## Biome-owned origins

Biome Community maintains the following origins:

- **core**: Hosts packages for common dependencies and compilers maintained by Biome Community.
- **biome**: Hosts packages for Biome products like Cinc Client, Cinc Auditor, and Cinc Automate.
- **biome-platform**: Hosts packages for Biome 360 Platform skills.
- **biome**: Hosts packages required for an on-prem Biome Builder deployment.

## Where can I create an origin

You can create origins in an on-prem Biome Builder deployment.
[Biome's public Biome Builder](https://bldr.biome.sh) doesn't support creating new origins.

## Create an origin

{{< readfile file="/biome/reusable/md/create_origins_builder.md" >}}

### Create an origin with the Biome CLI

{{< readfile file="/biome/reusable/md/create_origins_cli.md" >}}

To create key pair for your origin, see the [origin keys](#origin-keys) documentation.

## Origin membership and role-based access control

Prerequisites:

- [Download the Biome CLI]({{< relref "install_biome" >}})
- [Create a Biome Builder account]({{< relref "#builder-account" >}})
- [Generate a personal access token]({{< relref "#builder-token" >}})
- [Create an origin]({{< relref "#create-origin" >}}) or accept an invitation to an existing origin
- [Get origin keys]({{< relref "#origin-keys" >}})


### Role-based access control (RBAC) for Biome Builder (SaaS and on-prem)

New in: 1.6.140

Role-based access control (RBAC) provides your organization with better operational safety by letting you assign specific levels of access to each user that belongs to an origin. With RBAC in place, existing standard origin 'members' from earlier versions are assigned the 'Maintainer' role. This role has similar permissions of the previous generic 'member' role, and the areas of difference are detailed below. The origin owner role remains unchanged.

When you join or create an origin, Biome Builder identifies your personal access token and assigns it a membership role for that origin. Your membership role defines the level of access that you have to the resources in an origin. By default, you're assigned the "read-only" role when you join an origin, and you're assigned the 'owner' role when you create an origin.

Biome Builder has the following RBAC origin member roles:

Read-Only
: This user can read an origin's packages, channels, origin membership, jobs, keys, integrations, invitations, roles, settings but cannot add to, change, or delete anything else in the origin, including uploading packages and inviting users to the origin. Read-Only is the default membership role for users joining the origin.

Member
: In addition to read-only access, an origin 'Member' can upload and build packages in the 'unstable' channel, but they cannot promote packages to other channels.

Maintainer
: Existing origin 'members' are now 'Maintainers'. This role has full read and write access to packages, channels, origin membership, jobs, integrations, invitations, settings. However, the 'Maintainer' role is more limited than the past role, in that 'Maintainers' only have read access to packages, channels, origin membership, jobs, keys, integrations, and settings. Origin 'Maintainers' can read origin membership roles and see and send invitations, but they cannot otherwise change origin membership--their own or anybody else's. Finally, 'Maintainers' can neither read nor write origin secrets.

Administrator
: In addition to 'Maintainer' access, the 'Administrator' role adds the missing privileges for writing origin keys and membership roles, as well as for reading and writing origin secrets. Administrators have full read and write access to packages, channels, origin membership, jobs, keys, integrations, invitations, roles, secrets, settings.

Owner
: As in the past, the origin 'Owner' has full read and write access to the origin. Only Owners can delete the origin or transfer ownership to another member.

### Comparison of RBAC Member Roles and Actions

| Action | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| **Packages** |
| View packages | Y | Y | Y | Y | Y |
| Upload packages to `unstable` | N | Y | Y | Y | Y |
| Promote packages from `unstable` | N | N | Y | Y | Y |
| **Build Jobs** |
| View build jobs | Y | Y | Y | Y | Y |
| Trigger `unstable` build job | N | Y | Y | Y | Y |
| **Channels** |
| View channels | Y | Y | Y | Y | Y |
| Add/Update/Delete channels | N | N | Y | Y | Y |
| **Origin Keys** |
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |
| **Origin Membership** |
| View origin membership | Y | Y | Y | Y | Y |
| View invitations | Y | Y | Y | Y | Y |
| Send Invitations | N | N | Y | Y | Y |
| Revoke Invitations | N | N | Y | Y | Y |
| **Member Roles** |
| View member roles | Y | Y | Y | Y | Y |
| Update member roles | N | N | N | Y | Y |
| **Origin Settings** |
| View settings | Y | Y | Y | Y | Y |
| Add/Update/Delete settings | N | N | N | Y | Y |
| **Origin Secrets** |
| View secrets | N | N | N | Y | Y |
| Add/Update/Delete secrets | N | N | N | Y | Y |
| **Cloud Integrations** |
| View integrations | Y | Y | Y | Y | Y |
| Add/Update/Delete integrations | N | N | Y | Y | Y |
| **Ownership** |
| Transfer Origin | N | N | N | N | Y |
| Delete Origin | N | N | N | N | Y |

### Manage origin membership

In tandem with the changes to the Builder membership roles, we've also updated the `bio` CLI to support RBAC. We're working on adding role management to the Biome Builder site, but in the meantime, you'll need to use the CLI for now.

![Manage origin membership](/images/biome/origin-members.png)

#### Manage origin membership with `bio origin invitations`

Manage Biome Builder origin membership with the Biome CLI, using the [bio origin invitations]({{< relref "biome_cli/#bio-origin-invitations" >}}) command.

All Biome Builder users can accept, ignore, and see invitations for their accounts.

View origin invitations:

```bash
bio origin invitations list
```

Accept origin invitations:

```bash
bio origin invitations accept <ORIGIN> <INVITATION_ID>
```

Ignore origin invitations:

```bash
bio origin invitations ignore <ORIGIN> <INVITATION_ID>
```

Send origin membership invitations:

```bash
bio origin invitations send <ORIGIN> <INVITEE_ACCOUNT>
```

Origin administrators and owners can see all pending origin membership invitations:

```bash
bio origin invitations pending <ORIGIN>
```

Origin administrators and owners can rescind an origin membership invitation:

```bash
bio origin invitations rescind <ORIGIN> <INVITATION_ID>
```

Origin owners can transfer origin ownership to another member:

```bash
bio origin transfer [OPTIONS] <ORIGIN> <NEW_OWNER_ACCOUNT>
```

#### Manage membership roles with `bio rbac`

You can use role based access control (RBAC) from the command line.
An origin `MEMBER_ACCOUNT` is the name used to sign in to Biome builder. You can find the list of user names on an origin's _Members Tab_. (Builder > Origin > Members)

The RBAC command syntax is:

```bash
bio origin rbac <SUBCOMMAND>
```

The syntax for the `show` subcommand is:

```bash
bio origin rbac show <MEMBER_ACCOUNT> --origin <ORIGIN>
```

See an origin member's RBAC role:

```
bio origin rbac show bluewhale --origin two-tier-app
```

The syntax for the `set` subcommand is:

```bash
bio origin rbac set [FLAGS] [OPTIONS] <MEMBER_ACCOUNT> <ROLE> --origin <ORIGIN>
```

Set an origin membership RBAC role with:

```bash
bio origin rbac set bluewhale admin --origin two-tier-app
```

## Origin keys

Prerequisites:

- [Download the Biome CLI]({{< relref "install_biome" >}})
- [Create a Biome Builder account]({{< relref "#builder-account" >}})
- [Generate a personal access token]({{< relref "#builder-token" >}})
- [Create an origin with `bio origin create` or join an origin by invitation]({{< relref "#create-origin" >}})

When you create an origin, Biome Builder automatically generates _origin keys_.
Origin key cryptography is asymmetric: it has a public origin key that you can distribute freely, and a private origin key that you should distribute only to users belonging to the origin.
All Biome Builder users with access to the origin can view the origin public key revisions in the origin key tab (Builder > Origin > Keys) and download the origin public key, but only users with the origin 'administrator' or 'owner' roles can view or download the origin private key, or change the origin key pair.

| Keys Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View keys | Y | Y | Y | Y | Y |
| Add/Update/Delete keys | N | N | N | Y | Y |

Biome uses origin keys:

- When you build an artifact in your local environment, Biome signs the artifact with a public key
- When you upload an artifact to Biome Builder or Builder on-prem, Biome verifies that the artifact was signed with its public key
- When you install an artifact on a Biome Supervisor, Biome uses the public origin key to authorize the artifact's installation; Biome only installs artifacts for which it has the public origin key
- When you download an artifact to your local environment, Biome uses the public origin key to verify the artifact's integrity before it starts the installation

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

### Keys tab

When you create an origin, Biome Builder automatically generates an origin key pair and saves both keys. To view your origin keys on Biome Builder, navigate to your origin and select the **Keys** tab. (Builder > Origins > Keys) You will always be able to view and download origin public keys, but you will only see the private keys for origins in which you are an "administrator" or "owner".

![Viewing your origin keys](/images/biome/origin-keys.png)

#### Download origin keys

Download your private or public origin key by selecting the **download** icon from the right end of the key details, under the _Actions_ heading.

![Detail of the download icon](/images/biome/origin-key-download.png)

#### Upload origin keys

You can upload origin keys that you generate on the command line to Biome Builder by selecting either the **Upload a private key** or **Upload a public key** icon, and copy your key into the form that appears.

![Example form content for uploading an origin key in Builder](/images/biome/builder-key-upload.png)

### Managing origin keys with the CLI

Run Biome CLI commands from your local environment or from within the Biome Studio.

See the CLI documentation for more information on the [`bio origin key`]({{< relref "biome_cli/#bio-origin-key" >}}) commands.

#### Find your local origin keys

Biome stores your public and private origin keys at `~/.bio/cache/keys` on Linux systems, `C:\bio\cache\keys` on Windows, and at `/bio/cache/keys` inside of the Biome Studio environment.

##### Find your origin keys in your local environment

On Windows:

```PowerShell
Get-ChildItem C:\bio\cache\keys
```

On Linux or macOS:

```bash
ls -la ~/.bio/cache/keys
```

##### Find your existing origin keys from inside of the Biome Studio

On Windows:

```powershell
Get-ChildItem C:\bio\cache\keys
```

On Linux or macOS:

```bash
ls -la /bio/cache/keys
```

#### Generate origin keys with the CLI

When you create an origin through the site, Biome Builder automatically generates an origin key pair.

The Biome CLI creates origin key pairs through two different commands, for two different uses:

- Use [`bio setup`]({{< relref "install_biome" >}}) to generate your first origin key pair as part of setting up the `bio` CLI
- Use the `bio origin key generate <ORIGIN>` command to create an key pair for an origin created with the `bio origin create` command

Create origin keys with the `bio` command:

```bio
bio origin key generate <ORIGIN>
```

#### Download origin keys with the CLI

To get your public origin key using the command line, use:

```bio
bio origin key download <ORIGIN>
```

#### Upload origin keys with the CLI

Creating an origin with the `bio origin create` command registers the origin on Biome Builder without creating an origin key pair. The `bio origin key generate` command creates the key pair and saves them in your local environment, but it does not upload either origin key to Biome Builder.

- Only "administrators" and "owners" can upload new keys to an origin.
- Builder requires the public origin key to upload artifacts for that origin, so you'll need to upload it.
- Builder requires the private origin key to enable new artifact builds from packages with plans linked to that origin.

Upload origin keys with the `bio` command:

```bio
bio origin key upload <ORIGIN>
```

Upload the origin private key:

```bio
bio origin key upload --secret <ORIGIN>
```

Upload both origin keys at the same time:

```bio
bio origin key upload  --secfile <PATH_TO_PRIVATE_KEY> --pubfile <PATH_TO_PUBLIC_KEY>
```

#### Import origin keys with the CLI

Use `bio origin key import` to read the key from a standard input stream into Biome Builder:

```bio
bio origin key import <enter or paste key>
bio origin key import <PATH_TO_KEY>
curl <URL_THAT_RETURNS_KEY> | bio origin key import
```

##### Troubleshoot origin key import

On a macOS, you may encounter an upload failure.
To remediate this failure:

 * Check that your `BIO_AUTH_TOKEN` environment variable is properly set and initialized
 * Add your `SSL_CERT_FILE` to the environment variables in your interactive shell configuration file, such as your `.bashrc`.


```bash
  export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

Initialize the setting from the command line with:

```bash
 source ~/.bashrc
```

## Origin settings

The **Origin Settings** tab contains:

- Default Package Settings
- Origin Secrets

Everyone with origin membership can see the _Settings_ tab, but only origin administrators and owners can add, update, or delete settings content.

| Settings Actions | Read-Only | Member | Maintainer | Administrator | Owner |
|---------|-------|-------|-------|-------|-------|
| View settings | Y | Y | Y | Y | Y |
| Add/Update/Delete settings | N | N | N | Y | Y |
| **Origin Secrets Actions** |
| View secrets | N | N | Y | Y | Y |
| Add/Update/Delete secrets | N | N | N | Y | Y |

![The administrator or owner's view of the origin settings tab with a public default package setting and a saved origin secret](/images/biome/origin-secrets.png)

### Default package settings

The _Default Package Settings_ define the visibility of build artifacts (.bart files). Everyone with origin membership can view the origin settings, but only origin administrators and owners can add, update, or delete settings.

- Public packages are visible in search results and can be used by every Biome Builder user
- Private artifacts do not appear in search results and are available only to users with origin membership

Change the default setting for the origin by switching from **Public Packages** to **Private Packages**. The default setting required for each origin and users with more than one origin can set some as public and others as private. Packages can have different default visibility settings than their origin's. Change the default visibility setting in for individual packages in that package's setting tab (Builder > Origin > Package > Settings).

### Origin secrets

Everyone with origin membership can view origin secrets, but only origin administrators and owners can add, update, or delete settings. _Origin Secrets_ are located at the bottom of the _Settings_ tab (Builder > Origin > Settings > Origin Secrets) and they let you encrypt and store secrets as environment variables. Origin secrets are useful for plans that require access to protected resources at build time, such as private source-code repositories and cloud storage providers.

Origin secrets are retained by the origin and are available for any of that origin's packages. The origin secrets in your local environment are encrypted with an origin encryption key. Only Biome Builder can read encrypted origin secrets.

#### Manage origin secrets with the Biome CLI

You can view the list of origin secrets and delete them in Biome Builder.
However, the primary way of interacting with origin secrets is with the Biome CLI.

##### List secrets

To list all of the secrets in an origin, use:

```bio
bio origin secret list --origin <ORIGIN>
```

##### Set origin secrets as environment variables

Add your origin secrets as environment variables in your local environment:

```bash
export BIO_ORIGIN=<ORIGIN>
export BIO_AUTH_TOKEN=<TOKEN>
bio origin secret list
```

##### Save an origin secret

To save an origin secret give the secret a name and the key value:

```bio
bio origin secret upload AWS_ACCESS_KEY_ID <your-key-id>
bio origin secret upload AWS_SECRET_ACCESS_KEY <your-secret-access-key>
```

The output should similar to:

```bash
$ bio origin secret upload AWS_ACCESS_KEY_ID 1234567890EXAMPLE
↓ Downloading latest public encryption key
    79 B / 79 B | [========================================] 100.00 % 120.23 KB/s
☑ Cached biocat-20200123456789.pub
☛ Encrypting value for key AWS_ACCESS_KEY_ID.
✓ Encrypted AWS_ACCESS_KEY_ID=[REDACTED].
↑ Uploading secret for key AWS_ACCESS_KEY_ID.
✓ Uploaded secret for AWS_ACCESS_KEY_ID.
```

##### Delete an origin secret

To delete an origin secret from an origin with the CLI

```bio
bio origin secret delete AWS_ACCESS_KEY_ID
bio origin secret delete AWS_SECRET_ACCESS_KEY
```

See [Using Origin Secrets in Plans]({{< relref "plan_writing/#buildtime-workflow" >}}) for guidance on using origin secrets.

See the [`bio origin secret`]({{< relref "biome_cli/#bio-origin-secret" >}}) CLI documentation for more information on these commands.
