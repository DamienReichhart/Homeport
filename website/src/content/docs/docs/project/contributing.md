---
title: Contributing
description: How to report problems, suggest features and contribute code to Homeport.
---

Homeport is developed in the open at [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). Contributions of every size are welcome.

## Reporting a bug

[Open an issue](https://github.com/DamienReichhart/Homeport/issues/new) with:

- your Homeport version and platform, from **Settings** (for example `v0.1.0 · macos/aarch64`);
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

## Improving these docs

Every docs page has an **Edit page** link at the bottom. The pages are Markdown files in [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## How releases happen

Every push to `main` with a new version number builds installers for every platform, runs the full test suite, and publishes a GitHub release with checksums and build provenance. The process is described in [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
