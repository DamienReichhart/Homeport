---
title: System requirements
description: What Homeport needs to run, and the optional pieces that unlock SSH, TLS, Stack deploys, Kubernetes clusters and Helm.
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

On a Podman Host, Homeport also uses Podman's own API: [pods](/docs/guides/podman-pods/) need Podman 4.0 or later, and [Quadlet units](/docs/guides/quadlet/) are listed from Podman 5.7 and installed, edited and removed from 5.8. Starting and stopping a Quadlet unit needs the Host to be reached over SSH, or a Podman on this computer (Linux).

## Optional: Kubernetes clusters

To browse [Clusters](/docs/clusters/), Homeport needs a kubeconfig that reaches them (the one `kubectl` uses, or one you paste) and network access to their API servers. It speaks the Kubernetes API itself, so `kubectl` isn't needed.

- **Kubernetes versions**: clusters running Kubernetes 1.24 or later. Homeport uses only stable APIs (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`), and is built against the 1.33 API.
- **Exec plugins**: a cloud kubeconfig names a program to run on this computer for its credentials — the [AWS CLI](https://aws.amazon.com/cli/) (version 2) for EKS, `gke-gcloud-auth-plugin` from the Google Cloud CLI for GKE, `kubelogin` for AKS. Install the one your kubeconfig names; see [exec plugins](/docs/clusters/exec-plugins/).

## Optional: an OS keychain

Homeport encrypts SSH keys, TLS certificates, Clusters' kubeconfig credentials, Stack environments, registry logins and repositories' deploy keys and tokens with a key it keeps in your operating system's keychain:

- **macOS** — Keychain
- **Windows** — Credential Manager
- **Linux** — a Secret Service provider, such as GNOME Keyring or KWallet

Without one, Homeport shows a banner and still works with local sockets, plain TCP and SSH through **ssh-agent**. TLS Hosts, SSH Hosts with stored keys, Clusters, Stack environments, registry logins and private repositories need the keychain; without one, images are pulled anonymously and only public repositories can be added. See [security and your data](/docs/reference/security/).

## Optional: Docker Compose

To deploy [Managed, Linked and Synced Stacks](/docs/guides/stacks/), Homeport runs the `docker compose` command on **this** machine — not on the Host. Install the Docker CLI with the Compose plugin (Docker Desktop includes it), or the standalone `docker-compose` v2.

Without it, Stacks still appear — Linked Stacks included — and can be started, stopped and removed; only deploying needs Compose. **Settings → This machine** shows whether Homeport found it.

To deploy a Stack to a [Swarm](/docs/guides/swarm/), Homeport runs `docker stack deploy` on this machine instead, so it needs the `docker` command itself (the Compose plugin isn't used). Docker Desktop has it; on Linux, install your distribution's Docker CLI package (`docker-ce-cli`, `docker.io` or `docker`). No Engine needs to run here.

## Optional: git

[Synced Stacks](/docs/guides/synced-stacks/) run the `git` installed on **this** machine to clone and fetch their repositories: Git for Windows, Apple's command line tools (`xcode-select --install`), or your distribution's `git` package. On Windows, Homeport points it at the system's OpenSSH for SSH repositories. Every other feature works without git; **Settings → This machine** shows whether Homeport found it.

## Optional: Helm

To manage [Helm releases](/docs/clusters/helm/) on a Cluster, Homeport runs the `helm` command, version 3 or later, on **this** machine. It points helm at the Cluster through a private connection, so no cluster credential is written to disk, and helm uses its own chart repositories, as in a terminal. Install it with `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey or Scoop (Windows), or `snap install helm --classic`, your distribution's package or the script from [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Every other Cluster page works without it; **Settings → This machine** shows whether Homeport found it.

## Optional: a system tray

Homeport puts an icon in the system tray (the menu bar on macOS, the notification area on Windows) that lists every Host with its status. Windows and macOS always have one. On Linux it needs the **appindicator** library: the AppImage includes it, and the `.deb` and `.rpm` packages recommend it, so `apt`, `dnf` and `zypper` install it unless you turned recommended packages off. Otherwise install `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) or `libayatana-appindicator-gtk3` (Fedora). Some desktops hide tray icons — GNOME shows them only with an extension such as AppIndicator Support. Without a tray, Homeport works from its window, and closing the window quits it.

## What each feature needs

| Feature                                    | Keychain | Compose CLI | git | helm |
| ------------------------------------------ | :------: | :---------: | :-: | :--: |
| Deploying Stacks to a Swarm                |    —     | docker CLI  |  —  |  —   |
| Local socket and TCP Hosts                 |    —     |      —      |  —  |  —   |
| SSH Hosts with ssh-agent                   |    —     |      —      |  —  |  —   |
| SSH Hosts with a Homeport or imported key  |   Yes    |      —      |  —  |  —   |
| TLS Hosts                                  |   Yes    |      —      |  —  |  —   |
| Containers, images, volumes, networks      |    —     |      —      |  —  |  —   |
| Starting, stopping, removing Stacks        |    —     |      —      |  —  |  —   |
| Deploying Managed and Linked Stacks        |    —     |     Yes     |  —  |  —   |
| A Managed or Synced Stack's environment    |   Yes    |      —      |  —  |  —   |
| Registry logins (private images, pushes)   |   Yes    |      —      |  —  |  —   |
| Synced Stacks from a public repository     |    —     |     Yes     | Yes |  —   |
| Synced Stacks with a deploy key or a token |   Yes    |     Yes     | Yes |  —   |
| Building an image from a folder            |    —     |      —      |  —  |  —   |
| Kubernetes Clusters                        |   Yes    |      —      |  —  |  —   |
| Helm releases                              |   Yes    |      —      |  —  | Yes  |
