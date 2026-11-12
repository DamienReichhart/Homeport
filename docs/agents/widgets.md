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

`Keys` (`src/shared/ui/data`) is `basic::keys`, `DataTable` is `table::Table`, `SearchInput` is `form::search` (with `form::focus_search` for `/`); `PageHeader` is the shell's `shell::layout::page_header`. `WithShortcut` is `overlay::tooltip_with_keys`. A menu item is mapped to a page's message with `menu::Item::map`, takes a status dot (`menu::Item::status`), an empty state a `detail` (a failure under its title), and `select_owned` is a select over labels the view builds. `focus::key_area(..).on_captured(..)` hears the keys a field or a dialog used: only the shell uses it, for the shortcuts with Mod.

`src/shared/ui/feedback` is `widgets::feedback`, name for name: `error_message`, `Details` (the Details dialog), `Toasts::failure` (`toastError`), `confirm_dialog` (`ConfirmDialog`, `useConfirmedAction`), `empty_state`, `hint_view`, `spinner`, `status_dot`.

## The data table

`widgets::table` is `DataTable` and the Board's virtual list in one: a table over any rows that builds only those in view (and four more on each side), so 100,000 rows cost what 20 do. Look at it on the gallery's _Table_ page; `table/mod.rs` has the example to copy, `ui/features/app/hosts` a page that uses it.

