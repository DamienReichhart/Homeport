<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/lockup-dark.svg" />
    <img src="assets/brand/lockup-light.svg" alt="Homeport" height="64" />
  </picture>
</h1>

**Every Docker Engine, docked in one place.**

A desktop app for managing Docker Engines anywhere — this machine's, or remote ones over SSH, TLS or TCP — in the spirit of Docker Desktop. Built with [Tauri 2](https://tauri.app), React 19, TypeScript and Rust ([bollard](https://github.com/fussybeaver/bollard) for the Engine API, SQLite for its own data).

## Features

- **Hosts**: add local sockets (auto-detected: Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman), TCP, mutual TLS and SSH. SSH Hosts get their own Ed25519 key with a copy-paste command to authorize it; server keys are trusted on first use. Status, reconnection with backoff and a live event stream per Host.
- **Containers**: grouped by Stack, live CPU/memory, lifecycle and bulk actions, logs (follow, search, download), inspect, interactive terminals, file browser with upload/download, stats charts, processes, and a Run dialog.
- **Images, Volumes, Networks**: pull with progress, tag, run, history, prune; volume and network usage, create, connect/disconnect, guarded deletion.
- **Stacks**: every Compose project on a Host. Managed Stacks keep their YAML and encrypted environment in Homeport and deploy through the `docker compose` CLI; External Stacks can be started, stopped, removed or adopted.
- **Overview**: per-host cards, engine details, disk usage and clean-up, recent events.

Secrets (SSH keys, TLS material, stack environments) are encrypted in SQLite with a key kept in the OS keychain. See [GLOSSARY.md](GLOSSARY.md) and [docs/adr](docs/adr) for the vocabulary and the main decisions.

## Prerequisites

- [Bun](https://bun.sh) ≥ 1.4
- Rust (stable; `rust-toolchain.toml` pins components)
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

`tauri:build` writes installers to `target/release/bundle/`. Each one adds Homeport to the app menu: `.deb`/`.rpm` install a desktop entry, `.msi`/NSIS add a Start Menu shortcut, and the `.app` shows in Launchpad once it is in Applications. An AppImage has no install step, so it adds its own entry to `~/.local/share/applications` each time it runs, and the entry disappears from the menu once the file is deleted.

Integration tests talk to the local Docker Engine (and the compose CLI). They only create resources named `homeport-test-*` and remove them afterwards; run them with `cargo test --workspace -- --ignored`.

## Layout

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the rules that keep this scalable.

```
src/                 React frontend
  app/               shell: providers, router, layout (composes features)
  features/<name>/   self-contained vertical slices, public API via index.ts
  shared/            feature-agnostic api / ui / hooks / lib / styles
src-tauri/           Rust backend
  src/commands/      thin IPC handlers
  src/services/      business logic (no Tauri types, unit-testable)
  src/state.rs       managed app state
  src/error.rs       AppError -> { code, message } for the frontend
  capabilities/      Tauri permission sets
```
