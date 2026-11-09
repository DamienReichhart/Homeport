---
title: Settings
description: Theme and language, the system tray, starting at login, notifications, updates, registry logins, repositories, how often Synced Stacks are checked, what this machine supports, version and licences.
---

**Settings** is the last item in the sidebar.

## Appearance

**Theme** is **Match the system** by default, following your operating system's light or dark mode as it changes. Choose **Light** or **Dark** to fix it.

**Language** is **Match the system** by default: Homeport shows itself in your operating system's language when it has it, and in English otherwise. The choice names the language it found, such as **Match the system (Français)**. Pick a language to use that one instead; the whole window switches at once, without a restart, and so do the tray menu and notifications.

Homeport speaks English, French (Français), Spanish (Español), German (Deutsch), Italian (Italiano), Brazilian Portuguese (Português), Russian (Русский), Simplified Chinese (简体中文), Japanese (日本語), Korean (한국어), Hindi (हिन्दी) and Arabic (العربية), which it shows right to left. Regional variants follow their language: Canadian French shows French, European Portuguese shows Brazilian Portuguese. Traditional Chinese isn't available yet and shows English.

Dates, numbers and sizes follow the language too, in your system's regional style when it has one for it: _1.5 GB_ and _4 Nov 2026_ in British English, _1,5 Go_ and _4 nov. 2026_ in French. Names, IDs, commands, logs and what Engines and Clusters say stay as they are, and an error's **Details** keep the original text of the system or library that failed, so a bug report reads the same in every language.

## Desktop

How Homeport runs on this computer.

- **Close to the system tray** — off by default, so closing the window quits Homeport. Turn it on to hide the window in the [system tray](/docs/guides/overview/#system-tray) instead: Homeport keeps running with its Host connections open. Bring it back from the tray icon, or by opening Homeport again; quit from the tray's **Quit**. The switch is greyed out, with an explanation, on a desktop without a tray.
- **Start at login** — off by default. Turn it on and Homeport starts when you log in, so it watches your Hosts from the start. The switch shows what your system has: if you remove Homeport from your login items yourself, it shows as off. On Windows it's listed in **Task Manager → Startup apps**, on macOS in **System Settings → General → Login Items**, on Linux in `~/.config/autostart/`. An AppImage's entry follows the file when you move it, from the next time you open it.
- **Start hidden in the tray** — off by default, and only available while **Start at login** is on. When Homeport starts at login, it stays in the [system tray](/docs/guides/overview/#system-tray) without opening its window. Opening Homeport yourself always shows the window, and without a tray it always opens.

## Notifications

Homeport shows a notification from your operating system when something happens on a Host, whether its window is open, minimised or [closed to the tray](#desktop). Each kind has its own switch, all on by default, and a switch takes effect at once:

| Switch                                                    | Example                                                                     |
| --------------------------------------------------------- | --------------------------------------------------------------------------- |
| **A container stops with an error or runs out of memory** | _web-1 stopped on prod-server (exit code 137)_, _db ran out of memory on …_ |
| **A container turns unhealthy**                           | _api is unhealthy on prod-server_                                           |
| **A connected host becomes unreachable**                  | _prod-server is unreachable: …_                                             |
| **An unreachable host is back**                           | _prod-server is connected again._                                           |

Stopping a container — with **Stop** in Homeport, `docker stop`, `docker kill` or `docker compose down` — isn't a crash, so it shows nothing; nor does a container that exits with code 0. A Host that fails to connect when Homeport starts isn't an outage either: only one that was connected. A container stuck restarting, or a Host that keeps dropping, notifies at most once every 5 minutes.

Notifications come from the connections Homeport already holds, so nothing is installed on your Hosts, and they stop when Homeport quits. Your operating system decides how they look and whether they show: check its notification settings for Homeport if none appear.

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

### Backing up and restoring

**Back up…** saves every Host, Cluster, Stack, registry login, repository and setting, with their keys, certificates, tokens, passwords and environments, to one `.homeport` file. Type a passphrase of at least 12 characters twice: the whole file is encrypted with it, and it can't be recovered. Engine events and deploy history aren't included, nor anything on your Hosts or Clusters — containers, images, volumes and Kubernetes resources stay where they are.

**Restore…** opens such a file with its passphrase and lists what it holds: its Hosts and Clusters by name, and how many Stacks, registry logins and repositories. **Replace everything** then replaces all of Homeport's data with the backup's, on this or another computer:

- Everything connects and deploys at once: SSH and TLS Hosts, registries and repositories without asking for a key or password again, nor to trust a server's key.
- A [Linked Stack](/docs/guides/stacks/#linked-stacks) whose folder isn't on this computer shows as External until the folder is there, at the same path.
- [Synced Stacks](/docs/guides/synced-stacks/) clone their repository again on their next sync.
- The current data is copied to `homeport.db.before-restore-<date>` in Homeport's data folder first. Restoring a wrong file, a newer Homeport's, or one that fails halfway changes nothing.

To add some Hosts to the ones you have instead, use **Export** and **Import** on the Hosts page.

### Keeping Engine events

**Keep Engine events for** sets how many days each Host's [Engine events](/docs/guides/overview/#events) are kept: 30 by default, from 1 to 365. Each Host keeps at most 10,000 events whatever this says. Older ones are removed when Homeport starts and every hour, and at once when you lower the number.

## This machine

What Homeport can use on this computer:

- **OS keychain** — **Available** when SSH keys, certificates, Stack environments and registry logins can be stored encrypted; **Unavailable** otherwise. See [requirements](/docs/requirements/#optional-an-os-keychain).
- **Compose CLI** — the Compose version found, such as `docker compose v2.29.1`, or **Not found** if Stacks can be started and stopped but not deployed. See [Stacks](/docs/guides/stacks/#what-deploying-needs).
- **git** — the git found, such as `/usr/bin/git 2.43.0`, or what to install if Synced Stacks can't be used. See [requirements](/docs/requirements/#optional-git).
- **Helm CLI** — the helm found, such as `/usr/local/bin/helm v3.16.2`, or what to install for this system if [Helm releases](/docs/clusters/helm/) can't be listed or installed. Helm 2 is reported as unsupported. See [requirements](/docs/requirements/#optional-helm).

Homeport checks the CLIs each time this page opens, so one installed since launch shows up; after installing a keychain, quit and reopen Homeport.

## About

Shows Homeport's version and the platform it was built for, for example `v0.2.0 · macos/aarch64`. Include it when you [report an issue](https://github.com/DamienReichhart/Homeport/issues).

**Licenses** shows Homeport's licence (the GNU AGPL, version 3), a link to the source code of the version you run, and the notices of every open-source package it's built with.
