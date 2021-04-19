+++
title = "Pattern Library Introduction"
description = "Example code for Biome plans and more!"
gh_repo = "biome"
aliases = ["/habitat/pattern-library/"]

[menu]
  [menu.biome]
    title = "Pattern Library"
    identifier = "habitat/pattern_library"
    parent = "habitat/reference"
+++
## Biome Pattern Library

The Biome Pattern Library is an evolving set of design patterns to use as starting-points. These patterns are examples and require configuration and customization for your unique situation.

For help with Biome and these patterns, ask:

- Your customer support agent
- In the [Chef Discourse](https://discourse.chef.io/c/habitat/)

## Kubernetes Bastion Ring Pattern

A _bastion ring_ is a robust type of Supervisor network in which a small number of Supervisors are set up as permanent peers and that are dedicated to anchoring Supervisor network communication. These Supervisors are designated solely for communication between Supervisor and _do not run services_. These solely to anchor the entire Supervisor network. See [Supervisor Networks]({{< relref "sup_networks" >}}) for more information. The following examples demonstrate running a bastion ring in Kubernetes.

### Kubernetes Bastion Ring Plan

```bash
pkg_name=bio_bastion
pkg_origin=biome
pkg_version="0.1.0"
pkg_maintainer="irvingpop"
pkg_license=("Apache-2.0")
pkg_deps=(core/busybox-static)
pkg_svc_run="while true; do sleep 60; done"

do_build() {
  return 0
}

do_install() {
  return 0
}
```

### Kubernetes Bastion Ring Producer Pattern

```yaml
---
apiVersion: v1
kind: Service
metadata:
  name: bio-bastion
spec:
  ports:
  - name: gossip-listener
    protocol: UDP
    port: 9638
    targetPort: 9638
  - name: http-gateway
    protocol: TCP
    port: 9631
    targetPort: 9631
  selector:
    app: bio-bastion
  clusterIP: None

---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: bio-bastion
spec:
  selector:
    matchLabels:
      app: bio-bastion
  serviceName: bio-bastion
  replicas: 1
  template:
    metadata:
      labels:
        app: bio-bastion
    spec:
      terminationGracePeriodSeconds: 10
      securityContext:
        fsGroup: 42
      containers:
      - name: bio-bastion
        image: irvingpop/bio_bastion:latest
        args:
        - '--permanent-peer'
        resources:
          requests:
            memory: "100Mi"
            cpu: "100m" # equivalent to 0.1 of a CPU core
        ports:
        - name: gossip-listener
          protocol: UDP
          containerPort: 9638
        - name: http-gateway
          protocol: TCP
          containerPort: 9631
        readinessProbe:
          httpGet:
            path: /
            port: 9631
          initialDelaySeconds: 5
          periodSeconds: 10
        livenessProbe:
          httpGet:
            path: /
            port: 9631
          initialDelaySeconds: 15
          periodSeconds: 20
        volumeMounts:
        - name: bio-bastion
          mountPath: /hab/sup
  volumeClaimTemplates:
  - metadata:
      name: bio-bastion
    spec:
      accessModes: [ "ReadWriteOnce" ]
      # uncomment if you don't have a default storageclass
      # storageClassName: "standard"
      resources:
        requests:
          storage: 10Gi
```

### Kubernetes Bastion Ring Consumer Pattern

```yaml
  apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: cockroachdb
spec:
  selector:
    matchLabels:
      app: cockroachdb
  serviceName: cockroachdb
  replicas: 3
  template:
    metadata:
      labels:
        app: cockroachdb
    spec:
      terminationGracePeriodSeconds: 10
      securityContext:
        fsGroup: 42
      containers:
      - name: cockroachdb
        image: irvingpop/cockroach:latest
        args:
        - --peer
        - bio-bastion
        - --topology
        - leader
#        env:
#        - name: HAB_COCKROACH
#          value: |
        resources:
          requests:
            memory: "300Mi"
            cpu: "500m" # equivalent to 0.5 CPU core
        ports:
        - name: http
          containerPort: 8080
        - name: cockroachdb
          containerPort: 26257
        volumeMounts:
        - name: cockroachdb-data
          mountPath: /hab/svc/cockroach/data
  volumeClaimTemplates:
  - metadata:
      name: cockroachdb-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 10Gi
```

## bio pkg download Patterns

The `bio pkg download` command can be used to download individual
packages (along with their dependencies and keys) from Builder,
without installing them. This allows you to more easily transfer
packages from one Builder instance to another, or to take a selective
snapshot of particular packages.

While you can download packages one-at-a-time, it can be more
convenient to use a file to specify your packages. Two formats are
recognized: plain text and TOML.

### Plain Text Download Descriptors

The simplest thing you can do is create a plain text file with a
package identifier on each line, like so:

```text
# These are the packages needed to run a Supervisor
biome/bio-launcher
biome/bio
biome/bio-sup
```

Each line is a valid package identifier. You can also add comments using `#`.

To download these packages (and their dependencies), save that to a file named `supervisor.txt` and run:

```bash
bio pkg download --file=supervisor.txt
```

This will download the packages into your existing Biome cache directory.
Alternatively, you can specify a directory using the `--download-directory` option.

(You can also specify `--channel` and `--target` to further control which specific packages you download; run `bio pkg download --help` for more).

### TOML Download Descriptors

Plain text is fine for simple cases, but has drawbacks.
For instance, all packages will come from the same channel and will be for the same platform target.
For maximum flexibility, you'll want to use TOML to
write your download descriptor. Here is an example of one that the Biome core team uses to take periodic snapshots of everything needed to run Builder itself:

```toml
format_version = 1
file_descriptor = "Packages needed to run an instance of Builder"

[[x86_64-linux]]
channel = "stable"
packages = [
  # Supervisor and prerequisites
  "biome/bio-launcher",
  "biome/bio",
  "biome/bio-sup",

  # Utilities
  "core/sumologic",
  "core/nmap"
]

# Targets can be repeated to specify additional subsets of packages,
# possibly from different channels
[[x86_64-linux]]
channel = "stable"
packages = [
  # Builder services
  "biome/builder-api",
  "biome/builder-api-proxy",
  "biome/builder-jobsrv",
  "biome/builder-worker",
  "habitat/builder-memcached",
]

[[x86_64-linux-kernel2]]
channel = "stable"
packages = [
  # Supervisor and prerequisites
  "biome/bio-launcher",
  "biome/bio",
  "biome/bio-sup",

  "biome/builder-worker"
]

[[x86_64-windows]]
channel = "stable"
packages = [
  # Supervisor and prerequisites
  "biome/windows-service",
  "biome/bio",
  "biome/bio-sup",

  "biome/builder-worker"
]
```

This format allows us to specify multiple subsets of packages from different channels and for different architectures.
Here, we are pulling down all the core service packages, which run on Linux, but are also pulling down the platform-specific versions of the
`biome/builder-worker` package.
Without this format, we would have to invoke `bio pkg download` multiple times with different parameters.
The file allows us to capture our full intention in one place.
