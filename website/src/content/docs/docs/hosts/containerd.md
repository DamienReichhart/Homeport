---
title: containerd
description: Connect to containerd on this machine or over SSH, pick a namespace, run its containers' tasks, read their logs, open a shell, and manage images and snapshots.
---

**containerd** runs underneath k3s, nerdctl, BuildKit and many servers that have no Docker. Homeport connects to it directly, through its own gRPC API, as [ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md) decides. A containerd Host gets pages of its own in the sidebar: **Containers**, **Images**, **Snapshots** and **Namespaces**. Docker's pages, Stacks and the Board aren't offered on it.

## Adding a containerd Host

Choose **containerd** in the form's **Engine** field. It reaches containerd through its socket only, so the connection is a **Local socket** or **SSH**; TCP and TLS are greyed out, because containerd doesn't serve its API over the network.

| containerd                     | Socket                                                                    |
| ------------------------------ | ------------------------------------------------------------------------- |
| containerd's own service       | `/run/containerd/containerd.sock`                                         |
| k3s                            | `/run/k3s/containerd/containerd.sock`                                     |
| Docker Engine's own containerd | `/run/containerd/containerd.sock`, its containers in the `moby` namespace |

On Linux, **Found on this machine** lists this computer's containerd and k3s's when they answer. A socket you may not use yet is listed with how to fix it instead.

### Letting your user use the socket

containerd's socket belongs to root. To use it as another user, locally or over SSH, give a group access to it in `/etc/containerd/config.toml` and add the user to that group:

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Put the number that prints under `[grpc]` in `/etc/containerd/config.toml`, restart containerd, and log in again:

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**Rootless containerd** (`containerd-rootless.sh`) keeps its socket inside RootlessKit's namespace, where nothing outside it can reach it, not even Homeport. Homeport says so when it finds it; use containerd's system service instead.

### What each version and connection allows

Homeport connects to containerd **1.7** and later. Pulling goes through containerd's Transfer service, stable from **2.0**.

| Action                                              | containerd    | Connection                      |
| --------------------------------------------------- | ------------- | ------------------------------- |
| Namespaces, containers and tasks, images, snapshots | 1.7 and later | local socket or SSH             |
| Pull an image                                       | 2.0 and later | local socket or SSH             |
| Logs and a terminal                                 | 1.7 and later | SSH, or a local socket on Linux |

A containerd too old to connect leaves the Host **Misconfigured**, naming its version; one too old to pull says so when you pull.

## Namespaces

Every containerd object lives in a **namespace**: nerdctl uses `default`, Kubernetes `k8s.io`, Docker `moby`, BuildKit `buildkit`. The pages show one namespace at a time, picked under the Host switcher, which lists each one with how many containers and images it holds. Homeport remembers the pick for each Host between launches. Until you pick one, a Host shows `default`, or the only namespace holding something when `default` is empty, so a k3s Host opens on `k8s.io`.

The **Namespaces** page creates empty namespaces and removes empty ones. containerd refuses to remove a namespace that still holds anything, and Homeport says what it still holds: remove its containers, images and snapshots first.

## Containers and tasks

In containerd, a **container** is only metadata: its image, its spec, its snapshot. What runs is its **task**. The **Containers** page lists the namespace's containers with their task: **Running**, **Paused**, **Stopped** with its exit code, or **No task**. Each says which tool made it — **Kubernetes** (with its Pod), **nerdctl**, **Docker** or **ctr or other** — and Pods' pause containers are hidden unless you ask for them.

| Action          | What Homeport does                                                                                                                                   |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Start**       | Creates the task on the container's snapshot, logging where nerdctl logs, and starts it. A stopped task is replaced, as `ctr task start` needs.      |
| **Stop**        | Sends the image's stop signal (SIGTERM unless it names another), then SIGKILL after 10 seconds. A task still alive after that is reported.           |
| **Pause**       | Freezes the task; **Resume** thaws it.                                                                                                               |
| **Send signal** | Sends any signal, by name (`SIGHUP`) or number, to the task's main process.                                                                          |
| **Remove**      | Asks first; kills a running task, removes the task and the container, and its snapshot with the files it changed, unless you untick **Also remove**. |

