+++
title = "Exporting Packages"
description = "Export Biome packages to Docker, Kubernetes, Helm, or as a tar file"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Exporting Packages"
    identifier = "habitat/packages/pkg-exports Export Biome Packages"
    parent = "habitat/packages"
    weight = 40
+++

Biome Artifacts--`.hart` files--can be exported in a number of different formats depending on what you need and where you need it. This is powerful because you can use the same immutable Biome artifact by exporting it into a format that you need for a specific job.

You can export packages to a Docker container image or a tar file.

The command to export a package is `bio pkg export <FORMAT> <PKG_IDENT>`. See the [Biome CLI Reference Guide]({{< relref "biome_cli#bio-pkg-export" >}}) for more CLI information.

> **Note** If you specify an `origin/package` identifier, such as `core/postgresql`, the Biome CLI will check Builder for the latest stable version of the package and export that.

> If you wish to export a package that is not on Builder, create a Biome artifact by running the `build` command, then point `bio pkg` to the `.hart` file within the `/results` directory:

```bash
bio pkg export tar ./results/example-app.hart
```

Read on for more detailed instructions.

### Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. On Linux, the exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.

1. Create an interactive studio with the `bio studio enter` command.

1. [Build]({{< relref "pkg_build" >}}) the Biome package from which you want to create a Docker container image and then run the Docker exporter on the package.

    ```bash
    bio pkg export docker ./results/<hart-filename>.hart
    ```

    > **Note** The command above is for local testing only. If you have uploaded your package to Builder, you can export it by calling `bio pkg export docker origin/package`. The default is to use the latest stable release; however, you can override that by specifying a different channel in an optional flag.

    > **Note** On Linux, exporting your Biome artifact to a Docker image requires the Docker Engine supplied by Docker. Packages from distribution-specific or otherwise alternative providers are currently not supported.

    > **Note** In a Windows container studio, the `export` command will not be able to access the host docker engine. To export a Windows package or hart file built inside of a Windows container studio, first exit the studio and then export the `.hart` file in your local `results` directory.

1. You may now exit the studio. The new Docker image exists on your computer and can be examined with `docker images` or run with `docker run`.

1. Please note that when you run this docker container, you will need to pass the `HAB_LICENSE` environment variable into the container in order to accept the Biome license. If you don't, your container will abort at a license acceptance prompt. One way to do this would be `docker run --env HAB_LICENSE=accept-no-persist IMAGE`. Alternatively, if you use a scheduler to run these docker containers, you should add that environment variable to your scheduler configuration.

### Exporting to a Tarball

1. Enter the Biome studio by using `bio studio enter`.

2. Install or [build]({{< relref "pkg_build" >}}) the Biome package from which you want to create a tarball, for example:

    ```bash
    bio pkg install <ORIGIN>/<NAME>
    ```

3. Run the tar exporter on the package.

    ```bash
    bio pkg export tar <ORIGIN>/<NAME>
    ```

    If you receive an error, try running

    ```bash
    bio pkg export tar /results/<your_package>.hart
    ```

4. Your package is now in a tar file that exists locally on your computer in the format `<ORIGIN>-<NAME>-<VERSION>-<TIMESTAMP>.tar.gz` and can be deployed and run on a target machine.

5. If you wish to run this tar file on a remote machine (i.e. a virtual machine in a cloud environment), scp (or whatever transfer protocol you prefer) the file to whatever you wish to run it.

6. SSH into the virtual machine

7. Run these commands to set up the required user and group:

    ```bash
    sudo adduser --group bio
    sudo useradd -g bio hab
    ```

8. Next, unpack the tar file:

    ```bash
    sudo tar xf your-origin-package-version-timestamp.tar.gz
    sudo cp -R bio /hab
    ```

9. Now, start the Supervisor and load your service package using the `bio` binary, which is included in the tar archive:

    ```bash
    sudo /hab/bin/bio sup run
    sudo /hab/bin/bio svc load <ORIGIN>/<NAME>
    ```
