---
title: Hosts and connections
description: How Homeport connects to Docker Engines, what each Host status means, and how reconnection works.
---

A **Host** is a Docker Engine that Homeport connects to. It has a name and a way to reach it. Every container, image, volume, network and Stack you see belongs to exactly one Host.

## Kinds of connection

| Type             | Reach an Engine…                                 | Credentials                                | Guide                               |
| ---------------- | ------------------------------------------------ | ------------------------------------------ | ----------------------------------- |
| **Local socket** | on this machine, through a socket or pipe        | none                                       | [Local engines](/docs/hosts/local/) |
| **SSH**          | on a server you can log in to                    | a Homeport key, your own key, or ssh-agent | [SSH](/docs/hosts/ssh/)             |
| **TLS**          | exposed with `dockerd --tlsverify`               | CA certificate, client certificate and key | [TLS and TCP](/docs/hosts/tls-tcp/) |
| **TCP**          | exposed without encryption, on a trusted network | none                                       | [TLS and TCP](/docs/hosts/tls-tcp/) |

For a remote server, prefer **SSH**: it needs nothing exposed beyond the SSH port and is the simplest to set up securely.

## Adding, editing and removing Hosts

- **Add** a Host with **Add host…** in the Host switcher at the top of the sidebar, or **Add host** on the Overview.
- **Manage hosts** in the Host switcher lists every Host with its type, address and status. Select one to open its page.
- On a Host's page you can change its connection details and choose **Save**; Homeport reconnects with the new settings. A Host's connection type can't be changed — remove it and add it again instead.
- The **Enabled** switch disconnects a Host without forgetting it. A disabled Host doesn't retry until you turn it back on.
- **Remove host** forgets the Host, its stored credentials and the compose definitions of its Managed Stacks. Nothing running on the Host is touched: containers, images and volumes stay as they are.

Host names must be unique, ignoring case.

## Importing Hosts

**Add host** offers two ways to add many Hosts at once, from what this computer already describes. Each opens a preview: pick the entries to import, rename them, and choose **Import**.

- **Import from ~/.ssh/config** adds the servers your SSH config names, with their user, port, key and `ProxyJump` bastions. See [Import from ~/.ssh/config](/docs/hosts/ssh/#import-from-sshconfig).
- **Import Docker contexts** adds the Engines of the Docker CLI's contexts (`docker context ls`), with their TLS certificates.

Keys and certificates are read once, when you import, and stored encrypted like any other; neither `~/.ssh` nor `~/.docker` is ever changed.

### Docker contexts

Homeport reads every context under `~/.docker/contexts` (or under `$DOCKER_CONFIG` when it's set) and maps its Docker endpoint:

| Endpoint                                                  | Becomes                                                      |
| --------------------------------------------------------- | ------------------------------------------------------------ |
| `unix:///…`, `npipe:////./pipe/…`                         | a **Local socket** Host                                      |
| `tcp://host:port` with no TLS files                       | a **TCP** Host                                               |
| `tcp://host:port` with `ca.pem`, `cert.pem` and `key.pem` | a **TLS** Host, with its certificates                        |
| `ssh://user@host:port`                                    | an **SSH** Host using ssh-agent, with `/var/run/docker.sock` |

The built-in `default` context isn't offered: [local detection](/docs/hosts/local/) already finds this machine's Engine. A context is shown but can't be picked when:

- it sets `SkipTLSVerify`, since Homeport always checks the Engine's certificate;
- its TLS folder holds only some of the three files;
- its endpoint is something else, such as a Kubernetes one;
- a Host already reaches the same Engine. Its name says which.

A context whose name is already a Host's is offered as `name (2)`.

## Selecting a Host

Every resource page — Containers, Images, Volumes, Networks, Stacks and Events — shows the Host selected in the **Host switcher**. Switch Host there, or select a Host's card on the [Overview](/docs/guides/overview/). Homeport remembers your choice between launches.

## Statuses

Each Host shows a status next to its name. When the selected Host isn't connected, resource pages show its status with **Retry now** and **Open host settings**.

| Status                     | Meaning                                                                        | What happens next                                                           |
| -------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| **Connected**              | The Engine answered. Its version is shown.                                     | Homeport keeps watching the connection.                                     |
| **Connecting**             | Homeport is reaching the Engine.                                               | —                                                                           |
| **Disabled**               | The Host's **Enabled** switch is off.                                          | Nothing until you turn it on.                                               |
| **Unreachable**            | The network, socket or Engine didn't answer.                                   | Retries on its own, waiting longer each time.                               |
| **Access refused**         | The SSH server refused the key, ssh-agent failed, or the TLS handshake failed. | Fix the cause, then **Reconnect**.                                          |
| **Server key not trusted** | First SSH connection: the server's or a bastion's key needs your approval.     | [Trust the key](/docs/hosts/ssh/#trust-the-servers-key) on the Host's page. |
| **Server key changed**     | The SSH server presented a different key from the one you trusted.             | [Investigate first](/docs/hosts/ssh/#when-the-servers-key-changes).         |
| **Credentials lost**       | The stored credentials can't be decrypted.                                     | [Re-enter them](/docs/reference/troubleshooting/#credentials-lost).         |
| **Keychain unavailable**   | The Host needs stored secrets but there's no OS keychain.                      | See [requirements](/docs/requirements/#optional-an-os-keychain).            |
| **Misconfigured**          | Something in the Host's settings is missing or invalid.                        | The message says what; fix it and **Save**.                                 |

## Reconnection

Homeport connects to every enabled Host when it starts, and each Host has its own connection, so one slow server never holds up another.

While connected, Homeport listens to the Engine's event stream — which is how pages update by themselves — and checks the Engine every 15 seconds. If the connection drops, the Host becomes **Unreachable** and Homeport retries after 1 second, then 2, 4, 8 and so on, up to once a minute. A successful connection resets the wait.

Only **Unreachable** retries automatically. Every other problem needs something from you, and then **Reconnect** (on the Host's page) or **Retry now** tries again straight away.

## Events and live updates

Homeport doesn't poll. When the Engine reports a change — a container starting, an image being pulled — the affected pages refresh within a fraction of a second. The [Events](/docs/guides/overview/#events) page shows the raw stream.
