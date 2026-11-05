---
title: Local engines
description: Connect to Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop or Podman on this machine.
---

A **Local socket** Host talks to an Engine on this machine through its Unix socket, or a named pipe on Windows. It needs no credentials and no keychain.

## Automatic detection

Homeport looks for running Engines at the usual places and lists those that answer under **Found on this machine**, on the welcome screen and the **Add host** page. Choose **Add** next to one to create a Host named after it.

| Runtime                  | macOS and Linux                                                                                                  | Windows                           |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                           | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                   | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                     | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                  | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                    | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                              | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [machines on macOS](#podman-machines-on-macos) | `//./pipe/podman-machine-default` |

Each candidate is checked by asking the Engine for its version, so only running Engines are listed. When several paths lead to the same socket — on macOS, `/var/run/docker.sock` usually points to Docker Desktop, OrbStack or Rancher Desktop — the Engine is listed once, under the first name found.

### Podman machines on macOS

On macOS, Podman runs its Engine in a VM (`podman machine`) whose socket moves with each machine. Homeport asks the `podman` CLI for every running machine's socket (`podman machine list` and `podman machine inspect`) and lists each one as **Podman (_machine name_)**, for example **Podman (podman-machine-default)**. Stopped machines aren't listed; start one with `podman machine start` and open **Add host** again.

If Homeport can't run `podman` — it isn't on its PATH or in `/opt/homebrew/bin` or `/usr/local/bin` — it looks where Podman puts the sockets instead: `$TMPDIR/podman/<machine>-api.sock` for Podman 5, and `~/.local/share/containers/podman/machine/…/podman.sock` for Podman 4. When `podman-mac-helper` has linked `/var/run/docker.sock` to a machine, that machine is listed once, under its Podman name.

## Adding a socket by hand

If your Engine isn't detected — a Colima profile other than `default`, a Podman machine on Windows with another name than the default, or a custom path — add it yourself:

1. Choose **Add host…** in the Host switcher.
2. Select **Local socket**.
3. Enter a **Name** and the **Socket path**: a Unix socket such as `/run/user/1000/docker.sock`, or a named pipe such as `//./pipe/docker_engine` on Windows.
4. Choose **Add host**.

To find the socket your Docker CLI uses, run:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

For a Podman machine:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Permissions on Linux

The default socket `/var/run/docker.sock` belongs to the `docker` group. If your user isn't in it, the Host shows **Unreachable** with _Permission denied_. Add yourself and log in again:

```sh
sudo usermod -aG docker $USER
```

Membership of the `docker` group is equivalent to root on that machine; rootless Docker or Podman avoid that if it matters to you.
