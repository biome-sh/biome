+++
title = "About Biome"
aliases = ["/habitat/reference/", "/habitat/glossary/", "/habitat/diagrams/"]
gh_repo = "biome"

[cascade]
  product = ["biome"]

[menu]
  [menu.biome]
    title = "About Biome"
    identifier = "habitat/About Biome"
    parent = "biome"
    weight = 5
+++

Biome centers application configuration, management, and behavior around the application itself, not the infrastructure that the app runs on.
It provides automation that can programmatically and declaratively build, deploy, and manage your application and services, both stateful and stateless.
You can deploy and run your Biome app on many different infrastructure environments including bare metal, VM, containers, and PaaS.

## Biome Builder

[Biome Builder]({{< relref "builder_overview" >}}) acts as the core of Chef's Application Delivery Enterprise hub. You can run Biome Builder as a cloud-based service or on-premises.

Biome Builder provides package storage, search, and an API for clients.

The contents of your app are stored in the Biome Builder SaaS, where the Biome community can view and access them. You can also use the on-prem version of Biome Builder, where you can store and maintain your apps locally.

## Plans

A [plan]({{< relref "plan_writing" >}}) is the file where you define how you will build, deploy, and manage your app. A plan file is named `plan.sh` for Linux systems or `plan.ps1` for Windows, and your app can have plan files for both Linux and Windows operating systems. You can find your plan file in the `bioitat` directory, which you install at the root of your app with `bio plan init`.

## Supervisor

A Supervisor is a process manager that runs the app packages that you defined in your plan. A Supervisor has two primary responsibilities:

1. A Supervisor is a process manager and is responsible for running your app's services. It starts, stops, updates, and monitors the services according to your plan.
1. Supervisors can talk to each other. You can connect Supervisors together into a network and instruct them to send information to each other and take actions based on that information.

## Services

A [service]({{< relref "about_services" >}}) is your Biome package that is run and managed by a Supervisor. Services can be joined together into a [service group]({{< relref "service_groups" >}}), which is a collection of services with the same package and topology type that are connected together across a Supervisor network.

## Installing Biome

The Biome CLI can be [installed]({{< relref "install_habitat" >}}) on Linux, Mac, and Windows.
