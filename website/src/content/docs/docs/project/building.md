---
title: Building from source
description: Build and run Homeport from its source code.
---

Homeport is a [Tauri 2](https://tauri.app) app: a React and TypeScript interface over a Rust core, which talks to Docker Engines with [bollard](https://github.com/fussybeaver/bollard) and stores its data in SQLite.

## Prerequisites

- [Bun](https://bun.sh) 1.4 or later
- [Rust](https://www.rust-lang.org/tools/install), stable (the repository's `rust-toolchain.toml` picks the components)
- The [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your platform. On Debian and Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- For release builds, [cargo-about](https://github.com/EmbarkStudios/cargo-about), which generates the third-party licence notices:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Run it

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` starts the app with hot reload for the interface; Rust changes rebuild and restart it.

## Build installers

```sh
bun run tauri:build
```

Installers for your platform are written to `target/release/bundle/`. The build generates the licence notices first, and stops if a dependency's licence hasn't been reviewed in `about.toml`.

Local builds aren't signed for the in-app updater and leave the `.rpm` unsigned; only the [release workflow](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) has the signing keys. A copy you build yourself still checks for updates, but its **Install and restart** only installs versions signed by the release workflow.

## Useful commands

| Command               | What it does                                     |
| --------------------- | ------------------------------------------------ |
| `bun run check`       | Typecheck, lint, format check and frontend tests |
| `bun run rust:lint`   | `cargo clippy` with warnings as errors           |
| `bun run rust:test`   | Rust unit tests                                  |
| `bun run bindings`    | Regenerate the TypeScript bindings for the IPC   |
| `bun run licenses`    | Generate the third-party licence notices         |
| `bun run screenshots` | Recapture the screenshots used on this website   |

Integration tests talk to the local Docker Engine and the Compose CLI. They create only resources named `homeport-test-*` and remove them afterwards:

```sh
cargo test --workspace -- --ignored
```

## This website

The site lives in `website/` and is built with [Astro](https://astro.build) and [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

It's deployed to GitHub Pages by the **Pages** workflow on every push to `main` that touches it, and after every release so the download links stay current.
