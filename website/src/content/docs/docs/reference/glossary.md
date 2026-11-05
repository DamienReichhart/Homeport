---
title: Glossary
description: The words Homeport uses, and what they mean.
---

Homeport names things precisely. These terms appear throughout the app and these docs.

### Host

A Docker Engine that Homeport connects to, defined by a name and how to reach it: a local socket, TCP, TLS or SSH. Every container, image, volume, network and Stack belongs to exactly one Host. See [Hosts and connections](/docs/hosts/).

### Stack

A Compose project running on a Host, identified by its project name. Every Stack is Managed, Linked or External. See [Stacks](/docs/guides/stacks/).

### Managed Stack

A Stack whose compose definition and environment Homeport holds, so it can be deployed, redeployed and edited from Homeport.

### Linked Stack

A Stack whose compose files are a folder on this computer. Homeport keeps where the folder and its compose files are, reads and writes the files in place, and deploys with that folder as the project directory, so `build:`, `env_file:` and relative paths work. Found from the compose labels of running containers, or created from a folder. It becomes External while its files are missing. See [Linked Stacks](/docs/guides/stacks/#linked-stacks).

### External Stack

A Stack found running on a Host whose compose definition Homeport doesn't have, neither stored nor in a folder on this computer. It can be started, stopped and removed, but not redeployed or edited.

### Adopt

To turn an External Stack into a Managed Stack by supplying its compose definition.

### Registry login

The user name and password or access token Homeport keeps for one container registry, such as Docker Hub or GHCR, and uses on every Host to pull, push and build. See [Settings](/docs/guides/settings/#registries).

### Engine

The Docker Engine (`dockerd`), or a compatible one such as Podman, that runs containers on a machine. Homeport talks to it through the Docker Engine API.

### Socket

The address an Engine listens on: a Unix socket file such as `/var/run/docker.sock`, or a named pipe such as `//./pipe/docker_engine` on Windows.

### Keychain

Your operating system's secure store for passwords and keys — macOS Keychain, Windows Credential Manager or a Linux Secret Service — where Homeport keeps the key that encrypts its secrets.

### Trust on first use

How Homeport checks SSH servers: you confirm a server's key fingerprint on the first connection, and Homeport refuses to connect if the key later changes until you confirm again.
