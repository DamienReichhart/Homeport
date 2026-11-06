---
title: Settings
description: Theme, updates, registry logins, repositories, how often Synced Stacks are checked, what this machine supports, version and licences.
---

**Settings** is the last item in the sidebar.

## Appearance

**Theme** is **Match the system** by default, following your operating system's light or dark mode as it changes. Choose **Light** or **Dark** to fix it.

## Updates

- **Check for updates automatically** — on by default: Homeport looks for a new version when it starts and every 6 hours, and shows a bar at the top of the window when there is one.
- **Check now** — checks straight away, even with automatic checks off, and says whether you have the latest version.

When an update is available, the card shows what you can do, which depends on how you installed Homeport:

- **Windows, macOS and the AppImage** — **Install and restart** downloads the update, checks its signature, installs it and reopens Homeport.
- **`.deb` and `.rpm` packages** — your package manager installs updates, so the card shows the command to run, with a **Copy command** button.

**What's new** opens the [changelog](/changelog/). See [updating](/docs/reference/updating/) for more.

## Registries

Logins for container registries — Docker Hub, GitHub Container Registry (GHCR), or your own — kept once in Homeport and used on every Host: when you pull an image, run a container from one that isn't there yet, push, build, or deploy a Managed, Linked or Synced Stack. There is one login per registry address.

- **Add login** asks for the **Registry** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), the **User name** and the **Password or access token**. GHCR, and Docker Hub accounts with two-factor authentication, need an access token rather than the password.
- **Check login** asks the registry, from this computer, whether it accepts the login, as `docker login` does, without saving anything. A refused login says so with a hint; a registry Homeport can't reach says that instead.
- **Edit** changes the address or user name; leave the password empty to keep the stored one. **Remove** forgets the login: images from that registry are pulled anonymously again.

Docker Hub can be entered as `docker.io`, `index.docker.io` or `https://index.docker.io/v1/`; they are one registry. An image without a registry in its name, such as `nginx` or `bitnami/redis`, uses the Docker Hub login.

Passwords and tokens are encrypted with the OS keychain's key and never shown again, logged, or put in error details. Without a keychain, the card explains that logins can't be stored, and images are pulled anonymously. See [security](/docs/reference/security/).

Homeport doesn't read `~/.docker/config.json` or its credential helpers.

## Repositories

The git repositories [Synced Stacks](/docs/guides/synced-stacks/) are deployed from, kept once and usable on every Host.

- **Add repository** asks for the repository's URL and how Homeport reaches it: a **Deploy key** it makes for that repository (SSH URLs), a **Token** with its user name (HTTPS URLs), or **Public** for one anyone can read.
- The gear button shows a repository's deploy key, with where to add it on GitHub, GitLab, Gitea and Forgejo, and **New key** to replace it; or **Replace the token**. **Test access** lists its branches and tags from this computer. Once Homeport has trusted an SSH server's key, its fingerprint shows there, with **Reset** for when the forge really changed it.
- **Remove** forgets the repository and deletes its clone on this computer; it's refused while a Synced Stack uses it.

Keys and tokens are encrypted with the OS keychain's key and never shown again, logged or put in error details. Your own git configuration, credential helpers and SSH keys are never used. See [repositories](/docs/guides/synced-stacks/#repositories).

## Synced Stacks

**Check repositories every** sets how many minutes Homeport waits between checks of Synced Stacks' repositories for new commits: 5 by default, at least 1. Homeport checks only while it's open; nothing syncs while it's closed. See [polling and auto-deploy](/docs/guides/synced-stacks/#polling-and-auto-deploy).

## Data

What Homeport keeps on this computer.

**Keep Engine events for** sets how many days each Host's [Engine events](/docs/guides/overview/#events) are kept: 30 by default, from 1 to 365. Each Host keeps at most 10,000 events whatever this says. Older ones are removed when Homeport starts and every hour, and at once when you lower the number.

## This machine

What Homeport can use on this computer:

- **OS keychain** — **Available** when SSH keys, certificates, Stack environments and registry logins can be stored encrypted; **Unavailable** otherwise. See [requirements](/docs/requirements/#optional-an-os-keychain).
- **Compose CLI** — the Compose version found, such as `docker compose v2.29.1`, or **Not found** if Stacks can be started and stopped but not deployed. See [Stacks](/docs/guides/stacks/#what-deploying-needs).
- **git** — the git found, such as `/usr/bin/git 2.43.0`, or what to install if Synced Stacks can't be used. See [requirements](/docs/requirements/#optional-git).

Homeport checks them when it starts. After installing a keychain, Compose or git, quit and reopen Homeport.

## About

Shows Homeport's version and the platform it was built for, for example `v0.2.0 · macos/aarch64`. Include it when you [report an issue](https://github.com/DamienReichhart/Homeport/issues).

**Licenses** shows Homeport's MIT licence and the notices of every open-source package it's built with.
