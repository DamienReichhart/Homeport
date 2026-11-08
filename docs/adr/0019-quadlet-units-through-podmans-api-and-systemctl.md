# Quadlet units through Podman's API, and `systemctl` over SSH

## Context

v0.18.0 manages Podman's Quadlet units: the `.container`, `.pod`, `.volume`, `.network`, `.kube`, `.image` and `.build` files that systemd's generator turns into services, in `/etc/containers/systemd/` for rootful Podman and `~/.config/containers/systemd/` for rootless. Homeport's rule is to install nothing on a Host and to use only the APIs it already offers; until now an SSH Host only opened `direct-streamlocal` channels to its Engine socket, plus one shell check of that socket when it refuses (ADR 0002).

What Podman offers, from its source:

- **Podman 5.6**: `podman quadlet install|list|print|rm` on the command line only.
- **Podman 5.7**: `GET /libpod/quadlets/json` (name, unit name, path, systemd status, application).
- **Podman 5.8 and 6.x**: also `GET /libpod/quadlets/{name}/file`, `GET …/{name}/exists`, `POST /libpod/quadlets` (a tar or multipart body; `application`, `replace`, `reload-systemd`) and `DELETE /libpod/quadlets/{name}` (`force` stops a running unit first).
- **No version** starts, stops or restarts a unit's service through the API. Containers created by Quadlet carry the `PODMAN_SYSTEMD_UNIT` label on every version.
- The API acts as the socket's user: the rootful socket manages system units, the rootless one the user's.

## Decision

Option 2 of the three below: **Podman's API for everything it covers, and `systemctl` for starting, stopping and restarting, where Homeport can run it.**

- **Listing**: `GET /libpod/quadlets/json` on Podman 5.7 and later. On older Podman, units are built from the `PODMAN_SYSTEMD_UNIT` label of their containers, named by their service, with their file marked as not readable.
- **Files, installing, editing and removing**: through the API on Podman 5.8 and later, as a tar archive with the unit and its extra files, and with systemd reloaded by Podman. Editing installs the same name again with `replace=true` into the unit's application, so files installed beside it stay. Below 5.8 these actions are refused with `PodmanTooOld`, naming the version they need.
- **Starting, stopping and restarting** run `systemctl start|stop|restart <unit>.service` with `--user` for a rootless Podman's units, decided from the unit's path or else from Podman's `rootless` flag:
  - on an **SSH Host**, on a session channel of the SSH session Homeport already holds, as the socket probe does;
  - on a **local socket** on Linux, on this computer, since the Podman is this computer's;
  - on **TCP and TLS Hosts**, and local sockets on macOS and Windows (whose Podman runs in a VM), not at all: the buttons are disabled with the reason.
- **Only these commands, quoted, never `sudo`.** A unit name is checked against systemd's characters before it is quoted into the command. The command runs with the SSH user's rights (or this computer's user): system units need an SSH user allowed to manage them, and user units need the user's systemd instance (`loginctl enable-linger <user>` for it to outlive the session). systemd's refusals become `SystemdRefused`, and a missing user instance `UserUnitsNeedLinger`, with what to run.
- **Disclosed** in the Quadlet guide and in the security page, which lists the exact commands. The containerd ADR in v0.19.0, which has the same question for logs and exec, follows the same rule: an SSH command is a capability the Host already grants through the key in `authorized_keys`, run only for an action the user starts, from a fixed list, and documented.

## Considered Options

1. **API only**: list from 5.7, files, install and remove from 5.8; older Podman shows units from their containers' labels. No start or stop: users would use the containers' own actions, which systemd may undo under `Restart=`. Works on every connection kind and runs nothing new on the Host, but leaves out the action users need most once a unit is installed, since installing doesn't start it.
2. **API plus `systemctl` over SSH** (chosen): as 1, plus start, stop and restart over the SSH session or on this computer. TCP and TLS Hosts get everything but those three.
3. **Files over SSH for every version**: write unit files and run `systemctl daemon-reload` over SSH, ignoring the API. The widest Podman support, but SSH and local Hosts only, and Homeport would own the file layout, rootful and rootless paths, and every error Podman already reports.

## Consequences

- Starting a unit's service works on SSH and local Linux Hosts, on every Podman version that has Quadlet. TCP and TLS Hosts can list, install, edit and remove units, and say why they can't start one.
- Homeport now runs commands on SSH Hosts beyond the socket probe, all listed in the security page; adding one is a decision to record here.
- Podman 5.6 and older show units only once they have created containers, and can't install or edit them; the Quadlet guide has a table of what each version allows.
- No file is written on a Host except by Podman's own install endpoint.
