# Overview

This repo is community distro of [Chef Habitat Application Automation](https://github.com/habitat-sh/habitat).

It was generated from original using small tool [ForkMan](https://github.com/jsirex/forkman).

## Unforked

**NOTE: History was recreated! Force-Push to master has happened**

The last version fully compatible with Habitat was: 2.0.107. It is hard to keeep
full compatibility, especially when services are not available without license.

As we need to reproduce everything from scratch, there obvious be conflicts. For
example, is `core/bash` from habitat builder or biome builder. Further changes
to this fork may eventually conflict with upstream. Decision made to *unfork*
Biome. Biome now have its own properties:

- Path changed, for example, **/bio** is the prefix
- Environment variable names changed, for example, **BIO_ORIGIN**

Currently: user and group ids, ports were left unchanged.

## Backporting changes (automated)

In the workflow [Forkman](.github/workflows/forkman.yml).

Here is how can you repat that workflow manually:

1. Install `forkman` biome/habitat package: `bio pkg install -fb jsirex/forkman`
2. Clone this project
3. Update and **commit** [Forkman Configuration File](.forkman.yaml)
4. Add `habitat` repo as remote: `git remote add habitat https://github.com/habitat-sh/habitat.git`
5. Fetch latest changes: `git fetch habitat main`
6. Make sure you have up to date with `origin/forkman-raw` `forkman-raw` branch.
7. Patch project with  `forkman-patch-git -u habitat/main`
8. Reformat code:
   1. Get config with `git checkout origin/master -- rustfmt.toml`
   2. Format code `cargo fmt`
   3. Amend changes `git add -u && git commit --amend`
9. Create new branch from `origin/master` and try to merge `forkman-raw`.
10. If you satisfied with forkmans' job push changes / create PR for your branch and `forkman-raw` branch.
11. If after all you still need some manual fixes related directly to biome, do it as you do with regular project - send a PR.

**Avoid** manual **changing** of `forkman-raw` branch, because new run will override it.
It is highly recommended to not change code by hand because it requires effort for further support.

# Contributing

There are multiple options to contribute, you can choose multiple options from:

- Create PR here
- Create PR in upstream [Chef Habitat Application Automation](https://github.com/habitat-sh/habitat) project

Currently I'm doing my best to pulling changes from upstream. But community is
**open to change the Biome project** significantly. That means that someday I can't pull
changes as projects become incompatible.

