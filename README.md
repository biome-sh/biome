# Overview

This repo is community distro of [Chef Habitat Application Automation](https://github.com/habitat-sh/habitat).

It was generated from original using small tool [ForkMan](https://github.com/jsirex/forkman).

## Backporting changes

Here is how can you proceed:

1. Install `forkman` biome/habitat package: `bio pkg install -fb jsirex/forkman`
2. Clone this project
3. Update and **commit** [Forkman Configuration File](.forkman.yaml)
4. Add `habitat` repo as remote: `git remote add habitat https://github.com/habitat-sh/habitat.git`
5. Fetch latest changes: `git fetch habitat master`
6. Make sure you have up to date with `origin/forkman-raw` `forkman-raw` branch.
7. Patch project with  `forkman-patch-git -u habitat/master`
8. **Avoid** manual **changing** of `forkman-raw` branch, because new run will override it.
9. Create new branch from `origin/master` and try to merge `forkman-raw`.
10. If you satisfied with forkmans' job push changes / create PR for your branch and `forkman-raw` branch.
11. If after all you still need some manual fixes related directly to biome, do it as you do with regular project - send a PR.

It is highly recommended to not change code by hand because it requires effort for further support.
If it is possible to update `forkman`s' dictionary - update dictionary.

# Contributing

If you want new feature or bug fix - it is good idea to contribute to original [Chef Habitat Application Automation](https://github.com/habitat-sh/habitat) project.
