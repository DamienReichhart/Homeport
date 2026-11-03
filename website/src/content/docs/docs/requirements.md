---
title: System requirements
description: What Homeport needs to run, and the optional pieces that unlock SSH, TLS and Stack deploys.
---

## Operating systems

| System  | Versions                                                          | Architectures        |
| ------- | ----------------------------------------------------------------- | -------------------- |
| Windows | Windows 10 and 11                                                 | x64, ARM64           |
| macOS   | macOS 10.15 Catalina or later                                     | Apple silicon, Intel |
| Linux   | glibc 2.35 or later: Ubuntu 22.04, Debian 12, Fedora 36 and newer | x64, ARM64           |

Homeport uses the system's web view to draw its window: WebView2 on Windows (included with Windows 10 and 11), WebKit on macOS, and WebKitGTK on Linux (installed as a dependency of the `.deb` and `.rpm` packages, and bundled in the AppImage).

## A Docker Engine to manage

Homeport doesn't include a container runtime. It manages Docker Engines that already run somewhere:

- **On this machine** — Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop or Podman. Homeport finds them for you; see [local engines](/docs/hosts/local/).
- **On a server** — reachable over [SSH](/docs/hosts/ssh/), or over [TLS or TCP](/docs/hosts/tls-tcp/).

Any Engine that speaks the Docker Engine API works, including Podman's Docker-compatible socket.

## Optional: an OS keychain

Homeport encrypts SSH keys, TLS certificates and Stack environments with a key it keeps in your operating system's keychain:

- **macOS** — Keychain
- **Windows** — Credential Manager
- **Linux** — a Secret Service provider, such as GNOME Keyring or KWallet

Without one, Homeport shows a banner and still works with local sockets, plain TCP and SSH through **ssh-agent**. TLS Hosts, SSH Hosts with stored keys and Stack environments need the keychain. See [security and your data](/docs/reference/security/).

## Optional: Docker Compose

To deploy [Managed Stacks](/docs/guides/stacks/), Homeport runs the `docker compose` command on **this** machine — not on the Host. Install the Docker CLI with the Compose plugin (Docker Desktop includes it), or the standalone `docker-compose` v2.

Without it, Stacks still appear and can be started, stopped and removed; only deploying needs Compose. **Settings → This machine** shows whether Homeport found it.

## What each feature needs

| Feature                                   | Keychain | Compose CLI |
| ----------------------------------------- | :------: | :---------: |
| Local socket and TCP Hosts                |    —     |      —      |
| SSH Hosts with ssh-agent                  |    —     |      —      |
| SSH Hosts with a Homeport or imported key |   Yes    |      —      |
| TLS Hosts                                 |   Yes    |      —      |
| Containers, images, volumes, networks     |    —     |      —      |
| Starting, stopping, removing Stacks       |    —     |      —      |
| Deploying Managed Stacks                  |    —     |     Yes     |
| A Managed Stack's environment             |   Yes    |      —      |
