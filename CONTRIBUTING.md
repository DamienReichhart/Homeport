# Contributing to Homeport

Thanks for wanting to help. Bug reports, ideas and pull requests are all welcome.

## Before you open a pull request

- **Sign the CLA.** Every pull request from someone other than the maintainer must have its author sign the [Contributor License Agreement](CLA.md). The CLA check comments on your pull request with a sentence to reply with; once you have, the check turns green. **A pull request can't be merged until it has.** The CLA lets the maintainer keep offering Homeport under other licences alongside the AGPL ([ADR 0016](docs/adr/0016-agpl-cla-and-trademark-policy.md)); it doesn't take away your own rights to your work.
- **Contribute only your own work.** Don't copy code, text or images from elsewhere, and don't add a dependency under a copyleft or non-commercial license (`about.toml` lists those Homeport accepts).
- **Open an issue first** for anything larger than a fix, so we can agree on the approach.

## Working on the code

- Set up and build Homeport as the [building guide](https://damienreichhart.github.io/Homeport/docs/project/building/) and the [README](README.md#commands) describe.
- Follow [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), the vocabulary of [GLOSSARY.md](GLOSSARY.md), and the decisions in [docs/adr](docs/adr).
- Make small, [conventional](https://www.conventionalcommits.org/) commits: `feat(volumes): clone a volume`.
- Before pushing: `bun run check`, `bun run rust:lint` and `bun run rust:test`, and update the docs in `website/src/content/docs/` when behaviour changes.

## The name and logo

Homeport's code is under the [AGPL](LICENSE); its name and logo are covered by the [trademark policy](TRADEMARKS.md). A fork must use its own.
