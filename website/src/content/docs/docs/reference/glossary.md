---
title: Glossary
description: The words Homeport uses, and what they mean.
---

Homeport names things precisely. These terms appear throughout the app and these docs.

### Host

A Docker Engine that Homeport connects to, defined by a name and how to reach it: a local socket, TCP, TLS or SSH. Every container, image, volume, network and Stack belongs to exactly one Host. See [Hosts and connections](/docs/hosts/).

### Engine kind

What sort of Engine a host points to: Docker-compatible (Docker Engine, or Podman through its compatible API), containerd or Incus. It decides which pages a host shows: containerd and Incus hosts get their own. Podman isn't a kind of its own: Homeport recognises it each time it connects. See [Engine kinds](/docs/hosts/#engine-kinds).

### Namespace (containerd's)

On a containerd host, the space every container, image and snapshot lives in: nerdctl uses `default`, Kubernetes `k8s.io`, Docker `moby`. A containerd host's pages show one at a time. A Cluster's Namespaces are Kubernetes ones. See [containerd namespaces](/docs/hosts/containerd/#namespaces).

### Snapshot

On a containerd host, a layer of files on disk: an unpacked image layer, or a container's writable layer. See [snapshots](/docs/hosts/containerd/#snapshots). On an Incus host, a saved state of an instance that it can be restored to; see [instance snapshots](/docs/guides/instances/#snapshots).

### Instance

What an Incus host runs: a system container, which boots a whole Linux distribution on the host's kernel, or a virtual machine with a kernel of its own. Not a container, which runs one application. See [Instances](/docs/guides/instances/).

### Project (Incus's)

On an Incus host, the space its instances, images, profiles, custom volumes and networks live in. An Incus host's pages show one at a time, picked under the host switcher. See [projects](/docs/hosts/incus/#projects).

### Profile

On an Incus host, configuration and devices shared by the instances that list it. See [profiles](/docs/guides/instances/#profiles).

### Trust token

What `incus config trust add` prints on an Incus server, pasted into Homeport once to add that server over HTTPS. See [trust tokens](/docs/hosts/incus/#over-https-with-a-trust-token).

### Pod

On a Podman host, a group of containers that share one network and start and stop together, around an infra container that holds the ports the pod publishes. A Cluster's Pods are Kubernetes Pods. See [Podman pods](/docs/guides/podman-pods/).

### Quadlet unit

A file on a Podman host, such as `web.container`, that systemd turns into a service, `web.service`, so Podman's containers start at boot and systemd keeps them running. See [Quadlet units](/docs/guides/quadlet/).

### Cluster

A Kubernetes cluster or a Nomad cluster Homeport connects to through its API. A Kubernetes one is added from a context of a kubeconfig: a name, the API server's address, and the credentials the kubeconfig gives, stored encrypted. A Nomad one is added by the address of one of its agents, with an ACL token and TLS material, stored encrypted. Homeport installs nothing on either and only reaches what those credentials allow. Not a Swarm, which is found through its hosts and never added. Kubernetes' own objects keep their Kubernetes names: Namespace, Pod, Deployment, Service, Node… Nomad's keep Nomad's: job, task group, task, allocation, client node, namespace. See [Clusters](/docs/clusters/) and [Nomad clusters](/docs/clusters/nomad/).

### Allocation

On a Nomad cluster, one placement of a job's task group on a client node, running its tasks. Its logs and a shell in it go through Nomad's servers to the node. See [logs and shell](/docs/clusters/nomad/#logs-and-shell).

### Swarm

A Docker Swarm cluster, found through the hosts that belong to it and never added on its own. Homeport reaches it through any connected manager host, tells Swarms apart by their swarm ID, and stores nothing about one. It's named after its first manager host, since Docker gives every Swarm the same name. See [Swarms](/docs/guides/swarm/).

### Port forward

A port on this computer's loopback that Homeport connects, through the Kubernetes API, to a port of a Pod, or of a Service through one of its ready Pods, like `kubectl port-forward`. It ends when Homeport quits. See [Port forwarding](/docs/clusters/port-forward/).

### Release

A Helm release: one install of a chart on a Cluster, named in its Namespace, with a numbered revision for each install, upgrade and rollback. Helm keeps it on the cluster; a release made in a terminal and one made in Homeport are the same. See [Helm releases](/docs/clusters/helm/).

### Chart repository

A server helm downloads charts from, in helm's own list on this computer — the one `helm repo list` shows. Charts in an OCI registry are installed by their `oci://` reference instead. See [chart repositories](/docs/clusters/helm/#chart-repositories).

### Node

One Engine in a Swarm: a manager or a worker. A Cluster's Nodes are Kubernetes Nodes, the machines it runs Pods on. A node is a host only when Homeport connects to it, and a host's page links to its node.

### Service

What a Swarm runs: an image, its settings and a number of replicas, or one task on every node for a global service.

### Task

One slot of a service on one node, run as a container there. When a service doesn't converge, its tasks say why. On a containerd host, a task is what runs for a container, which is only metadata: starting a container creates its task.

### Stack

A Compose project running on a Host, or a swarm stack running on a Swarm, identified by its project name. Every Stack is Managed, Linked, Synced or External. See [Stacks](/docs/guides/stacks/) and [swarm stacks](/docs/guides/swarm/#swarm-stacks).

### Managed Stack

A Stack whose compose definition and environment Homeport holds, so it can be deployed, redeployed and edited from Homeport.

### Linked Stack

A Stack whose compose files are a folder on this computer. Homeport keeps where the folder and its compose files are, reads and writes the files in place, and deploys with that folder as the project directory, so `build:`, `env_file:` and relative paths work. Found from the compose labels of running containers, or created from a folder. It becomes External while its files are missing. See [Linked Stacks](/docs/guides/stacks/#linked-stacks).

### Synced Stack

A Stack whose compose definition comes from a git repository. Homeport keeps a clone of it on this computer, fetches a branch or tag on demand or on an interval while Homeport is open, shows what changed, and deploys a commit from the compose file's folder, so `build:` and relative paths work. It's never edited in Homeport: its files change through git; only its environment and sync settings are Homeport's. See [Synced Stacks](/docs/guides/synced-stacks/).

### Repository

A git repository Homeport may read for Synced Stacks: its URL and how Homeport authenticates to it — a deploy key it generated, a user name and token, or nothing for a public one. Kept once for every Host, and shared by every Synced Stack made from it.

### Deploy key

The Ed25519 key Homeport generates for one repository, which you add to it on your git forge with read access only. Never your own SSH key.

### External Stack

A Stack found running on a Host whose compose definition Homeport doesn't have, neither stored, in a folder on this computer, nor in a repository it follows. It can be started, stopped and removed, but not redeployed or edited.

### Adopt

To turn an External Stack into a Managed Stack by supplying its compose definition.

### Registry login

The user name and password or access token Homeport keeps for one container registry, such as Docker Hub or GHCR, and uses on every Host to pull, push and build. See [Settings](/docs/guides/settings/#registries).

### Helper container

A short-lived `busybox` container Homeport creates on a Host to open a volume — to browse, back up, restore or clone it — since the Engine API can't read one. It has no network, mounts the volume read-only unless it must write to it, and is removed when the action ends. It never shows in the Containers list or on the Events page. See [Browsing a volume's files](/docs/guides/volumes-networks/#the-helper-container).

### Board

The page listing the containers of every connected Host in one table, each labelled with its Host. See [Board](/docs/guides/overview/#board).

### Command palette

The search box opened with <kbd>Mod</kbd> <kbd>K</kbd>: go to a page, switch Host, run a command, or find a container, image, volume, network or Stack on any connected Host. See [Keyboard](/docs/guides/keyboard/#command-palette).

### Engine

The Docker Engine (`dockerd`), or a compatible one such as Podman, that runs containers on a machine. Homeport talks to it through the Docker Engine API.

### Socket

The address an Engine listens on: a Unix socket file such as `/var/run/docker.sock`, or a named pipe such as `//./pipe/docker_engine` on Windows.

### Keychain

Your operating system's secure store for passwords and keys — macOS Keychain, Windows Credential Manager or a Linux Secret Service — where Homeport keeps the key that encrypts its secrets.

### Trust on first use

How Homeport checks SSH servers: you confirm a server's key fingerprint on the first connection, and Homeport refuses to connect if the key later changes until you confirm again.
