---
title: Quadlet units
description: List a Podman Host's Quadlet units with their systemd status, read their files, install, edit and remove them, and start or stop their services.
---

**Quadlet** is how Podman runs long-lived services under systemd. A Quadlet unit is a file — `web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build` — that systemd's generator turns into a service, `web.service`, which starts the container at boot and restarts it when it stops. Rootful Podman keeps its units in `/etc/containers/systemd/`; rootless Podman keeps each user's in `~/.config/containers/systemd/`.

The **Quadlet** page appears in the sidebar while a Podman Host is selected. It goes through Podman's own API on the socket the Host already uses, and through `systemctl` for starting and stopping, as [ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md) decides.

## What each Podman and connection allows

Podman's API grew Quadlet support over two releases, and it never starts or stops a unit's service. So what Homeport can do depends on the Host's Podman and on how Homeport reaches it:

| Action                            | Podman           | Connection                      |
| --------------------------------- | ---------------- | ------------------------------- |
| List units with their status      | 5.7 and later    | any                             |
| List units from their containers  | 4.0 to 5.6       | any                             |
| Read a unit's file                | 5.8 and later    | any                             |
| Install, edit and remove units    | 5.8 and later    | any                             |
| Start, stop and restart a service | any with Quadlet | SSH, or a local socket on Linux |

On Podman before 5.7, units are found from the `PODMAN_SYSTEMD_UNIT` label Podman puts on the containers they create, so a unit appears only once it has created a container, named by its service. The page says so above the list.

Starting and stopping need `systemctl`, which Homeport runs where Podman runs: over the SSH session of an SSH Host, or on this computer for a local socket on Linux. A TCP or TLS Host, or a local socket on macOS or Windows (where Podman runs in a virtual machine), can do everything else; the buttons are disabled with the reason.

## The Quadlet page

| Column         | Shows                                                                                                                         |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| **Unit**       | The unit's file name, and the application it was installed with.                                                              |
| **Kind**       | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` or `artifact`.                                              |
| **Service**    | The systemd service it generates, or **not generated yet** until systemd has loaded it.                                       |
| **Status**     | **Running**, **Starting**, **Stopping**, **Stopped**, **Failed**, **Not loaded by systemd** or **Template**, as systemd says. |
| **Containers** | How many containers it created.                                                                                               |

Each row has **Start** for a stopped unit or **Stop** for a running one. Select a unit to open its page. The list updates on its own as its containers start and stop.

A unit **Not loaded by systemd** is installed, but systemd's generator skipped it: usually a mistake in the file. On the Host, `systemctl --user daemon-reload` (or `systemctl daemon-reload` for rootful Podman) and `journalctl` then say why.

On the **Containers** page, a container a unit created shows the unit's service next to its name; select it to open the unit.

## A unit's page

- The header has **Start**, **Stop** and **Restart**, **Edit** and **Remove**.
- **Service** shows the systemd unit, whether it's a system unit or a user unit (`systemctl --user`), where its file is on the Host, and its application and pod when it has them.
- **Unit file** shows the file as Podman reads it, highlighted.
- **Containers** lists the containers it created, each opening its container page.

Stopping a unit's container from the Containers page fights systemd, which starts it again under `Restart=`. Stop the unit instead.

## Installing a unit

**New unit** (or `n` on the Quadlet page, or **New unit** in the command palette) opens an editor with a starting point for the kind you pick, or a file you open from this computer with **Open file…**. Name it with its extension, such as `web.container`; the file needs the section of its kind, `[Container]` for a `.container`.

**Add files…** puts other files beside the unit — a `Containerfile` for a `.build`, the YAML of a `.kube`, an env file — in an application folder named after the unit, as `podman quadlet install` does.

Podman writes the files where its user's units go and reloads systemd, so the service exists straight away. Installing doesn't start it: choose **Start** once it's there. A unit whose name is taken is refused; edit that one instead.

## Editing and removing

**Edit** opens the unit's file. **Save** writes it in place and reloads systemd; the files installed beside it stay. A running service keeps its old settings until you **Restart** it.

**Remove** deletes the unit's file and reloads systemd. **Stop its running service first** is on by default; without it, Podman refuses to remove a running unit. Containers, volumes and images the unit created are kept.

## What Homeport runs on the Host

For **Start**, **Stop** and **Restart**, and nothing else, Homeport runs one command on the Host's server over SSH (or on this computer for a local socket), with the SSH user's rights:

```bash
systemctl --user start 'web.service'
```

`--user` is there for a rootless Podman's units and left out for rootful Podman's; `start` is `stop` or `restart` for those actions. Homeport checks the service name and quotes it, and never uses `sudo`. So:

- **System units** (rootful Podman) need an SSH user allowed to manage them, such as `root`.
- **User units** (rootless Podman) need the user's systemd instance. For it to run without a login session, and for the units to start at boot, enable lingering once, as that user on the Host:

```bash
loginctl enable-linger "$USER"
```

The [security page](/docs/reference/security/#what-homeport-runs-on-your-hosts) lists every command Homeport runs on a Host.
