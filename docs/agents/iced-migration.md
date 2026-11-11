# Migrating the UI from Tauri and React to iced

**Read this before taking any ticket of the milestone _v2.0.0: Native UI on iced_.** Every ticket points here for the rules; the ticket says only what is specific to it.

## Why, and what it means

Homeport's UI is a React app in a Tauri webview, talking to the Rust crates over 348 IPC commands. It becomes a native iced application in one binary: no webview, no IPC, no generated TypeScript contract. The crates under `crates/` do not change in kind; what changes is everything above them: the UI (~138k lines of TypeScript), the desktop shell (`src-tauri`, 8.3k lines), the tests, the captures, the packaging and the updater.

**Parity is strict.** Nothing the Tauri app does may be lost; where iced allows it, do better. The React feature is the specification: its folder under `src/features/<name>/` and its tests say what the page shows and does.

## Branching and how tickets land

- Integration branch: `feature/front-iced`. It is the only branch these tickets target; `dev` and `main` keep shipping the Tauri app until the whole milestone is done.
- One ticket = one branch off `feature/front-iced` = one pull request into it, `Closes #n`. Keep PRs reviewable: a ticket marked **L** or **XL** says where to cut it into several PRs.
- Until the last cleanup ticket, **Tauri and React stay in the tree and keep building**. Add the iced side next to them; never break `bun run check` or `cargo test --workspace` on the branch.
- A ticket may be taken once every issue that blocks it (GitHub's native dependencies) is closed. Assign yourself first.

## The target architecture

The iced app is the crate `homeport-app` in `app/`; its binary is `homeport-iced` until the cleanup ticket renames it `homeport`.

```
app/src/
├── app/        what the UI stands on: AppState, setup, the events the crates push   (exists)
├── api/        the use cases: one async function per former IPC command, taking
│               &AppState and returning AppResult<T>. No iced here, so
│               `cargo test -p homeport-app` runs without a window.
│               One module per domain: hosts, engine, containerd, incus, clusters, stacks, app
├── data/       the live data layer: cached resources by key, staleness, polling,
│               invalidation by Host/Cluster scope, and the subscriptions for streams
├── i18n/       Fluent catalogues, the lookup, the language and its direction
├── ui/
│   ├── shell/      window, sidebar, navigation, selection, gates, palette, shortcuts
│   ├── theme/      tokens from docs/BRAND.md, fonts, icons, light/dark, styles
│   ├── widgets/    the kit: table, terminal, editor, diff, chart, log view, forms,
│   │               dialogs, toasts… no domain knowledge
│   └── features/<name>/   one folder per former `src/features/<name>`:
│                      state.rs (State), message.rs (Message), update.rs, view/ (one
│                      function or component per file), mod.rs (the public API)
└── platform/   tray, notifications, autostart, single instance, window state,
                dialogs, updater, logging: the former `src-tauri/src/app/desktop`
```

Rules, enforced by `app/tests/layout.rs`:

1. **Dependencies point inward:** `ui/features` → `ui/widgets` + `ui/shell` + `data` + `api` → `app` → the crates. A feature never imports another feature (shared domain goes to `ui/shell` or `data`). `api` never imports `ui`.
2. **Elm architecture.** A feature has a `State`, a `Message` enum, `update(&mut State, Message, &Context) -> Task<Message>` and `view(&State, &Context) -> Element<Message>`. `view` is pure and cheap; all I/O is a `Task` or a `Subscription`.
3. **No blocking in `update` or `view`.** Anything that touches the network, the disk, the database, the keychain or a lock held across `await` goes through `Task::perform` on the runtime iced owns.
4. **Thin use cases, logic in crates.** An `api` function parses its arguments and calls one crate, as the former `src-tauri/src/commands/*` did (ADR 0005 still holds). Logic that needs several stores goes to the crate that owns them.
5. **One error type.** Failures are `AppError { code, message, hint, detail }` from `homeport-core`, shown with the shared error widgets (never a bare string), logged with the command's name.
6. **Strings are never literals.** Every user-visible string is a Fluent message read with `t!` or `rich!`, in English first and in all 11 languages before the ticket closes. State keeps what it was told (an `AppError`, a count), not text, so a language change redraws it. See `docs/i18n/README.md`.
7. **Same files and folders rules as the backend:** a file past 600 lines before its tests is split, a folder past 14 entries gets subfolders, tests next to the code, `name/mod.rs` for a module with submodules. Clippy runs with `-D warnings`.

## Porting recipe for a feature

1. Read the React feature (`src/features/<name>/`: `manifest.ts`, `api/`, `model/`, `ui/`) and its tests. List every state, action, dialog, empty/error/loading state, shortcut and palette command. Put that list in the PR description as the parity checklist.
2. Port the IPC commands it uses into `app/src/api/<domain>/` (they are in `src-tauri/src/commands/`). If the `api` ticket of that domain already did, use them.
3. Port the pure logic in `model/` to Rust unit tests first (the TypeScript tests are the oracle: same inputs, same outputs).
4. Build `State`/`Message`/`update`, then the `view`, with the shared widgets. If a widget is missing, stop and raise it on the widget's ticket rather than writing a private copy.
5. Register the feature in `ui/shell` (route, sidebar entry with its scope, palette commands, shortcuts), as `manifest.ts` did.
6. Tests: `update` tests for every message that does something; a headless `iced_test` snapshot for each page state (empty, loading, error, populated) in English, and one RTL snapshot (Arabic) for each page with a table or a form.
7. Compare against the running Tauri app on the same data. Anything different is a bug unless the PR says why.

## Reading data and following streams

`app/src/data/` is to the iced app what TanStack Query and `subscribe()` were to React; a feature never loads, caches or refreshes by itself.

- **Keys** (`data::keys`): one module per family of the React query keys, with the same parts (`hostKeys.containers(hostId)` is `keys::host::containers(host_id)`). Every key of a Host is in `keys::host::scope(id)`, of a Cluster in `keys::cluster::scope(id)` (Nomad: `keys::nomad::scope(id)`), of a Swarm in `keys::swarm::scope(id)`. What only your feature reads is filed under one of them with `.with(part)`.
- **Queries** (`data::Query`): a key, the use case that reads it, and its policy (`stale_after`, `no_retry`, `poll`, `keep`: React Query's `staleTime`, `retry`, `refetchInterval`, `gcTime`, with the same defaults). The lists several features read are in `data::resources`.
- **`Data`**, in the window's state: `data.watch(state, &query)` when the page opens (a cached read is shown at once and read again only if stale; two pages share one read), `data.read::<T>(&key)` in the view (a `Resource`: `Idle`, `Loading`, `Loaded`, `Refreshing`, `Failed` with the previous data), `data.unwatch(&key)` when it closes and `data.release(&scope)` when the selection moves, which abort the reads nobody waits for. Map its `Task<data::Message>` into your message and give them back to `Data::update`.
- **Live updates**: `Data::on_event` already puts statuses in the cached lists, marks what a Host or Cluster that went down holds as `Failed` (data kept), reads it again when it comes back, and makes Engine lists stale from Engine events, 250 ms after the last of a burst. A feature adds nothing for that; polling (`.poll(interval)`) is for what no event tells, and pauses while the window is minimised or hidden.
- **Actions** (`data::Action`): `action.start(state, |state| use_case(...))`, then `data.settle(state, &mut action, finished, [keys…])`, which invalidates on success. The view reads `is_pending()` and `error()`.
- **Streams** (`data::streams`): `shapes::lines`, `view`, `job` and `terminal` turn a `*_subscribe` use case into a stream of `Update`s (`Started`, `Batch`, `Ended`, `Failed`) for `Task::run`, or for `streams::subscription`. Batches come at most 60 times a second; a view's stream is cancelled when dropped, a terminal's session closed, and a job goes on until `stream_cancel`. A job is always started with `Task::run`, never as a subscription: a subscription that comes back would start it again.

Their tests need no window: a paused tokio clock (`#[tokio::test(start_paused = true)]`) and loaders that count their calls (`app/src/data/tests.rs`).

## Testing a feature

`app/src/test_support/` (built for tests only) replaces vitest, Testing Library and the IPC mocks. A feature is tested in three layers:

1. **Use cases** (`api/`): against `crate::app::testing::app()`, an `AppState` on a new database in a temporary folder, with the receiving half of its events. No window, no keychain.
2. **`update`**: build the `State`, send it messages, assert on the state. `test_support::tasks::messages(task).await` runs what `update` returned and gives the messages it produced, to feed back in; `tasks::is_none` tells a `Task::none()`. `test_support::fixtures` has the sample Hosts, containers, networks, files, stats and services of the React tests.
3. **`view`**: a snapshot per page state.

   ```rust
   #[test]
   fn the_page_in_arabic_is_mirrored() {
       let hosts = hosts();
       assert_snapshot("hosts-ar", "ar", &Theme::Light, (360.0, 160.0), || hosts.view());
   }
   ```

   `assert_snapshot(name, language, theme, size, view)` sets the app's language (holding a lock, since it belongs to the process), draws the view in software at twice the size, and compares it with `app/tests/snapshots/<name>.png`. A few pixels of anti-aliasing may differ; a moved label may not.

   - **Add or change one:** run `UPDATE_SNAPSHOTS=1 cargo test -p homeport-app <test name>`, **open the PNG and look at it** (the Arabic one above showed a list that started on the wrong side), then commit it.
   - A missing snapshot fails; it is never written silently.
   - When one differs, what was drawn is kept as `<temp dir>/homeport-snapshots/<name>.actual.png`; CI uploads that folder.
   - Name them `<feature>-<state>-<language>`: `containers-empty-en`, `containers-ar`.
   - A test that reads or sets the language outside a snapshot holds `test_support::snapshot::in_language(tag)`.

`app/tests/smoke.rs` starts the real binary on an empty data folder and expects it to open its window and close cleanly: `xvfb-run -a cargo test -p homeport-app --test smoke -- --ignored` (the `iced smoke` CI job, on the three systems). On Linux the app needs `libxkbcommon-x11` at run time.

`app/tests/layout.rs` checks the rules of _The target architecture_; `app/src/i18n/check.rs` checks the catalogues (`docs/i18n/README.md`).

## Definition of done (every ticket)

- [ ] Acceptance criteria of the ticket all met, with the parity checklist ticked in the PR.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` pass.
- [ ] `bun run check` still passes (Tauri and React are untouched until cleanup).
- [ ] No string literal in the UI; the catalogue check passes for 11 languages.
- [ ] Works with the keyboard alone; focus order and shortcuts match the React page.
- [ ] Docs updated if behaviour or architecture changed (`docs/ARCHITECTURE.md` is rewritten by its own ticket; leave a note there in the PR if your change belongs in it).

## Verifying against a real Engine

Most tickets need a Docker Engine: `docker run -d --name hp-test nginx` on this machine's socket is enough for the container, image, volume and network pages. Kubernetes tickets use `kind`, Nomad tickets `nomad agent -dev`, Incus tickets a local Incus; the ticket says which. `crates/foundation/storage/examples/db_fixture.rs` builds a database with sample data.

## Glossary

Use the terms of `GLOSSARY.md` (Host, Cluster, Stack, Managed/Linked/Synced Stack, Swarm…) in code, messages and PRs.
