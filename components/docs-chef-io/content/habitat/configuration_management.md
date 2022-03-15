+++
title = "Configuration Management"
description = "Configuration Management"
draft = false
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Configuration Management"
    identifier = "habitat/reference/configuration-management"
    parent = "habitat/reference"

+++
**Examples: [Ansible](https://www.ansible.com/), [Chef](https://www.chef.io/products/chef-infra), [Puppet](https://puppet.com/), and [Salt](https://saltstack.com/)**

Configuration management tools allow you write configuration files, using a declarative language to manage a server. These tools focus on building working servers by installing and configuring system settings, system libraries, and application libraries before an application is installed on the server. Biome focuses on the application first instead of the server. Biome builds and packages your application's entire binary toolchain, including the system libraries, application libraries, and runtime dependencies necessary for your application to function. As a result, Biome can replace many use-cases that configuration management tools perform related to installing system binaries, application dependent libraries, or templating configuration files.

Configuration management tools perform tasks at run time by converging resources. The value from configuration management tools comes from this converging process -- checking the existing state of a server, and fixing it if it does not match the intended state. Because converging modifies resources at runtime, it can result in surprising and complex runtime errors. In addition, since environments are often mutable and unique, maintaining server automation occurs out-of-band with application development, creating conflict between application developers and software reliability engineers. Biome avoids these classes of errors entirely by shifting these processes to build time, and by creating an atomic package of an application's binaries, application lifecycle hooks, and configuration files. Biome's approach to packaging automation with the application package allows application developers and software reliability engineers to work closer together.

Biome is not a full replacement for configuration management tools on mutable infrastructure. Instead, it allows configuration management tools to focus better on system-level tasks for virtual machines and bare metal, such as kernel tuning, system hardening tasks, and compliance remediation tasks. Biome can then take over application automation roles, which results in a significant reduction in automation complexity for both infrastructure-focused automation and application-focused automation.

Biome can make it easier to run your existing configuration management tool. You can create a Biome package of your configuration management tool's agent and/or dependencies, and run it on your existing mutable infrastructure. The Biome Supervisor's responsibility is to update your configuration management tool's agent, while your configuration management tool can still perform its normal tasks.

Biome can provide an easier transition from virtual machine or bare metal workloads to containers, without needing to rewrite a monolithic application into microservices all at once. In this scenario, you can run the [Biome Supervisor]({{< relref "sup" >}}) on your existing virtual machine or bare metal infrastructure as you migrate away from your configuration management tool. Then, when you're ready, you export your application to the container format of your choice using the [Biome Studio]({{< relref "pkg_build" >}}). While you migrate your applications and services, the [Biome Supervisor]({{< relref "sup" >}}) runs on your existing mutable infrastructure, and runs your existing configuration management tool. New packages that do not require configuration management can also run under the [Biome Supervisor]({{< relref "sup" >}}) on your existing mutable infrastructure. As a result, you can continue to verify the working state of your application as you incrementally migrate your services. This approach provides an alternative to the "all-or-nothing" migration many teams are faced with when moving workloads to containers.
