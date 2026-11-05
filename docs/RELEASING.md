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
3. Commit and push to `main` (or merge `dev` into it).

Pushing to `main` without changing the version fails the Release workflow, on purpose: that version is already out.

A version with a suffix (`1.0.0-beta.1`) is published as a prerelease and isn't marked _latest_. Prereleases ship no MSI, because WiX only accepts numeric suffixes, and stay out of the package repositories and the update manifest: only people who download them get them.

## Platforms

| Runner             | Target                      | Installers                         |
| ------------------ | --------------------------- | ---------------------------------- |
| `ubuntu-22.04`     | `x86_64-unknown-linux-gnu`  | `.deb`, `.rpm`, `.AppImage`        |
| `ubuntu-22.04-arm` | `aarch64-unknown-linux-gnu` | `.deb`, `.rpm`, `.AppImage`        |
| `windows-latest`   | `x86_64-pc-windows-msvc`    | NSIS `-setup.exe`, `.msi`          |
| `windows-latest`   | `aarch64-pc-windows-msvc`   | NSIS `-setup.exe` (cross-compiled) |
| `macos-latest`     | `aarch64-apple-darwin`      | `.dmg`                             |
| `macos-latest`     | `x86_64-apple-darwin`       | `.dmg` (cross-compiled)            |

Each build also uploads the in-app updater's files: a `.sig` next to each installer, and on macOS `Homeport_<version>_<arch>.app.tar.gz` (the updater installs the zipped app, not the `.dmg`) with its `.sig`.

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

The Pages workflow builds them on every deploy with `bun scripts/repositories.ts --out website/dist`: it downloads the `.deb` and `.rpm` files of the latest 5 stable releases from GitHub Releases (from 0.3.0, the first built for them), indexes them with `apt-ftparchive` and `createrepo_c`, signs the metadata with the package key, and writes `updates/latest.json` from the latest release's signed installers. GitHub Releases stay the only source: nothing is committed. Pull requests build everything with a throwaway key.

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
