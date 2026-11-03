# The Rust backend is a workspace of layered crates

The backend started as one crate split into `commands/` and `services/`, the layout `docs/ARCHITECTURE.md` describes for the frontend's slices but horizontal on the Rust side: a feature touched `commands/`, `services/`, the command registry, the app state and `error/`, and nothing stopped a service from importing Tauri, another domain or the error module from importing a service. So the logic now lives in crates under `crates/`, layered from `homeport-core` (the error model, the Host model) through `homeport-storage`, `homeport-transport` and `homeport-engine` up to the domain crates `homeport-hosts` and `homeport-stacks` (with `homeport-settings` and `homeport-updates`). `src-tauri/` becomes an app shell that only wires them to Tauri. Each crate depends only on crates below it, only the shell depends on Tauri, and the domain crates don't depend on each other. Cargo checks all three, where a single crate relied on discipline alone.

## Considered Options

- **Tidy the layers in one crate** (sub-folders under `commands/` and `services/`): a mechanical change, but it keeps the horizontal layout and enforces nothing.
- **Vertical slices in one crate** (`features/<name>/{commands, service, store}` beside a shared kernel): matches the frontend, but keeps a service one `use tauri` away from Tauri, and every test still compiles the webview.

## Consequences

- `cargo test -p homeport-<crate>` builds without Tauri or WebKit, and crates compile in parallel.
- The orphan rule allows `From<…> for AppError` only in `homeport-core`. It implements it for bollard and russh behind its `bollard` and `russh` features; other foreign errors (SQLite, Tauri, the updater) are converted by named functions next to the code that meets them.
- `ErrorCode` stays one enum in `homeport-core`, since the frontend receives a single error type: every crate's new failure mode adds a variant there.
- Dependency versions and lints are declared once in the root `Cargo.toml`. Every crate takes the workspace version, so a release still bumps one number.
- Test helpers other crates need sit behind `test-support` features.
