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

`LogViewer` (`src/features/containers/ui/logs`, the read-only `XTerm` every log view drew) is `log::Viewer` (_The log viewer_). A row of controls that goes on a second line when it lacks room (`flex flex-wrap`, with `ms-auto` on what goes to the end) is `basic::flow(items).push_from(index)`: it wraps the way the language reads, which iced's own wrapping row doesn't.

`Keys` (`src/shared/ui/data`) is `basic::keys`, `DataTable` is `table::Table`, `SearchInput` is `form::search` (with `form::focus_search` for `/`); `PageHeader` is the shell's `shell::layout::page_header`. `WithShortcut` is `overlay::tooltip_with_keys`. A menu item is mapped to a page's message with `menu::Item::map`, takes a status dot (`menu::Item::status`), an empty state a `detail` (a failure under its title), and `select_owned` is a select over labels the view builds. `focus::key_area(..).on_captured(..)` hears the keys a field or a dialog used: only the shell uses it, for the shortcuts with Mod.

`src/shared/ui/feedback` is `widgets::feedback`, name for name: `error_message`, `Details` (the Details dialog), `Toasts::failure` (`toastError`), `confirm_dialog` (`ConfirmDialog`, `useConfirmedAction`), `empty_state`, `hint_view`, `spinner`, `status_dot`.

## The data table

`widgets::table` is `DataTable` and the Board's virtual list in one: a table over any rows that builds only those in view (and four more on each side), so 100,000 rows cost what 20 do. Look at it on the gallery's _Table_ page; `table/mod.rs` has the example to copy, `ui/features/app/hosts` a page that uses it.

