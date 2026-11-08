# Architecture

## Principles

1. **Vertical slices.** A feature owns its UI, API calls, and types. Deleting a feature = deleting a folder + 1 registration line.
2. **Dependencies point inward.** `app` → `features` → `core` → `shared` (ADR 0004). A layer never imports the layers to its left; features, core modules and the UI kit are imported through their `index.ts` only, and import cycles are errors. ESLint enforces this (`eslint.config.js`).
3. **One IPC gateway.** Frontend code calls Rust only via `commands` from `@/shared/api`, and only from a feature's `api/` folder or from `core`, never from its components: the generated bindings with every rejection normalized to `AppError { code, message, hint, detail }`, tagged with the command and written to the app log. Show a failure with `<ErrorMessage>` (in place, with Details and Retry) or `toastError` (transient); both open the shared Details dialog. Writing errors is covered in `crates/core/src/error/mod.rs`. Tests mock IPC with `mockCommands` (`src/test/ipc.ts`).
4. **Thin commands, logic in crates.** The Rust backend is a Cargo workspace (ADR 0005). `src-tauri/src/commands/*` parse input and delegate to the crates under `crates/`, which never see Tauri, so `cargo test -p homeport-<crate>` runs without the webview. See [Backend layout](#backend-layout).
5. **Least privilege.** Every plugin/API the frontend may use is listed in `src-tauri/capabilities/`. Add permissions per feature, never wildcard. Plugins driven from Rust only (updater, single-instance, window-state) get no permission at all.
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
│       ├── manifest.ts   routes, sidebar entry and command palette commands
│       └── index.ts      the public API: the manifest and what other features use
├── core/           the domain every feature stands on
│   ├── feature/    the manifest contract (each sidebar entry's scope: host, cluster or app),
│   │               palette commands, useActionRequest
│   ├── selection/  what the pages show, a Host or a Cluster, remembered across launches
│   ├── host/       the selected Host (useHostSelection), statuses, HostGate, the Engine
│   │               resources' keys and lists, and the live updates that refresh them
│   ├── cluster/    Clusters: their keys (clusterKeys.scope), list, statuses and ClusterGate
│   ├── swarm/      the Swarms found through their manager Hosts (ADR 0016): their keys,
│   │               the list, SwarmGate, and the live updates of their manager Hosts
│   └── settings/   the user's settings
├── shared/         no domain knowledge
│   ├── api/        IPC gateway: bindings, commands, errors, streams, app-level queries
│   ├── ui/         the UI kit: primitives/ (shadcn/ui), layout/, feedback/, data/, chart/,
│   │               terminal/, editor/, all exported from index.ts
│   └── lib/, hooks/
└── test/           render helpers, IPC mocks and fixtures
```

Tests sit next to the code they test. A file past 300 lines gets an ESLint warning: it usually holds more than one thing. A folder past about ten files gets subfolders by role.

**Selection.** The pages show one Host or one Cluster (`@/core/selection`); a sidebar entry's `scope` says which its pages are about, and the sidebar, the shortcuts and the palette only offer the entries of the selected kind, plus the app's. A Host page opened while a Cluster is selected, or the reverse, goes to the Overview.

**Query keys.** Every key of a Host starts with `hostKeys.scope(hostId)` (`@/core/host`), and every key of a Cluster with `clusterKeys.scope(clusterId)` (`@/core/cluster`), so one invalidation refreshes everything a Host or a Cluster has. The lists of Engine resources (containers, images, volumes, networks, Stacks, system usage, events) are read through `resourceQueries`, because several features read them and Engine events invalidate them (`useEngineEventUpdates`). Every other key lives in the feature's `api/queries.ts`.

## Backend layout

Each crate depends only on crates below it in this list, so the graph has one direction and Cargo refuses a cycle:

| Crate (folder)                               | Holds                                                                                                                                                                                                         |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport` (`src-tauri/`)                    | The Tauri shell: `app/` (bootstrap, state, events), `commands/` (one module per feature), `error.rs`                                                                                                          |
| `homeport-hosts` (`crates/hosts/`)           | Hosts: their store, a supervised connection to each (`registry/`), their kept Engine events (`events`), the use cases (`service/`), imports, export                                                           |
| `homeport-clusters` (`crates/clusters/`)     | Clusters: their store with the kubeconfig credentials sealed, a supervised connection to each through kube-rs (`registry/`), classifying the Kubernetes API's failures, the use cases (`service`)             |
| `homeport-stacks` (`crates/stacks/`)         | Stacks: telling them apart, their store, the runtime (each source and target), lifecycle, the compose CLI, Synced Stacks and their poller                                                                     |
| `homeport-git` (`crates/git/`)               | Synced Stacks' repositories: their sealed credentials, and the `git` CLI run with them per command (ADR 0014)                                                                                                 |
| `homeport-registries` (`crates/registries/`) | Registry logins: their store, matching them to image references, the login check and Docker Hub (HTTP)                                                                                                        |
| `homeport-settings`, `homeport-updates`      | The user's preferences; how this copy of Homeport is updated                                                                                                                                                  |
| `homeport-engine` (`crates/engine/`)         | The Engine's resources as functions over a bollard client; what runs in a container under `containers/`; the helper containers that open volumes (`volumes/`, ADR 0015); Swarm mode under `swarm/` (ADR 0016) |
| `homeport-transport` (`crates/transport/`)   | Reaching a Host: local, TCP, TLS, SSH and bastions (ADR 0002, 0012), the compose proxy, classifying failures                                                                                                  |
| `homeport-storage` (`crates/storage/`)       | The SQLite database, its migrations, the secrets sealed in it, export files (ADR 0002, 0013)                                                                                                                  |
| `homeport-core` (`crates/core/`)             | `AppError` and `ErrorCode`, the Host model, helpers for this computer                                                                                                                                         |

Rules:

- **Only the shell knows Tauri.** A crate that must tell the app something defines a trait the shell implements (`homeport_hosts::registry::Notifier` and `homeport_clusters::registry::ClusterNotifier`, forwarded to the webview by `app/events.rs`; `homeport_stacks::synced::poller::PollEvents`, implemented in `app/polling.rs`, which also deploys for it).
- **Domain crates don't depend on each other.** `hosts`, `clusters` and `stacks` all sit on the lower crates; a command that needs both combines them (`stacks_compose_subscribe` reads the Host into a `StackTarget`, the Stack's definition into a `StackSource`, and hands both to `homeport_stacks::runtime`). A backup of Homeport's data (`commands/backup.rs`) likewise gathers each domain's section and restores them in one `Db::transaction`.
- **Foreign errors.** The orphan rule only lets `homeport-core` implement `From<…> for AppError`. It does so for bollard and russh behind its `bollard` and `russh` features; every other foreign error gets a named function next to the only code that meets it (`homeport_storage::db::sqlite_error`, `error::window` and `error::update` in the shell).
- **Shared test helpers** sit behind a crate's `test-support` feature (`homeport-transport`'s dialer over the local socket, `homeport-engine`'s throwaway resources) and are only enabled as dev-dependencies.
- **One version per dependency and one set of lints**, declared in the root `Cargo.toml` (`[workspace.dependencies]`, `[workspace.lints]`).

## Website layout

The website (`website/`, Astro and Starlight) has the frontend's layers (ADR 0006), enforced by `website/eslint.config.js`:

```
website/src/
├── app/            the shell: layouts/SiteLayout, chrome/ (header, footer, theme toggle),
│                   starlight/ (component overrides), config/sidebar.ts, markdown/ plugins,
│                   styles/ (theme.css for both, docs.css, site/ for the marketing pages)
├── features/       one folder per page, plus docs/ for the components MDX uses:
│   └── <name>/
│       ├── ui/           components; sections/ for the page's sections
│       ├── model/        logic, and what client scripts import
│       ├── data/         lists that more than one component reads
│       └── index.ts      the public API
├── core/           what several features share about Homeport: site (the project's facts),
│                   release (GitHub releases), packages (the Linux package repositories)
├── shared/         no domain knowledge: ui/ (Icon, Screenshot, TextLink, ScrollReveal), lib/
├── pages/          routes: a layout and a feature's sections, nothing else
└── content/docs/   the user docs (Markdown and MDX)
```

A component's styles live in the component. One that overrides a global rule must outrank it (`.cta.card`), because Vite doesn't keep the global stylesheet ahead of a page's CSS. Logic goes in `model/` (or `core/`, `shared/lib/`) so it can be tested: `bun test`, with the tests next to the code.

## Adding a feature (example: `notes`)

Frontend:

1. `src/features/notes/{api,model,ui}` and an `index.ts` that exports only the public surface.
2. `manifest.ts` with `defineFeature({ routes, nav, commands })`, exported from `index.ts`, and one line in `src/app/router/features.ts`. A command in `commands` is listed in the command palette (`src/app/palette/`); one that starts something on a page opens it with `ctx.open(path, action)`, and the page starts it with `useActionRequest(action, start)`.

Backend:

1. Put the logic in the crate that owns its domain, with unit tests. A new domain (its own store and use cases), or code that needs a heavy dependency the other crates shouldn't carry, gets a new crate instead: `crates/notes/` named `homeport-notes`, with `publish = false` and `[lints] workspace = true`, listed in the root `[workspace.dependencies]`.
2. `src-tauri/src/commands/notes.rs` (+ `pub mod notes;` in `commands/mod.rs`), thin `#[tauri::command]` functions.
3. Annotate commands with `#[specta::specta]`, register them in `commands::builder()`, run `bun run bindings`; add the crate's service to `AppState` (`src-tauri/src/app/state.rs`) and build it in `app/setup.rs`.
4. Add a variant to `ErrorCode` (`crates/core/src/error/code.rs`) if new failure modes are needed, and classify them in the crate that meets them. Declare each error's text in the crate's `messages.rs` (`messages!`: key and English template) and run `bun run bindings`, which regenerates `src/shared/i18n/en/errors.json` (ADR 0011; the guide at the top of `crates/core/src/error/mod.rs`).

Website page (example: `compare`):

1. `website/src/features/compare/ui/sections/*.astro`, one per section, and an `index.ts` that exports them.
2. `website/src/pages/compare.astro`: `SiteLayout` and the sections, in order.

## Building blocks

- **UI**: shadcn/ui components (Tailwind v4) live in `src/shared/ui/primitives`; add more with `bunx shadcn add <name>` and export them from `src/shared/ui/index.ts`. Import every component from `@/shared/ui`.
- **Brand**: colours, type and the logo are tokens in `src/shared/styles/global.css` and `<Logo />`/`<LogoMark />` in `src/shared/ui`; see [BRAND.md](BRAND.md). Use the tokens (`bg-success`, `text-warning`, `--chart-n`), not Tailwind's palette.
- **Keyboard**: `useShortcut({ keys, label, group, handler })` from `@/shared/hooks` registers a shortcut while a component is mounted; the help sheet (`?`) and the palette list them. `Mod` is ⌘ on macOS and Ctrl elsewhere; plain keys don't fire in text fields, the editor or dialogs, and nothing fires in the terminal. A page's main action takes `n` with `usePageAction`.
- **Desktop**: the tray (`src-tauri/src/app/tray/`), notifications (`app/notifications.rs`, from `homeport_hosts::alerts`), single-instance and window-state (`app/window.rs`) and start at login (`app/autostart.rs`) are driven from Rust only; their choices reach the frontend as typed events (`TrayHostChosen`).
- **Text**: every user-facing string comes from the catalogue in `src/shared/i18n/en/` through `t('feature.key', { param })` from `@/shared/i18n` (FormatJS, ICU MessageFormat). Errors carry a `key` and `params` that the frontend renders from the same catalogue; Rust declares each error's key and English template in its crate's `messages!` block and `bun run bindings` generates `errors.json` (ADR 0011).
- **Server state**: TanStack Query, with `queryOptions` factories in each `api/queries.ts`. Engine resource lists are refreshed from pushed Engine events instead of polling (see Query keys above).
- **Volumes' contents**: browsing, backing up, restoring and cloning a volume go through a short-lived helper container (`homeport_engine::volumes::helper`, ADR 0015), hidden from the lists and events; its image is pulled only once the user agrees.
- **Git**: Synced Stacks run the user's `git` with a repository's credentials for one command at a time (a private SSH agent or an environment-only header), never the user's own git configuration or keys; clones and worktrees live in the app data dir (ADR 0014).
- **Streams**: logs, stats, exec, pulls, pushes, builds, image saves and loads, volume backups and restores, and compose output use Tauri channels. Start one with `subscribe()` from `@/shared/api` in the feature's `api/streams.ts`; the returned function cancels it on the backend.
- **Storage**: SQLite (`homeport.db` in the app data dir) via `homeport_storage::Db`, migrated on open from `crates/storage/src/db/migrations/`. Secrets are sealed with `homeport_storage::SecretBox` (ADR 0002); export files and backups with a passphrase, by `homeport_storage::crypto::bundle` (ADR 0013). Engine events are kept per Host by `homeport_hosts::events::EventStore`, written in batches by a thread of its own.
- **Updates**: `tauri-plugin-updater`, driven from Rust (`src-tauri/src/commands/updates.rs`), never from the frontend. `homeport-updates` decides from the bundle type whether Homeport installs an update itself (NSIS, MSI, `.app`, AppImage) or leaves it to the package manager (`.deb`, `.rpm`). The manifest and the package repositories are built from the releases and served by the website (ADR 0003, [RELEASING.md](RELEASING.md#package-repositories-and-updates)); the `.deb`/`.rpm` maintainer scripts and the install script live in `packaging/linux/`.

## Deliberately not included (add when needed)

- Global state library (Zustand/Redux), Windows and macOS code signing (see [RELEASING.md](RELEASING.md#code-signing)).
