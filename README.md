# Homeport

Cross-platform desktop app built with [Tauri 2](https://tauri.app), React 19, TypeScript and Rust.

## Prerequisites

- [Bun](https://bun.sh) ≥ 1.4
- Rust (stable; `rust-toolchain.toml` pins components)
- Tauri system dependencies: <https://tauri.app/start/prerequisites/>
  (Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev libayatana-appindicator3-dev libssl-dev`)

## Commands

| Command               | What it does                                     |
| --------------------- | ------------------------------------------------ |
| `bun install`         | Install JS dependencies                          |
| `bun run tauri:dev`   | Run the desktop app with hot reload              |
| `bun run tauri:build` | Produce an installer/bundle                      |
| `bun run check`       | Typecheck + lint + format check + frontend tests |
| `bun run rust:lint`   | `cargo clippy` with warnings as errors           |
| `bun run rust:test`   | Rust unit tests                                  |

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
