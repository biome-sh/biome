+++
title = "About Biome Builder"
description = "Biome Builder is Chef's Application Delivery Enterprise hub"
gh_repo = "biome"
aliases = ["/habitat/using-builder/"]

[menu]
  [menu.biome]
    title = "Biome Builder"
    identifier = "habitat/builder/builder"
    parent = "habitat/builder"
    weight = 10

+++

Biome Builder acts as the core of Chef's Application Delivery Enterprise hub. Biome Builder was first launched as a cloud service and as the repository of all available plan templates built by Chef and the supporting community. Due to the fact that the application source code is stored alongside the build package, many users expressed a preference for storing packages and running Biome Builder on-prem. As a result, Biome Builder can be consumed either as a cloud based or on-premises solution. Plan files are stored in the Biome Builder SaaS, where they can be viewed and accessed by the Biome community and then shared with the on-premises version of the builder where they can then be copied and maintained locally.

## Biome Builder Enterprise Components Include:

* **Application Manifest** - The Application Manifest provides a single application directory, which includes---at a minimum---the compiled app artifact, dynamic links to all direct and transitive runtime dependencies ,and instructions to install and run the app.
* **Deployment Channel Management** -  Pre-canned deployment channels that can be used as-is or a user can custom design their own channels. Apps that are deployed through Biome can subscribe to a channel and be auto-upgraded whenever the app is promoted.
* **Origin Management** - Save your application delivery content in uniquely named spaces that you can control.
* **Content Library** - Hundreds of pre-built [application delivery packages](https://bldr.biome.sh/#/pkgs/core) and core dependencies, which makes it easy to get started with Biome.
* **Custom Data and Reporting APIs** - Rich APIs enable the capability to export to CSV or JSON.
* **DevOps Integration APIs** - Provides an API so that clients can find and download the necessary packages to run their applications. Additional APIs also enable easy integration with other popular DevOps tools, including Jenkins, Terraform, Artifactory, Hashi Vault, and many others.
* **Role Based User Access** - Improves your organizations operational safety by letting you assign specific levels of access to each origin member.

For more information on how the SaaS and On-Prem versions of Biome Builder work together, read the blog - [Biome Builder On-Prem Enhancements that Extend Support to Airgap Environments and Simplify Set-Up](https://www.chef.io/blog/chef-biome-product-announcement-builder-on-prem-enhancements-that-extend-support-to-airgap-environments-and-simplify-set-up)
