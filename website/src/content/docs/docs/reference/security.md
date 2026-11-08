---
title: Security and your data
description: What Homeport stores, where, how secrets are encrypted, and what it never does.
---

Homeport holds credentials that open your servers. This page explains exactly what it keeps, and how.

## What Homeport stores

Everything lives in one SQLite database, `homeport.db`, in your user's application data folder:

| System  | Folder                                            |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

It contains:

| Data                                                 | Stored        |
| ---------------------------------------------------- | ------------- |
| Host names, addresses, SSH users and socket paths    | Plain text    |
| SSH public keys and the server keys you trusted      | Plain text    |
| SSH private keys and their passphrases               | **Encrypted** |
| TLS CA certificates, client certificates and keys    | **Encrypted** |
| Managed Stack compose files                          | Plain text    |
| Managed Stack environments                           | **Encrypted** |
| Linked Stack folders and compose file paths          | Plain text    |
| Deploy history: compose files and profiles           | Plain text    |
| Deploy history: environments                         | **Encrypted** |
| Registry addresses and user names                    | Plain text    |
| Registry passwords and access tokens                 | **Encrypted** |
| Repository URLs, deploy keys' public halves          | Plain text    |
| Repositories' server keys you trusted                | Plain text    |
| Repository deploy keys and access tokens             | **Encrypted** |
| Cluster names, API server URLs and namespaces        | Plain text    |
| How each Cluster signs in, its kubeconfig context    | Plain text    |
| Cluster CA, client certificates and keys             | **Encrypted** |
| Cluster tokens, passwords and OIDC tokens            | **Encrypted** |
| Exec plugins' commands, arguments and environment    | **Encrypted** |
| Your theme, last selection and namespace per Cluster | Plain text    |
| Each Host's Engine events, for the Events page       | Plain text    |

Homeport doesn't copy anything else from your Engines: containers, images, volumes and logs are read live and never saved, except when you download logs or files, or back up a volume, yourself. The Engine events it keeps — what happened to which container, image, volume or network, with their labels — are kept for 30 days by default ([Settings → Data](/docs/guides/settings/#data)), at most 10,000 per Host, and removed with their Host or with **Clear history**.

### Linked Stack folders

