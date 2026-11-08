---
title: Incus
description: Connect to Incus on this machine, over SSH or over HTTPS with a trust token, and pick the project its pages show.
---

**Incus** runs system containers and virtual machines, the **Instances** of [the guide](/docs/guides/instances/). Homeport connects to it through its own REST API, as [ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md) decides. An Incus Host gets pages of its own in the sidebar: **Instances**, **Images**, **Profiles**, **Storage** and **Networks**. Docker's pages, Stacks and the Board aren't offered on it.

## Adding an Incus Host

Choose **Incus** in the form's **Engine** field, then how to reach it:

| Connection       | When                                               | What it needs                                                |
| ---------------- | -------------------------------------------------- | ------------------------------------------------------------ |
| **Local socket** | Incus runs on this computer (Linux)                | your user in the `incus-admin` group, or `incus` (see below) |
| **SSH**          | Incus runs on a server you can log in to           | the SSH user in the `incus-admin` group, or `incus`          |
| **TLS** (HTTPS)  | Incus listens on the network, or SSH isn't allowed | a trust token, or a client certificate Incus already trusts  |

Plain **TCP** is greyed out: Incus doesn't serve its API without TLS. Homeport connects to **Incus 6.0** (the long-term support release) and later.

### Its sockets

| Socket                            | Who may use it                   | What it reaches                                     |
| --------------------------------- | -------------------------------- | --------------------------------------------------- |
| `/var/lib/incus/unix.socket`      | the `incus-admin` group          | everything: every project, storage pool and network |
| `/var/lib/incus/unix.socket.user` | the `incus` group (`incus-user`) | the user's own project only, made on first use      |

On Linux, **Found on this machine** lists both when they answer, the restricted one as **Incus (restricted to your project)**. A socket you may not use yet is listed with the command that fixes it:

```bash
sudo usermod -aG incus-admin "$USER"
```

Group changes apply to new logins: log out and in again (or reconnect over SSH) before adding the Host. A stopped Incus says which units start it:

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Over HTTPS with a trust token

Incus listens on the network once `core.https_address` is set. On the Incus server:

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

The second command prints a **trust token**: one line of letters and digits. In Homeport, pick **Incus** and **TLS**, keep **Trust token**, and paste it. Homeport shows what it reads (the name it's for, the server's addresses and certificate fingerprint, and when it expires), then on **Add**:

1. makes its own client certificate, an ECDSA P-384 key made in memory;
2. tries the token's addresses in order, and keeps the first one whose certificate has the fingerprint the token names; any other is refused, with both fingerprints;
3. sends the token with its certificate, and checks that Incus now trusts it.

The Host then appears in `incus config trust list` under the token's name. A token works once and expires; a used, revoked or expired one is refused with the command that makes a new one. To revoke Homeport later, remove its certificate there:

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Existing certificate** takes a client certificate and key Incus already trusts (`incus config trust add-certificate`), with Incus's own certificate to trust (`/var/lib/incus/server.crt`, or `cluster.crt` in a cluster).

### The server's certificate is pinned

Incus's certificate is self-signed, so there's no CA to check it against. Homeport keeps the certificate it trusted with the Host and accepts exactly that one afterwards, compared by its SHA-256 fingerprint, whatever address it's reached at. If Incus is reinstalled or its certificate renewed, the Host is refused with the fingerprint it expected and the one it got: check the new one on the server with `incus info`, then add the Host again with a new token. The client key and both certificates are stored encrypted like every other secret; see [security](/docs/reference/security/#incus-certificates).

## Projects

Incus keeps Instances, images, profiles, custom volumes and networks in **projects**. The pages show one project at a time, picked under the Host switcher, which lists each with how many Instances and images it holds. Homeport remembers the pick for each Host between launches. Until you pick one, a Host shows `default`, or its only project on the restricted socket. Storage pools belong to the whole server; their volumes are shown for the selected project.

## Events

Pages refresh from Incus's lifecycle events, in every project: an Instance starting or stopping, a snapshot taken, an image added, a profile or network changed. They're also kept on the **Events** page. Incus keeps no history of them, so what happened while Homeport was closed isn't filled in.

While connected, Homeport asks Incus `GET /1.0` every 15 seconds. A stopped Incus makes the Host **Unreachable**, and Homeport reconnects on its own when it answers again.

## What Homeport does on the Host

Nothing runs on an Incus Host apart from Incus itself: consoles and terminals go through Incus's own websockets, and remote images are downloaded by Incus. Homeport reads a remote image server's list of images from this computer, for the image pickers.
