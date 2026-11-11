# The widget kit of the iced app

`app/src/ui/widgets/` is what every page is built from. It knows no Host and no container. If a page needs a control or an overlay that isn't here, add it here: a page never writes its own.

Look at all of it with `cargo run -p homeport-app --example widgets` (themes, English and Arabic, everything pressable). Its tests draw it to `app/tests/snapshots/widgets-*.png` and drive it with the keyboard (`app/src/ui/widgets/gallery/tests.rs`).

## From the React primitives

| `src/shared/ui/primitives` | In the iced app                                                                 |
| -------------------------- | ------------------------------------------------------------------------------- |
| `button`                   | `basic::button::{text, icon, labelled}` with `Variant` and `ButtonSize`         |
| `badge`                    | `basic::badge`                                                                  |
| `card`                     | `basic::card`                                                                   |
| `separator`                | `basic::separator`, `separator_vertical` (iced's `rule`)                        |
| `progress`                 | `basic::progress` (iced's `progress_bar`)                                       |
| `scroll-area`              | `basic::scroll_area` (iced's `scrollable`)                                      |
| `collapsible`              | `basic::collapsible`                                                            |
| `skeleton`                 | `feedback::skeleton`                                                            |
| `alert`                    | `feedback::error_message` (the only alert the app showed)                       |
| `sonner`                   | `feedback::Toasts`                                                              |
| `dialog`                   | `overlay::dialog` in `overlay::modal(..).close_on_scrim()`                      |
| `alert-dialog`             | `overlay::dialog` in `overlay::modal`; `feedback::confirm_dialog` for the usual |
| `popover`                  | `overlay::popover`                                                              |
| `tooltip`                  | `overlay::tooltip`                                                              |
| `dropdown-menu`            | `overlay::menu::dropdown`                                                       |
| `select`                   | `form::select`                                                                  |
| `command`                  | `form::select(..).searchable(true)`; the palette itself is the shell's          |
| `tabs`                     | `form::tabs`                                                                    |
| `toggle`, `toggle-group`   | `form::toggle_group`, `form::segmented`                                         |
| `input`                    | `form::input` (iced's `text_input`)                                             |
| `input-group`              | `form::input_group`; `form::password`, `form::number_input` are built on it     |
| `textarea`                 | `form::textarea` (iced's `text_editor`, growing with its text)                  |
| `label`                    | `form::field`, which also holds the help and the error                          |
| `checkbox`, `switch`       | iced's `checkbox` and `toggler`, styled by the theme                            |
| `table`                    | the table ticket                                                                |

`src/shared/ui/feedback` is `widgets::feedback`, name for name: `error_message`, `Details` (the Details dialog), `Toasts::failure` (`toastError`), `confirm_dialog` (`ConfirmDialog`, `useConfirmedAction`), `empty_state`, `hint_view`, `spinner`, `status_dot`.

## Rules for a page

- **Rows.** Write a row start to end with `widgets::line` and columns with `widgets::stack`; align with `start()` and `end()`. The page then mirrors in Arabic by itself. For text that fills a row, use `widgets::filling(text)`, and `widgets::paragraph` for body text: a `text` told to fill its width _and_ align right is drawn out of place in a right-to-left script.
- **Buttons.** Use the kit's (`basic::button`, or `focus::pressable` for an odd one), never iced's `button`: iced's takes no keyboard focus.
- **Tab.** The window sends `focus::traverse(shift)` for a Tab key press nothing captured (`event::listen_with`, status `Ignored`); the gallery shows it. A dialog or an open panel captures Tab and keeps focus inside.
- **Overlays.** Their state is the page's (`menu::State`, `select::State`, a `bool` for a popover, an `Option` for a dialog), fed by their messages. What opened one keeps its focus underneath, so focus is back there when it closes.
- **Failures.** Make a `feedback::Surfaced::new(error, "use_case_name")` where the failure reaches the page: that writes the log line. Show it with `error_message` in place or `Toasts::failure` in passing; both lead to the Details dialog. State keeps the `AppError`, never its text.
- **Forms.** A use case's refusal goes through `form::Errors::route(error, &[(key, field)])`: under its field when its key is about one, else back to the form for `error_message`.

## What differs from the React app, and why

- The focus ring is drawn just outside a button, as CSS did; a field shows focus as its border (iced draws a field inside its bounds).
- A dialog's scrim dims more (25 %) and doesn't blur: iced has no blur.
- `EmptyState`'s edge is a whole hairline: iced draws no dashed border.
- `Skeleton` and a connecting `StatusDot` don't pulse; the spinner turns.
- A tooltip waits 300 ms under the pointer (the React one showed at once) and shows at once on keyboard focus.
- Every tab of a `tabs` row is a Tab stop; Radix made the chosen one the only stop.
- In a right-to-left row, Tab goes through the buttons left to right, the order they are laid out in. Arrows, Home and End follow the language.
- A plain `select` doesn't scroll its list to the option the arrows reach; a list long enough to scroll should be `searchable`, whose `State::reveal` does.
- iced's `checkbox` and `toggler` keep their box before the label in Arabic.
