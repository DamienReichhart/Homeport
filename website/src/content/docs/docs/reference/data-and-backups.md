---
title: Your data and backups
description: Where Homeport keeps its data on each system, how to back it up and restore it, and what to re-enter when secrets are lost.
---

Homeport keeps what it knows on your computer: how to reach your Hosts and Clusters, your Stacks' definitions and environments, your registry logins, repositories and settings. This page says where that is, what to copy, and how to get it back. For how it's protected, see [Security and your data](/docs/reference/security/).

What runs on your Hosts and Clusters isn't Homeport's data: containers, images, volumes and Kubernetes or Nomad objects stay where they are, and no backup of Homeport holds them. To copy a volume's content, [back it up to a file](/docs/guides/volumes-networks/#backing-up-and-restoring) from the Volumes page.

## What lives where

### The data folder

| System  | Folder                                            |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

On Linux the folder is under `$XDG_DATA_HOME` when that's set. If Homeport can't open its database, its window shows the file's full path, with **Open folder**.

| In the folder                        | What it is                                                                                                                                               |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | The database: every Host, Cluster, Stack, registry login, repository and setting, the deploy history and the Engine events. Secrets in it are encrypted. |
| `homeport.db-wal`, `homeport.db-shm` | SQLite's write-ahead log and its index, there while Homeport runs. The latest changes may be only in the `-wal` file until Homeport quits.               |
| `homeport.db.before-restore-<date>`  | A copy of the database made just before each [restore](#restoring-a-backup). Delete the ones you no longer want.                                         |
| `repositories/` and `synced-stacks/` | The clones and working copies of [Synced Stacks](/docs/guides/synced-stacks/)' repositories. Homeport makes them again from the repositories themselves. |

[What Homeport stores](/docs/reference/security/#what-homeport-stores) lists what the database holds, and which parts are encrypted.

### The keychain entry

The key that encrypts the secrets isn't in the folder. It's one entry in your operating system's keychain, with the service `com.homeport.app` and the account `secrets-key`: in **Keychain Access** on macOS, **Credential Manager → Windows Credentials** on Windows, and **Passwords and Keys** (Seahorse) or KWallet on Linux. It belongs to your user account on this computer.

The database and that entry go together. A database without its key still opens, with every name, address and compose file, but none of its secrets can be decrypted: Hosts and Clusters show **Credentials lost**. That's why a copy of the folder isn't enough to move to another computer, and why the [in-app backup](#backing-up-in-homeport) is the recommended one.

### Logs and the window's place

| System  | Logs                                    | Window size and position                                            |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

Neither is worth backing up. **Settings → About → Open log folder** opens the logs.

### What stays yours

- **Linked Stacks' folders.** Homeport keeps only where a [Linked Stack](/docs/guides/stacks/#linked-stacks)'s folder and compose files are. The folder, its compose files and its `.env` are yours: back them up as you do your other projects, usually with git.
- **Your kubeconfig, `~/.ssh` and `~/.docker`.** Homeport reads them when you import from them and never changes them.
- **helm's chart repositories and logins.** They're helm's own, shared with your terminal.

## Backing up in Homeport

**Settings → Data → Back up…** writes everything to one `.homeport` file. This is the backup to prefer, because it carries the secrets themselves, protected by a passphrase instead of this computer's keychain: it restores on any computer.

1. Open **Settings**, and under **Data** choose **Back up…**.
2. Type a passphrase of at least 12 characters, twice. Homeport never stores it, and nothing can open the file without it.
3. Choose where to save the file.

Homeport then says what it saved, and names anything it had to save without its secret because that secret couldn't be decrypted on this computer.

| In the backup                                                                               | Not in the backup                                                |
| ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| Hosts, with their keys, passphrases, certificates, bastions and the server keys you trusted | Engine events                                                    |
| Clusters, with their certificates, tokens and exec plugin settings                          | Deploy history                                                   |
| Managed Stacks' compose files and environments                                              | Linked Stacks' folders and files                                 |
| Linked Stacks' folder and file paths                                                        | Synced Stacks' clones (made again on the next sync)              |
| Synced Stacks' repository, branch or tag, sync settings and environment                     | Logs, and the window's size and position                         |
| Stacks deployed to a Swarm                                                                  | Anything on your Hosts and Clusters: containers, images, volumes |
| Registry logins, with their passwords and tokens                                            | helm's chart repositories                                        |
| Repositories, with their deploy keys and tokens                                             | Port forwards, which end when Homeport quits                     |
| Your settings                                                                               |                                                                  |

The whole file is encrypted with the passphrase, so without it the file reveals nothing, not even a Host's name. It holds every secret Homeport has: keep it, and its passphrase, as safe as the keys themselves. [Backups](/docs/reference/security/#backups) describes the encryption.

Back up before you try a prerelease, before you reinstall your system, and whenever you've added Hosts you wouldn't want to set up again.

### Restoring a backup

1. On this or another computer, open **Settings**, and under **Data** choose **Restore…**.
2. Pick the `.homeport` file and type its passphrase. Homeport lists what the file holds, and which Homeport made it and when, before it changes anything.
3. Choose **Replace everything**.

Restoring replaces all of Homeport's data with the backup's; it doesn't merge. Homeport first copies the current database to `homeport.db.before-restore-<date>` in the data folder, then replaces everything in one step, so a failure leaves your data as it was. Every secret is encrypted again with this computer's keychain key.

Afterwards:

- Hosts and Clusters reconnect at once, without asking for a key, a password or a server key to trust.
- A Linked Stack whose folder isn't on this computer, at the same path, shows as External until it is.
- Synced Stacks clone their repository again on their next sync.
- The Events page and the deploy history start empty.

A backup made by an older Homeport restores in a newer one. One made by a newer Homeport is refused: update Homeport first.

To go back to what you had before a restore, quit Homeport, rename `homeport.db` to something else, rename the `homeport.db.before-restore-<date>` you want to `homeport.db`, delete any `homeport.db-wal` and `homeport.db-shm` left beside it, and open Homeport again.

### Moving only some Hosts

**Export** and **Import** on the Hosts page move the Hosts you pick, with their secrets, in a file of the same kind, and add them to the Hosts the other computer already has. See [moving to another computer](/docs/reference/security/#moving-to-another-computer).

## Copying the files yourself

A copy of the database suits a system backup of this computer, such as Time Machine, File History or your own script. It keeps what the in-app backup leaves out, the Engine events and the deploy history, but **not the keychain key**: it restores with its secrets only for the same user on the same computer, or one whose keychain was restored with it. Anywhere else, every name, address and compose file comes back, and the secrets have to be [entered again](#if-the-secrets-are-lost).

### While Homeport is closed

Quit Homeport, from the tray's **Quit** if it's [closed to the tray](/docs/guides/settings/#desktop), then copy the whole data folder. Once Homeport has quit, `homeport.db` holds everything and the `-wal` and `-shm` files are gone.

### While Homeport is running

Don't copy `homeport.db` alone while Homeport runs: the latest changes may still be in `homeport.db-wal`, and a copy taken mid-write can be inconsistent. Ask SQLite for a consistent copy instead, with the `sqlite3` command-line tool (included with macOS; on Linux and Windows, install it from your distribution's `sqlite3` package or from [sqlite.org](https://sqlite.org/download.html)):

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

The result is one complete file, with no `-wal` beside it. To check it:

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

It prints `ok`.

### Restoring a copy

1. Quit Homeport.
2. In the data folder, move `homeport.db` aside, and delete `homeport.db-wal` and `homeport.db-shm` if they're there: they belong to the database you're replacing.
3. Put your copy there, named `homeport.db`. If you copied the whole folder, put the folder back instead.
4. Open Homeport.

Homeport upgrades a copy made by an older version when it opens it. A copy last used by a newer Homeport is refused, untouched, with [a message naming the version to install](/docs/reference/troubleshooting/#this-database-was-last-used-by-homeport--or-later).

## If the secrets are lost

After a keychain reset, a new user account, or a copied database opened on another computer, Homeport keeps everything it can still read and marks what it can't. Nothing is deleted, and what isn't a secret (names, addresses, compose files, folder paths) is intact. Enter again:

| What                               | How                                                                                                                                                     |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **TLS Hosts**                      | On the Host's page, load the CA certificate, client certificate and client key again, and choose **Save**.                                              |
| **SSH Hosts with an imported key** | Paste the private key and its passphrase again, and choose **Save**.                                                                                    |
| **SSH Hosts with a Homeport key**  | Choose **Regenerate key**, then [authorise the new key](/docs/hosts/ssh/#authorise-homeports-key) on the server. Remove the old `homeport@` line there. |
| **Incus Hosts over HTTPS**         | Remove the Host and [add it again with a new trust token](/docs/hosts/incus/#over-https-with-a-trust-token).                                            |
| **Kubernetes Clusters**            | **Refresh from kubeconfig** on the Cluster's page.                                                                                                      |
| **Nomad Clusters**                 | Paste the ACL token, and the certificates if it uses them, in the Cluster's **Settings**.                                                               |
| **Stack environments**             | Type a Managed or Synced Stack's environment again in its page. Linked Stacks keep theirs in their folder's `.env`, which Homeport never held.          |
| **Registry logins**                | In **Settings → Registries**, edit each login and type its password or token.                                                                           |
| **Repositories**                   | In **Settings → Repositories**, make a **New key** and add it to the repository on your forge, or **Replace the token**.                                |

Local socket, TCP and ssh-agent Hosts hold no secret and keep working. The environments recorded in the deploy history can't be entered again; the history's compose files stay readable.

See [Credentials lost](/docs/reference/troubleshooting/#credentials-lost) in troubleshooting.

## Removing your data

[Removing your data](/docs/reference/updating/#removing-your-data) lists the folders and the keychain entry to delete, and the keys to remove from your servers.
