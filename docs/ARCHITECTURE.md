# Architecture

## Principles

1. **Vertical slices.** A feature owns its UI, API calls, and types. Deleting a feature = deleting a folder + 1 registration line.
2. **Dependencies point inward.** `app` → `features` → `core` → `shared` (ADR 0004). A layer never imports the layers to its left; features, core modules and the UI kit are imported through their `index.ts` only, and import cycles are errors. ESLint enforces this (`eslint.config.js`).
3. **One IPC gateway.** Frontend code calls Rust only via `commands` from `@/shared/api`, and only from a feature's `api/` folder or from `core`, never from its components: the generated bindings with every rejection normalized to `AppError { code, message, hint, detail }`, tagged with the command and written to the app log. Show a failure with `<ErrorMessage>` (in place, with Details and Retry) or `toastError` (transient); both open the shared Details dialog. Writing errors is covered in `src-tauri/src/error/mod.rs`. Tests mock IPC with `mockCommands` (`src/test/ipc.ts`).
4. **Thin commands, fat services.** Rust `commands/*` parse input and delegate; logic lives in `services/*` with no Tauri types so it is unit-tested with plain `cargo test`.
5. **Least privilege.** Every plugin/API the frontend may use is listed in `src-tauri/capabilities/`. Add permissions per feature, never wildcard.
6. **Generated contract.** Commands and their types are exported to `src/shared/api/bindings.ts` by `tauri-specta` (on every debug run, or `bun run bindings`). Never edit that file; a Rust test fails when it is stale.

## Frontend layout

```
src/
├── app/            the shell: nothing but wiring
│   ├── layout/     AppLayout, Sidebar, banners
│   ├── providers/  query client, global providers, live updates
│   └── router/     features.ts (the manifests, in sidebar order), router.tsx, CrashPage
├── features/       one folder per feature, all with the same layout:
│   └── <name>/
│       ├── api/          queries.ts (queryOptions and keys), mutations.ts, streams.ts
│       ├── model/        pure logic and types, unit-tested
│       ├── ui/           components, one exported component per file;
│       │                 dialogs/, and subfolders (list/, detail/…) when a page has parts
│       ├── manifest.ts   routes and sidebar entry
│       └── index.ts      the public API: the manifest and what other features use
├── core/           the domain every feature stands on
│   ├── feature/    the manifest contract
│   ├── host/       Host selection, statuses, HostGate, the Engine resources' keys and lists,
│   │               and the live updates that refresh them
│   └── settings/   the user's settings
├── shared/         no domain knowledge
│   ├── api/        IPC gateway: bindings, commands, errors, streams, app-level queries
│   ├── ui/         the UI kit: primitives/ (shadcn/ui), layout/, feedback/, data/, chart/,
│   │               terminal/, editor/, all exported from index.ts
│   └── lib/, hooks/
└── test/           render helpers, IPC mocks and fixtures
```

Tests sit next to the code they test. A file past 300 lines gets an ESLint warning: it usually holds more than one thing. A folder past about ten files gets subfolders by role.

**Query keys.** Every key of a Host starts with `hostKeys.scope(hostId)` (`@/core/host`), so one invalidation refreshes everything a Host has. The lists of Engine resources (containers, images, volumes, networks, Stacks, system usage, events) are read through `resourceQueries`, because several features read them and Engine events invalidate them (`useEngineEventUpdates`). Every other key lives in the feature's `api/queries.ts`.

## Adding a feature (example: `notes`)

Frontend:

1. `src/features/notes/{api,model,ui}` and an `index.ts` that exports only the public surface.
2. `manifest.ts` with `defineFeature({ routes, nav })`, exported from `index.ts`, and one line in `src/app/router/features.ts`.

Backend:

1. `src-tauri/src/services/notes.rs` (+ `pub mod notes;` in `services/mod.rs`) with unit tests.
2. `src-tauri/src/commands/notes.rs` (+ `pub mod`), thin `#[tauri::command]` functions.
3. Annotate commands with `#[specta::specta]`, register them in `commands::builder()`, run `bun run bindings`; add the service to `AppState` in `state.rs`.
4. Add a variant to `AppError` if new failure modes are needed.

When a service grows large, promote it to a workspace crate under `crates/` (the root `Cargo.toml` is already a workspace).

## Building blocks

- **UI**: shadcn/ui components (Tailwind v4) live in `src/shared/ui/primitives`; add more with `bunx shadcn add <name>` and export them from `src/shared/ui/index.ts`. Import every component from `@/shared/ui`.
- **Brand**: colours, type and the logo are tokens in `src/shared/styles/global.css` and `<Logo />`/`<LogoMark />` in `src/shared/ui`; see [BRAND.md](BRAND.md). Use the tokens (`bg-success`, `text-warning`, `--chart-n`), not Tailwind's palette.
- **Server state**: TanStack Query, with `queryOptions` factories in each `api/queries.ts`. Engine resource lists are refreshed from pushed Engine events instead of polling (see Query keys above).
- **Streams**: logs, stats, exec, pulls and compose output use Tauri channels. Start one with `subscribe()` from `@/shared/api` in the feature's `api/streams.ts`; the returned function cancels it on the backend.
- **Storage**: SQLite (`homeport.db` in the app data dir) via `services/db.rs`, migrated on open. Secrets are sealed with `services/crypto.rs` (ADR 0002).
- **Updates**: `tauri-plugin-updater`, driven from Rust (`commands/updates.rs`), never from the frontend. `services/updates.rs` decides from the bundle type whether Homeport installs an update itself (NSIS, MSI, `.app`, AppImage) or leaves it to the package manager (`.deb`, `.rpm`). The manifest and the package repositories are built from the releases and served by the website (ADR 0003, [RELEASING.md](RELEASING.md#package-repositories-and-updates)); the `.deb`/`.rpm` maintainer scripts and the install script live in `packaging/linux/`.

## Deliberately not included (add when needed)

- Global state library (Zustand/Redux), i18n, Windows and macOS code signing (see [RELEASING.md](RELEASING.md#code-signing)).
