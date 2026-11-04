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
6. Open a pull request against the `dev` branch.

See [building from source](/docs/project/building/) to get set up.

### Adding text to the app

The app's text lives in a catalogue, never inline, so it can be translated ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). To add a string:

1. Add a key to the feature's file in [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` for the images feature, `ui.ts` for the UI kit, `common.ts` only for words that mean the same everywhere (Cancel, Close, Retry). Keys start with the file's name and say what the text is for: `images.remove.title`, `images.column.size`.
2. Write the value as an [ICU message](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` for a value, `{count, plural, one {# image} other {# images}}` instead of building plurals by hand, `<strong>…</strong>` for emphasis rendered with `tRich`. One key per sentence; don't glue fragments together, because word order changes between languages.
3. Use it with `t('images.remove.title', { name })` from `@/shared/i18n` — in components, mutations and toasts alike. Keys are typed, so a typo fails the type check. Use literal keys: when the text depends on a value, map the value to keys (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) rather than building `` `containers.done.${action}` ``.

`bun run lint` fails when a component writes text inline: JSX text, props people read (`title`, `aria-label`, `placeholder`, `description`, `label`…), and the text of toasts and confirmations. Strings that read as code rather than words — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — and proper nouns such as Docker or Podman pass. To keep a literal on purpose, say why on the line before: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` also fails on a key used in code but missing from the catalogue, a key nothing uses, and a key built at runtime.

Error messages are written in Rust: each crate declares its keys and English templates in its `messages!` block, and `bun run bindings` writes them to `src/shared/i18n/en/errors.json`. Don't edit that file by hand.

## Improving these docs

Every docs page has an **Edit page** link at the bottom. The pages are Markdown files in [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## How releases happen

Every push to `main` with a new version number builds installers for every platform, runs the full test suite, and publishes a GitHub release with checksums and build provenance. The process is described in [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
