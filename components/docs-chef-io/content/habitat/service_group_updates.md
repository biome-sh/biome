+++
title = "Service Group Updates"
description = "Update service groups with supervisor configuration"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Service Group Updates"
    identifier = "habitat/services/Service Group Updates"
    parent = "habitat/services"
    weight = 50
+++

The Biome Supervisor can be configured to leverage an optional _update strategy_,
which describes how the Supervisor and its peers within a service group should
respond when a new version of a package is available.

To use an update strategy, configure the Supervisor to subscribe to Biome
Builder, and more specifically, a channel for new versions.

## Configuring an Update Strategy

Biome supports three update strategies: `none`, `rolling`, and `at-once`.

To start a Supervisor with the auto-update strategy, pass the `--strategy` argument
to a Supervisor run command, and optionally specify the depot URL:

```bash
bio sup run --strategy rolling --url https://bldr.biome.sh
bio svc load <ORIGIN>/<NAME>
```

### None Strategy

This strategy means your package will not automatically be updated when a newer
version is available. By default, Supervisors start with their update strategy
set to `none` unless explicitly set to one of the other two update strategies.

### Rolling Strategy

This strategy requires Supervisors to update to a newer version of their package
one at a time in their service group. An update leader is elected which all Supervisors
within a service group will update around. All update followers will first ensure
they are running the same version of a service that their leader is running, and
then the leader will poll Builder for a newer version of the service's package.

Once the update leader finds a new version, it will update and wait until all other
alive members in the service group have also been updated before once again attempting
to find a newer version of software to update to. Updates will happen more or less
one at a time until completion with the exception of a new node being introduced into the service
group during the middle of an update.

If your service group is also running with the `--topology leader` flag, the leader
of that election will never become the update leader, so all followers within a leader
topology will update first.

It's important to note that because we must perform a leader election to determine
an update leader, *you must have at least 3 Supervisors running a service group
to take advantage of the rolling update strategy*.

### At-Once Strategy

This strategy does no peer coordination with other Supervisors in the service group;
it merely updates the underlying Biome package whenever it detects that a
new version has either been published to a depot or installed to the local Chef
Biome `pkg` cache. No coordination between Supervisors is done, each Supervisor
will poll Builder on their own.
