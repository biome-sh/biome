+++
title = "Biome Overview"
aliases = ["/habitat/reference/", "/habitat/glossary/", "/habitat/diagrams/"]
gh_repo = "biome"

[cascade]
  product = ["biome"]

[menu]
  [menu.biome]
    title = "Overview"
    identifier = "habitat/Overview"
    parent = "biome"
    weight = 1
+++

Biome is a workload-packaging, orchestration, and deployment system that allows you to build, package, deploy, and manage applications and services without worrying about which infrastructure your application will deploy on, and without any rewriting or refactoring if you switch to a different infrastructure.

Biome separates the platform-independent parts of your application—the build dependencies, runtime dependencies, lifecycle events, and application codebase—from the operating system or deployment environment that the application will run on, and bundles it into an immutable Biome Package.
The package is sent to the Biome Builder (SaaS or on-prem), which acts as a package store like Docker Hub where you can store, build, and deploy your Biome package.
Biome Supervisor pulls packages from Biome Builder, and will start, stop, run, monitor, and update your application based on the plan and lifecycle hooks you define in the package.
Biome Supervisor runs on bare metal, virtual machines, containers, or Platform-as-a-Service environments.
A package under management by a Supervisor is called a service.
Services can be joined together in a service group, which is a collection of services with the same package and topology type that are connected together across a Supervisor network.

## Components

### Biome Builder

{{< readfile file="content/habitat/reusable/md/biome_builder_overview.md" >}}

For more information, see the [Biome Builder]({{< relref "/habitat/builder_overview" >}}) documentation.

### Biome Package

A Biome Package is an artifact that contains the application codebase, lifecycle hooks, and a manifest that defines build and runtime dependencies of the application.
The package is bundled into a Biome Artifact (.HART) file, which is a binary distribution of a given package built with Biome.
The package is immutable and cryptographically signed with a key so you can verify that the artifact came from the place you expected it to come from.
Artifacts can be exported to run in a variety of runtimes with zero refactoring or rewriting.

### Plan

{{< readfile file="content/habitat/reusable/md/biome_plans_overview.md" >}}

For more information, see the [plan]({{< relref "plan_writing" >}}) documentation.

### Services

{{< readfile file="content/habitat/reusable/md/biome_services_overview.md" >}}

See the [services documentation]({{< relref "about_services" >}}) for more information.

### Biome Studio

{{< readfile file="content/habitat/reusable/md/biome_studio_overview.md" >}}

See the [Biome Studio documentation]({{< relref "studio" >}}) for more information.

### Biome Supervisor

{{< readfile file="content/habitat/reusable/md/biome_supervisor_overview.md" >}}

See the [Biome Supervisor documentation]({{< relref "sup" >}}) for more information.

## When Should I Use Biome?

Biome allows you to build and package your applications and deploy them anywhere without having to refactor or rewrite your package for each platform.
Everything that the application needs to run is defined, without assuming anything about the underlying infrastructure that the application is running on.

This will allow you to repackage and modernize legacy workloads in-place to increase their manageability, make them portable, and migrate them to modern operating systems or even cloud-native infrastructure like containers.

You can also develop your application if you are unsure of the infrastructure your application will run on, or in the event that business requirements change and you have to switch your application to a different environment.

## Next Steps

- [Download and install the Biome CLI]({{< relref "/habitat/install_habitat" >}}).
- [Create an account]({{< relref "/habitat/builder_account" >}}) on the [Biome Builder SaaS](https://bldr.habitat.sh).
- Try our [getting started guide](get_started) for Biome.

## Additional Resources

### Download

- [Download Biome](https://www.chef.io/downloads/tools/habitat)
- [Install documentation]({{< relref "/habitat/install_habitat" >}})

### Learning

- [Learn Chef: Deliver Applications with Biome](https://learn.chef.io/courses/course-v1:chef+Biome101+Perpetual/about)
- [Biome webinars](https://www.chef.io/webinars?products=chef-habitat&page=1)
- [Resource Library](https://www.chef.io/resources?products=chef-habitat&page=1)

### Community

- [Biome on Discourse](https://discourse.chef.io/c/habitat/12)
- [Biome in the Chef Blog](https://www.chef.io/blog/category/chef-habitat)
- [Biome Community Resources](https://community.chef.io/tools/chef-habitat)

### Support

- [Chef Support](https://www.chef.io/support)

### GitHub Repositories

- [Biome repository](https://github.com/biome-sh/biome)
- [Biome Core Plans repository](https://github.com/habitat-sh/core-plans)
- [Biome Builder repository](https://github.com/biome-sh/builder)
- [Biome Builder on-prem repository](https://github.com/biome-sh/on-prem-builder)
