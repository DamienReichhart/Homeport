---
title: FAQ
description: Common questions about Homeport.
---

### Is Homeport free?

Yes. Homeport is free and open source under the [MIT licence](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE), for personal and commercial use.

### Does Homeport replace Docker Desktop?

Not entirely. Homeport is a manager: it doesn't run a Docker Engine itself. On Linux, install Docker Engine and use Homeport as its interface. On macOS and Windows you still need something that runs the Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop or Podman — and Homeport can sit on top of any of them, along with your remote servers.

### Does it work with Podman?

Yes, through Podman's Docker-compatible API socket. Homeport detects the usual Podman sockets on Linux and the default Podman machine pipe on Windows; on macOS, [add the socket by hand](/docs/hosts/local/#adding-a-socket-by-hand).

### Does it work with Kubernetes or Docker Swarm?

Homeport manages Docker Engines and Compose Stacks. It doesn't manage Kubernetes, and doesn't have Swarm services or nodes, though it shows the containers Swarm runs on a Host.

### Do I need to install anything on my servers?

No — only a Docker Engine and an SSH server, which you most likely already have. Homeport doesn't install agents.

### Do I need Docker installed on my computer to manage remote servers?

No. Homeport speaks the Engine API and SSH itself. The only exception is deploying Managed Stacks, which uses the `docker compose` command on your computer.

### Can several people share Homeport's Hosts?

Each person runs their own Homeport with their own Hosts and keys. With a Homeport-created key per person and per server, you can see and revoke each one's access in `authorized_keys`.

### Can I move my Hosts to another computer?

Yes. On the **Hosts** page, choose **Export**, pick the Hosts and a passphrase of at least 12 characters, and save the file. On the other computer, choose **Import** on the same page, open the file with the passphrase and pick what to add. Keys, certificates and trusted server keys come along, so the Hosts connect without asking you anything; see [moving to another computer](/docs/reference/security/#moving-to-another-computer).

Copying `homeport.db` doesn't work for secrets: they're encrypted with a key kept in this computer's keychain, and show as **Credentials lost** elsewhere. Stacks aren't exported yet.

### Does Homeport collect data about me?

No. There's no account, telemetry or analytics. The only request Homeport makes on its own from your computer is the update check, which downloads a small file from this website and sends nothing about you; you can turn it off in **Settings → Updates**. Separately, the newer-image check asks your images' registries, through each Host's Engine, which version their tags point to; for a local Host that request leaves from your computer too. See [security and your data](/docs/reference/security/#what-homeport-connects-to).

### Can Homeport use my private registries?

Yes. Add a login for each registry in **Settings → Registries** — Docker Hub, GHCR or your own. Homeport hands it to the Engine to pull, push and build on any Host, and pulls a Managed Stack's images with it before deploying. Passwords and tokens are encrypted with your OS keychain. Homeport doesn't read the logins of `docker login` on this computer.

### How does Homeport know a newer image exists?

It asks the image's registry, through the Host's Engine, which digest the tag points to now, and compares it with the one the Host pulled. Nothing is downloaded, but Docker Hub counts each question against its pull rate limit, like a `docker pull` that finds nothing new. So Homeport keeps each answer for 6 hours, asks at most four registries at once, and only asks again sooner when you choose **Check for newer images**. A Docker Hub login in **Settings → Registries** raises the limit. If a registry can't be reached or is rate-limiting, Homeport shows no mark for its images and writes a line to its log.

### How do I update?

On Windows, macOS and with the AppImage, Homeport tells you when a new version is out: choose **Install and restart**. On Linux, the `.deb` and `.rpm` packages update with the rest of your system from the Homeport package repository. See [updating](/docs/reference/updating/).

### Is Homeport on Homebrew, winget, Flathub or the AUR?

Not yet. They're planned; the [installation page](/docs/installation/#coming-later) links to each one's issue.

### Is there a command-line interface?

No — Homeport is a desktop app. Your existing `docker` CLI works alongside it, and Homeport updates live when you change things from the terminal.

### How can I help?

Report bugs and suggest features in [GitHub issues](https://github.com/DamienReichhart/Homeport/issues), or see [contributing](/docs/project/contributing/).
