+++
title = "Create an Origin"
description = "Create an Origin on Biome Builder"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Create an Origin"
    identifier = "habitat/origins Create an Origin"
    parent = "habitat/origins"
    weight = 10
+++

An origin is a space on Biome Builder where you can store, share, and build packages. It is a unique namespace within Biome Builder, and while you can delete or transfer an origin, you can't rename an origin after it is created. One example of an origin is the "core" origin, which is the set of foundational packages managed and versioned by the core Biome maintainers.

You can join existing origins by invitation and you can create your own origins.
For more on invitations, see [origin membership and RBAC]({{< relref "origin_rbac.md#origin-membership" >}}).

### Create an Origin

![Biome Builder without origins](/images/habitat/create-origin.png)

To create an origin, select the **Create origin** button on the _My Origins_ page which opens the _Create New Origin_ form. (Biome Builder > My Origins )

![Creating an origin](/images/habitat/create-origin-form.png)

First, enter a unique name that you want to associate with your packages.  Biome will only let you create an origin with a unique name. Some examples that you'll see in Biome Builder are team names, user names, and abstract concepts.

Next, choose a privacy setting to set as the default for new packages. You can override this setting when uploading individual packages from the CLI or by connecting a plan file that declares a package as private. The difference between public and private packages is:

- Anyone can find and use public packages
- Only users with origin membership can find and use private packages

When you select **Save and Continue**, Biome Builder:

1. Creates your origin
1. Creates an [origin key pair]({{< relref "origin_keys.md" >}})
1. Redirects Biome Builder to the origin page

![Origin successfully created](/images/habitat/create-origin-done.png)

#### Create an Origin with the Biome CLI

Use the [bio origin]({{< relref "biome_cli.md#bio-origin" >}}) commands to manage your origins from the command line.

Create an origin from the command line with the [bio origin create]({{< relref "biome_cli.md#bio-origin-create" >}}) command

```bash
bio origin create <origin>
```

The results of this command differ slightly from creating an origin on the Biome Builder site. The CLI command:

1. Creates an origin on the Biome Builder site
1. Does _not_ generate an origin key pair

For more information, see the [`bio origin create`]({{< relref "biome_cli.md#bio-origin-create" >}}) CLI documentation.
