+++
title = "Biome and Containers"
description = "Biome and Containers"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Biome and Containers"
    identifier = "habitat/containers/containers"
    parent = "habitat/containers"
    weight = 10

+++
**Examples: [Docker](https://www.docker.com/) and [CoreOS](https://coreos.com/)**

Containers enable you to build an immutable snapshot of your runtime environment, including your operating system, system libraries, application libraries, and application. The container is built with a CLI tool, and then pushed to a container-specific artifact repository, known as a container registry. Biome is not a container format and exports your application to the container format of your choice.

Biome builds more secure containers by exporting your application and any of its runtime dependencies directly into the container. When you build your application with Biome, your application takes ownership of the entire toolchain of its runtime dependencies. As a result, you no longer have to rely on a large operating system and unnecessary system libraries. This enables you to include only the binaries your application uses inside your container, which can decrease your container size. By eliminating the need for a large operating system, you also avoid including binaries that an attacker could use, which further increases the security of your container. Visit the [Running Biome Containers]({{< relref "running_biome_linux_containers" >}}) section of the docs to find more details about how containers are built with Biome. Finally, [Biome's HTTP API]({{< relref "monitor_services" >}}) allows all of your application's runtime dependencies to be audited at any time. If a brand new vulnerability is revealed, [Biome's HTTP API]({{< relref "monitor_services" >}}) makes it easy to programmatically inspect and audit the entire toolchain of your runtime environment without needing to worry about how your containers got built in the first place.

If your situation requires it, Biome makes it simple to switch from containerized to non-containerized workloads. This is because Biome packages only have a requirement on the kernel version of your operating system. (Linux: kernel 2.6.32 or later, Windows: Windows Server 2012 or later and Windows 8 64-bit or later). You can take the same .hart file you use to export to a Docker container and run it on a virtual machine or bare metal. By only requiring the kernel, Biome gives you the ability to switch container formats or to switch to non-containerized workloads without significant rework.

