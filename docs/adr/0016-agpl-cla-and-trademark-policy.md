# Homeport is AGPL-3.0-only, contributions need a CLA, and the name is a trademark

Homeport was MIT-licensed. That lets anyone ship a closed, paid fork of it, and gives the maintainer no way to fund the project later. From 0.12.0, Homeport is licensed under the **GNU Affero General Public License, version 3 only**, every outside contribution needs a signed **Contributor License Agreement**, and the **name and logo** are covered by a trademark policy. Versions up to 0.11 stay MIT forever.

## The license: AGPL-3.0-only

- **AGPL rather than MIT**: every distributed fork must publish its full source under the same licence, so Homeport stays open source wherever it goes.
- **AGPL rather than GPL**: a future server part (sync, a team dashboard, a web UI) is covered too, since offering a modified version over a network counts as distributing it.
- **`-only` rather than `-or-later`**: the maintainer, not the FSF, decides whether a future AGPL version applies.
- For users, nothing changes: Homeport is free to use, personally and commercially.
- AGPL §6 asks every copy to point to its exact source: **Settings → Licenses** links to the running version's tag.
- Relicensing needed no one else's consent: every commit was authored by the maintainer, or by an AI agent the maintainer ran on the project (authored `Claude`), checked with `git shortlog -sne` over the whole history on 2026-11-06.

## Contributions: a CLA, not a DCO

As long as the maintainer holds all the copyright, they can sell commercial licences or a paid edition alongside the AGPL build. A Developer Certificate of Origin only certifies who wrote a change; it grants no right to relicense it. So every pull request author other than the maintainer and the bots signs [`CLA.md`](../../CLA.md): an Apache ICLA adapted to grant the right to license contributions under **any** terms, proprietary included, and to have contributors confirm their work is their own and brings in no third-party code. **It must be reviewed by a lawyer before the first outside contribution is merged.**

The CLA check (`.github/workflows/cla.yml`) is CLA Assistant Lite (`contributor-assistant/github-action`), pinned to v2.6.1's commit. It runs on `pull_request_target` and on comments, never checks out the pull request, and has only the permissions signing needs. Contributors sign by replying with a fixed sentence; signatures go to `signatures/cla.json` on the `cla-signatures` branch, outside the protected branches. The maintainer, `dependabot[bot]`, `github-actions[bot]` and `claude[bot]` are never asked. Commit author names aren't allowlisted, since anyone can set one: agents commit under the maintainer's identity.

The action's last release dates from September 2024 and runs on Node 20. If GitHub stops running it, switch to the hosted CLA Assistant app, which keeps the same flow.

The maintainer makes the check a required status on `dev` and `main`, and tests it with a second account: its pull request stays blocked until it signs, while the maintainer's passes without being asked.

## Dependencies: no copyleft

A single GPL, LGPL, AGPL, SSPL or non-commercial dependency would make a commercial licence impossible to sell. `about.toml > accepted` says so, and `scripts/accepted.test.ts` fails if such a license is added. MPL-2.0 (file-level copyleft, without the "Incompatible With Secondary Licenses" notice) and OFL-1.1 (fonts bundled, not sold alone) stay acceptable.

## The name: a trademark policy

A licence covers the code, not the name. [`TRADEMARKS.md`](../../TRADEMARKS.md) lets anyone redistribute unmodified builds, package Homeport for a distribution, say a product works with Homeport and write about it, but not name a fork or another product "Homeport", use the logo for one, or imply endorsement: a fork renames itself and replaces the logos. The maintainer searches the name in classes 9 and 42 (INPI, EUIPO, USPTO) before 1.0, and files for it if it's free; if it's taken, Homeport is renamed before 1.0, not after.

## Third-party names

Homeport's own brand no longer leans on Docker's: the tagline becomes **"Every container host, on one board."** (keeping ADR 0007's wording, without an Engine's name, which also fits the containerd, Incus, Swarm, Kubernetes and Nomad support planned for 1.0), and the README and docs no longer compare Homeport to Docker Desktop. The README, the website's footer and terms, and Settings → Licenses say that Docker, Podman, Kubernetes, Helm, Nomad, containerd, Incus, Colima, OrbStack and Rancher Desktop are trademarks of their owners and that Homeport isn't affiliated with any of them. No third-party logo is used as branding.

Factual uses stay: the lists of supported Engines, socket paths, install instructions for the `docker compose` CLI, the definition of Engine, error messages naming the Docker CLI, and code identifiers. Telling users what Homeport works with is nominative use.

## Considered Options

- **Staying MIT**: simplest, but a closed fork could sell Homeport's work, and nothing would fund it.
- **GPL-3.0**: covers the desktop app, not a future server part.
- **A DCO**: easy for contributors, but leaves each one holding copyright the maintainer can't relicense.
- **Copyright assignment**: stronger than a licence grant, but heavier for contributors and unenforceable in some countries; a broad licence grant is enough.
