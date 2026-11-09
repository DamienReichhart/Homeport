---
title: Contributing
description: How to report problems, suggest features and contribute code to Homeport.
---

Homeport is developed in the open at [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). Contributions of every size are welcome.

## Reporting a bug

[Open an issue](https://github.com/DamienReichhart/Homeport/issues/new) with:

- your Homeport version and platform, from **Settings** (for example `v0.2.0 · macos/aarch64`);
- the kind of Host (local, SSH, TLS or TCP) and its Engine version;
- what you did, what you expected, and what happened — the exact message helps most;
- anything relevant from the [log file](/docs/reference/security/#logs).

Please report security problems privately, through [GitHub's vulnerability reporting](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Suggesting a feature

Open an issue describing the problem you want solved, not only the solution. Knowing how you work with your Engines helps find the right shape for it.

## Contributing code

1. Read [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Features are self-contained vertical slices, the frontend calls Rust only through the generated IPC bindings, and Rust logic lives in services that are unit-tested without Tauri.
2. Use the project's vocabulary from [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) — Host, Stack, Managed, External, Adopt — in code, UI text and commits. Main decisions are recorded in [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. Follow the [brand guide](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) for colours and voice: plain, specific, calm, in British spelling.
4. Make sure these pass:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Write commit messages as [Conventional Commits](https://www.conventionalcommits.org) — `feat(stacks): …`, `fix(hosts): …`. Release notes are generated from them.
6. Open a pull request against the `dev` branch. CI runs the frontend checks and the licence check on Linux, and `cargo clippy` and `cargo test` on Linux, macOS and Windows, so code behind `#[cfg(windows)]` or `#[cfg(target_os = "macos")]` is compiled before it's merged. A test that needs a Docker Engine is marked `#[ignore = "needs a local Docker Engine"]`; a separate Linux job runs those against the runner's Engine.

See [building from source](/docs/project/building/) to get set up.

### Adding text to the app

The app's text lives in a catalogue, never inline, so it can be translated ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). To add a string:

1. Add a key to the feature's file in [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` for the images feature, `ui.ts` for the UI kit, `common.ts` only for words that mean the same everywhere (Cancel, Close, Retry). Keys start with the file's name and say what the text is for: `images.remove.title`, `images.column.size`.
2. Write the value as an [ICU message](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` for a value, `{count, plural, one {# image} other {# images}}` instead of building plurals by hand, `<strong>…</strong>` for emphasis rendered with `tRich`. One key per sentence; don't glue fragments together, because word order changes between languages.
3. Use it with `t('images.remove.title', { name })` from `@/shared/i18n` — in components, mutations and toasts alike. Keys are typed, so a typo fails the type check. Use literal keys: when the text depends on a value, map the value to keys (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) rather than building `` `containers.done.${action}` ``.

`bun run lint` fails when a component writes text inline: JSX text, props people read (`title`, `aria-label`, `placeholder`, `description`, `label`…), and the text of toasts and confirmations. Strings that read as code rather than words — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — and proper nouns such as Docker or Podman pass. To keep a literal on purpose, say why on the line before: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` also fails on a key used in code but missing from the catalogue, a key nothing uses, and a key built at runtime.

Error messages are written in Rust: each crate declares its keys and English templates in its `messages!` block, and `bun run bindings` writes them to `src/shared/i18n/en/errors.json`. Don't edit that file by hand.

## Translating Homeport

The app and the website speak English, French, Spanish, German, Italian, Brazilian Portuguese, Russian, Simplified Chinese, Japanese, Korean, Hindi and Arabic. Each language uses one word per product term: look them up in [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) before translating, and add a row there when you choose a new one.

### Adding or changing a string

Whatever the string, `bun run check` (and so CI) fails until every language has it, with the same `{arguments}` and tags, and with every plural form that language needs.

- **App text**: add the key to `src/shared/i18n/en/<namespace>.ts` (see [Adding text to the app](#adding-text-to-the-app)), then to the file of the same name in every other language folder (`fr/`, `de/`, `zh-CN/`…). Translate it if you can. If you can't, copy the English text there and say so in the pull request, so a translator can follow up; never leave the key out.
- **An error message or hint**: declare it in the crate's `messages!` block and run `bun run bindings`, which writes `en/errors.json`; then add the key to each language's `errors.json`. Text the operating system shows — the tray menu and notifications — works the same way with a `native.` key and `native.json`.
- **Website text** (home, download, changelog, header and footer): add the key to the right file of `website/src/core/i18n/strings/en/` (`site`, `home`, `download` or `changelog`) and to the same file of every other language's folder; `astro check` fails when one lacks it. A docs sidebar label goes in `website/src/app/config/sidebar/<code>.ts`.
- **A docs page**: the English page is in `website/src/content/docs/docs/`. When you change it, update its French page at the same path under `website/src/content/docs/fr/docs/` in the same pull request. Other languages show the English page, with a note, until someone translates it.

Check one language with `bun run i18n:check --locale fr`.

### Adding a language

Files, plus one line for the website:

1. **App**: copy `src/shared/i18n/en/` to `src/shared/i18n/<tag>/` (a BCP 47 tag such as `pl` or `zh-TW`), write its `meta.ts` (`{ name: 'Polski', dir: 'ltr' }`, the language's own name for itself; `rtl` for a right-to-left script) and translate every file, keeping each key and turning each `export default { … }` into `export default { … } satisfies Translation<typeof en>` as the other languages do. `bun run i18n:check --locale <tag>` lists what's missing. Settings lists the new language, and the OS's language picks it, with no other change.
2. **Website**: add the language to `website/src/core/site/locales.ts`, then its strings in a copy of `website/src/core/i18n/strings/en/` named `<code>/` (registered in `TABLES` in `core/i18n/index.ts`), its sidebar labels in `website/src/app/config/sidebar/<code>.ts` (registered in `sidebar.ts`; these two lines are the only code a language needs, because those files load outside Vite, which is what finds folders), and Starlight's note for untranslated pages in `website/src/content/i18n/<tag>.json`. Its pages, URLs, language switcher entries, `hreflang` links and sitemap alternates follow from the list. Translated docs go under `website/src/content/docs/<code>/docs/`.
3. **Screenshots**: `bun run screenshots --locale <tag>` captures every shot in the new language into `website/src/assets/screenshots/<tag>/`; pages without one show the English shot.
4. **Terms**: add `docs/i18n/terms-<tag>.md`.

If a test doesn't see a language you just added, delete `node_modules/.vitest-cache`: Vitest can keep an old list of catalogue files.

## Improving these docs

Every docs page has an **Edit page** link at the bottom. The pages are Markdown files in [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## How releases happen

Every push to `main` with a new version number builds installers for every platform, runs the full test suite, and publishes a GitHub release with checksums and build provenance. The process is described in [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
