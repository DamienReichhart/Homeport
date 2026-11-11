# Text in the iced app

The iced app (`app/`) shows every string from a [Fluent](https://projectfluent.org) catalogue, in 12 languages (English and the 11 of `docs/i18n/terms-*.md`). ADR 0011 still holds: keys, not literals; English is the reference; a language is a folder.

Until the React app is removed, **the formatjs catalogues in `src/shared/i18n` stay the source**. The Fluent files under `app/i18n/<language>/<namespace>.ftl` are written from them and must not be edited by hand.

## Add or change a string

1. Add the key to the English namespace file (`src/shared/i18n/en/<namespace>.ts`) and to the same file of every other language, as before. `bun run i18n:check` tells you what's missing.
2. Run `bun run i18n:fluent`. It rewrites `app/i18n/**` and `app/tests/fixtures/i18n-golden.json`.
3. Use it from Rust by its dotted key:

   ```rust
   use crate::t;

   text(t!("containers.title"))
   text(t!("containers.summary", running = 3, count = 12))
   ```

   A message with tags (`<strong>`, `<mono>`, `<code>`, `<link>`) is read with `rich!`, which returns the spans to style; its arguments can't bring markup of their own.

4. `cargo test -p homeport-app --lib i18n` must pass (see _The check_).

`bun run check` runs `i18n:fluent --check`, so a formatjs change without step 2 fails.

## Add a language

1. Add its folder to `src/shared/i18n` as ADR 0011 describes (`meta.ts` with the language's own name and its direction, and every namespace file), and its term list `docs/i18n/terms-<tag>.md`.
2. Run `bun run i18n:fluent`. The iced app finds the new folder by itself: `i18n::LOCALES` lists every folder of `app/i18n`.
3. Add its row to the table of `every_language_formats_numbers_dates_sizes_and_relative_times` in `app/src/i18n/tests.rs`, and bump the count in `check.rs`.

## How formatjs maps to Fluent

| formatjs (ICU)                          | Fluent                                                                                               |
| --------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `containers.empty.title`                | `containers-empty-title` (ids have no dots; code keeps dots)                                         |
| `{name}`                                | `{ $name }`                                                                                          |
| `{n, plural, =0 {…} one {…} other {…}}` | `{ $n -> [0] … [one] … *[other] … }`, exact numbers first                                            |
| `#` in a plural                         | `{ NUM($n) }`: the count in the language's digits and grouping                                       |
| `{kind, select, a {…} other {…}}`       | `{ $kind -> [a] … *[other] … }`                                                                      |
| `<strong>…</strong>`                    | kept in the text; `rich!` splits it into spans                                                       |
| `{n, number}`, dates, ordinals, offsets | not converted: the converter refuses them. Format the value with `i18n::format` and pass it as text. |

## Language and direction

- `i18n::set_language(setting, &i18n::locale::preferred())` shows the app in the language the Settings value stands for (`system` or a tag), with the same rules as the React app: exact tag, then base language, never Simplified Chinese for a Traditional Chinese system, else English. The window sends itself `Message::LanguageChanged` and the next draw is in the new language; nothing restarts.
- Layouts never ask which language it is. They take `i18n::direction()` and use `Direction::ordered` (the order of a row's children), `pick` (start/end alignment) and `sides` (start/end paddings). Arabic is the only right-to-left language today.

## Numbers, dates, sizes, relative times

`i18n::format` is the only place that formats them (ICU4X, CLDR data built into the binary): `number`, `percent`, `bytes`, `relative`, `date_time`, `date`, `time`, `short_id`. They format in the OS's variant of the app's language when it has one (`en-GB`, `fr-CA`).

The output is CLDR's, as the webview's `Intl` was, but not byte-for-byte what a given browser printed: for example the medium date in Arabic and Korean is numeric here.

## Errors from the crates

An `AppError` carries the key of its message and hint (`errors.*`) and their parameters. `i18n::errors::message` and `i18n::errors::hint` render them in the app's language, and fall back to the error's English text when no catalogue has the key.

## The check

`app/src/i18n/check.rs` replaces `bun run i18n:check` for the Fluent catalogues, on every `cargo test`:

- every `.ftl` file parses, and holds only messages of its namespace, each once;
- every language has exactly the English messages, with the same arguments and tags;
- every plural has the categories its language needs (Russian `few` and `many`, Arabic `zero` and `two`…);
- every key a `t!` or `rich!` names exists;
- for the namespaces listed in `PORTED`, every message is named by the code (an unused key fails). A namespace joins the list when its last page is ported; the cleanup ticket checks them all;
- the catalogues render what formatjs renders: 4,000+ renderings of every message with a plural or a select, in every language, are compared with `app/tests/fixtures/i18n-golden.json`.
