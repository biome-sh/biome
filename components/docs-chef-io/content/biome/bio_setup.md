+++
title = "Configure the Biome CLI"
description = "Set up the Biome CLI"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Set up the Biome CLI"
    identifier = "habitat/install/bio-setup Install Biome"
    parent = "habitat/install"
    weight = 20
+++

Once Biome has been installed, the `bio` CLI makes it easy to get your workstation configured by guiding through the setup process. To set up your workstation, run `bio cli setup` and follow the instructions.

<img alt="screenshot of bio cli setup output in CLI" src="/images/biome/bio-setup.png">

Setup asks you to create a new origin and a set of origin keys.

Optionally, you can also provide a Biome personal access token to upload packages to the public depot and share them with the Biome community. See the [access token documentation]({{< relref "builder_profile#create-a-personal-access-token" >}}) for details on generating and using your access token.

> For more information about using Biome Builder, see the section on [Using Builder]({{< relref "builder_overview" >}}).

You will also be asked if you want to register Supervisor control gateway secret (see [Remote Command-and-Control of Supervisors]({{< relref "sup_remote_control" >}}) for further details).

You can change your settings at any time by re-running the `bio cli setup` command.

<img alt="screenshot of completed bio cli setup in CLI" src="/images/biome/bio-setup-complete.png">

That's it. You're all set up and ready to use Biome to build and run packages!
