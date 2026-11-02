# Architecture

## Principles

1. **Vertical slices.** A feature owns its UI, API calls, and types. Deleting a feature = deleting a folder + 2 registration lines.
2. **Dependencies point inward.** `app` → `features` → `shared`. `shared` never imports from `features` or `app`; features import other features only through their `index.ts`. ESLint enforces this (`eslint.config.js`).
3. **One IPC gateway.** Frontend code calls Rust only via `commands` from `@/shared/api`: the generated bindings with every rejection normalized to `AppError { code, message }`. Tests mock IPC with `mockCommands` (`src/test/ipc.ts`).
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

## Deliberately not included (add when needed)

- Global state library (Zustand/Redux), data-fetching cache (TanStack Query), i18n, a UI kit, a database layer, auto-updater, code signing. None are needed for the base; each fits cleanly into `app/providers` or `shared/`.
