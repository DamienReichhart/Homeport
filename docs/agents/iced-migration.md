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
│   └── features/<group>/<name>/   one folder per former `src/features/<name>`, under
│                      what it is about (app, docker, podman, containerd, incus,
│                      kubernetes, nomad): state.rs (State), message.rs (Message),
│                      update.rs, view/ (one function or component per file),
│                      mod.rs (the public API)
└── platform/   tray, notifications, autostart, single instance, window state,
                dialogs, updater, logging: the former `src-tauri/src/app/desktop`
```

Rules, enforced by `app/tests/layout.rs`:

1. **Dependencies point inward:** `ui/features` → `ui/widgets` + `ui/shell` + `data` + `api` → `app` → the crates. A feature never imports another feature (shared domain goes to `ui/shell` or `data`). `api` never imports `ui`.
2. **Elm architecture.** A feature has a `State`, a `Message` enum, `update(&mut State, Message, &Context) -> Task<Message>` and `view(&State, &Context) -> Element<Message>`, with `open` and `request` beside them (_The shell and a feature's page_). `view` is pure and cheap; all I/O is a `Task` or a `Subscription`.
3. **No blocking in `update` or `view`.** Anything that touches the network, the disk, the database, the keychain or a lock held across `await` goes through `Task::perform` on the runtime iced owns.
4. **Thin use cases, logic in crates.** An `api` function parses its arguments and calls one crate, as the former `src-tauri/src/commands/*` did (ADR 0005 still holds). Logic that needs several stores goes to the crate that owns them.
5. **One error type.** Failures are `AppError { code, message, hint, detail }` from `homeport-core`, shown with the shared error widgets (never a bare string), logged with the command's name.
6. **Strings are never literals.** Every user-visible string is a Fluent message read with `t!` or `rich!`, in English first and in all 11 languages before the ticket closes. State keeps what it was told (an `AppError`, a count), not text, so a language change redraws it. See `docs/i18n/README.md`.
7. **Same files and folders rules as the backend:** a file past 600 lines before its tests is split, a folder past 14 entries gets subfolders, tests next to the code, `name/mod.rs` for a module with submodules. Clippy runs with `-D warnings`.

## Porting recipe for a feature

1. Read the React feature (`src/features/<name>/`: `manifest.ts`, `api/`, `model/`, `ui/`) and its tests. List every state, action, dialog, empty/error/loading state, shortcut and palette command. Put that list in the PR description as the parity checklist.
2. Port the IPC commands it uses into `app/src/api/<domain>/` (they are in `src-tauri/src/commands/`). If the `api` ticket of that domain already did, use them.
3. Port the pure logic in `model/` to Rust unit tests first (the TypeScript tests are the oracle: same inputs, same outputs).
4. Build `State`/`Message`/`update`, then the `view`, with the shared widgets (`docs/agents/widgets.md` lists them and the rules for using them). If a widget is missing, stop and raise it on the widget's ticket rather than writing a private copy.
5. The feature is already registered in `ui/shell/registry` (routes, sidebar entry with its scope, gate, palette commands, shortcuts), as `manifest.ts` was: check its line against what you port, and replace its stub (_Replacing a stub_).
6. Tests: `update` tests for every message that does something; a headless `iced_test` snapshot for each page state (empty, loading, error, populated) in English, and one RTL snapshot (Arabic) for each page with a table or a form.
7. Compare against the running Tauri app on the same data. Anything different is a bug unless the PR says why.

## The shell and a feature's page

`app/src/ui/shell/` is what `src/app` and `src/core` were: the sidebar, the router, the selection, the gates, the pickers, the palette and the shortcuts. It knows no feature; it hosts whatever `ui::features::Page` gives it.

- **Routes** (`shell::router`): `Route` has a variant for every route pattern of the manifests (`Route::Container { id, tab }` is `containers/:id/:tab?`). A navigation is a `Request`: a route, an optional action for the page to start (`ctx.open(path, action)` in React) and an optional search text (`?q=`).
- **Registry** (`shell::registry`): `FEATURES` lists every feature in sidebar order, as data: its routes, sidebar entry (label, icon, scope), what the Host's Engine or the Cluster's kind must be, its gate, its palette commands and its page shortcuts (`n`, `/`). The sidebar, the palette, the keymap and the gates are built from it. `registry/tests.rs` pins the list and, while `src/features/*/manifest.ts` are in the tree, reads them and fails on any difference.
- **Gates** (`shell::gates`): `decide(route, world)` says whether the page shows (`Verdict::Open(Subject)`) or what stands before it. A page is opened only once its gate lets it through, and opened anew when its `Subject` changes (another Host, another namespace), so a page never has to handle "not connected".
- **Selection**: `ctx.world.selection`, remembered in the Settings (`last_selection`, with `last_host_id` for older versions). When it moves, the shell calls `Data::release` on the old scope and sends a page of the other kind to the Overview.

A feature is four functions and two types in `ui/features/<group>/<name>/mod.rs`:

```rust
pub struct State { … }
pub enum Message { … }
/// The page `ctx.location` leads to (a feature may have several routes), and what it starts by doing.
pub fn open(ctx: &Context<'_>) -> (State, Task<Message>)
pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message>
pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message>
/// Starts an action as if its button was pressed: a palette command's (`"run"`), `n`, or `/` (`registry::SEARCH`).
pub fn request(state: &mut State, action: &str, ctx: &Context<'_>) -> Task<Message>
```

`Context` (`shell::Context`) is what the page is given, borrowed for one call:

| Field or method                                                | What it is                                                                                                                                   |
| -------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `ctx.state`                                                    | the `Arc<AppState>` the use cases take                                                                                                       |
| `ctx.data`                                                     | the data layer, to `read` in `view`                                                                                                          |
| `ctx.world`, `ctx.host()`, `ctx.cluster()`                     | the Hosts, the Clusters, the selection, the remembered Settings                                                                              |
| `ctx.subject`                                                  | what the gate let through: the Host's id, with the containerd namespace or the Incus project; the Cluster's id and Namespace; the Swarm's id |
| `ctx.location`                                                 | the route, the action and the search text of the navigation, and its `key`                                                                   |
| `ctx.direction`                                                | the direction the language reads in                                                                                                          |
| `ctx.watch(query)`                                             | `Data::watch`, let go of by the shell when the page closes: never call `unwatch` yourself                                                    |
| `ctx.invalidate(key)`                                          | reads again what an action changed                                                                                                           |
| `ctx.open(request)`, `ctx.back()`, `ctx.select(…)`             | navigation and selection                                                                                                                     |
| `ctx.failed(surfaced)`, `ctx.details(surfaced)`                | a failure as a toast, or its Details dialog                                                                                                  |
| `ctx.copy(text)`                                               | the clipboard                                                                                                                                |
| `ctx.namespace_picker(Message::Picker)`, `ctx.picked(message)` | the Kubernetes or Nomad Namespace picker for the page's header, and its messages handed back                                                 |

What `update` asks through `ctx` is done by the shell right after `update` returns. `shell::layout::page_header` is `PageHeader`.

### Replacing a stub

Every feature's `mod.rs` is one line today: `pub use crate::ui::shell::stub::{open, request, update, view, Message, State};`. To port a feature:

1. Delete that line and define the two types and four functions above in the same folder (split into `state.rs`, `update.rs`, `view/` as it grows). Nothing else changes: `ui/features/mod.rs` already sends the feature's routes to this module, and the registry already has its sidebar entry, commands and shortcuts.
2. In `open`, match on `ctx.location.route` if the feature has several pages, `ctx.watch` what the page shows, and start from `ctx.location.query` if it is a list.
3. In `request`, start the actions the registry names for the feature (its commands' `Run::Open(_, Some(action))` and its shortcuts' `Run::Request(action)`).
4. If the manifest changed since, fix the feature's line in `shell/registry/defs/`; the test says what differs.
5. Test with `shell::page::testing::Bench`: `Bench::at(route)` gives a `Context` on an empty data layer (`bench.data.set(&key, value)`), `bench.watched()` and `bench.asked()` say what the page asked for. `ui/features/app/hosts` is the example.

A feature never names another feature: go there with `ctx.open(Route::…)`.

### What the shell leaves to other tickets

The shell has the hook for each of these and nothing behind it yet. None is done until its ticket lands:

- **Links out of the app** (Documentation, GitHub): the shell sends `shell::Event::OpenUrl`, and the window only logs it. The opener comes with the dialogs and opener layer (#253).
- **The update bar**: it shows when the window sends `shell::Message::UpdateFound`, which nothing sends, and has no install or restart button. "Check for updates" in the palette opens the Settings and sends `shell::Event::CheckForUpdates`, which is only logged. Both are the updater's (#255).
- **Global search in the palette** (`src/features/search`, the containers, images and volumes found as you type): not ported; the palette lists pages, Hosts and commands only.
- **The crash page** (`CrashPage.tsx`): iced has no boundary that catches a page's panic; nothing stands in for it.
- **Tray choices** (`TrayChoices.tsx`: a Host picked in the tray, Add host): the tray is the platform layer's (#252), which will send `shell::Message::Select` and `shell::Message::Go`. `Data::set_hidden` is its to call too.
- **The terminal's keys**: nothing fires in a terminal in React. The terminal widget's ticket must make its keys reach no shortcut: today a key a widget uses still lets the shortcuts with Mod fire (`shortcuts::keys::Target::Terminal` exists and nothing gives it).

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
