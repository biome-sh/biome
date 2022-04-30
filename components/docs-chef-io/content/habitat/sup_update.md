+++
title = "Updating the Supervisor"
description = "Updating the Supervisor"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Updating Supervisors"
    identifier = "habitat/supervisors/updating"
    parent = "habitat/supervisors"
    weight = 130

+++

Each new Biome release brings a new Supervisor that includes all bug fixes and enhancements since the last release. It's a good idea to stay relatively up to date with your running Supervisors. When updating Supervisors in production, you usually will not want to shut down all of your running services while you perform the update. That could incur an outage or require a maintenance window. Biome provides a couple of ways for you to update a Supervisor without stopping your running services.

## Manual Update

While an older version of the Supervisor is running, install the newer Supervisor into your local Biome package repository:

```bash
bio pkg install biome/bio-sup
```

This will not update the running Supervisor. It simply downloads the new Supervisor and stores it in your `/hab/pkgs` store. To update the running Supervisor, restart it with:

```bash
bio sup restart
```

This will not restart the running services; it will only restart the supervisor service.

## Automatic Update

You can configure the Supervisor to automatically update itself when new Supervisor releases become available. This is done by adding the `--auto-update` flag to `bio sup run` or by setting `auto_update` to `true` in `/hab/sup/default/config/sup.toml`. By default, the Supervisor will check for updates every 60 seconds. This can be adjusted by changing the `--auto-update-period` to a different number of seconds.

If you want the Supervisor to look for updated Supervisor releases in a on-prem depot or in a channel other than `stable`, then use the `--url` and `--channel` arguments of `bio sup run` to point to the desired Builder URL and release channel.
