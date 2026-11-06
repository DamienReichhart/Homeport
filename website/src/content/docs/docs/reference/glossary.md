---
title: Glossary
description: The words Homeport uses, and what they mean.
---

Homeport names things precisely. These terms appear throughout the app and these docs.

### Host

A Docker Engine that Homeport connects to, defined by a name and how to reach it: a local socket, TCP, TLS or SSH. Every container, image, volume, network and Stack belongs to exactly one Host. See [Hosts and connections](/docs/hosts/).

### Stack

A Compose project running on a Host, identified by its project name. Every Stack is Managed, Linked, Synced or External. See [Stacks](/docs/guides/stacks/).

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

### Engine

The Docker Engine (`dockerd`), or a compatible one such as Podman, that runs containers on a machine. Homeport talks to it through the Docker Engine API.

### Socket

The address an Engine listens on: a Unix socket file such as `/var/run/docker.sock`, or a named pipe such as `//./pipe/docker_engine` on Windows.

### Keychain

Your operating system's secure store for passwords and keys — macOS Keychain, Windows Credential Manager or a Linux Secret Service — where Homeport keeps the key that encrypts its secrets.

### Trust on first use

How Homeport checks SSH servers: you confirm a server's key fingerprint on the first connection, and Homeport refuses to connect if the key later changes until you confirm again.
