/**
 * The files winget, the Scoop bucket and the AUR need for a release (issues
 * #6, #7 and #8): winget's three manifests for the NSIS and MSI installers, a
 * Scoop manifest for the NSIS installers, and a PKGBUILD with its .SRCINFO
 * that repackages the `.deb`. Pure functions only; `channels.ts` reads the
 * release's checksums and writes them.
 */

export const REPO = 'DamienReichhart/Homeport';
const HOMEPAGE = 'https://damienreichhart.github.io/Homeport/';
const DESCRIPTION = 'Manage Docker Engines anywhere, local or remote over SSH, TLS or TCP';
const DOWNLOAD = `https://github.com/${REPO}/releases/download`;

/** The release files each channel downloads, by Tauri's names. */
export const files = (version: string) => ({
  windowsX64: `Homeport_${version}_x64-setup.exe`,
  windowsArm64: `Homeport_${version}_arm64-setup.exe`,
  msiX64: `Homeport_${version}_x64_en-US.msi`,
  debAmd64: `Homeport_${version}_amd64.deb`,
  debArm64: `Homeport_${version}_arm64.deb`,
});

export class ChannelError extends Error {}

/** Reads `SHA256SUMS.txt` (`<hash>  <file>` lines) into a map from file to hash. */
export function parseSums(text: string): Map<string, string> {
  const sums = new Map<string, string>();
  for (const line of text.split('\n')) {
    const match = /^([0-9a-f]{64})\s+\*?(.+)$/.exec(line.trim());
    if (match) sums.set(match[2]!.trim(), match[1]!);
  }
  return sums;
}

function hashOf(sums: Map<string, string>, file: string): string {
  const hash = sums.get(file);
  if (!hash) throw new ChannelError(`SHA256SUMS.txt has no ${file}`);
  return hash;
}

function checkVersion(version: string) {
  if (!/^\d+\.\d+\.\d+$/.test(version))
    throw new ChannelError(
      `${version} isn't a stable version; prereleases skip winget, Scoop and the AUR`,
    );
}

/** winget's package identifier: `winget install DamienReichhart.Homeport`. */
export const WINGET_ID = 'DamienReichhart.Homeport';
const WINGET_SCHEMA = '1.9.0';

/**
 * The three manifests a `microsoft/winget-pkgs` pull request adds under
 * `manifests/d/DamienReichhart/Homeport/<version>/`: the version, the
 * installers (NSIS per user on x64 and ARM64, the MSI per machine on x64)
 * and the English description.
 */
export function wingetManifests(
  version: string,
  sums: Map<string, string>,
): { version: string; installer: string; locale: string } {
  checkVersion(version);
  const f = files(version);
  const url = (file: string) => `${DOWNLOAD}/v${version}/${file}`;
  const header = (type: string) =>
    `# yaml-language-server: $schema=https://aka.ms/winget-manifest.${type}.${WINGET_SCHEMA}.schema.json\n`;
  const footer = (type: string) => `ManifestType: ${type}\nManifestVersion: ${WINGET_SCHEMA}\n`;
  const installer = (arch: string, type: string, scope: string, file: string) =>
    [
      `- Architecture: ${arch}`,
      `  InstallerType: ${type}`,
      `  Scope: ${scope}`,
      `  InstallerUrl: ${url(file)}`,
      `  InstallerSha256: ${hashOf(sums, file).toUpperCase()}`,
    ].join('\n');
  return {
    version: `${header('version')}PackageIdentifier: ${WINGET_ID}\nPackageVersion: ${version}\nDefaultLocale: en-US\n${footer('version')}`,
    installer: [
      header('installer') + `PackageIdentifier: ${WINGET_ID}`,
      `PackageVersion: ${version}`,
      'UpgradeBehavior: install',
      'ReleaseDate: ' + new Date().toISOString().slice(0, 10),
      'Installers:',
      installer('x64', 'nullsoft', 'user', f.windowsX64),
      installer('arm64', 'nullsoft', 'user', f.windowsArm64),
      installer('x64', 'wix', 'machine', f.msiX64),
      footer('installer'),
    ].join('\n'),
    locale: [
      header('defaultLocale') + `PackageIdentifier: ${WINGET_ID}`,
      `PackageVersion: ${version}`,
      'PackageLocale: en-US',
      'Publisher: Damien Reichhart',
      `PublisherUrl: https://github.com/DamienReichhart`,
      `PackageName: Homeport`,
      `PackageUrl: ${HOMEPAGE}`,
      'License: MIT',
      `LicenseUrl: https://github.com/${REPO}/blob/main/LICENSE`,
      `ShortDescription: ${DESCRIPTION}`,
      `ReleaseNotesUrl: https://github.com/${REPO}/releases/tag/v${version}`,
      'Tags:',
      '- docker',
      '- containers',
      '- podman',
      '- ssh',
      footer('defaultLocale'),
    ].join('\n'),
  };
}

