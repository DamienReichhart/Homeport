# Architecture

## Principles

1. **Vertical slices.** A feature owns its UI, API calls, and types. Deleting a feature = deleting a folder + 2 registration lines.
2. **Dependencies point inward.** `app` → `features` → `shared`. `shared` never imports from `features` or `app`; features import other features only through their `index.ts`. ESLint enforces this (`eslint.config.js`).
3. **One IPC gateway.** Frontend code calls Rust only via `commands` from `@/shared/api`: the generated bindings with every rejection normalized to `AppError { code, message, hint, detail }`, tagged with the command and written to the app log. Show a failure with `<ErrorMessage>` (in place, with Details and Retry) or `toastError` (transient); both open the shared Details dialog. Writing errors is covered in `src-tauri/src/error/mod.rs`. Tests mock IPC with `mockCommands` (`src/test/ipc.ts`).
4. **Thin commands, fat services.** Rust `commands/*` parse input and delegate; logic lives in `services/*` with no Tauri types so it is unit-tested with plain `cargo test`.
5. **Least privilege.** Every plugin/API the frontend may use is listed in `src-tauri/capabilities/`. Add permissions per feature, never wildcard.
6. **Generated contract.** Commands and their types are exported to `src/shared/api/bindings.ts` by `tauri-specta` (on every debug run, or `bun run bindings`). Never edit that file; a Rust test fails when it is stale.

## Adding a feature (example: `notes`)

Frontend:

1. `src/features/notes/{api,components,types.ts,index.ts}`; export only the public surface from `index.ts`.
2. Add the route in `src/app/routes/router.tsx` and a nav entry in `src/app/routes/navigation.ts`.

Backend:

1. `src-tauri/src/services/notes.rs` (+ `pub mod notes;` in `services/mod.rs`) with unit tests.
2. `src-tauri/src/commands/notes.rs` (+ `pub mod`), thin `#[tauri::command]` functions.
3. Annotate commands with `#[specta::specta]`, register them in `commands::builder()`, run `bun run bindings`; add the service to `AppState` in `state.rs`.
4. Add a variant to `AppError` if new failure modes are needed.

When a service grows large, promote it to a workspace crate under `crates/` (the root `Cargo.toml` is already a workspace).

## Building blocks

- **UI**: shadcn/ui components (Tailwind v4) live in `src/shared/ui`; add more with `bunx shadcn add <name>`.
- **Brand**: colours, type and the logo are tokens in `src/shared/styles/global.css` and `<Logo />`/`<LogoMark />` in `src/shared/ui`; see [BRAND.md](BRAND.md). Use the tokens (`bg-success`, `text-warning`, `--chart-n`), not Tailwind's palette.
- **Server state**: TanStack Query. Query keys are centralised in `src/shared/api/keys.ts` so `app/providers/LiveUpdates.tsx` can invalidate them from pushed Engine events instead of polling.
- **Streams**: logs, stats, exec, pulls and compose output use Tauri channels. Start one with `subscribe()` from `@/shared/api`; the returned function cancels it on the backend.
- **Storage**: SQLite (`homeport.db` in the app data dir) via `services/db.rs`, migrated on open. Secrets are sealed with `services/crypto.rs` (ADR 0002).
- **Updates**: `tauri-plugin-updater`, driven from Rust (`commands/updates.rs`), never from the frontend. `services/updates.rs` decides from the bundle type whether Homeport installs an update itself (NSIS, MSI, `.app`, AppImage) or leaves it to the package manager (`.deb`, `.rpm`). The manifest and the package repositories are built from the releases and served by the website (ADR 0003, [RELEASING.md](RELEASING.md#package-repositories-and-updates)); the `.deb`/`.rpm` maintainer scripts and the install script live in `packaging/linux/`.

## Deliberately not included (add when needed)

- Global state library (Zustand/Redux), i18n, Windows and macOS code signing (see [RELEASING.md](RELEASING.md#code-signing)).
