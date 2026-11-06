# Synced Stacks use the `git` CLI, with per-command credentials

A Synced Stack's compose definition comes from a git repository, often a private one. Homeport needs to clone it on this machine, fetch a branch or tag, read files at a commit, compare two commits and lay one out in a folder to deploy from, without ever writing a deploy key or a token to disk in clear (ADR 0002). Homeport runs the user's `git` program for all of it, the way it runs `docker compose` (ADR 0001), and hands it the repository's credentials for that one command only:

- **SSH** (`git@host:owner/repo.git`, `ssh://…`): Homeport serves a private SSH agent for the command, on a Unix socket with 0600 permissions in a 0700 folder (a named pipe that refuses remote clients on Windows), holding only the repository's deploy key, opened from SQLite. `git` runs `ssh` with `SSH_AUTH_SOCK` pointing at it and `-F none -o IdentityFile=none -o BatchMode=yes`, so neither the user's `~/.ssh/config` nor their own keys are used. The agent is `russh`'s, which SSH Hosts already depend on; it lives as long as the command.
- **HTTPS** (`https://…`): the user name and token go in an `Authorization: Basic` header set through `GIT_CONFIG_COUNT` environment variables, never on the command line. Every credential helper is switched off for the command (`credential.helper=` and an empty `GIT_CONFIG_GLOBAL`), so nothing is cached in the user's keychain or `~/.git-credentials`, and `GIT_TERMINAL_PROMPT=0` makes a refused token fail instead of prompting.
- **Server host keys** of SSH remotes are trusted on first use and kept in SQLite with the repository, like SSH Hosts' (ADR 0002): the first command runs with an empty, private `known_hosts` and `StrictHostKeyChecking=accept-new`, and Homeport stores the line `ssh` wrote; every later command gets that line back with `StrictHostKeyChecking=yes`, so a changed key fails as `git_host_key_changed` until the user resets it.

Each repository has one bare clone in the app data folder (`repositories/<id>.git`), shared by its Synced Stacks. Each Synced Stack has its own worktree of it (`synced-stacks/<id>/`), checked out detached at the commit being deployed, and compose runs from the compose file's folder in it, through the Linked deploy path (ADR 0008): `build:`, `env_file:` and relative paths work. The worktree stays in the same place across deploys, so relative bind mounts keep pointing at the same files. Homeport only ever fetches into the clone and checks out commits; it never commits, pushes or edits a file in either.

## Considered Options

- **gix** (pure Rust): no runtime requirement and HTTPS with the token in memory, but its SSH transport also runs the `ssh` program, so keys would need the same agent; doing without it means writing a git transport over a `russh` exec channel and keeping it working against a fast-moving API. Checking a commit out to a folder is also less complete than in `git`.
- **git2** (libgit2 and libssh2): `Cred::ssh_key_from_memory` keeps keys in memory, but it brings C libraries and OpenSSL to every platform's build, Ed25519 depends on how libssh2 was built, and host-key checks go through callbacks with little to show the user.
- **The `git` CLI with the user's own configuration**: the least code, but it would use the user's SSH keys and credential helpers, and cache tokens Homeport was given, outside Homeport's control.

## Consequences

- Synced Stacks need `git` on this machine, as deploying needs the compose CLI. Without it, Homeport says how to install it; every other kind of Stack still works. Git for Windows's own `ssh` can't use a named-pipe agent, so on Windows Homeport points `git` at the system's OpenSSH (`C:\Windows\System32\OpenSSH\ssh.exe`) when it's there.
- Deploy keys are generated per repository (Ed25519, like SSH Hosts' keys) and sealed in SQLite with tokens; without a keychain only public HTTPS repositories work.
- A repository's content isn't a secret under ADR 0002: the clone and worktrees are plain files in the app data folder. A Stack's own environment stays sealed and goes to compose as process variables, as for Managed Stacks.
- Nothing syncs while Homeport is closed: fetching happens on demand, or on an interval while it runs. Homeport installs nothing on a server, so there are no webhooks.
- Forgetting a Synced Stack removes its worktree, unless files the Stack wrote there (through relative bind mounts) would go with it; then the folder is kept and the log says where. The clone goes when no Synced Stack uses its repository.
