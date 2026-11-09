# Releasing

Every push to `main` runs the **Release** workflow (`.github/workflows/release.yml`) and releases the version set in the repo:

1. **Prepare**: `bun scripts/release.ts plan` reads the version from `src-tauri/tauri.conf.json`, `package.json`, `Cargo.toml` (`[workspace.package]`) and `Cargo.lock`. The run **fails** if they disagree, if that version already has a `v*` tag, or if the [public signing keys](#signing-keys) aren't committed. Otherwise it writes the release notes from the [Conventional Commits](https://www.conventionalcommits.org) since the previous release.
2. **CI and Build**, in parallel: the whole of `ci.yml` (frontend checks, licenses, Rust fmt, and clippy and tests on Linux, macOS and Windows), and a native build for each platform. Each build signs its installers for the in-app updater (`.sig` files), and the Linux builds sign the `.rpm` with the package key.
3. **Publish**, once CI and every build passed: records a [build provenance attestation](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations) for each file, then creates the release with every installer, updater file and `SHA256SUMS.txt`. That creates the tag on the commit that was built.

Once the release is out, the **Pages** workflow rebuilds the [website](https://damienreichhart.github.io/Homeport/) so its download page and changelog list the new version, and republishes the [package repositories and the update manifest](#package-repositories-and-updates) with it. From then on, installed copies see the update.

Nothing is published unless every job succeeds. To retry, re-run the failed jobs, run the workflow from the Actions tab, or push a fix.

## Releasing a new version

Once, before the first release: [create the signing keys](#signing-keys).

1. Set the new version in `src-tauri/tauri.conf.json`, `package.json` and `Cargo.toml` (`[workspace.package] version`).
2. Run `cargo check` so `Cargo.lock` picks it up.
3. Record the migrations this release ships. For each file of `crates/storage/src/db/migrations/` that isn't in `crates/storage/src/db/migrations/RELEASED` yet, append a line with its name, its SHA-256 and the new version:

   ```sh
   sha256sum crates/storage/src/db/migrations/0018_database_info.sql
   # 0018_database_info.sql <the hash> 1.0.0
   ```

   From then on the file is frozen; see [Data compatibility](#data-compatibility). A release without a new migration adds no line.

4. Make the database fixture of the version being released, and commit it with the rest:

   ```sh
   cargo run -p homeport-storage --features test-support --example db_fixture -- 1.0.0
   ```

   It writes `crates/storage/tests/fixtures/1.0.0.db` from the working tree's migrations, since the version has no tag yet. Every release gets one, with or without a new migration, prereleases included: `every_release_has_a_fixture` fails on `dev` once the tag exists without it. A fixture is never overwritten, so if a migration changed after you made this version's, delete the file and run the command again; never touch an earlier release's.

5. Run `bun run rust:test`: the new fixture and every earlier one must open with today's code.
6. Commit and push to `main` (or merge `dev` into it).

Pushing to `main` without changing the version fails the Release workflow, on purpose: that version is already out.

A version with a suffix (`1.0.0-beta.1`) is published as a prerelease and isn't marked _latest_. Prereleases ship no MSI, because WiX only accepts numeric suffixes, and stay out of the package repositories and the update manifest: only people who download them get them.

## Data compatibility

**Every version of Homeport opens the data of every earlier version, back to 0.1.0, and upgrades it without losing anything: Hosts, Stacks, Clusters, settings and secrets.** The website makes the same promise to users ([Your data across versions](https://damienreichhart.github.io/Homeport/docs/reference/updating/#your-data-across-versions)); these rules and tests keep it:

- **Migrations are append-only.** A change to the schema is a new file in `crates/storage/src/db/migrations/`, added at the end of `MIGRATION_SLICE`. A released `.sql` file is never edited, renamed, reordered or removed: a database that already ran it would end up with a different schema than a new one. `RELEASED`, next to the files, lists each released migration with its SHA-256 and the first release that shipped it, and `released_migrations_are_unchanged` (`crates/storage/src/db/mod.rs`) fails when a released file's bytes no longer match, or when a file isn't in `MIGRATION_SLICE`.
- **A migration that reshapes a table copies every column.** Rebuilding a table (create the new one, copy, drop, rename, as `0015_swarm_stacks.sql` does) carries over every column of every row, sealed ones included. Two tests check it against a sample database of every release since 0.1.0 (`crates/storage/tests/fixtures/<version>.db`, built from that release's own migrations with rows of every kind it could store): `every_fixture_migrates_without_losing_a_row` (`crates/storage/tests/fixtures.rs`) counts the rows of each table before and after, and `src-tauri/tests/upgrade.rs` opens each fixture with today's code and reads every Host, Stack (Managed, Linked, Synced), deploy, registry login, repository, Cluster, setting and Engine event back through the stores, unsealing every secret. `every_release_has_a_fixture` compares the fixtures with `git tag`, so no release goes unchecked.
- **Stored shapes stay readable.** What a release wrote in a column (a connection's JSON, a list of compose files, a sealed secret's layout) is read by every later release, or converted by a migration. The upgrade test fails on a shape today's stores no longer read.
- **Secrets are never re-sealed under a new key, cipher or layout without a migration that proves it.** The keychain entry (service `com.homeport.app`, account `secrets-key`) and the sealed format (ADR 0002) stay as they are. A change to either ships with the code that opens the old form and seals the new one, and with fixtures sealed the old way that the upgrade test unseals.
- **Downgrades are refused without touching the data.** Migrations only go forward. `Db::open` reads the schema version from the file's header before opening it for writing, and refuses a database migrated by a newer Homeport with `ErrorCode::DbFromNewerVersion` ("This database was last used by Homeport {version} or later."), naming the oldest version able to open it, which each database records in `database_info` (`0018_database_info.sql`), a table that never changes shape because older versions read it. Nothing is written: not the file, and no `-wal` or `-shm` beside it. The window then shows the message, the database's path and **Open folder** instead of exiting. Tests: `a_database_from_a_newer_homeport_is_refused_untouched`, `a_newer_database_that_names_no_version_is_refused_too`, `a_newer_database_left_open_by_a_crash_is_refused_without_losing_its_log` and `opening_records_the_oldest_homeport_that_can_read_the_file` (`crates/storage/src/db/mod.rs`).
- **Backups follow the same rule.** A backup or a Hosts export made by an older Homeport restores in a newer one; one made by a newer Homeport is refused with `ErrorCode::BundleTooNew` (ADR 0013).

### When a released migration has a bug

Fix it with a new migration, never by editing the released one. The databases that already ran the faulty file keep what it did, so the new migration must leave both kinds in the same state: those that ran the bug, and new ones that run both files in a row. Write it so that it's safe on either (`UPDATE … WHERE` the bad value, `CREATE INDEX IF NOT EXISTS`, a table rebuild), and add a test in `crates/storage/src/db/mod.rs` that starts from the faulty state, as `databases_from_0_10_keep_their_stacks_and_history` and its neighbours do for past releases. The fixtures of the affected releases already hold that state, and stay as they were made.

If the hash test fails because a released file changed by accident (line endings, a reformatted comment), restore the file: `git checkout v<release> -- crates/storage/src/db/migrations/<file>`. Never update the hash in `RELEASED` to make the test pass.

## Platforms

| Runner             | Target                      | Files in the release                                                                 |
| ------------------ | --------------------------- | ------------------------------------------------------------------------------------ |
| `ubuntu-22.04`     | `x86_64-unknown-linux-gnu`  | `Homeport-<version>-linux-x64.deb`, `…-linux-x64.rpm`, `…-linux-x64.AppImage`        |
| `ubuntu-22.04-arm` | `aarch64-unknown-linux-gnu` | `Homeport-<version>-linux-arm64.deb`, `…-linux-arm64.rpm`, `…-linux-arm64.AppImage`  |
| `windows-latest`   | `x86_64-pc-windows-msvc`    | `Homeport-<version>-windows-x64-setup.exe` (NSIS), `…-windows-x64.msi`               |
| `windows-latest`   | `aarch64-pc-windows-msvc`   | `Homeport-<version>-windows-arm64-setup.exe` (NSIS, cross-compiled)                  |
| `macos-latest`     | `aarch64-apple-darwin`      | `Homeport-<version>-macos-arm64.dmg`, `…-macos-arm64-update.app.tar.gz`              |
| `macos-latest`     | `x86_64-apple-darwin`       | `Homeport-<version>-macos-x64.dmg`, `…-macos-x64-update.app.tar.gz` (cross-compiled) |

### File names

Every file in a release is named `Homeport-<version>-<os>-<arch>[-<variant>].<ext>`, with `<os>` one of `windows`, `macos`, `linux` and `<arch>` one of `x64`, `arm64`, so a download says which system and processor it is for in the same words everywhere. Tauri names its bundles in each format's own vocabulary (`Homeport_1.0.0_amd64.deb`, `Homeport-1.0.0-1.x86_64.rpm`, `Homeport_1.0.0_aarch64.dmg`); the **Collect the installers** step of each build copies them under the release names with `bun scripts/release-assets.ts`. It fails the build when a bundle it was asked for is missing, when an updater file has no `.sig`, or when two files would get the same name. The names and those checks are in `scripts/release-assets-plan.ts`.

The in-app updater's files are named so that nobody takes them for an installer: each `.sig` sits next to the file it signs (`<file>.sig`), and on macOS the zipped app the updater installs (it can't apply a `.dmg`) is `Homeport-<version>-macos-<arch>-update.app.tar.gz`, with its `.sig`. The release notes say that neither is a download, and the website never offers them.

The `.deb` and `.rpm` files follow the same pattern, not `dpkg`'s or `rpm`'s (`homeport_1.0.0_amd64.deb`, `homeport-1.0.0-1.x86_64.rpm`). `apt install ./<file>`, `dnf install ./<file>` and `zypper install ./<file>` read a package's name and architecture from inside it, never from the file name, and only the file name changes: the package inside is the one Tauri built. One tool does go by the name: `apt-ftparchive --arch amd64` indexes only the files named `*_amd64.deb`, and silently leaves the others out. So `scripts/repositories.ts` stores each `.deb` in the APT repository's pool as `Homeport_<version>_<amd64|arm64>.deb` (`aptPoolName`), whatever the release calls it. `createrepo_c` indexes every `.rpm` in its folder, so the RPM repository keeps the release names.

Releases up to 0.22 carry Tauri's names. Everything that reads a release reads both: the update manifest and the package repositories (`scripts/repositories-plan.ts`), and the website's download page and changelog (`website/src/core/release/model.ts`). Keep it so: installed copies of every version read the same `updates/latest.json`, and the repositories keep older releases. The Scoop manifest and the AUR package (`scripts/channels-plan.ts`) are only ever rendered for the release being published, so they use the release names alone.

Linux builds on Ubuntu 22.04 so that they run on distributions with glibc 2.35 or later. These are the same targets as `about.toml > targets`. Add a platform to both.

## Signing keys

Two keys sign what users install. Create both once with the wizard, which also sets the secrets and walks through the backup:

```sh
scripts/setup-signing.sh
```

| Key                                                            | Signs                                                 | Public half (committed)                        | Private half (secrets)                                            |
| -------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------- | ----------------------------------------------------------------- |
| Package key (RSA 4096)                                         | The APT and RPM repository metadata, and every `.rpm` | `packaging/linux/homeport.asc`                 | `PACKAGES_GPG_PRIVATE_KEY`, `PACKAGES_GPG_PASSPHRASE`             |
| Updater key ([minisign](https://jedisct1.github.io/minisign/)) | Every installer the in-app updater installs           | `tauri.conf.json > plugins > updater > pubkey` | `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` |

The package key has no expiry date; the wizard saves its revocation certificate with the backup. The `.deb` and `.rpm` ship `homeport.asc`, and the website shows its fingerprint, computed from the committed file.

**Keep the offline backup.** Without the private keys, no release can be signed. Replacing a key has a cost:

- **Updater key**: every installed copy only accepts updates signed with the key built into it. After a new key, existing copies stop updating until users install a new version by hand. Avoid it unless the key leaked.
- **Package key**: the `.deb` refreshes `/etc/apt/keyrings/homeport.asc` on upgrade, but that upgrade must first verify against the old key. Publish one release signed with the old key that ships the new one, then switch. `dnf` and `zypper` ask users to trust the new key.

If the package key leaks, publish its revocation certificate (`gpg --import` it, then export the key again) and replace it.

## Package repositories and updates

The website's GitHub Pages site also serves, at a permanent URL ([ADR 0003](adr/0003-package-repositories-on-github-pages.md)):

| Path                   | What                                                                                      |
| ---------------------- | ----------------------------------------------------------------------------------------- |
| `apt/`                 | APT repository, suite `stable`, component `main`, `amd64` and `arm64`; `InRelease` signed |
| `apt/homeport.sources` | The APT source (deb822) that the install script, the manual setup and the `.deb` add      |
| `rpm/`                 | RPM repository for `dnf` and `zypper`; `repodata/repomd.xml.asc` signed                   |
| `rpm/homeport.repo`    | The repository file that the install script, the manual setup and the `.rpm` add          |
| `homeport.asc`         | The package key                                                                           |
| `install.sh`           | The quick install script (`packaging/linux/install.sh`)                                   |
| `updates/latest.json`  | The in-app updater's manifest                                                             |

The Pages workflow builds them on every deploy with `bun scripts/repositories.ts --out website/dist`: it downloads the `.deb` and `.rpm` files of the latest 5 stable releases from GitHub Releases (from 0.3.0, the first built for them), indexes them with `apt-ftparchive` and `createrepo_c`, signs the metadata with the package key, and writes `updates/latest.json` from the latest release's signed installers. It takes the packages by their extension and the updater files under [either naming](#file-names), so a repository can hold releases from before and after the rename. GitHub Releases stay the only source: nothing is committed. Pull requests build everything with a throwaway key.

**The packages register the repository.** On their first install (or first upgrade from a version without this), the `.deb` and `.rpm` add `homeport.sources` or `homeport.repo` unless it's already there, so a downloaded package still gets updates. A marker in `/var/lib/homeport/` keeps a removed repository from coming back. `apt purge` and `dnf`/`zypper remove` remove it. The scripts are in `packaging/linux/`.

**The in-app updater** reads `updates/latest.json` at launch and every 6 hours (Settings can turn that off). Windows, macOS and AppImage installs download the installer from GitHub Releases, check its signature and the version it was signed for, install it and restart. `.deb` and `.rpm` installs only show the package manager command: `apt`, `dnf` and `zypper` own those files.

To try the repositories locally (needs `apt-ftparchive`, `createrepo_c` and the package key's secret half in gpg):

```sh
bun scripts/repositories.ts --out /tmp/site --packages target/release/bundle/deb
```

`--packages` treats the files in a folder as the only release, instead of downloading releases. `HOMEPORT_REPO_URL=http://… sh install.sh` points the install script at such a copy.

## Scoop and the AUR

**Deferred until after 1.0** ([#7](https://github.com/DamienReichhart/Homeport/issues/7), [#8](https://github.com/DamienReichhart/Homeport/issues/8), milestone _Later_). Everything below is in place but dormant: no secret is set, so each release skips both channels with a notice, and the website lists them under **Coming later**. Shipping either is only the one-time setup below.

After a stable release is published, the **channels** job of the Release workflow renders, from the release's `SHA256SUMS.txt`, the Scoop manifest (`scoop/homeport.json`, both NSIS installers) and the AUR package (`aur/PKGBUILD` and `aur/.SRCINFO`, `homeport-bin` repackaging both `.deb` files) with `bun scripts/channels.ts`, and keeps them as the run's `channels` artifact. Then it updates each channel whose secret is set, and skips the others with a notice. Prereleases skip the job.

| Channel | Repository                                                  | Secret                                                                                          |
| ------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| Scoop   | `DamienReichhart/scoop-bucket`, file `bucket/homeport.json` | `SCOOP_BUCKET_TOKEN`: a fine-grained token with **Contents: read and write** on that repository |
| AUR     | `ssh://aur@aur.archlinux.org/homeport-bin.git`              | `AUR_SSH_PRIVATE_KEY`: a private key whose public half is on the AUR account                    |

Setting them up, once:

1. **Scoop**: create the public repository `DamienReichhart/scoop-bucket` with an empty `bucket/` folder (a README saying `scoop bucket add homeport https://github.com/DamienReichhart/scoop-bucket` helps), create the token, and add it to this repository's Actions secrets.
2. **AUR**: create an account on aur.archlinux.org, add an SSH public key to it, and add the private key as `AUR_SSH_PRIVATE_KEY`. The first push creates `homeport-bin`.
3. Re-run the last release's **channels** job (or wait for the next release), then check `scoop install homeport/homeport` on Windows and `yay -S homeport-bin` on Arch.
4. Move Scoop and the AUR out of `COMING_LATER` in `website/src/core/packages/repositories.ts` and document them on the installation page.

Homeport knows these installs: on a Scoop install (an NSIS installer under `scoop\apps`) and on Arch (the `.deb` repackaged), the in-app updater shows `scoop update homeport` or `yay -Syu homeport-bin` instead of installing updates itself (`crates/updates`).

To render the files locally from a published release:

```sh
gh release download v0.7.0 --pattern SHA256SUMS.txt --dir /tmp
bun scripts/channels.ts --version 0.7.0 --sums /tmp/SHA256SUMS.txt --out /tmp/channels
```

## Code signing

### macOS

Without a certificate the app is ad-hoc signed (`bundle.macOS.signingIdentity: "-"`). It runs, but Gatekeeper blocks a downloaded copy until the user allows it in **System Settings → Privacy & Security** (or runs `xattr -dr com.apple.quarantine /Applications/Homeport.app`).

To sign and notarize, add these repository secrets (**Settings → Secrets and variables → Actions**). The build uses them once they are set:

| Secret                       | Value                                                                                              |
| ---------------------------- | -------------------------------------------------------------------------------------------------- |
| `APPLE_CERTIFICATE`          | A _Developer ID Application_ certificate exported as `.p12`, base64-encoded (`base64 -i cert.p12`) |
| `APPLE_CERTIFICATE_PASSWORD` | The `.p12` export password                                                                         |
| `APPLE_SIGNING_IDENTITY`     | e.g. `Developer ID Application: Damien Reichhart (TEAMID)`                                         |
| `APPLE_ID`                   | The Apple ID email used for notarization                                                           |
| `APPLE_PASSWORD`             | An [app-specific password](https://support.apple.com/102654) for that Apple ID                     |
| `APPLE_TEAM_ID`              | The 10-character team ID                                                                           |

### Windows

The installers are unsigned, so SmartScreen warns on first launch ("More info → Run anyway"). Signing needs a certificate (Azure Trusted Signing, or an OV/EV code-signing certificate). Configure it through `bundle.windows.signCommand` as described in [Tauri's Windows signing guide](https://tauri.app/distribute/sign/windows/), and pass its credentials to the **Build** step like the Apple secrets.

### Linux

The repositories and every `.rpm` are signed with the [package key](#signing-keys), which `apt`, `dnf` and `zypper` check. A downloaded `.deb` isn't signed itself; users can check it against `SHA256SUMS.txt` or with `gh attestation verify <file> --repo DamienReichhart/Homeport`.