- **The page keeps a `table::State<Key>`** (made once, in `open`: `State::new().selecting(Mode::Multi).sorted(Sort::by("name", Order::Ascending))`) and writes two functions that the view and `update` both call: `columns()` and `rows(..) -> table::Rows` (`Rows::new(&list, |row| row.id.clone())`). `Table::new(&state, columns(), rows, Message::Table).view()` draws it; `state.update(message, &rows, &columns())` takes every `table::Message` and gives back `Event::Opened(key)`, `Event::Selected` or `Event::Arranged`. `update` needs the rows and the columns because the keyboard goes through the rows in the order they show: it works that order out again if the list changed since the view. A page's state is one variant among fifty: box the table's (`Box<table::State<_>>`).
- **Columns are data**: `Column::new(id, t!(..), |row| cell)`, then `.width(Width::Fill(2))` (a share; `Width::Fixed` for pixels), `.between(min, max)`, `.align(Align::End)` for numbers, `.sort_by(|a, b| ..)` to make the header a button, `.descending_first()` for numbers, `.typed(|row| row.name.clone())` for the column typing looks in, `.hideable(false)` for the one that stays. A cell is any element, cut where its column ends; `table::text` is a line of `text-sm` that never wraps. Sort text with `table::sort::natural`: whatever the case, digits by their value (`host-2` before `host-10`), as TanStack's automatic sort; sort by the value, never by its translated label.
- **Rows are known by their key**, never by their place: the selection, the line the keyboard is on, an open menu and the scroll position survive a list that is read again, sorted or filtered, and rows added or removed above what shows don't move it. A row that left the list is let go of by the table (deselected, its menu closed); a selected row the search only hides stays selected.
- **Searching** is `Rows::filter(|row| table::matches(&query, &[&row.name]))`: the React `filter`, with the fields to look in named.
- **Groups** are `Rows::group_by(|row| row.stack.clone())`: the rows of a group under a header that closes and opens, the groups by name and before the rows of none, as the containers of a Stack. `Table::group_header(|name, rows| GroupHeader { label, cells })` says what the header shows beside its chevron and under which columns (a Stack's totals). The header's checkbox selects its rows.
- **Sorting, filtering and a selection are checked on every view**, a pass over the list (0.7 to 1.3 ms for 20,000 sorted rows, 3 to 4 ms for 100,000). A page that knows when its rows change says so with `Rows::revision(n)` and pays nothing in between; it must then change `n` when the rows or the search text do.
- **Text that doesn't fit** is `basic::truncated(text)`, which ends with an ellipsis instead of being cut through a letter; wrap it in `overlay::tooltip` where the React cell had a `title`. Beside a badge or a mark, put it in a container that fills and tell the row to fill (_Rules for a page_). `table::width_for(sample)` is the least width for a column as long as the language makes it (a size with its unit).
- **Always sorted.** `State::always_sorted()` makes a header go from one way to the other and never to unsorted: a table React sorted by one column at all times (the Board).
- **Selecting.** `Mode::Multi` adds the column of checkboxes. A press selects the row, Ctrl or Cmd adds or removes one, Shift selects from the last one; the checkbox over the rows selects those that show. With `State::opening_on_press()` a plain press opens the row instead, as `onRowClick` did. `state.mark(Some(key))` is `selectedId`: one row drawn as selected and scrolled to once it is in the list. `.selection_bar(Some(buttons))` is a line over the table for what can be done to the selected rows; pass `None` when nothing is selected, never leave the call out (its room stays, and so does the tree). The room is kept on purpose: a table whose press selects would otherwise move its rows under the pointer between the two presses that open one. A page that has a line of filters or a header puts the buttons there instead, as the React pages did, and no band stands over its table (`ui/domain/containers::actions::view::selection_bar` in the Containers page's filters and the Board's header).
- **Menus.** `.row_menu(|row| items)` puts a `…` button at the end of each row; the secondary button on a row, the menu key and Shift+F10 open it too. The items are asked only for the row whose menu is open.
- **Columns.** The end of a header is dragged to resize its column, no further than what the others' least widths leave; two presses there give it its own width back. `table::column_items(&state, &columns(), Message::Table)` are the items of a menu that hides and shows them. Columns whose least widths don't fit scroll sideways (Shift with the wheel, the arrows across, the scrollbar under the rows): nothing is pinned.
- **No rows.** `.loading(resource.is_pending())` shows skeletons under the columns, `.failed(Some(error_message(..)))` the failure in place of the rows, `.empty(empty_state(..).view())` or `.empty_label(t!(..))` what an empty list says ("Nothing here." unless told).
- **Height.** The table is as tall as its rows up to `.max_height(600.0)`, then scrolls: a page is in a scroll area, which gives all the height asked for. `.fill()` takes the height a pane gives instead.
- **What a page keeps**: `state.sort().keys()`, `state.widths()` and `state.hidden()` are plain data, to store when `Event::Arranged` comes and to give back with `State::sorted`, `with_widths` and `with_hidden`. The table stores nothing itself.
- **Accessibility**: nothing is wired. The hooks wait on the screen-reader spike (#228), which has not run; `ui.table.selectAll` and `selectRow` are in the catalogue and unused until then.

The keyboard, while the rows have focus (`table/grid/input.rs`):

| Key                     | What it does                                         |
| ----------------------- | ---------------------------------------------------- |
| Up, Down                | the line before, the line after, scrolled into view  |
| Page Up, Page Down      | a screen of lines                                    |
| Home, End               | the first line, the last                             |
| Shift with any of them  | selects from where the selection started to that row |
| Space                   | selects or deselects the row                         |
| Enter                   | opens the row                                        |
| Space, Enter on a group | closes or opens the group                            |
| Left, Right on a group  | close it, open it (the other way round in Arabic)    |
| Left, Right elsewhere   | scroll the columns that don't fit                    |
| Ctrl or Cmd + A         | selects every row that shows                         |
| Escape                  | deselects everything                                 |
| Menu, Shift + F10       | opens the row's menu; Escape there gives focus back  |
| Letters                 | go to the next row that starts with them (`typed`)   |
| Tab                     | leaves the rows                                      |

Tab goes through the checkbox over the rows, each header that sorts (Enter and Space sort, Shift adds the column to the sort), then the rows as one stop. The ring shows on the line the keyboard is on, for a key of the rows (not for Ctrl or Shift held for a press), and nowhere while that line is scrolled out of view. A table with a `typed` column takes the letters while its rows have focus: a page's one-letter shortcuts don't fire then.

What a view costs is held two ways (`table/tests/`). `counts.rs` runs with every `cargo test` and counts, for 100,000 rows, the cells built (a screen of them) and the rows compared, filtered and asked for their key (none, or one pass where the list says so): numbers no machine changes. `perf.rs` times a frame (`cargo test -p homeport-app --lib table::tests::perf -- --ignored --nocapture --test-threads=1`) and fails when the table's own share of a scroll step, or the `view` of a changed row, at 20,000 rows of 8 columns goes over the ticket's 2 ms (eight times that in a build that is not optimised; `TABLE_PERF_OPTIMISED=1` with the `--config` flags its rustdoc gives holds it to 2 ms). On 4 cores of a cloud VM, a 1100 × 720 window, software renderer, optimised, two runs:

| Rows               | A scroll step: `view` + layout + draw                  | Software renderer, the whole window |
| ------------------ | ------------------------------------------------------ | ----------------------------------- |
| 100, 5 columns     | 0.3 ms                                                 | 16 ms                               |
| 100,000, 5 columns | 0.3 to 0.4 ms (sorted: 3 to 4, or 0.3 with a revision) | 16 ms                               |
| 20,000, 8 columns  | 0.8 to 1.1 ms sorted (0.3 with a revision)             | 15 to 16 ms                         |

A row that changed costs 0.65 to 0.72 ms of `view` at 20,000 rows. The pixels are the software renderer's, the same for 100 rows as for 100,000, and by themselves about a frame at 60 a second on that machine; the window draws with wgpu, which was not measured.

## Rules for a page

- **Rows.** Write a row start to end with `widgets::line` and columns with `widgets::stack`; align with `start()` and `end()`. The page then mirrors in Arabic by itself. For text that fills a row, use `widgets::filling(text)`, and `widgets::paragraph` for body text: a `text` told to fill its width _and_ align right is drawn out of place in a right-to-left script.
- **A row that must keep its last item.** A `line` is as wide as what it holds, and then gives a filling item all the room before the items after it: a badge after a long text is pushed out of its cell. Tell the row to fill (`line(..).width(Length::Fill)`) and cut the text (`filling(text).clip(true)`): the badge keeps its place (a container's status and its health, an image and its "newer" mark).
- **Buttons.** Use the kit's (`basic::button`, or `focus::pressable` for an odd one), never iced's `button`: iced's takes no keyboard focus.
- **Tab.** The window sends `focus::traverse(shift)` for a Tab key press nothing captured (`focus::tab_presses()`). A dialog or an open panel captures Tab and keeps focus inside.
- **Overlays.** Their state is the page's (`menu::State`, `select::State`, a `bool` for a popover, an `Option` for a dialog), fed by their messages. What opened one keeps its focus underneath, so focus is back there when it closes. A page returns its dialog from its `dialog` function and the shell lays it over the window (`docs/agents/iced-migration.md`); what hosts a whole window lays a dialog with `overlay::over(page, dialog)` and toasts with `toast::over(.., toasts)`, even when there is none: they build the same tree either way, and iced keeps a widget's state (what is scrolled, what has focus) only while the tree around it keeps its shape. Toasts go over the dialog (`toast::over(overlay::over(page, dialog), toasts)`), so a failure is said while the dialog is up.
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
- The rows of a table are one Tab stop, moved in with the arrows; in React each row's first cell was a button and a stop, and so was the chevron of a Stack's header. Space selects the row the keyboard is on (in React it pressed that button, which opened the row); Enter opens it.
- A press on a row selects it and a double press opens it, unless the page asks for `opening_on_press`, which is what React did everywhere. Ranges with Shift, Ctrl or Cmd + A, Escape, the keyboard on rows, typing to a row, resizable and hideable columns and the menu on the secondary button are new.
- The checkbox over the rows, when only some are selected, is the React one as it was drawn: an empty box with the mark, not a filled one with a dash. A group's checkbox shows the same mark when only some of its rows are selected; React's was either on or off.
- A table's scrollbar is its own (it scrolls rows it has not built): its thumb is darker than a scroll area's `--border`, which could not be seen on a selected row, and it is on the left in Arabic. It has no kinetic or touch scrolling.
- The filter is given the fields to look in; React's looked in every column that had a value.
- No expandable rows (no React table had any), no columns to move, no row heights measured from their content (one height per table, `State::row_height`), no pinned column when the columns scroll sideways.
