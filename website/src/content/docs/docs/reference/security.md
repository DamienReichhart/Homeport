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
| Your theme and last selected Host                 | Plain text    |

Homeport doesn't copy anything from your Engines: containers, images and logs are read live and never saved, except when you download logs or files yourself.

## How secrets are encrypted

- Each secret is sealed on its own with **XChaCha20-Poly1305**, an authenticated cipher, so tampering is detected as well as reading prevented.
- The 256-bit key is random, created on first launch, and kept in your **OS keychain** — macOS Keychain, Windows Credential Manager, or the Secret Service on Linux — under the service `com.homeport.app`. It's never stored next to the data.
- There's no master password: the keychain already ties the key to your user account.

Someone with a copy of `homeport.db` alone can read Host names and addresses, but not your keys, certificates or Stack environments.

### Without a keychain

Homeport shows a banner and won't store secrets. Local socket, TCP and ssh-agent Hosts still work. See [requirements](/docs/requirements/#optional-an-os-keychain).

### If the keychain key is lost

Resetting the keychain, or moving `homeport.db` to another computer or user, leaves secrets that can't be decrypted. Homeport doesn't delete anything: affected Hosts show **Credentials lost** until you re-enter their credentials. See [troubleshooting](/docs/reference/troubleshooting/#credentials-lost).

## Keys never touch the disk in clear

Homeport has its own SSH client, so it never writes a private key to a file for `ssh` to read, and never changes `~/.ssh`. Decrypted keys stay in memory.

When `docker compose` deploys a Stack to an SSH or TLS Host, Homeport opens a short-lived private socket on this computer — readable only by your user, or a named pipe that refuses remote clients on Windows — and forwards it over its own connection, for that one command.

## Trusting servers

The first time Homeport connects to an SSH server, it asks you to confirm the server's key fingerprint, and it refuses to connect if that key later changes until you confirm again. TLS Hosts verify the server's certificate against your CA and the server's name. See [SSH Hosts](/docs/hosts/ssh/#first-connection).

## What Homeport never does

- **No account.** There is nothing to sign up for.
- **No telemetry, analytics or crash reporting.**
- **No update checks** or other calls home. Homeport connects only to the Hosts you add — and when you pull an image, the Engine contacts the registry.

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
