# The app's text comes from FormatJS catalogues, and errors travel as keys

Every user-facing string of the desktop app is read from a catalogue through FormatJS (`react-intl`), never written inline. There is one catalogue per language in `src/shared/i18n/<locale>/`, split into one file per namespace (`common`, one per feature, `errors`), with flat dotted keys such as `images.remove.title` and ICU MessageFormat values: `{name}` arguments and `{count, plural, one {…} other {…}}` plurals. Components and mutations call `t('images.remove.title', { name })` from `@/shared/i18n`; keys are typed from the English catalogue, so a missing key fails the type check.

Errors stay classified in Rust, but no longer travel as English only: an `AppError` carries a stable `key` and named `params` next to its `code` and English `message`, and so does its hint. The frontend renders message and hint from the catalogue (`errors.*` keys) and falls back to `message` when a key is unknown. Rust keeps building the English `message` from the same template, for the app log and bug reports. The English error catalogue is generated from Rust: each crate declares its messages once — key and English template — in a `messages!` block, and `bun run bindings` writes them to `src/shared/i18n/en/errors.json`. A later language translates that file like any other catalogue.

## Considered Options

**Library**

- **i18next / react-i18next**: the most used, with typed keys through declaration merging and `eslint-plugin-i18next`. ICU MessageFormat needs a plugin (`i18next-icu`) on top of its own plural suffixes (`_one`, `_other`), so two syntaxes would coexist; the runtime is the largest of the three.
- **Lingui**: the smallest runtime and the best extraction, but its macros need a Babel or SWC plugin, which the Vite and Rolldown pipeline (`@vitejs/plugin-react` 6) doesn't run, and Vitest would need it too.
- **FormatJS (`react-intl`)**: ICU MessageFormat natively (the plural rules come from the browser's `Intl.PluralRules`), `createIntl` works outside React for toasts and mutations, keys can be typed from a catalogue, and it runs unchanged under Vitest and Bun. Chosen. Adds about 20 kB gzipped.

**Where errors are translated**

- **Rust translates** with its own catalogue (Fluent) and the locale the frontend reports: messages stay next to the code that classifies them, but there would be two catalogues in two formats, errors raised before the frontend has told Rust its locale (at startup, from background tasks) would come out in the wrong language, and the log would need a second, English rendering anyway.
- **Rust sends a key and parameters, the frontend renders** (chosen): one catalogue format for the whole app, the language can change without asking the backend, and the log stays English because Rust still renders `message`. `ErrorCode` alone is too coarse for this (`Validation` covers dozens of sentences), so the key is separate from the code: `code` is for logic (`e.is(ErrorCode::PortInUse)`), `key` for text.

## Consequences

- The payload is `{ code, key, params, message, hint: { key, params, text, commands } | null, detail }`. `params` are strings or numbers, named in the template. `detail` (raw Engine, OS or library text) and hint `commands` are never translated and never go into `params`.
- Adding an error means adding a line to the crate's `messages!` block (`KEY = "area.case" => "English with {param}"`), building the error with `AppError::new(code, msg::KEY.with("param", value))`, and running `bun run bindings`. A Rust test fails when `errors.json` is out of date; a frontend test fails when a key used by Rust has no entry.
- Templates are ICU even in Rust: an apostrophe before `{` quotes it, as in `'{'`. Rust's renderer handles `{name}` arguments and that quoting, not plurals; plural error messages are written as two keys.
- Adding a UI string means a key in the feature's namespace file, used with `t()`. `bun run lint` rejects user-facing literals in components and toasts, and `bun run check` fails on a key used in code but missing from the catalogue, or a catalogue key nothing uses.
- Translating the app (French in v0.22.0) means a `fr/` folder with the same files, a language setting, and locale-aware formatting of dates, numbers and sizes; no component changes.
- Proper nouns (Homeport, Docker, Podman), commands, code samples and units are not translated.