For a [Linked Stack](/docs/guides/stacks/#linked-stacks), Homeport stores only where its folder and compose files are, and a fingerprint of the files to tell when they change — never a copy of them. It reads the compose files when you open the Stack, and writes to them only when you choose **Save** in its **Definition** tab, replacing each edited file in one step and keeping its permissions; it won't overwrite a file that changed since you opened it unless you say so. Homeport never reads or writes the folder's `.env`: Compose reads it when it runs. Forgetting a Linked Stack never touches the folder.

### Repositories

For [Synced Stacks](/docs/guides/synced-stacks/), Homeport keeps each repository's URL, and either a deploy key it generated for that repository (its private half encrypted) or your user name and an access token (encrypted). A repository's content isn't a secret: its clone sits in a `repositories` folder next to `homeport.db`.

### Swarms

Homeport stores nothing about a [Swarm](/docs/guides/swarm/): it finds one through its manager hosts each time it connects. Swarm secret values go to the Swarm and aren't kept by Homeport: a value you type or load from a file is sent to the Engine once, never written to `homeport.db`, a file or the log, and dropped from memory when the dialog closes. The Engine never sends a secret's value back, so Homeport can't show one either. Join tokens are fetched from a manager when you need them, and never stored or logged.

### Clusters

A [Cluster](/docs/clusters/)'s credentials come from your kubeconfig and are stored only encrypted, like SSH keys; files the kubeconfig refers to are read once and inlined, a pasted kubeconfig is never written to a file, and your kubeconfig is never changed. An [exec plugin](/docs/clusters/exec-plugins/) runs on this computer, as you, and the token or certificate it prints is kept in memory only, until it expires. Homeport stores nothing else about what runs on a Cluster: Pods, workloads and the rest are read live. [Port forwards](/docs/clusters/port-forward/) listen on this computer's loopback only (`127.0.0.1` and `::1`), so nothing else on your network can use them, and they end when Homeport quits. Changes made with [Apply YAML or Edit](/docs/clusters/yaml/) are recorded on the cluster under the field manager `homeport`.

[Helm](/docs/clusters/helm/) runs your `helm` CLI against a Cluster without your kubeconfig and without writing any cluster credential to disk: for each command, Homeport opens a private connection on this computer's loopback (`127.0.0.1`, a random port) that only accepts requests carrying a random token, and gives helm a temporary kubeconfig, readable by you only, holding that token and nothing else. Homeport adds the Cluster's own credentials to each request on its way, and deletes the kubeconfig and closes the connection when the command ends. Chart values go to helm on its standard input, never in a file. helm's own chart repositories, registry logins and caches stay helm's, as in a terminal; Homeport only adds public repositories and stores no repository password.

**Kubernetes Secret values** are fetched only when you choose **Reveal** for one key, one at a time: lists, a Secret's page and its YAML carry its keys and sizes, never its values. A revealed value stays in that page's memory — not in any cache, not in `homeport.db`, not in the log — and is hidden again when you leave the page.

## How secrets are encrypted

- Each secret is sealed on its own with **XChaCha20-Poly1305**, an authenticated cipher, so tampering is detected as well as reading prevented.
- The 256-bit key is random, created on first launch, and kept in your **OS keychain** — macOS Keychain, Windows Credential Manager, or the Secret Service on Linux — under the service `com.homeport.app`. It's never stored next to the data.
- There's no master password: the keychain already ties the key to your user account.

Someone with a copy of `homeport.db` alone can read Host and Cluster names and addresses, but not your keys, certificates, tokens, Stack environments or registry passwords.

### Without a keychain

Homeport shows a banner and won't store secrets. Local socket, TCP and ssh-agent Hosts still work. See [requirements](/docs/requirements/#optional-an-os-keychain).

### If the keychain key is lost

Resetting the keychain, or moving `homeport.db` to another computer or user, leaves secrets that can't be decrypted. Homeport doesn't delete anything: affected Hosts show **Credentials lost** until you re-enter their credentials. See [troubleshooting](/docs/reference/troubleshooting/#credentials-lost). To move your data, back it up or export your Hosts instead.

### Moving to another computer

**Export** on the Hosts page writes the Hosts you pick to one file, with their keys, certificates and trusted server keys, encrypted with a passphrase you choose:

- The whole file is encrypted (XChaCha20-Poly1305, with the key derived from the passphrase by Argon2id), so without the passphrase it reveals nothing — not even Host names or addresses. Any change to the file makes it unreadable.
- The passphrase needs at least 12 characters. Homeport never stores or logs it; if you lose it, the file can't be opened.
- **Import** on the other computer asks for the passphrase, then stores each Host's secrets encrypted under that computer's own keychain key, like any other.
- A Host whose credentials are already lost on this computer is exported without them, and the export says which.

Treat the file like the keys it holds: delete it once imported. The format is described in [ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

### Backups

**Settings → Data → Back up…** writes everything above — Hosts, Clusters, Stacks, registry logins, repositories and settings, with their secrets opened — to one file in the same encrypted format, with a passphrase you type twice. Engine events and deploy history aren't included. A secret that can't be opened on this computer is left out, and the backup says which.

**Restore…** asks for the passphrase and lists what the file holds before changing anything. Restoring then:

1. copies the current database to `homeport.db.before-restore-<date and time>`, next to `homeport.db`;
2. replaces every Host, Cluster, Stack, login, repository and setting in one step, so a failure leaves your data as it was;
3. encrypts every secret again with this computer's keychain key.

A backup made by a newer Homeport is refused; one made by an older Homeport restores. The backup holds every secret Homeport has: keep it, and its passphrase, as safe as the keys themselves.

## Keys never touch the disk in clear

Homeport has its own SSH client, so it never writes a private key to a file for `ssh` to read, and never changes `~/.ssh`. Decrypted keys stay in memory.

When `docker compose` deploys a Stack to an SSH or TLS Host, Homeport opens a short-lived private socket on this computer — readable only by your user, or a named pipe that refuses remote clients on Windows — and forwards it over its own connection, for that one command.

Repository keys and tokens never reach the disk either. For each `git` command, Homeport serves a private SSH agent holding only that repository's deploy key — on a socket only your user can open, or a named pipe that refuses remote clients on Windows — or passes the token in an HTTPS header through the command's environment. Your own git configuration, credential helpers and SSH keys are left out, so nothing is cached in your keychain or `~/.git-credentials`.

Registry logins never reach `docker compose` either: Homeport pulls a Stack's images itself, through the Engine API, before running `docker compose up`, so the command's environment and the files it's given hold no password or token.

## Trusting servers

The first time Homeport connects to an SSH server, it asks you to confirm the server's key fingerprint, and it refuses to connect if that key later changes until you confirm again. TLS Hosts verify the server's certificate against your CA and the server's name. See [SSH Hosts](/docs/hosts/ssh/#first-connection).

## What Homeport connects to

- **The Hosts you add.** When you pull, push or build an image, the Engine contacts the registry, not Homeport. Homeport hands it the stored login for that registry with the request, over the Host's connection; the Engine doesn't keep it.
- **Your images' registries, through each Host's Engine**, to tell whether a [newer image](/docs/guides/containers/#newer-images) is published: when the Containers, Images or Stacks page loads (each answer is kept for 6 hours) and when you choose **Check for newer images**. The Engine asks the registry for the digest a tag points to, with the stored login for private registries, and downloads nothing. For a local Host, the Engine runs on your computer, so the request leaves from it.
- **Docker Hub, when you browse tags** in the pull dialog: Homeport lists a repository's tags from `hub.docker.com`, from this computer, with your Docker Hub login when one is stored. Searching repositories goes through the Host's Engine instead.
- **A registry, when you check its login** in **Settings → Registries**: Homeport asks the registry's `/v2/` endpoint, and its token service, from this computer, sending the user name and password or token over HTTPS (plain HTTP only for a registry on this computer, such as `localhost:5000`).
- **Your repositories' servers**, for [Synced Stacks](/docs/guides/synced-stacks/): when you test access, create or sync a Synced Stack, and on an interval while Homeport is open if polling is on. Homeport runs `git` from this computer with that repository's deploy key or token.
- **The Clusters you add**, through the Kubernetes API server their kubeconfig names (and its proxy, when it has one), with the credentials it gives, checking the server's certificate against the kubeconfig's CA. Exec plugins contact their own cloud (AWS, Google Cloud, Azure) from this computer, as they do for `kubectl`.
- **Chart repositories and OCI registries, through helm**, when you add or update a repository, search charts, or install or upgrade a chart: `helm` downloads indexes and charts from this computer, as it does in a terminal.
- **This website, to check for updates**: at launch and every 6 hours, Homeport downloads `updates/latest.json`, a small file listing the latest version. The request carries nothing about you, your Hosts or your containers, but like any web request it reaches GitHub Pages, which hosts this site, from your IP address. Turn it off in **Settings → Updates**. Updates themselves download from GitHub Releases, and Homeport installs one only if it's signed with Homeport's update key. See [updating](/docs/reference/updating/).

## What Homeport runs on your Hosts

Homeport talks to a Host's Engine through its API, and runs commands on a Host's server only for what that API can't do:

| When                                                                          | Where                                                        | Command                                                                                              |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| An SSH Host's server refuses to open its Engine socket                        | the SSH server                                               | a POSIX shell check of whether the socket exists and is readable, and whether an Engine is installed |
| **Start**, **Stop** or **Restart** of a [Quadlet unit](/docs/guides/quadlet/) | the SSH server, or this computer for a local socket on Linux | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                           |

They run with the SSH user's rights (or yours, on this computer) and never with `sudo`. Values are checked and quoted before they go into a command. Installing, editing and removing Quadlet units goes through Podman's API, which writes the files itself; Homeport writes no file on a Host.

## What Homeport never does

- **No account.** There is nothing to sign up for.
- **No telemetry, analytics or crash reporting.**
- **Nothing installed on your Clusters.** Homeport only uses the Kubernetes API, with your credentials' rights, and changes nothing without you asking: whatever removes or replaces something asks first, naming the cluster.
- **Nothing installed on your Hosts.** The only commands Homeport runs on a Host are [listed above](#what-homeport-runs-on-your-hosts). The only container Homeport ever starts of its own is a short-lived [helper container](/docs/guides/volumes-networks/#the-helper-container) when you open, back up, restore or clone a volume; it has no network and is removed when it's done. Its image is pulled only once you agree.
- **No silent installs.** An update is installed only when you choose **Install and restart**; Linux packages are only ever updated by your package manager.

The [privacy policy](/privacy/) sets out what this means in law: who is responsible for your data, on what basis, and how to exercise your rights.

## Logs

Homeport writes a small log file — a few dozen kilobytes, replaced when full — for diagnosing problems. It contains connection events and errors, not your credentials or container output.

| System  | Folder                                  |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

## Trusting a download

Releases are built by GitHub Actions from the public source code. Each file comes with a SHA-256 checksum and a signed build provenance attestation that ties it to the exact commit and workflow that built it. See [verifying downloads](/docs/reference/verifying/).

## Reporting a vulnerability

Please report security problems privately through [GitHub's private vulnerability reporting](https://github.com/DamienReichhart/Homeport/security/advisories/new) rather than a public issue.
