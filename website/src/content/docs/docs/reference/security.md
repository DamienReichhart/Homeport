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

| Data                                              | Stored        |
| ------------------------------------------------- | ------------- |
| Host names, addresses, SSH users and socket paths | Plain text    |
| SSH public keys and the server keys you trusted   | Plain text    |
| SSH private keys and their passphrases            | **Encrypted** |
| TLS CA certificates, client certificates and keys | **Encrypted** |
| Managed Stack compose files                       | Plain text    |
| Managed Stack environments                        | **Encrypted** |
| Linked Stack folders and compose file paths       | Plain text    |
| Deploy history: compose files and profiles        | Plain text    |
| Deploy history: environments                      | **Encrypted** |
| Registry addresses and user names                 | Plain text    |
| Registry passwords and access tokens              | **Encrypted** |
| Your theme and last selected Host                 | Plain text    |

Homeport doesn't copy anything from your Engines: containers, images and logs are read live and never saved, except when you download logs or files yourself.

### Linked Stack folders

For a [Linked Stack](/docs/guides/stacks/#linked-stacks), Homeport stores only where its folder and compose files are, and a fingerprint of the files to tell when they change — never a copy of them. It reads the compose files when you open the Stack, and writes to them only when you choose **Save** in its **Definition** tab, replacing each edited file in one step and keeping its permissions; it won't overwrite a file that changed since you opened it unless you say so. Homeport never reads or writes the folder's `.env`: Compose reads it when it runs. Forgetting a Linked Stack never touches the folder.

## How secrets are encrypted

- Each secret is sealed on its own with **XChaCha20-Poly1305**, an authenticated cipher, so tampering is detected as well as reading prevented.
- The 256-bit key is random, created on first launch, and kept in your **OS keychain** — macOS Keychain, Windows Credential Manager, or the Secret Service on Linux — under the service `com.homeport.app`. It's never stored next to the data.
- There's no master password: the keychain already ties the key to your user account.

Someone with a copy of `homeport.db` alone can read Host names and addresses, but not your keys, certificates, Stack environments or registry passwords.

### Without a keychain

Homeport shows a banner and won't store secrets. Local socket, TCP and ssh-agent Hosts still work. See [requirements](/docs/requirements/#optional-an-os-keychain).

### If the keychain key is lost

Resetting the keychain, or moving `homeport.db` to another computer or user, leaves secrets that can't be decrypted. Homeport doesn't delete anything: affected Hosts show **Credentials lost** until you re-enter their credentials. See [troubleshooting](/docs/reference/troubleshooting/#credentials-lost). To move Hosts, export them instead.

### Moving to another computer

**Export** on the Hosts page writes the Hosts you pick to one file, with their keys, certificates and trusted server keys, encrypted with a passphrase you choose:

- The whole file is encrypted (XChaCha20-Poly1305, with the key derived from the passphrase by Argon2id), so without the passphrase it reveals nothing — not even Host names or addresses. Any change to the file makes it unreadable.
- The passphrase needs at least 12 characters. Homeport never stores or logs it; if you lose it, the file can't be opened.
- **Import** on the other computer asks for the passphrase, then stores each Host's secrets encrypted under that computer's own keychain key, like any other.
- A Host whose credentials are already lost on this computer is exported without them, and the export says which.

Treat the file like the keys it holds: delete it once imported. The format is described in [ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

## Keys never touch the disk in clear

Homeport has its own SSH client, so it never writes a private key to a file for `ssh` to read, and never changes `~/.ssh`. Decrypted keys stay in memory.

When `docker compose` deploys a Stack to an SSH or TLS Host, Homeport opens a short-lived private socket on this computer — readable only by your user, or a named pipe that refuses remote clients on Windows — and forwards it over its own connection, for that one command.

Registry logins never reach `docker compose` either: Homeport pulls a Stack's images itself, through the Engine API, before running `docker compose up`, so the command's environment and the files it's given hold no password or token.

## Trusting servers

The first time Homeport connects to an SSH server, it asks you to confirm the server's key fingerprint, and it refuses to connect if that key later changes until you confirm again. TLS Hosts verify the server's certificate against your CA and the server's name. See [SSH Hosts](/docs/hosts/ssh/#first-connection).

## What Homeport connects to

- **The Hosts you add.** When you pull, push or build an image, the Engine contacts the registry, not Homeport. Homeport hands it the stored login for that registry with the request, over the Host's connection; the Engine doesn't keep it.
- **Your images' registries, through each Host's Engine**, to tell whether a [newer image](/docs/guides/containers/#newer-images) is published: when the Containers, Images or Stacks page loads (each answer is kept for 6 hours) and when you choose **Check for newer images**. The Engine asks the registry for the digest a tag points to, with the stored login for private registries, and downloads nothing. For a local Host, the Engine runs on your computer, so the request leaves from it.
- **Docker Hub, when you browse tags** in the pull dialog: Homeport lists a repository's tags from `hub.docker.com`, from this computer, with your Docker Hub login when one is stored. Searching repositories goes through the Host's Engine instead.
- **A registry, when you check its login** in **Settings → Registries**: Homeport asks the registry's `/v2/` endpoint, and its token service, from this computer, sending the user name and password or token over HTTPS (plain HTTP only for a registry on this computer, such as `localhost:5000`).
- **This website, to check for updates**: at launch and every 6 hours, Homeport downloads `updates/latest.json`, a small file listing the latest version. The request carries nothing about you, your Hosts or your containers, but like any web request it reaches GitHub Pages, which hosts this site, from your IP address. Turn it off in **Settings → Updates**. Updates themselves download from GitHub Releases, and Homeport installs one only if it's signed with Homeport's update key. See [updating](/docs/reference/updating/).

## What Homeport never does

- **No account.** There is nothing to sign up for.
- **No telemetry, analytics or crash reporting.**
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