- **The page keeps a `table::State<Key>`** (made once, in `open`: `State::new().selecting(Mode::Multi).sorted(Sort::by("name", Order::Ascending))`) and writes two functions that the view and `update` both call: `columns()` and `rows(..) -> table::Rows` (`Rows::new(&list, |row| row.id.clone())`). `Table::new(&state, columns(), rows, Message::Table).view()` draws it; `state.update(message, &rows, &columns())` takes every `table::Message` and gives back `Event::Opened(key)`, `Event::Selected` or `Event::Arranged`. `update` needs the rows and the columns because the keyboard goes through the rows in the order they show: it works that order out again if the list changed since the view. A page's state is one variant among fifty: box the table's (`Box<table::State<_>>`).
- **Columns are data**: `Column::new(id, t!(..), |row| cell)`, then `.width(Width::Fill(2))` (a share; `Width::Fixed` for pixels), `.between(min, max)`, `.align(Align::End)` for numbers, `.sort_by(|a, b| ..)` to make the header a button, `.descending_first()` for numbers, `.typed(|row| row.name.clone())` for the column typing looks in, `.hideable(false)` for the one that stays. A cell is any element, cut where its column ends; `table::text` is a line of `text-sm` that never wraps. Sort text with `table::sort::natural`: whatever the case, digits by their value (`host-2` before `host-10`), as TanStack's automatic sort; sort by the value, never by its translated label.
- **Rows are known by their key**, never by their place: the selection, the line the keyboard is on, an open menu and the scroll position survive a list that is read again, sorted or filtered, and rows added or removed above what shows don't move it. A row that left the list is let go of by the table (deselected, its menu closed); a selected row the search only hides stays selected.
- **Searching** is done as the query is typed, whatever the case unless `Aa` is on, as text unless `.*` makes it a regular expression (the `regex` crate's: linear time, no look-around). Matches are marked and counted ("3 of 17"), Enter in the field or F3 goes to the next and Shift with either to the one before, round at the ends, bringing the match into view (which pauses the view). `Filter` shows only the lines that match, and what the viewer says itself ("1,234 lines skipped", "end of logs"). Nothing of it waits in `update`: plain text of up to 256 bytes is made into its matcher there, anything else (a regular expression, whose cost is its author's) by a task, within 4,096 bytes and 2 MiB; up to 2,048 unread lines of up to 512 KiB together are read in `update`, more are read 65,536 at a time by a task on a blocking thread, so typing a letter over a million lines returns at once and the count grows as slices come back. `state.search_error()` is an `AppError`: a query that can't be read (`ui.log.invalidRegex`, the crate's words as its detail), which marks the field and is said beside it, or a search whose task died twice on the same lines (`ui.log.searchFailed`); a slice is read again once before that, and no line is ever counted as read that was not.
- **Groups** are `Rows::group_by(|row| row.stack.clone())`: the rows of a group under a header that closes and opens, the groups by name and before the rows of none, as the containers of a Stack. `Table::group_header(|name, rows| GroupHeader { label, cells })` says what the header shows beside its chevron and under which columns (a Stack's totals). The header's checkbox selects its rows.
- **Sorting, filtering and a selection are checked on every view**, a pass over the list (0.7 to 1.3 ms for 20,000 sorted rows, 3 to 4 ms for 100,000). A page that knows when its rows change says so with `Rows::revision(n)` and pays nothing in between; it must then change `n` when the rows or the search text do.
- **Text that doesn't fit** is `basic::truncated(text)`, which ends with an ellipsis instead of being cut through a letter. `.lines(2)` lets it wrap onto a second line before anything is cut (a row that holds a name over an id has the room), and `.with_tip()` shows the whole text under the pointer while it is cut, and only then: a cut text always has one. On a `Stage`, `frame.fitted()` gives what each such text says and what it shows, for a test that holds a page to showing a text whole. A status with a health badge is `ui/domain/containers::cells::status`: the badge under the status, not beside it. Beside a badge or a mark, put it in a container that fills and tell the row to fill (_Rules for a page_). `table::width_for(sample)` is the least width for a column as long as the language makes it (a size with its unit).
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

## The log viewer

`widgets::log` is `LogViewer`: the lines of one or several sources as they come, followed, searched, selected and saved. It is not a terminal: it keeps lines, not a screen. Look at it on the gallery's _Log_ page (a sample, a million lines, or a stream of 50,000 lines a second); `log/mod.rs` has the example to copy.

- **The page keeps a `log::State`**, made once in `open` with how many lines to keep: `log::State::new(cap)` (the cap is a setting upstream; `.wrapping(false)`, `.with_timestamps(true)` and `.with_sources(labels)` say how it starts). `log::Viewer::new(&state, Message::Log).toolbar(controls).failed(failure).height(..).view()` draws it: `toolbar` is what the page adds before the viewer's own controls (which container, how far back, stdout and stderr apart), `failed` an `error_message` shown over the lines when the stream broke. It takes the height it is given; in a page that scrolls, give it one (`.height(420.0)`).
- **`state.update(message)` takes every `log::Message`** and returns `(Task<log::Message>, Option<log::Event>)`. Map the task to the page's message and return it: it is how a search reads a large buffer, how a regular expression is built, how a large selection is copied, and how Ctrl + F reaches the search field. Act on the event: `Event::Copy(text)` is for `ctx.copy`; `Event::Download(lines)` is what to save (below); `Event::Timestamps(on)` tells a page whose stream is asked for timestamps only while they show to open it again, as the React viewer did; `Event::Cleared` says the Clear button was pressed.
- **A stream feeds it** through the page's `subscription` (`streams::subscription(.., shapes::logs)`, or `shapes::lines` over another `*_subscribe`). For each `Update::Batch { items, dropped }`: `let batch = shapes::log_lines(items)`, then `state.skipped(dropped)`, `state.extend(batch.lines.into_iter().map(log::Line::from))` (return its task, mapped), `state.end()` if `batch.ended`, and keep `batch.error` as a `Surfaced` for `.failed(..)`. Batches come at most 60 times a second, so a stream of 50,000 lines a second is 60 calls of about 833 lines; `extend` looks at the new lines only. `Update::Failed` is a failure too; `Update::Ended` needs nothing more. When the page opens the stream again (other options, another container), `state.reset()` first: no line, no source and nothing a program left open (a colour, half a sequence) is kept, as React reset its terminal; the query and how the lines show stay. `state.set_sources(labels)` then gives the new stream's sources their order. `state.clear()` is the Clear button's: the stream goes on, so its sources keep their colours.
- **Timestamps.** A line of a stream that was asked for timestamps says so: `log::Line::from(line).timestamped(true)`. The Timestamps toggle then shows and hides them without the stream being opened again, so ask the Engine for timestamps from the start (`LogOptions::timestamps`) and leave them hidden (`with_timestamps(false)`). The timestamp is what stands before the first space when it starts with a digit (up to 64 printable ASCII bytes); it is taken off before the program's text is read, so nothing the program writes reaches it (a carriage return starts the message over, not the line), and a line that is nothing but its timestamp has an empty message.
- **Several sources** (`Line::source`, which `From<LogLine>` fills): each line starts with its source's label, padded to the longest and in one of six colours, in the order the sources were first seen; `with_sources` gives that order before any line came. Each source has its own terminal state, so a colour one of them left open does not run into the others' lines.
- **Following.** The view stays on the latest line. A scroll up (the wheel, a key, the thumb) pauses it: it then stays on its line whatever comes after and whatever is dropped before, until that line is itself dropped (it is then on the oldest line kept). It follows again when scrolled back to the end, with End, the Follow toggle, or "Jump to latest", which shows while the view is paused away from the latest line, in room of its own under the rows (two rows' worth, inside the frame, at the end of the row in either direction): it covers no line. Paused on the latest line (the Follow toggle turned off there) nothing shows, and a notch of the wheel down follows again.
- **Colours.** SGR sequences are read (the 16 colours and their bright forms, the 256, 24-bit colours with `;` or `:`, bold, dim, italic, underline, inverse, strike, and every reset); every other escape sequence is taken out and never drawn; a sequence cut by the end of a line is finished by the start of the next. The 16 are the theme's terminal palette; any colour that would not read on what is behind it (3:1) is moved towards the text's colour until it does, so both themes stay readable. Bold is the bright form of the first eight colours and nothing else: the monospaced face has one weight, so bold text in the default colour or in any other colour draws as it would without (the React terminal did the same: `fontWeightBold: 'normal'`). A code no terminal knows changes nothing; the colour of an underline (`58`) is read and not drawn. A tab goes to the next multiple of eight cells of the message, and a carriage return starts the message over (a progress bar keeps its last state).
- **Wrapping.** A line longer than the viewer goes on the next row at the viewer's last cell, as in a terminal (not at a word); the Wrap toggle turns that off, and long lines then scroll sideways (Shift with the wheel, Left and Right, the thumb under the rows). A wrapped line is as many rows as its cells divided by the viewer's (a character of two cells that would straddle a row's last cell starts the next row, so a line that holds one has its row starts listed instead): the font is monospaced, so no line is ever measured, and the view is anchored to a line and a row of it, never to a distance in pixels. Whatever the width, only the lines in view are looked at, and a row that stays in view is not shaped again.
- **Searching** is done as the query is typed, whatever the case unless "Aa" is on, as text unless ".\*" makes it a regular expression (the `regex` crate's: linear time, no look-around). Matches are marked and counted ("3 of 17"), Enter in the field or F3 goes to the next and Shift with either to the one before, round at the ends, bringing the match into view (which pauses the view). A regular expression that can't be read marks the field and says so; nothing is searched until it is mended. `Filter` shows only the lines that match. Up to 2,048 unread lines are read in `update`; more are read 65,536 at a time by a task on a blocking thread, so typing a letter over a million lines returns at once and the count grows as slices come back.
- **Selecting** is by text, as in a terminal: a drag, a press with Shift from where the selection started, two presses for a word, three for a line, Ctrl or Cmd + A for everything. Ctrl or Cmd + C and the Copy button give `Event::Copy` with what shows (labels and timestamps included, colours not), a wrapped line as one line. A selection over more than 4,096 lines or 1 MiB is read by a task. A drag past the rows takes the view and the selection a row further each time the pointer moves.
- **Saving.** `Event::Download(lines)` carries every line kept, filtered or not, as they were when the button was pressed (`lines.lines()`, `lines.write_to(file)`, `lines.len()`): `source | text` without colours, with timestamps while they show. It is cheap to hold and can be written on another thread. The file dialog is #253's: until then a page can only drop it.
- **In Arabic** the controls, the scrollbar and "Jump to latest" mirror; the log's text starts on the left and Left and Right scroll it the same way.
- **Accessibility**: nothing is wired, as for the table (#228).

The keyboard, while the rows have focus (`log/pane/input.rs`); they are one Tab stop, after the controls:

| Key                | What it does                                     |
| ------------------ | ------------------------------------------------ |
| Up, Down           | a row up, a row down                             |
| Page Up, Page Down | a screen of rows                                 |
| Home               | the oldest line                                  |
| End                | the latest line, which is followed again         |
| Left, Right        | scroll long lines sideways, when they don't wrap |
| Ctrl or Cmd + A    | selects every line that shows                    |
| Ctrl or Cmd + C    | copies the selection                             |
| Ctrl or Cmd + F    | goes to the search field                         |
| F3, Shift + F3     | the next match, the one before                   |
| Escape             | deselects                                        |

`/` on a page is `form::focus_search(state.search_id())`.

Its limits:

- **Memory** is the text, sixteen bytes a line, sixteen a coloured run and sixteen a line that matches the search: about 130 MB for a million lines of a hundred bytes. The cap is the only bound; nothing is written to disk.
- **Cells are counted, not measured**, by Unicode's tables (`unicode-width`, which was already in the tree): two for East Asian wide characters and emoji, none for what combines (accents, Arabic and Indic signs, joiners; what a joiner joins is part of the character before), one for the rest. Text the monospaced face draws (Latin, Cyrillic, Greek, box drawing) is exact. Each run of another script is placed at its own cell, and so is the text after it, but within the run the OS's font decides: its glyphs may be narrower or wider than their cells, so a mark of the search or a selection's edge inside such a run can be off by part of a character, and a run that reads right to left is laid out by the shaper while a selection across it follows cells.
- **A line of megabytes** (minified JSON) is walked once when it first shows: it gets marks every 128 cells, the starts of its rows if a wide character keeps them from being a division, and the list of its matches (for 64 such lines at a time). A layout then walks what it shows (`log/tests/layout.rs` counts it). Building that list is a search of the whole line, once per query.
- **A batch of 50,000 lines at once** (the stream's whole backlog, when the window was behind) is one `extend` of 15 ms: a frame is late once. A batch larger than the cap is trimmed as it goes, never held whole.
- **The selection has no caret**: the keyboard selects everything or nothing. A drag past the rows scrolls only while the pointer moves.
- **The scrollbar counts lines**, not rows: among wrapped lines of very different lengths the thumb moves unevenly.
- **Sources**: 65,534 at most; the widest line counted for the sideways scroll is never forgotten until the lines are cleared.

How much a layout does is counted on every `cargo test`, in numbers no machine changes: what it shapes (`log/tests/driven.rs`: nothing when nothing changed, one row for a row scrolled in, a screen for a jump, among 300,000 lines) and what it walks of a line of a megabyte with a hundred thousand matches (`log/tests/layout.rs`: the line once, then a screen's worth).

What a frame costs is held by `log/tests/perf/`, which fails when the viewer's own share of a frame (`update`, `view`, layout, draw) goes over its budget. The numbers that count are those of the build that ships, the workspace's release profile (`opt-level = "s"`, LTO): `cargo test --release -p homeport-app --lib log::tests::perf -- --ignored --nocapture --test-threads=1`. Without `--release` the tests say the build is not optimised and give eight times each budget. A million lines of a busy service with a colour in each and timestamps showing, either about 120 cells (they fit a row) or about 330 (three rows each), a 1100 × 720 window of 38 rows, 4 cores of a cloud VM, software renderer:

| What, with 1,000,000 lines kept                                   | Budget | Release, two runs             | Not optimised   |
| ----------------------------------------------------------------- | ------ | ----------------------------- | --------------- |
| A frame that changes nothing                                      |        | 0.10 ms                       | 0.6 ms          |
| A scroll step of three rows, lines of three rows                  | 2 ms   | 0.21 to 0.22 ms               | 1.4 ms          |
| The same among 1,000 lines                                        |        | 0.16 to 0.18 ms               | 1.4 ms          |
| The same lines not wrapped                                        | 2 ms   | 0.25 to 0.28 ms               | 1.9 ms          |
| A scroll step, lines that fit a row                               | 2 ms   | 0.26 to 0.27 ms               | 1.7 ms          |
| A page down, lines of three rows: every row new                   | 4 ms   | 0.80 to 0.98 ms               | 6.4 ms          |
| A page down, lines that fit a row                                 | 4 ms   | 1.3 to 1.4 ms                 | 10.8 ms         |
| The thumb dragged from one end to the other                       | 4 ms   | 1.0 ms                        | 7.4 ms          |
| 833 lines appended (a sixtieth of a second at 50,000 a second)    | 1 ms   | 0.23 to 0.24 ms               | 2.5 ms          |
| The same, read and matched for a search                           | 2 ms   | 0.31 to 0.48 ms               | 5.8 ms          |
| The same into a buffer of 1,000 lines                             |        | 0.22 to 0.33 ms               | 2.9 ms          |
| 50,000 lines appended at once                                     |        | 14.5 to 14.7 ms               | 149 ms          |
| A frame following 50,000 lines a second, lines that fit a row     | 4 ms   | 1.3 to 1.4 ms                 | 14.5 ms         |
| Its worst frame of two seconds                                    | 8 ms   | 1.8 to 2.0 ms                 | 23.5 ms (of 33) |
| The same, lines of three rows (worst frame)                       | 4 ms   | 1.1 to 1.4 ms (2.1 to 2.2 ms) | 13.2 ms (20.8)  |
| The same with a search on, which reads and matches the new lines  | 4 ms   | 1.5 to 1.7 ms                 | 18.1 ms         |
| The same, paused 200 rows up                                      | 4 ms   | 0.35 to 0.36 ms               | 3.7 ms          |
| The longest `update` of a search (typing, an answer coming back)  | 2 ms   | 0.14 to 0.20 ms               | 0.9 ms          |
| The query built in `update`: 256 bytes of text, whatever the case | 2 ms   | 0.32 ms                       | 1.2 ms          |
| The first step to a match of 4,750,004, then each step            | 4 ms   | 0.03 ms, then 0.001 ms        | 0.25 ms         |

Following the stream takes the viewer 8.5 % of the time in the release build (170 ms of work for two seconds of lines) and 87 % when not optimised: a debug build follows without room to spare, and its worst frame is a frame and a half. A search of the million lines is read in 16 slices on another thread in 0.12 to 0.15 s (2 to 4 s not optimised). The release build first failed one budget: the answer of a slice in which every line matched took 3.4 to 4.0 ms of `update`, until the matching lines were kept in blocks. The software renderer's pixels, 10 to 21 ms for the whole window, are not the viewer's and are the same for 1,000 lines as for 1,000,000; the window draws with wgpu, which was not measured.

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
- The log viewer is a list of lines, not a terminal (React drew logs in a read-only xterm.js): sequences that move the cursor or clear the screen are taken out rather than obeyed, and a carriage return only starts its line over.
- The log view pauses by itself when scrolled up and follows again at the end; in React it followed until the Follow toggle was turned off, and scrolling up while lines came was undone by the next line. "Jump to latest" is new.
- The log's Timestamps toggle hides and shows what the lines carry; React opened the stream again each time (a page can still do that: `Event::Timestamps`). Only what starts with a digit before the first space is a timestamp; React took whatever stood there, the first word of a line without one included.
- The log's search is done as it is typed, counts its matches, and has the case and regular expression toggles and the filter; React searched on Enter, whatever the case, as text. The match the search is on is amber seen through in the dark theme, so the light text on it reads; xterm drew a solid amber.
- A program's colours are moved until they read on the viewer (3:1); xterm drew them as asked, black on the dark theme's black included. A line of stderr is tinted and marked; React drew both streams alike.
- The labels of a multi-source log are padded to the longest label there is now; React padded each line to the longest known when it was written. Each source has its own colour state; React's sources shared one terminal's.
- A log's wrap toggle, its text selection across the scrollback with Ctrl or Cmd + A and C, the lines skipped said between the lines, and a cap the page gives are new: xterm always wrapped, kept 50,000 rows, and the viewer kept 100,000 lines to save. Text is placed by cells counted from Unicode's tables rather than by a terminal's own; glyphs of scripts the monospaced face lacks may not fill their cells.
- The lines a log saves have no escape sequences; React saved the text as the program wrote it, colours' sequences included.
- A log stream that breaks is said over the lines with `error_message` (`Viewer::failed`), with its Details; React wrote the message in red among the lines.
