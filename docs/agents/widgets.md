# The widget kit of the iced app

`app/src/ui/widgets/` is what every page is built from. It knows no Host and no container. If a page needs a control or an overlay that isn't here, add it here: a page never writes its own.

Look at all of it with `cargo run -p homeport-app --example widgets --features gallery` (themes, English and Arabic, everything pressable). Its tests draw it to `app/tests/snapshots/widgets-*.png` and drive it with the keyboard (`app/src/ui/widgets/gallery/tests.rs`).

The theme has its own, `cargo run -p homeport-app --example gallery --features gallery`, held to `app/tests/snapshots/theme-*.png` in both themes and both directions. Both galleries are built for tests and with the `gallery` feature only: their English literals are not in the app.

- **Icons** take the colour of the text around them (`theme::icon`), as Lucide's `currentColor`: give one a style only where it differs from that text.
- **Weights.** Use `theme::font::medium()`, `semibold()` and `bold()` for text in the app's language, the constants only for text that is Latin whatever the language: Arabic, Hindi, Japanese, Korean, Chinese and Russian are drawn by the OS's fonts, which have Regular and Bold.

## From the React primitives

| `src/shared/ui/primitives` | In the iced app                                                                                                       |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `button`                   | `basic::button::{text, icon, labelled}` with `Variant` and `ButtonSize`                                               |
| `badge`                    | `basic::badge`                                                                                                        |
| `card`                     | `basic::card`                                                                                                         |
| `separator`                | `basic::separator`, `separator_vertical` (iced's `rule`)                                                              |
| `progress`                 | `basic::progress` (iced's `progress_bar`)                                                                             |
| `scroll-area`              | `basic::scroll_area` (iced's `scrollable`)                                                                            |
| `collapsible`              | `basic::collapsible`                                                                                                  |
| `skeleton`                 | `feedback::skeleton`                                                                                                  |
| `alert`                    | `feedback::error_message` (the only alert the app showed)                                                             |
| `sonner`                   | `feedback::Toasts`                                                                                                    |
| `dialog`                   | `overlay::dialog` in `overlay::modal(..).close_on_scrim()`                                                            |
| `alert-dialog`             | `overlay::dialog` in `overlay::modal`; `feedback::confirm_dialog` for the usual                                       |
| `popover`                  | `overlay::popover`                                                                                                    |
| `tooltip`                  | `overlay::tooltip`                                                                                                    |
| `dropdown-menu`            | `overlay::menu::dropdown`                                                                                             |
| `select`                   | `form::select`                                                                                                        |
| `command`                  | `form::select(..).searchable(true)`; the palette itself is the shell's                                                |
| `tabs`                     | `form::tabs`                                                                                                          |
| `toggle`, `toggle-group`   | `form::toggle_group`, `form::segmented`                                                                               |
| `input`                    | `form::input` (iced's `text_input`)                                                                                   |
| `input-group`              | `form::input_group`; `form::password`, `form::number_input` are built on it                                           |
| `textarea`                 | `form::textarea` (iced's `text_editor`, growing with its text)                                                        |
| `label`                    | `form::field`, which also holds the help and the error                                                                |
| `checkbox`, `switch`       | iced's `checkbox` and `toggler`, styled by the theme; `form::checkbox` where it is `indeterminate` or must take focus |
| `table`                    | `table::Table` (_The data table_)                                                                                     |

`Keys` (`src/shared/ui/data`) is `basic::keys`, `DataTable` is `table::Table`; `PageHeader` is the shell's `shell::layout::page_header`. A menu item takes a status dot (`menu::Item::status`), an empty state a `detail` (a failure under its title), and `select_owned` is a select over labels the view builds. `focus::key_area(..).on_captured(..)` hears the keys a field or a dialog used: only the shell uses it, for the shortcuts with Mod.

`src/shared/ui/feedback` is `widgets::feedback`, name for name: `error_message`, `Details` (the Details dialog), `Toasts::failure` (`toastError`), `confirm_dialog` (`ConfirmDialog`, `useConfirmedAction`), `empty_state`, `hint_view`, `spinner`, `status_dot`.

## The data table

`widgets::table` is `DataTable` and the Board's virtual list in one: a table over any rows that builds only those in view (and four more on each side), so 100,000 rows cost what 20 do. Look at it on the gallery's _Table_ page; `table/mod.rs` has the example to copy, `ui/features/app/hosts` a page that uses it.

- **The page keeps a `table::State<Key>`** (made once, in `open`: `State::new().selecting(Mode::Multi).sorted(Sort::by("name", Order::Ascending))`), sends it every `table::Message` with the rows and the key function the view was given (`state.update(message, &rows, key)`), and reads what comes back: `Event::Opened(key)`, `Event::Selected`, `Event::Arranged`. A page's state is one variant among fifty: box the table's (`Box<table::State<_>>`).
- **Columns are data**, built in `view`: `Column::new(id, t!(..), |row| cell)`, then `.width(Width::Fixed(96.0))` or `Width::Fill(2)`, `.between(min, max)`, `.align(Align::End)` for numbers, `.sort_by(|a, b| ..)` to make the header a button, `.descending_first()` for numbers. A cell is any element; `table::text` is a line of `text-sm` that never wraps and is cut where its column ends.
- **Rows are known by their key**, never by their place: the selection, the row the keyboard is on, an open menu and the scroll position survive a list that is read again, sorted or filtered. Rows added or removed above what shows don't move it. `state.prune(..)` forgets selected rows that left the list.
- **Searching** is `.filter(|row| table::matches(&query, &[&row.name]))`: the React `filter`, with the fields to look in named.
- **Sorting and filtering are checked on every view**, a pass over the list when it is sorted or filtered (0.3 to 0.7 ms for 20,000 rows, 2 to 3 ms for 100,000). A page that knows when its rows change says so with `.revision(n)` and pays nothing in between; it must then change `n` when the rows or the search text do.
- **Selecting.** `Mode::Multi` adds the column of checkboxes. A press selects the row, Ctrl or Cmd adds or removes one, Shift selects from the last one; with `State::opening_on_press()` a plain press opens the row instead, as `onRowClick` did. `.selection_bar(Some(buttons))` is a line over the table for what can be done to the selected rows; pass `None` when nothing is selected, never leave the call out (its room stays, and so does the tree).
- **Menus.** `.row_menu(|row| items)` puts a `…` button at the end of each row; the secondary button on a row, the menu key and Shift+F10 open it too. The items are asked only for the row whose menu is open.
- **No rows.** `.loading(resource.is_pending())` shows skeletons, `.failed(Some(error_message(..)))` the failure in place of the rows, `.empty(empty_state(..).view())` or `.empty_label(t!(..))` what an empty list says ("Nothing here." unless told).
- **Height.** The table is as tall as its rows up to `.max_height(600.0)`, then scrolls: a page is in a scroll area, which gives all the height asked for. `.fill()` takes the height a pane gives instead.
- **What a page keeps**: `state.sort().keys()` and `state.widths()` are plain data, to store when `Event::Arranged` comes and to give back with `State::sorted` and `State::with_widths`. The table stores nothing itself.

The keyboard, while the rows have focus (`table/grid/input.rs`):

| Key                    | What it does                                         |
| ---------------------- | ---------------------------------------------------- |
| Up, Down               | the row before, the row after, scrolled into view    |
| Page Up, Page Down     | a screen of rows                                     |
| Home, End              | the first row, the last                              |
| Shift with any of them | selects from where the selection started to that row |
| Space                  | selects or deselects the row                         |
| Enter                  | opens the row                                        |
| Ctrl or Cmd + A        | selects every row that shows                         |
| Escape                 | deselects everything                                 |
| Menu, Shift + F10      | opens the row's menu; Escape there gives focus back  |
| Tab                    | leaves the rows                                      |

Tab goes through the checkbox over the rows, each header that sorts (Enter and Space sort, Shift adds the column to the sort), then the rows as one stop.

What a frame costs (`cargo test -p homeport-app --lib table::perf -- --ignored --nocapture --test-threads=1`, a 1100 × 720 window, software renderer, 4 cores of a cloud VM, the app at `opt-level=2`):

| Rows               | A scroll step: `view` + layout + draw                  | Software renderer, the whole window |
| ------------------ | ------------------------------------------------------ | ----------------------------------- |
| 100, 5 columns     | 0.25 ms                                                | 15.6 ms                             |
| 100,000, 5 columns | 0.3 to 0.4 ms (sorted: 2 to 3, or 0.3 with a revision) | 15 to 16 ms                         |
| 20,000, 8 columns  | 0.6 to 1.0 ms sorted (0.4 to 0.5 with a revision)      | 15 to 18 ms                         |

Two runs, hence the ranges. A row that changed costs 0.5 to 1.0 ms of `view` at 20,000 rows (the ticket allows 2). The pixels are the software renderer's, the same for 100 rows as for 100,000, and by themselves about a frame at 60 a second on that machine; the window draws with wgpu, which was not measured. An unoptimised test build takes ten times longer over the pixels and three to five times longer over the rest.

## Rules for a page

- **Rows.** Write a row start to end with `widgets::line` and columns with `widgets::stack`; align with `start()` and `end()`. The page then mirrors in Arabic by itself. For text that fills a row, use `widgets::filling(text)`, and `widgets::paragraph` for body text: a `text` told to fill its width _and_ align right is drawn out of place in a right-to-left script.
- **Buttons.** Use the kit's (`basic::button`, or `focus::pressable` for an odd one), never iced's `button`: iced's takes no keyboard focus.
- **Tab.** The window sends `focus::traverse(shift)` for a Tab key press nothing captured (`focus::tab_presses()`). A dialog or an open panel captures Tab and keeps focus inside.
- **Overlays.** Their state is the page's (`menu::State`, `select::State`, a `bool` for a popover, an `Option` for a dialog), fed by their messages. What opened one keeps its focus underneath, so focus is back there when it closes. Always lay a dialog over the page with `overlay::over(page, dialog)` and toasts with `toast::over(.., toasts)`, even when there is none: they build the same tree either way, and iced keeps a widget's state (what is scrolled, what has focus) only while the tree around it keeps its shape. Toasts go over the dialog (`toast::over(overlay::over(page, dialog), toasts)`), so a failure is said while the dialog is up.
- **Focus.** A ring shows for focus the keyboard gave (Tab, an arrow, a dialog opened with a key), not for a click: `focus::modality` keeps who was last there. One ring shows at a time: a button's hides while the menu or the list it opened is up. A button inside a field (`button::keep_focus`) leaves focus in the field. A menu and a select are modal: a press outside closes them and presses nothing; a popover lets the press through.
- **The window** (`ui/mod.rs`) hosts the one `Toasts` and the one Details dialog, and takes `Message::ThemeChanged` and `Message::LanguageChanged` when the Settings change.
- **Failures.** Make a `feedback::Surfaced::new(error, "use_case_name")` where the failure reaches the page: that writes the log line. Show it with `error_message` in place or `Toasts::failure` in passing; both lead to the Details dialog. State keeps the `AppError`, never its text.
- **Forms.** A use case's refusal goes through `form::Errors::route(error, &[(key, field)])`: under its field when its key is about one, else back to the form for `error_message`.

## What differs from the React app, and why

- The focus ring is drawn just outside a button, as CSS did; a field shows focus as its border (iced draws a field inside its bounds).
- A dialog's scrim dims more (25 %) and doesn't blur: iced has no blur.
- `EmptyState`'s edge is a whole hairline: iced draws no dashed border.
- `Skeleton` and a connecting `StatusDot` don't pulse; the spinner turns.
- A tooltip waits 300 ms under the pointer (the React one showed at once) and shows at once on keyboard focus. A press or Escape sends it away; focus given by a click doesn't show it.
- A submenu that had to open on the other side keeps its chevron pointing the way the language reads.
- Every tab of a `tabs` row is a Tab stop; Radix made the chosen one the only stop.
- In a right-to-left row, Tab goes through the buttons left to right, the order they are laid out in. Arrows, Home and End follow the language.
- A plain `select` doesn't scroll its list to the option the arrows reach; a list long enough to scroll should be `searchable`, whose `State::reveal` does.
- iced's `checkbox` and `toggler` keep their box before the label in Arabic.
- An empty state's description is centred line by line left to right; right to left its block is centred and its lines start on the right.
- A select's button keeps one line and cuts a label too long for it.
- The table. React's `DataTable` cut long lists into pages (`pageSize`) and only the Board drew a virtual list; here every table is virtual and none has pages (`ui.table.page`, `previous` and `next` are unused).
- The rows of a table are one Tab stop, moved in with the arrows; in React each row's first cell was a button and a stop. Space selects the row the keyboard is on (in React it pressed that button, which opened the row); Enter opens it.
- A press on a row selects it and a double press opens it, unless the page asks for `opening_on_press`, which is what React did everywhere. Ranges with Shift, Ctrl or Cmd + A, Escape, the keyboard on rows, resizable columns and the menu on the secondary button are new.
- The checkbox over the rows, when only some are selected, is the React one as it was drawn: an empty box with the mark, not a filled one with a dash.
- A table wider than its columns' minimums cuts the last columns: it doesn't scroll sideways (the React one did, `overflow-x-auto`).
- No type-ahead, no column to hide or to move, no grouped or expandable rows yet: nothing in React's `DataTable` had them, the containers page drew its Stack groups by hand.
