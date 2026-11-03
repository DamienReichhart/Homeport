# Releasing

Every push to `main` runs the **Release** workflow (`.github/workflows/release.yml`) and releases the version set in the repo:

1. **Prepare**: `bun scripts/release.ts plan` reads the version from `src-tauri/tauri.conf.json`, `package.json`, `Cargo.toml` (`[workspace.package]`) and `Cargo.lock`. The run **fails** if they disagree, or if that version already has a `v*` tag. Otherwise it writes the release notes from the [Conventional Commits](https://www.conventionalcommits.org) since the previous release.
2. **CI and Build**, in parallel: the whole of `ci.yml` (frontend checks, licenses, Rust fmt/clippy/tests), and a native build for each platform.
3. **Publish**, once CI and every build passed: records a [build provenance attestation](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations) for each installer, then creates the release with every installer and `SHA256SUMS.txt`. That creates the tag on the commit that was built.

Once the release is out, the **Pages** workflow rebuilds the [website](https://damienreichhart.github.io/Homeport/) so its download page and changelog list the new version.

Nothing is published unless every job succeeds. To retry, re-run the failed jobs, run the workflow from the Actions tab, or push a fix.

## Releasing a new version

1. Set the new version in `src-tauri/tauri.conf.json`, `package.json` and `Cargo.toml` (`[workspace.package] version`).
2. Run `cargo check` so `Cargo.lock` picks it up.
3. Commit and push to `main` (or merge `dev` into it).

Pushing to `main` without changing the version fails the Release workflow, on purpose: that version is already out.

A version with a suffix (`1.0.0-beta.1`) is published as a prerelease and isn't marked _latest_. Prereleases ship no MSI, because WiX only accepts numeric suffixes.

## Platforms

| Runner             | Target                      | Installers                         |
| ------------------ | --------------------------- | ---------------------------------- |
| `ubuntu-22.04`     | `x86_64-unknown-linux-gnu`  | `.deb`, `.rpm`, `.AppImage`        |
| `ubuntu-22.04-arm` | `aarch64-unknown-linux-gnu` | `.deb`, `.rpm`, `.AppImage`        |
| `windows-latest`   | `x86_64-pc-windows-msvc`    | NSIS `-setup.exe`, `.msi`          |
| `windows-latest`   | `aarch64-pc-windows-msvc`   | NSIS `-setup.exe` (cross-compiled) |
| `macos-latest`     | `aarch64-apple-darwin`      | `.dmg`                             |
| `macos-latest`     | `x86_64-apple-darwin`       | `.dmg` (cross-compiled)            |

Linux builds on Ubuntu 22.04 so that they run on distributions with glibc 2.35 or later. These are the same targets as `about.toml > targets`. Add a platform to both.

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

Packages are unsigned. Users can check them against `SHA256SUMS.txt` or with `gh attestation verify <file> --repo DamienReichhart/Homeport`.
