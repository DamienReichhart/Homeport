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
│   ├── domain/     what several features show of one domain: a container's tone,
│   │               ports, menu and dialogs, the "newer image" mark, a pod's name
│   └── features/<group>/<name>/   one folder per former `src/features/<name>`, under
│                      what it is about (app, docker, podman, containerd, incus,
│                      kubernetes, nomad): state.rs (State), message.rs (Message),
│                      update.rs, view/ (one function or component per file),
│                      mod.rs (the public API)
└── platform/   tray, notifications, autostart, single instance, window state,
                dialogs, updater, logging: the former `src-tauri/src/app/desktop`
```

Rules, enforced by `app/tests/layout.rs`:

1. **Dependencies point inward:** `ui/features` → `ui/domain` → `ui/widgets` + `ui/shell` + `data` + `api` → `app` → the crates. A feature never imports another feature: what two of them share (what `@/features/containers` exported to the Board, what `core/host` drew) goes to `ui/domain`, and to `ui/shell` or `data` when it is the shell's or the data layer's own. `api` never imports `ui`, and the shell never imports `ui/domain`. `platform` stands beside the shell, under the window: it uses `api` and `app`, names nothing of `ui` or `data`, and of the kit only the gallery (which stands for a page) names it.
2. **Elm architecture.** A feature has a `State`, a `Message` enum, `update(&mut State, Message, &Context) -> Task<Message>` and `view(&State, &Context) -> Element<Message>`, with `open`, `request`, `dialog`, `refresh` and `subscription` beside them (_The shell and a feature's page_). `view` is pure and cheap; all I/O is a `Task` or a `Subscription`.
3. **No blocking in `update` or `view`.** Anything that touches the network, the disk, the database, the keychain or a lock held across `await` goes through `Task::perform` on the runtime iced owns.
4. **Thin use cases, logic in crates.** An `api` function parses its arguments and calls one crate, as the former `src-tauri/src/commands/*` did (ADR 0005 still holds). Logic that needs several stores goes to the crate that owns them.
5. **One error type.** Failures are `AppError { code, message, hint, detail }` from `homeport-core`, shown with the shared error widgets (never a bare string), logged with the command's name.
6. **Strings are never literals.** Every user-visible string is a Fluent message read with `t!` or `rich!`, in English first and in all 11 languages before the ticket closes. State keeps what it was told (an `AppError`, a count), not text, so a language change redraws it. See `docs/i18n/README.md`. The one exception is a symbol that is a word of no language: the log viewer's `Aa` (match case) and `.*` (regular expression) toggles, each named under the pointer by the catalogue; a test of the viewer holds its toolbar to those two, and the terminal's search bar, which shows the same two from there, to none of its own.
7. **Same files and folders rules as the backend:** a file past 600 lines before its tests is split, a folder past 14 entries gets subfolders, tests next to the code, `name/mod.rs` for a module with submodules. Clippy runs with `-D warnings`.

## Porting recipe for a feature

1. Read the React feature (`src/features/<name>/`: `manifest.ts`, `api/`, `model/`, `ui/`) and its tests. List every state, action, dialog, empty/error/loading state, shortcut and palette command. Put that list in the PR description as the parity checklist.
2. Port the IPC commands it uses into `app/src/api/<domain>/` (they are in `src-tauri/src/commands/`). If the `api` ticket of that domain already did, use them.
3. Port the pure logic in `model/` to Rust unit tests first (the TypeScript tests are the oracle: same inputs, same outputs).
4. Build `State`/`Message`/`update`, then the `view`, with the shared widgets (`docs/agents/widgets.md` lists them and the rules for using them). If a widget is missing, stop and raise it on the widget's ticket rather than writing a private copy.
5. The feature is already registered in `ui/shell/registry` (routes, sidebar entry with its scope, gate, palette commands, shortcuts), as `manifest.ts` was: check its line against what you port, and replace its stub (_Replacing a stub_).
6. Tests: `update` tests for every message that does something; a headless `iced_test` snapshot for each page state (empty, loading, error, populated) in English, and one RTL snapshot (Arabic) for each page with a table or a form.
7. Compare against the running Tauri app on the same data. Anything different is a bug unless the PR says why. Write each difference that is meant in the feature's module doc, under "What differs from the React page, on purpose" (`ui/features/docker/containers/list/mod.rs`), so the next reader finds it beside the code.

## The shell and a feature's page

`app/src/ui/shell/` is what `src/app` and `src/core` were: the sidebar, the router, the selection, the gates, the pickers, the palette and the shortcuts. It knows no feature; it hosts whatever `ui::features::Page` gives it.

- **Routes** (`shell::router`): `Route` has a variant for every route pattern of the manifests (`Route::Container { id, tab }` is `containers/:id/:tab?`). A navigation is a `Request`: a route, an optional action for the page to start (`ctx.open(path, action)` in React), an optional search text (`?q=`) and the other search parameters of the address (`Request::with_param("host", id)` is `/board?host=…`). A page that kept its filters in the address keeps them with `ctx.keep(query, params)`: its location carries them, the page stays as it is, and `open` reads them from `ctx.location` when the window comes back.
- **Registry** (`shell::registry`): `FEATURES` lists every feature in sidebar order, as data: its routes, sidebar entry (label, icon, scope), what the Host's Engine or the Cluster's kind must be, its gate, its palette commands and its page shortcuts (`n`, `/`). The sidebar, the palette, the keymap and the gates are built from it. `registry/tests.rs` pins the list and, while `src/features/*/manifest.ts` are in the tree, reads them and fails on any difference.
- **Gates** (`shell::gates`): `decide(route, world)` says whether the page shows (`Verdict::Open(Subject)`) or what stands before it. A page is opened only once its gate lets it through, and opened anew when its `Subject` changes (another Host, another namespace), so a page never has to handle "not connected".
- **Selection**: `ctx.world.selection`, remembered in the Settings (`last_selection`, with `last_host_id` for older versions). When it moves, the shell calls `Data::release` on the old scope and sends a page of the other kind to the Overview.

A feature is seven functions and two types in `ui/features/<group>/<name>/mod.rs`:

```rust
pub struct State { … }
pub enum Message { … }
/// The page `ctx.location` leads to (a feature may have several routes), and what it starts by doing.
pub fn open(ctx: &Context<'_>) -> (State, Task<Message>)
pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message>
pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message>
/// Starts an action as if its button was pressed: a palette command's (`"run"`), `n`, or `/` (`registry::SEARCH`).
pub fn request(state: &mut State, action: &str, ctx: &Context<'_>) -> Task<Message>
/// The dialog the page has open: the shell lays it over the whole window.
pub fn dialog<'a>(state: &'a State, ctx: &Context<'a>) -> Option<Modal<'a, Message>>
/// What the page reads, or the Hosts, may have changed: watch what is there now, take what the view needs.
pub fn refresh(state: &mut State, ctx: &Context<'_>) -> Task<Message>
/// The streams the page follows while it shows.
pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message>
```

- **`dialog`**: a page is drawn in a scroll area beside the sidebar, where a dialog can't be laid (`overlay::over` fills what it is given). The page keeps the dialog's state and returns it here; the shell lays it over the sidebar and the page, under the palette. A page never calls `overlay::over` itself.
- **`refresh`** is called after every message of the window while the page stays (a read ended, a status came, the page's own message): the page has no message of its own for "the data changed". Most pages need nothing there (`Task::none()`): `view` reads the data layer. A page over many Hosts (the Board) watches the Hosts that connected since and rebuilds what its view would otherwise work out on every draw. It must do nothing when what it reads is the same value as last time (compare the `Arc`s).
- **`subscription`** is where a view's stream goes (`streams::subscription`): it runs while the function returns it and is stopped when it no longer does, or when the page closes. The page's number is part of its identity, so a page opened anew starts its own.

`Context` (`shell::Context`) is what the page is given, borrowed for one call:

| Field or method                                                | What it is                                                                                                                                   |
| -------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `ctx.state`                                                    | the `Arc<AppState>` the use cases take                                                                                                       |
| `ctx.data`                                                     | the data layer, to `read` in `view`                                                                                                          |
| `ctx.world`, `ctx.host()`, `ctx.cluster()`                     | the Hosts, the Clusters, the selection, the remembered Settings                                                                              |
| `ctx.subject`                                                  | what the gate let through: the Host's id, with the containerd namespace or the Incus project; the Cluster's id and Namespace; the Swarm's id |
| `ctx.location`                                                 | the route, the action and the search text of the navigation, and its `key`                                                                   |
| `ctx.direction`                                                | the direction the language reads in                                                                                                          |
| `ctx.watch(query)`, `ctx.unwatch(key)`                         | `Data::watch`, let go of by the shell when the page closes; `ctx.unwatch` only for a part of the page it no longer shows (a tab, a folder)   |
| `ctx.invalidate(key)`                                          | reads again what an action changed                                                                                                           |
| `ctx.open(request)`, `ctx.back()`, `ctx.select(…)`             | navigation and selection                                                                                                                     |
| `ctx.failed(surfaced)`, `ctx.details(surfaced)`                | a failure as a toast, or its Details dialog                                                                                                  |
| `ctx.act(work, report, then)`                                  | runs a mutation whose toast and invalidation outlive the page (_Reading data and following streams_)                                         |
| `ctx.failed_of(title, surfaced)`                               | the same under a title that names what failed: one container among those an action was asked for                                             |
| `ctx.done(\|\| t!(..))`                                        | what an action did, as a toast (`toast.success`); the closure is asked at each draw, so it reads in the language of the moment               |
| `ctx.put(key, value)`                                          | files what an action answered as if it had been read (`setQueryData`)                                                                        |
| `ctx.keep(query, params)`                                      | keeps the page's filters in its location (`setSearchParams(.., { replace: true })`)                                                          |
| `ctx.keep_route(route)`                                        | keeps the page's tab in its route (`navigate(path, { replace: true })`): the page stays, nothing is added to the history                     |
| `ctx.copy(text)`, `ctx.open_url(url)`                          | the clipboard; a web, mail or phone link for the OS to open, a failure being said in a toast                                                 |
| `ctx.platform`                                                 | the system the app runs on: its file dialogs, the files the user picked, its file manager (_The platform layer_)                             |
| `ctx.namespace_picker(Message::Picker)`, `ctx.picked(message)` | the Kubernetes or Nomad Namespace picker for the page's header, and its messages handed back                                                 |

What `update` asks through `ctx` is done by the shell right after `update` returns. `shell::layout::page_header` is `PageHeader`.

### Replacing a stub

Every feature's `mod.rs` is one line today: `pub use crate::ui::shell::stub::{dialog, open, refresh, request, subscription, update, view, Message, State};`. To port a feature:

1. Delete that line and define the two types and seven functions above in the same folder (split into `state.rs`, `update.rs`, `view/` as it grows). Nothing else changes: `ui/features/mod.rs` already sends the feature's routes to this module, and the registry already has its sidebar entry, commands and shortcuts.
2. In `open`, match on `ctx.location.route` if the feature has several pages, `ctx.watch` what the page shows, and start from `ctx.location.query` if it is a list.
3. In `request`, start the actions the registry names for the feature (its commands' `Run::Open(_, Some(action))` and its shortcuts' `Run::Request(action)`).
4. If the manifest changed since, fix the feature's line in `shell/registry/defs/`; the test says what differs.
5. A feature whose routes are several tickets (the Containers list and a container's page) keeps the stub for the routes it has not ported: its `State` and `Message` are enums of its own page and `shell::stub`'s (`ui/features/docker/containers`).
6. Test with `shell::page::testing::Bench`: `Bench::at(route)` gives a `Context` on an empty data layer (`bench.data.set(&key, value)`), `Bench::on_host(route, host)` is a page behind a Host's gate, `bench.watched()` and `bench.asked()` say what the page asked for, `bench.took()` says it with its values (a toast's text, the route opened) and `bench.run(task)` runs what `update` returned. `bench.read_watched()` reads what the page asked to watch, as the shell does: with no Engine, a Host's read fails with `HostNotConnected`, which is the failure a page shows in place of what it could not read. `took()` also gives what the actions started with `ctx.act` told the shell. `bench.os` stands for the OS: `bench.os.answer(path)` is what the page's next dialog answers (none is a cancel) and `bench.os.took()` what was asked of it (_The platform layer_). A page has no Engine in its tests: an action fails with `HostNotConnected`, and `bench.told(outcome)` stands for an answer that worked. On a `Stage`, `frame.perform(task)` does what a task asks of the widgets (the focus `/` gives the search). `ui/features/app/hosts` is the smallest example, `ui/features/docker/containers` and `ui/features/app/board` the first whole ones.

A feature never names another feature: go there with `ctx.open(Route::…)`, and build what it shares with another from `ui/domain`.

### What the shell leaves to other tickets

The shell has the hook for each of these and nothing behind it yet. None is done until its ticket lands:

- **The update bar**: it shows when the window sends `shell::Message::UpdateFound`, which nothing sends, and has no install or restart button. "Check for updates" in the palette opens the Settings and sends `shell::Event::CheckForUpdates`, which is only logged. Both are the updater's (#255).
- **Global search in the palette** (`src/features/search`, the containers, images and volumes found as you type): not ported; the palette lists pages, Hosts and commands only.
- **The crash page** (`CrashPage.tsx`): iced has no boundary that catches a page's panic; nothing stands in for it.
- **Tray choices** (`TrayChoices.tsx`: a Host picked in the tray, Add host): the tray is the platform layer's (#252), which will send `shell::Message::Select` and `shell::Message::Go`. `Data::set_hidden` is its to call too.

## Reading data and following streams

`app/src/data/` is to the iced app what TanStack Query and `subscribe()` were to React; a feature never loads, caches or refreshes by itself.

- **Keys** (`data::keys`): one module per family of the React query keys, with the same parts (`hostKeys.containers(hostId)` is `keys::host::containers(host_id)`). Every key of a Host is in `keys::host::scope(id)`, of a Cluster in `keys::cluster::scope(id)` (Nomad: `keys::nomad::scope(id)`), of a Swarm in `keys::swarm::scope(id)`. What only your feature reads is filed under one of them with `.with(part)`.
- **Queries** (`data::Query`): a key, the use case that reads it, and its policy (`stale_after`, `no_retry`, `poll`, `keep`: React Query's `staleTime`, `retry`, `refetchInterval`, `gcTime`, with the same defaults). The lists several features read are in `data::resources`.
- **`Data`**, in the window's state: `data.watch(state, &query)` when the page opens (a cached read is shown at once and read again only if stale; two pages share one read), `data.read::<T>(&key)` in the view (a `Resource`: `Idle`, `Loading`, `Loaded`, `Refreshing`, `Failed` with the previous data), `data.unwatch(&key)` when it closes and `data.release(&scope)` when the selection moves, which abort the reads nobody waits for. Map its `Task<data::Message>` into your message and give them back to `Data::update`.
- **What a page watches is kept fresh for as long as it watches it**: an Engine event reads it again. A page with tabs or folders lets go of the reads of a tab that was left and a folder it moved on from (`ctx.unwatch`), so they are not read again behind it.
- **Live updates**: `Data::on_event` already puts statuses in the cached lists, marks what a Host or Cluster that went down holds as `Failed` (data kept), reads it again when it comes back, and makes Engine lists stale from Engine events, 250 ms after the last of a burst; a command run in a container (`exec_*`: a healthcheck's, a terminal's, the `find` of a folder's listing) makes nothing stale. A feature adds nothing for that; polling (`.poll(interval)`) is for what no event tells, and pauses while the window is minimised or hidden.
- **Mutations** go through `ctx.act(work, report, then)`, and nothing else: `work` is the future that calls the use cases, `report` makes an `Outcome` of their answer (`Outcome::quiet().saying(|| t!(..)).failing(surfaced).invalidating(key).putting(key, value)`: the toast, the failure's toast, what is read again, what is filed as read), and `then` is the message the page gets if it still shows (its dialog closes, its button stops waiting). The shell reports the outcome itself, before any message, so the toast and the invalidation happen even when the page was left before the Engine answered: what `useMutation`'s `onSuccess` and `onError` did goes in `report`, never in the page's `update`, whose messages are dropped once the page is gone. An action over several things reports one failure: the first one's message, with every item that failed in its detail (`ui/domain/containers/actions`). `data::Action` remains for what only the open page shows (a form's pending state and its error in place).
- **Streams** (`data::streams`): `shapes::lines`, `view`, `job` and `terminal` turn a `*_subscribe` use case into a stream of `Update`s (`Started`, `Batch`, `Ended`, `Failed`) for `Task::run`, or for `streams::subscription`. Batches come at most 60 times a second; a view's stream is cancelled when dropped, a terminal's session closed, and a job goes on until `stream_cancel`. A job is always started with `Task::run`, never as a subscription: a subscription that comes back would start it again. A view's stream is returned by the page's `subscription`.
- **Terminals** (`data::streams::session`): a `session::Backend` says what a terminal runs in (`Engine`, `Containerd`, `IncusExec`, `IncusConsole`, `Pod`, `Nomad`, each over its `*_exec_start` or console use case), a `TerminalSession` is made with it and the terminal's size (`TerminalSession::new(backend, cols, rows)`: a session never opens without one), `session.subscription(ctx.state, number)` follows it at that size (the page's own `number` tells two terminals on one container apart) and closes the session when the page lets go of it, and the session sends what is typed and the new sizes: `session.input(bytes)` and `session.resize(cols, rows)` only queue, and the future `session.started(state, id)` gives on `Update::Started` (for `Task::perform`) tells the size first, then writes them in order through the backend's `*_input` and `*_resize`. What that future ends with goes to `session.written(result)`: a write that failed is kept for the page to show (`session.failure()`; a session that ended has none), and once the session ended or failed `input` and `resize` let go of what they are given, quietly. `session::output(items)` is a batch's text, its exit and its failure; the text goes to `terminal::State::feed` (`docs/agents/widgets.md`, _The terminal_). Typing is not a mutation: it goes through the session, never `ctx.act`. No feature uses it yet: the pages with terminals are their tickets'.

Their tests need no window: a paused tokio clock (`#[tokio::test(start_paused = true)]`) and loaders that count their calls (`app/src/data/tests.rs`).

## The platform layer

`app/src/platform/` is what the app asks of the system beyond a window: the file dialogs, what opens a link or a folder, the files the user picked, the app's own folders. It replaces `tauri-plugin-dialog`, `tauri-plugin-opener`, `src/shared/lib/dialogs.ts` and the commands `read_text_file`, `write_text_file`, `open_data_folder` and `open_log_folder`. The tray, the window's state, the updater and the logging are other tickets' and will sit beside it.

A page reaches it through `ctx.platform` (a `Platform`, which the window keeps and the shell lends):

| React                                                             | iced                                                                                                                                                                        |
| ----------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pickFile(title, filters, defaultPath)`                           | `ctx.platform.pick_file(OpenFile::new(title).filter(Filter::new(name, &["yml"])).starting_in(folder))`                                                                      |
| `pickFiles(title)`                                                | `ctx.platform.pick_files(OpenFile::new(title))`                                                                                                                             |
| `pickFolder(title)`                                               | `ctx.platform.pick_folder(OpenFolder::new(title))`                                                                                                                          |
| `pickSavePath(title, defaultPath, filters)`                       | `ctx.platform.save_file(SaveFile::new(title, name).filter(..))`                                                                                                             |
| `openUrl(url)`                                                    | `ctx.open_url(url)`                                                                                                                                                         |
| `commands.readTextFile(path)`                                     | `platform::files::read_text(path)`                                                                                                                                          |
| `commands.writeTextFile(path, text)`                              | `platform::files::write_text(path, text)`, or `files::save(path, write)` when it is long                                                                                    |
| `commands.openLogFolder()`, `openDataFolder()`                    | `ctx.platform.open_log_folder()`, `ctx.platform.open_data_folder(ctx.state)`                                                                                                |
| `revealItemInDir` (unused in React)                               | `ctx.platform.reveal(path)`                                                                                                                                                 |
| `openPath` (unused in React, and not allowed by `opener:default`) | none, on purpose: the OS runs a file handed to it when it is a program, a script or a shortcut. `ctx.platform.open_folder(folder)` opens a folder and refuses anything else |

- **A dialog is a `Task`**, mapped into the page's message: `ctx.platform.save_file(request).map(Message::SaveTo)`. It opens over the app's window (asked for on the window's own thread, where macOS wants a sheet started), so `update` returns at once. It always ends with one answer, an `AppResult`:
  - `Ok(Some(path))` (or the files): what was picked.
  - `Ok(None)` (or no file): the user closed it, or the app's window went away. An answer, never an `AppError`; the page does nothing of it.
  - `Err(error)`: no dialog could be shown (`errors.file.dialogUnavailable`, with what to install as its hint). That only happens on Linux, where the dialog is the desktop's: no session bus, no portal, a portal with no file chooser, one that fails or that leaves the bus with its dialog open. The page says it as any failure (`ctx.failed(Surfaced::new(error, "save_file"))`); it must never look like a cancel.

  What the page needs once the dialog answers (the lines to save) stays in its state meanwhile.

- **One dialog at a time.** The OS's dialogs are modal to the window. A dialog asked for while one is open answers `Ok(None)` at once, and the first is the one the user sees. A page that keeps something for the answer ignores its button while its own dialog is open (`gallery/log.rs`), so a second press changes nothing of the first.
- **Everything else is a future** for `Task::perform` or `ctx.act`: reading, writing, opening. None blocks `update`. A mutation's rule holds here too: what must be said when the page is gone (the "Logs saved" toast, the failure) goes through `ctx.act`'s `report`.
- **Reading** (`files::read_text`) is `read_text_file`'s: a regular file only, 1 MB at most however large it claims to be, text only. A pipe, a device or a file of `/proc` is refused or cut at the limit, never waited on.
- **A file is written whole or not at all** (`files::replace`, on the workspace's `local_files::Replacement`): beside its target under a random name, private to the user (0600) from its first byte, then renamed over it and its folder synced. A write that fails part way, or a full disk, leaves the file that was there and nothing beside it. What that keeps and what it doesn't, against `write_text_file`, which wrote into the file:
  - kept: the permissions of the file replaced; a new file gets what any new file gets (the umask). A symbolic link is written through, to a file that is there or not; links that loop are refused.
  - not kept: the file's owner and group when another user's file is replaced, its ACL and extended attributes, its creation time, and its other names: a hard link keeps the old content.
  - a read-only file in a folder that can be written to is replaced (and stays read-only), where `write_text_file` was refused; a file that could be written in a folder that can't is refused, where it was written into.
  - a pipe, a device or a socket is refused (`errors.file.notRegular`); `write_text_file` wrote into it.
  - a file that can't be renamed over because it is a mount of its own (one file bound into a container or a sandbox, `EBUSY`) is written into instead: the one case where a failure part way can cut the file short.
  - files the app left unfinished (`.homeport-….part`, from a write that was killed) are taken away from the folder being written to once they are a day old. No other file is touched.
- **A long write tells how far it is, and can be stopped**: `files::save(path, |out, meter| ..)` gives `Saving { done, progress, stop }`. `done` is the work for `ctx.act` and answers `Saved::Whole` or `Saved::Stopped`; `progress` is a stream for `Task::run`, a step a percent at most, that ends with the write; `stop.now()` is a Cancel button's. The write is on a thread of its own and can't be interrupted: it is stopped between two chunks, the unfinished file goes, and the target is as it was. Dropping `done` stops it the same way, which is what closing the app does: the app then waits for that thread to end at its next chunk, not for the file. `log::Download::write_counted(out, |written| meter.at(written, total))` is the log viewer's.
- **The name a save dialog suggests is made safe** by `SaveFile::new` (`platform::names::suggested`): no path separator, no character Windows refuses, no format character or line separator (a right-to-left override makes `gpj.exe` read `exe.jpg`), no trailing dot or space, never a Windows device (`con.tar` is `_con.tar`; `COM0`, `LPT¹`, `CONIN$` too), 255 bytes at most with the extension kept (both parts of `.tar.gz`). The rules are the same on every system, so a container's name or a path inside a volume can be handed as it is. A name with nothing left of it suggests none: the dialog's field is left empty rather than filled with a word of the app's. `names::with_extension(name, "tar", fallback)` adds an extension the name doesn't end with already.
- **Filters** are written once, as extensions (`Filter::new("Archives", &["tar", "tar.gz"])`, `*` for every file), and mapped for the system that shows them (`dialogs::for_system`): macOS knows a file by the last part of its name, so `tar.gz` is given as `gz` there and a filter for every file means no filter.
- **Links**: `ctx.open_url` opens `http`, `https`, `mailto` and `tel` links only, as the Tauri app's capability did; anything else (a `file:` URL, a program's scheme) is refused with `errors.file.linkRefused`, which has no hint: there is nowhere else to open it. So what a container or a repository names can't start a program on this computer.
- **A file is never handed to the OS to open**: the OS runs it when it is a program, a script or a shortcut, and the Tauri app had no way to (`opener:default` has no `open-path`). `open_folder` opens a folder and refuses anything else; `reveal` shows a file where it lies.
- **Failures are `AppError`s of the catalogue**: `errors.file.*` for a file (`local_file`), `errors.file.folderNotOpened` for a folder, `errors.file.linkNotOpened` for a link the OS had nothing to open with, `errors.file.dialogUnavailable` for a dialog, `errors.file.noAppFolder` for a computer that names no home.
- **Folders**: the data folder is `AppState::data_dir`; the log folder is where the Tauri app wrote (`platform::folders::log_dir`: `~/Library/Logs/com.homeport.app` on macOS, `com.homeport.app/logs` under the local data folder elsewhere), and a failure on a computer that names no such folder. Nothing writes logs there until the logging layer (#254) does.

`gallery/log.rs` (the log viewer's download, with its progress) and `gallery/data.rs` (the file browser's Upload and Download) are the worked examples; `app/src/platform/mod.rs` has the pattern in short.

**The test seam.** The dialogs and the opener are traits (`platform::dialogs::Dialogs`, `platform::opener::Opener`) and `Platform` is a handle on two of them. In tests they are `platform::fake::Fake`: `fake.answer(path)` is what the next dialog answers (with none left, it is cancelled), `fake.without_dialogs()` is a desktop that can show none, `fake.refuse(kind)` makes the opener fail, `fake.took()` gives what was asked, and `fake.dir()` is a temporary folder to save in, which also holds the fake's log folder. A page's `Bench` has one as `bench.os`.

There is no default platform. A window and the gallery are built on the one they are given (`Homeport::new(platform)`, `Gallery::new(platform)`), and `Platform::native()` is written in two places only, where a program starts: `ui::run` and `gallery::run`. `tests/layout.rs` holds the crate, its tests, its examples and its benches to that, so nothing but a running program can open a dialog of the OS or a folder of the user's.

The Linux dialogs are run against a portal that is one: `dialogs/portal_tests.rs` starts a bus of its own (`dbus-daemon`, which the machine that runs the tests must have) with a `org.freedesktop.portal.Desktop` on it that answers as the desktop's would, picks, cancels, fails, hangs or dies. The wire format is the real one and no dialog shows.

**What differs by system**:

|         | Dialogs                                                                                                                                                                                                                                                                                  | Opening a link or a folder (`open` 5)                 | Showing a file in its folder                                                                                                                                                              |
| ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux   | the XDG desktop portal (`ashpd` 0.13 over D-Bus): the desktop's own chooser, GNOME's, KDE's or GTK's by the backend installed. No toolkit is linked. The parent window is named to the portal (X11 id, Wayland `xdg-foreign` handle). No portal is an error, not a dialog from elsewhere | `xdg-open`, then `gio open`, `gnome-open`, `kde-open` | `gdbus` calling `org.freedesktop.FileManager1.ShowItems`; without `gdbus` (it comes with GLib) or a file manager that answers, the file's folder is opened instead                        |
| Windows | `IFileDialog` (`rfd` 0.17), owned by the app's window                                                                                                                                                                                                                                    | `ShellExecuteW`                                       | `explorer.exe /select,"path"`, one argument put on the command line as Explorer reads it. Its exit code says nothing: a file it can't find shows a folder of its choosing and is not told |
| macOS   | `NSOpenPanel` and `NSSavePanel` (`rfd` 0.17), as a sheet of the app's window                                                                                                                                                                                                             | `/usr/bin/open`                                       | `open -R`                                                                                                                                                                                 |

On Linux the app asks the desktop's portal itself (`dialogs/portal.rs`), and `rfd` is a dependency of the other systems only (`app/Cargo.toml`, by target). The reason is the Tauri app: `rfd` builds with one Linux backend, Cargo gives a crate one set of features for the whole workspace, and `tauri-plugin-dialog` needs the GTK one. This way the Tauri app's dependencies are what they were before, and `cargo tree -p homeport-app -i gtk-sys` finds nothing. `ashpd` is the one crate added: it stands on the `zbus` the keychain already brought, with the features it already had.

There is no `zenity` fallback, on purpose, against the ticket's "portal and GTK": a GTK desktop gets GTK's chooser through `xdg-desktop-portal-gtk`, and a desktop with no portal at all is told what to install rather than shown an unparented dialog from a program it may not have either.

**To check by hand on the three systems (#284)**, since no test shows a real dialog:

- Linux, on GNOME, on KDE and on a desktop with `xdg-desktop-portal-gtk` only, under X11 and under Wayland: each of the four dialogs opens over the window and keeps it from being used; filters filter; the save dialog suggests the name; cancel does nothing. With `xdg-desktop-portal` stopped (`systemctl --user stop xdg-desktop-portal`) or removed: a toast says no dialog could be opened and what to install. With the portal killed while its dialog is open: the same toast, at once.
- Windows: the dialogs are owned by the window; a link opens in the browser; "show in folder" selects a file whose path has a space and a comma.
- macOS: the dialogs are sheets of the window and never hang; a `tar.gz` filter lets archives be picked and "All files" lets everything; `open -R` selects the file.
- Everywhere: saving over a file that exists asks once (the OS's question) and replaces it; saving into a folder that can't be written to is a toast and leaves no file; a million lines from the gallery's Log page show their progress; closing the app during that save leaves no `.homeport-….part` behind; "Open log folder" and, on a failed start, "Open folder" open the file manager.

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
