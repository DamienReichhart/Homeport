<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/lockup-dark.svg" />
    <img src="assets/brand/lockup-light.svg" alt="Homeport" height="64" />
  </picture>
</h1>

**Every Docker Engine, on one board.**

[Website](https://damienreichhart.github.io/Homeport/) · [Documentation](https://damienreichhart.github.io/Homeport/docs/) · [Download](https://damienreichhart.github.io/Homeport/download/)

A desktop app for managing Docker Engines anywhere — this machine's, or remote ones over SSH, TLS or TCP — the Swarms they form, and the Kubernetes clusters your kubeconfig reaches, in the spirit of Docker Desktop. Built with [Tauri 2](https://tauri.app), React 19, TypeScript and Rust ([bollard](https://github.com/fussybeaver/bollard) for the Engine API, [kube-rs](https://kube.rs) for Kubernetes, SQLite for its own data).

## Features

- **Hosts**: add local sockets (auto-detected: Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman, including each running Podman machine on macOS), TCP, mutual TLS and SSH. SSH Hosts get their own Ed25519 key with a copy-paste command to authorize it, can be reached through one or more bastions (`ProxyJump`), and every server's key is trusted on first use. Import Hosts from `~/.ssh/config` (keys and `ProxyJump` included) and from Docker contexts (TLS certificates included), and move them to another computer in a passphrase-protected export file. Status, reconnection with backoff, and a live event stream per Host whose history is kept across restarts.
- **Containers**: grouped by Stack, marked when a newer image is published, live CPU/memory, lifecycle and bulk actions, logs (follow, search, download), inspect, interactive terminals, file browser with upload/download, stats charts, processes, and a Run dialog.
- **Images, Volumes, Networks**: search Docker Hub and its tags, pull and push with progress and stored registry logins, build from a folder on this machine, save to and load from a tar file, tag, run, history, prune; each volume's size, a read-only file browser, backing a volume up to a tar file and restoring it, and cloning it, through a short-lived helper container that leaves nothing behind; volume and network usage, create, connect/disconnect, guarded deletion.
- **Stacks**: every Compose project on a Host. Managed Stacks keep their YAML and encrypted environment in Homeport and deploy through the `docker compose` CLI; Linked Stacks are project folders on this computer, found from their compose labels or picked by hand, edited in place and deployed from the folder, builds included; Synced Stacks deploy a compose file from a git repository (a per-repository deploy key or a token, kept encrypted), show the new commits and the diff before each sync, and are checked for new commits while Homeport is open, with an optional auto-deploy (nothing syncs while it's closed); External Stacks can be started, stopped, removed or adopted. New Stacks start from templates; a Stack can combine several compose files and choose its Compose profiles; every redeploy shows what changes first, each deploy is kept in a history a Managed Stack can roll back to, and an "Update available" badge offers to pull newer images and redeploy in one step.
- **Swarms**: every Docker Swarm a Host belongs to, found through its manager Hosts (once each, by swarm ID) and managed through any connected one, with nothing installed on a node: nodes (drain, pause, promote, demote, labels, remove), services (create, scale, update image, environment and replicas, roll back, logs from every task), tasks with their errors, secrets and configs (a secret's value is sent once and never kept), and swarm stacks, External ones found by their namespace and removed like `docker stack rm`, and Managed, Linked and Synced Stacks deployed with `docker stack deploy`. Start a Swarm on a Host, join one or leave it, with the join token fetched from a manager.
- **Clusters**: Kubernetes clusters added from a kubeconfig, this machine's (`KUBECONFIG` merged as kubectl does) or a pasted one kept in memory, their credentials sealed; EKS, GKE and AKS through their exec plugins, OIDC users, and local clusters (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube) found in one click. Browse Namespaces, Deployments, StatefulSets, DaemonSets, Jobs, CronJobs, Pods, Services, Ingresses, ConfigMaps, Secrets (each value revealed on demand, never stored), Nodes, Events and any other API resource, custom resources included with the columns of `kubectl get`, kept live by watches, with a clear "not allowed" where RBAC says no. Follow a Pod's or a workload's logs (the previous instance too), open a terminal in a container, forward a port to this computer, scale, restart, suspend and run CronJobs, delete with a confirmation naming the cluster, and apply or edit YAML with a dry-run diff and server-side apply. Manage Helm releases through the `helm` CLI on this machine — list them with their history, values and resources, install a chart from a repository or an OCI registry with a values editor and a preview, upgrade, roll back and uninstall, and manage helm's chart repositories — with no cluster credential ever written to disk. Nothing is installed on a cluster.
- **Overview and Board**: per-host and per-cluster cards, engine details, disk usage and clean-up, recent events, a cluster's Nodes, Pods, unready workloads and warnings; the Board lists the containers of every Host in one table, with filters and actions on a selection that spans Hosts.
- **Keyboard**: a command palette (Mod+K) to go to any page, switch Host or run a command; shortcuts for every page and Host, listed with `?`.
- **Desktop**: a system tray icon with every Host's status, closing to the tray, starting at login (hidden if you like), OS notifications when a container dies or turns unhealthy and when a Host is lost or back, one running Homeport, and a window that reopens where it was.
- **Backups**: everything Homeport holds — Hosts, Clusters, Stacks, registry logins, repositories, settings and their secrets — in one passphrase-protected file, restored on this or another computer without re-entering a key.

Secrets (SSH keys, TLS material, kubeconfig credentials, stack environments, registry passwords and tokens) are encrypted in SQLite with a key kept in the OS keychain. See [GLOSSARY.md](GLOSSARY.md) and [docs/adr](docs/adr) for the vocabulary and the main decisions.

## Download

On Debian, Ubuntu, Fedora, RHEL and openSUSE, install it from the Homeport package repository, so `apt`, `dnf` or `zypper` keeps it up to date:

```sh
curl -fsSL https://damienreichhart.github.io/Homeport/install.sh | sh
```

The [installation guide](https://damienreichhart.github.io/Homeport/docs/installation/#manual-setup) shows the same setup step by step, without the script.

Installers for Windows, macOS and Linux (x64 and ARM) are on the [download page](https://damienreichhart.github.io/Homeport/download/) and the [releases page](https://github.com/DamienReichhart/Homeport/releases/latest). Windows, macOS and AppImage installs update themselves from inside Homeport. Homebrew, winget, Scoop, the AUR, Flathub and Snap are planned after 1.0 ([#5 to #10](https://github.com/DamienReichhart/Homeport/milestone/18)).

Pushing a new version to `main` publishes it; see [docs/RELEASING.md](docs/RELEASING.md).

## Prerequisites

- [Bun](https://bun.sh) ≥ 1.4
- Rust (stable; `rust-toolchain.toml` pins components)
- [cargo-about](https://github.com/EmbarkStudios/cargo-about) for release builds: `cargo install cargo-about --locked --features cli`
- Tauri system dependencies: <https://tauri.app/start/prerequisites/>
  (Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev libayatana-appindicator3-dev libssl-dev`)
- At runtime: an OS keychain (macOS Keychain, Windows Credential Manager or a Secret Service such as GNOME Keyring) for SSH/TLS/stack secrets, the `docker compose` CLI to deploy Managed, Linked and Synced Stacks, and `git` for Synced Stacks. All are optional; the app says what is unavailable without them.

## Commands

| Command               | What it does                                     |
| --------------------- | ------------------------------------------------ |
| `bun install`         | Install JS dependencies                          |
| `bun run tauri:dev`   | Run the desktop app with hot reload              |
| `bun run tauri:build` | Produce an installer/bundle                      |
| `bun run check`       | Typecheck + lint + format check + frontend tests |
| `bun run rust:lint`   | `cargo clippy` with warnings as errors           |
| `bun run rust:test`   | Rust unit tests                                  |
| `bun run bindings`    | Regenerate `src/shared/api/bindings.ts`          |
| `bun run licenses`    | Generate the third-party license notices         |

`tauri:build` writes installers to `target/release/bundle/`. Each one adds Homeport to the app menu: `.deb`/`.rpm` install a desktop entry, `.msi`/NSIS add a Start Menu shortcut, and the `.app` shows in Launchpad once it is in Applications. An AppImage has no install step, so it adds its own entry to `~/.local/share/applications` each time it runs, and the entry disappears from the menu once the file is deleted.

Integration tests talk to the local Docker Engine (and the compose CLI). They only create resources named `homeport-test-*` and remove them afterwards; run them with `cargo test --workspace -- --ignored`.

## Licenses

`bun run licenses` writes `src-tauri/licenses/THIRD_PARTY_LICENSES.txt` (gitignored): every Rust crate (via `cargo about`) and npm production dependency compiled into Homeport, with its license text. It fails when a dependency's license isn't in `about.toml > accepted`, so review a new license before adding it there. `tauri build` runs it first (`beforeBuildCommand`), and a release build without the file stops with an error.

Every bundle ships `LICENSE` and the notices in its resources (`licenses/`), and Settings shows both under **Licenses**. The `.deb` also installs them in `/usr/share/doc/homeport/`, the `.rpm` in `/usr/share/licenses/homeport/`, and the MSI/NSIS installers show `LICENSE`.

## Website

The website and user documentation live in [`website/`](website), built with [Astro](https://astro.build) and [Starlight](https://starlight.astro.build) and published to GitHub Pages by `.github/workflows/pages.yml`: on every push to `main` that touches it, and after every release, so the download page always lists the latest installers. The same deploy publishes the APT and RPM repositories, the install script and the in-app updater's manifest next to the site (`bun scripts/repositories.ts`, see [docs/RELEASING.md](docs/RELEASING.md#package-repositories-and-updates)).

```sh
cd website
bun install
bun run dev     # http://localhost:4321/Homeport/
bun run lint    # checks the layers (docs/ARCHITECTURE.md#website-layout) and file sizes
bun run test    # unit tests (bun test), next to the code they test
bun run build   # type-checks, validates links and builds into website/dist/
```

User docs are Markdown in `website/src/content/docs/docs/`. The app screenshots in `website/src/assets/screenshots/` come from the real UI with demo data: `bun run screenshots` (at the repo root, needs Firefox) retakes them, and `bun scripts/brand-images.ts` (in `website/`) regenerates the social card and icons.

## Layout

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the rules that keep this scalable.

```
src/                 React frontend
  app/               shell: providers, router, layout (composes features)
  features/<name>/   self-contained vertical slices, public API via index.ts
  shared/            feature-agnostic api / ui / hooks / lib / styles
src-tauri/           Rust app shell (Tauri)
  src/app/           bootstrap, managed state, events pushed to the webview
  src/commands/      thin IPC handlers, one module per feature
  capabilities/      Tauri permission sets
crates/              Rust backend logic, one crate per layer (no Tauri types)
  core/              AppError, the Host model, helpers for this computer
  storage/           SQLite database and sealed secrets
  transport/         reaching a Host: local, TCP, TLS, SSH, compose proxy
  engine/            containers, images, volumes, networks… over bollard
  hosts/  stacks/    the two domains: Hosts and Stacks
  git/               Synced Stacks' repositories and the git CLI
  settings/ updates/ preferences, and how Homeport updates itself
packaging/linux/     repository definitions, signing key, install script and
                     .deb/.rpm maintainer scripts
scripts/             release planning, repository builder, signing-key wizard,
                     license notices, screenshots
website/             the website and user documentation
```

## License

[AGPL-3.0-only](LICENSE) © Damien Reichhart. Versions up to 0.11 were released under the MIT License and stay available under it. Third-party software is listed in the generated notices described above.