A container's page shows its runtime, snapshot, labels and OCI spec. Homeport doesn't create containers on containerd: containerd has no networking of its own, which nerdctl and CNI add.

The **kubelet** manages Kubernetes' containers: it starts them again after a stop and makes them again after a removal. Their pages say so; change the Pod on its Cluster instead. Removing a nerdctl container from here leaves nerdctl's own records of it (its name and network), as `ctr` would; `nerdctl rm` is the cleaner way for those.

## Logs

containerd keeps no logs: the tool that starts a task decides where its output goes. The **Logs** tab reads it where that tool wrote it:

| Made by                 | Where its output goes                                                     | What Homeport shows                                 |
| ----------------------- | ------------------------------------------------------------------------- | --------------------------------------------------- |
| nerdctl                 | a JSON log file in nerdctl's state folder, named by its `nerdctl/` labels | that file, followed                                 |
| Kubernetes (`k8s.io`)   | `/var/log/pods/<pod>/<container>/N.log`, named in CRI's record of it      | that file, followed                                 |
| `ctr` and other clients | wherever they pointed it, usually nowhere                                 | **This container's output isn't recorded**, and why |

nerdctl containers logging to another driver (journald, fluentd) name it instead. Homeport reads the file with `tail` on the Host's machine (see [below](#what-runs-on-the-host)); log files usually belong to root, so the SSH user must be allowed to read them.

## A terminal in a container

The **Exec** tab opens a shell in a running container — bash when the image has it, sh otherwise — or the command you give, as a numeric user (`uid` or `uid:gid`) and in a folder of your choice. **Open again** starts a fresh one with what you typed.

containerd takes a process's input and output as paths on its machine. So Homeport runs a short script there that makes two FIFOs in a private temporary folder, and copies the terminal through them; containerd starts the process on those FIFOs, from the container's own process spec. Closing the tab kills the process and removes the folder.

## Images

The **Images** page lists the namespace's images with their platforms, the containers made from them, and their size for the Host's platform, as `ctr images ls` counts it.

**Pull image** fetches an image through containerd itself, then unpacks it for the Host's platform into its default snapshotter, so `nerdctl run` can use it straight away. Give a full reference (`ghcr.io/home/app:1.2`) or a short one (`nginx`, which becomes `docker.io/library/nginx:latest`). A private registry's image is pulled with its login from **Settings → Registries**, sent to containerd only when the registry asks for it, and never stored on the Host.

An image a container of the namespace was made from can't be removed; remove the containers first. containerd frees an image's content once nothing holds it.

## Snapshots

Snapshots are where containerd's disk goes: the layers of unpacked images, and each container's writable layer. The **Snapshots** page shows one snapshotter's (`overlayfs` first, or any other the Host loaded) as a tree by parent, largest first, with their usage as `ctr snapshot usage` counts it and the total at the top.

Each says what uses it: a container, by its snapshot, or an image, by its layers. Snapshots nothing uses and nothing is built on — such as the one left behind by a container removed with `ctr containers rm --keep-snapshot` — can be shown alone with **Only unused**, and removed. The others can't be removed from here.

## What runs on the Host

Everything above goes through containerd's API, except logs and terminals, which need the files of containerd's machine. For those Homeport runs these commands, over the SSH session it already holds, or on this computer for a local socket on Linux, with the SSH user's rights and never with `sudo`:

| For        | Command                                                                                                               |
| ---------- | --------------------------------------------------------------------------------------------------------------------- |
| Logs       | `tail -n <lines> [-F] -- '<log file>'`; for a Kubernetes container without CRI's record, `ls` of its log folder first |
| A terminal | `mktemp -d`, `mkfifo -m 600` for two FIFOs in that folder, `cat` through them, and `rm -rf` of the folder             |

A local socket on macOS or Windows can't run them, and logs and terminals then say they need SSH. The [security page](/docs/reference/security/#what-homeport-runs-on-your-hosts) lists every command Homeport runs on a Host.

## Events

Pages refresh from containerd's own events, in every namespace: a task starting or exiting, a container or image created or removed, a snapshot committed. containerd keeps no history of them, so what happened while Homeport was closed isn't filled in.