/**
 * The bucket's `homeport.json`. Scoop runs the NSIS installer silently into
 * its own folder (`/D` must come last), and `checkver`/`autoupdate` let the
 * bucket's own tooling follow new releases too.
 */
export function scoopManifest(version: string, sums: Map<string, string>): string {
  checkVersion(version);
  const f = files(version);
  const manifest = {
    version,
    description: DESCRIPTION,
    homepage: HOMEPAGE,
    license: 'MIT',
    notes: 'Homeport leaves its updates to Scoop: run `scoop update homeport`.',
    architecture: {
      '64bit': {
        url: `${DOWNLOAD}/v${version}/${f.windowsX64}`,
        hash: hashOf(sums, f.windowsX64),
      },
      arm64: {
        url: `${DOWNLOAD}/v${version}/${f.windowsArm64}`,
        hash: hashOf(sums, f.windowsArm64),
      },
    },
    installer: { args: ['/S', '/D=$dir'] },
    uninstaller: { file: 'uninstall.exe', args: ['/S'] },
    checkver: { github: `https://github.com/${REPO}` },
    autoupdate: {
      architecture: {
        '64bit': { url: `${DOWNLOAD}/v$version/Homeport_$version_x64-setup.exe` },
        arm64: { url: `${DOWNLOAD}/v$version/Homeport_$version_arm64-setup.exe` },
      },
      hash: { url: `${DOWNLOAD}/v$version/SHA256SUMS.txt` },
    },
  };
  return `${JSON.stringify(manifest, null, 2)}\n`;
}

/** The AUR's `homeport-bin`, built from the release's `.deb` files. */
export interface Pkgbuild {
  pkgbuild: string;
  srcinfo: string;
}

const DEPENDS = ['webkit2gtk-4.1', 'gtk3'];

export function aurPackage(version: string, sums: Map<string, string>, pkgrel = 1): Pkgbuild {
  checkVersion(version);
  const f = files(version);
  const amd64 = hashOf(sums, f.debAmd64);
  const arm64 = hashOf(sums, f.debArm64);
  const pkgbuild = `# Maintainer: Damien Reichhart
# Generated by scripts/channels.ts from the v${version} release; don't edit it here.
pkgname=homeport-bin
pkgver=${version}
pkgrel=${pkgrel}
pkgdesc='${DESCRIPTION}'
arch=('x86_64' 'aarch64')
url='${HOMEPAGE}'
license=('MIT')
depends=(${DEPENDS.map((d) => `'${d}'`).join(' ')})
provides=('homeport')
conflicts=('homeport')
source_x86_64=("${DOWNLOAD}/v\${pkgver}/Homeport_\${pkgver}_amd64.deb")
source_aarch64=("${DOWNLOAD}/v\${pkgver}/Homeport_\${pkgver}_arm64.deb")
sha256sums_x86_64=('${amd64}')
sha256sums_aarch64=('${arm64}')
noextract=("Homeport_\${pkgver}_amd64.deb" "Homeport_\${pkgver}_arm64.deb")

package() {
  local deb
  case "$CARCH" in
    x86_64) deb="Homeport_\${pkgver}_amd64.deb" ;;
    aarch64) deb="Homeport_\${pkgver}_arm64.deb" ;;
  esac
  bsdtar -xOf "$deb" 'data.tar.*' | bsdtar -xf - -C "$pkgdir"
  # The APT repository files only serve the .deb's own updates.
  rm -rf "$pkgdir/usr/share/homeport"
  install -Dm644 "$pkgdir/usr/lib/Homeport/licenses/LICENSE" \\
    "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
`;
  const srcinfo = `pkgbase = homeport-bin
\tpkgdesc = ${DESCRIPTION}
\tpkgver = ${version}
\tpkgrel = ${pkgrel}
\turl = ${HOMEPAGE}
\tarch = x86_64
\tarch = aarch64
\tlicense = MIT
${DEPENDS.map((d) => `\tdepends = ${d}`).join('\n')}
\tprovides = homeport
\tconflicts = homeport
\tnoextract = ${f.debAmd64}
\tnoextract = ${f.debArm64}
\tsource_x86_64 = ${DOWNLOAD}/v${version}/${f.debAmd64}
\tsha256sums_x86_64 = ${amd64}
\tsource_aarch64 = ${DOWNLOAD}/v${version}/${f.debArm64}
\tsha256sums_aarch64 = ${arm64}

pkgname = homeport-bin
`;
  return { pkgbuild, srcinfo };
}
