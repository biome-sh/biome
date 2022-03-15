+++
title = "Running Biome on Servers (Linux and Windows)"
description = "Running Biome on Servers (Linux and Windows)"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Running Biome on Servers (Linux and Windows)"
    identifier = "habitat/supervisors/running-biome-servers"
    parent = "habitat/supervisors"
    weight = 25
+++

Biome can be run on bare metal servers, as well as virtual machines. Currently, Biome can run on Linux and Windows platforms, and in all cases, running a Supervisor boils down to running `bio sup run`. How that happens depends on which platform you choose to use.

## Running Biome on Linux

First, you must [install Biome]({{< relref "install_habitat" >}}) itself on the machine.

Second, many packages default to running as the `hab` user, so you should ensure that both a `hab` user and group exist:

```bash
sudo groupadd bio
sudo useradd -g bio hab
```

Finally, you will need to wire Biome up to your systems init system. This may be SysVinit, systemd, runit, etc. The details will be different for each system, but in the end, you must call `bio sup run`.

### Running under systemd

A basic systemd unit file for Biome might look like this. This assumes that you have already created the `hab` user and group, as instructed above, and that your `bio` binary is linked to `/bin/bio`.

```toml
    [Unit]
    Description=The Biome Supervisor

    [Service]
    ExecStart=/bin/bio sup run

    [Install]
    WantedBy=default.target
```

Depending on your needs and deployment, you will want to modify the options passed to `bio sup run`. In particular, if you wish to participate in larger Supervisor networks, you will need to pass at least one `--peer` option.

## Running Biome on Windows

As with Linux, you must first [install Biome]({{< relref "install_habitat" >}}) on the machine. Unlike Linux, however, the Windows Supervisor has no requirements for any `hab` user.

On Windows, you can run the Supervisor as a Windows Service. You can use the `windows-service` Biome package to host the Supervisor inside the Windows Service Control Manager:

```powershell
bio pkg install biome/windows-service
```
