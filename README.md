<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/lockup-dark.svg" />
    <img src="assets/brand/lockup-light.svg" alt="Homeport" height="64" />
  </picture>
</h1>

**Every Docker Engine, on one board.**

[Website](https://damienreichhart.github.io/Homeport/) · [Documentation](https://damienreichhart.github.io/Homeport/docs/) · [Download](https://damienreichhart.github.io/Homeport/download/)

A desktop app for managing Docker Engines anywhere — this machine's, or remote ones over SSH, TLS or TCP — in the spirit of Docker Desktop. Built with [Tauri 2](https://tauri.app), React 19, TypeScript and Rust ([bollard](https://github.com/fussybeaver/bollard) for the Engine API, SQLite for its own data).

## Features

- **Hosts**: add local sockets (auto-detected: Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman), TCP, mutual TLS and SSH. SSH Hosts get their own Ed25519 key with a copy-paste command to authorize it; server keys are trusted on first use. Status, reconnection with backoff and a live event stream per Host.
- **Containers**: grouped by Stack, marked when a newer image is published, live CPU/memory, lifecycle and bulk actions, logs (follow, search, download), inspect, interactive terminals, file browser with upload/download, stats charts, processes, and a Run dialog.
- **Images, Volumes, Networks**: search Docker Hub and its tags, pull and push with progress and stored registry logins, build from a folder on this machine, save to and load from a tar file, tag, run, history, prune; volume and network usage, create, connect/disconnect, guarded deletion.
- **Stacks**: every Compose project on a Host. Managed Stacks keep their YAML and encrypted environment in Homeport and deploy through the `docker compose` CLI; External Stacks can be started, stopped, removed or adopted.
- **Overview**: per-host cards, engine details, disk usage and clean-up, recent events.

Secrets (SSH keys, TLS material, stack environments, registry passwords and tokens) are encrypted in SQLite with a key kept in the OS keychain. See [GLOSSARY.md](GLOSSARY.md) and [docs/adr](docs/adr) for the vocabulary and the main decisions.

## Download

On Debian, Ubuntu, Fedora, RHEL and openSUSE, install it from the Homeport package repository, so `apt`, `dnf` or `zypper` keeps it up to date:

```sh
curl -fsSL https://damienreichhart.github.io/Homeport/install.sh | sh
```

The [installation guide](https://damienreichhart.github.io/Homeport/docs/installation/#manual-setup) shows the same setup step by step, without the script.

Installers for Windows, macOS and Linux (x64 and ARM) are on the [download page](https://damienreichhart.github.io/Homeport/download/) and the [releases page](https://github.com/DamienReichhart/Homeport/releases/latest). Windows, macOS and AppImage installs update themselves from inside Homeport. Homebrew, winget, Scoop, the AUR, Flathub and Snap are planned ([issues #5 to #10](https://github.com/DamienReichhart/Homeport/issues?q=is%3Aissue+label%3Aenhancement)).

Pushing a new version to `main` publishes it; see [docs/RELEASING.md](docs/RELEASING.md).

## Prerequisites

- [Bun](https://bun.sh) ≥ 1.4
- Rust (stable; `rust-toolchain.toml` pins components)
- [cargo-about](https://github.com/EmbarkStudios/cargo-about) for release builds: `cargo install cargo-about --locked --features cli`
- Tauri system dependencies: <https://tauri.app/start/prerequisites/>
  (Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev libayatana-appindicator3-dev libssl-dev`)
- At runtime: an OS keychain (macOS Keychain, Windows Credential Manager or a Secret Service such as GNOME Keyring) for SSH/TLS/stack secrets, and the `docker compose` CLI to deploy Managed Stacks. Both are optional; the app says what is unavailable without them.

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
  settings/ updates/ preferences, and how Homeport updates itself
packaging/linux/     repository definitions, signing key, install script and
                     .deb/.rpm maintainer scripts
scripts/             release planning, repository builder, signing-key wizard,
                     license notices, screenshots
website/             the website and user documentation
```

## License

[MIT](LICENSE) © Damien Reichhart. Third-party software is listed in the generated notices described above.
