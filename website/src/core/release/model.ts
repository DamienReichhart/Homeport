/** A release of Homeport and the installers it ships. */
export type Os = 'windows' | 'macos' | 'linux';
export type Arch = 'x64' | 'arm64';
export type Format = 'nsis' | 'msi' | 'dmg' | 'deb' | 'rpm' | 'appimage';

export interface Installer {
  name: string;
  url: string;
  size: number;
  os: Os;
  arch: Arch;
  format: Format;
  sha256?: string;
}

export interface Release {
  tag: string;
  version: string;
  name: string;
  url: string;
  publishedAt: string;
  prerelease: boolean;
  body: string;
  installers: Installer[];
  checksumsUrl?: string;
}

type Kind = Pick<Installer, 'os' | 'arch' | 'format'>;

/** What follows `Homeport-<version>-<os>-<arch>` in an installer's name, and the system it installs on. */
const NAMED: [suffix: string, os: Os, format: Format][] = [
  ['-setup.exe', 'windows', 'nsis'],
  ['.msi', 'windows', 'msi'],
  ['.dmg', 'macos', 'dmg'],
  ['.deb', 'linux', 'deb'],
  ['.rpm', 'linux', 'rpm'],
  ['.AppImage', 'linux', 'appimage'],
];
const NAME = /^Homeport-.+-(windows|macos|linux)-(x64|arm64)(-setup\.exe|\.[A-Za-z]+)$/;

/**
 * Recognises an installer by its file name. Releases name their files by
 * system and processor, `Homeport-1.0.0-macos-arm64.dmg`; those up to 0.22,
 * which the changelog still lists, carry Tauri's names, in each format's own
 * words: `Homeport_0.8.0_aarch64.dmg`, `Homeport_0.8.0_amd64.deb`,
 * `Homeport-0.8.0-1.x86_64.rpm`. `undefined` for anything that isn't a
 * download: checksums, and the in-app updater's files (`.sig` signatures and
 * the zipped macOS app, `…-update.app.tar.gz` or `….app.tar.gz`).
 * `scripts/release-assets-plan.ts` gives the files these names.
 */
export function classify(name: string): Kind | undefined {
  const named = NAME.exec(name);
  if (named) {
    const kind = NAMED.find(([suffix, os]) => suffix === named[3] && os === named[1]);
    return kind && { os: kind[1], arch: named[2] as Arch, format: kind[2] };
  }
  const arch: Arch = /aarch64|arm64/i.test(name) ? 'arm64' : 'x64';
  if (/-setup\.exe$/i.test(name)) return { os: 'windows', arch, format: 'nsis' };
  if (/\.msi$/i.test(name)) return { os: 'windows', arch, format: 'msi' };
  if (/\.dmg$/i.test(name)) return { os: 'macos', arch, format: 'dmg' };
  if (/\.deb$/i.test(name)) return { os: 'linux', arch, format: 'deb' };
  if (/\.rpm$/i.test(name)) return { os: 'linux', arch, format: 'rpm' };
  if (/\.AppImage$/i.test(name)) return { os: 'linux', arch, format: 'appimage' };
  return undefined;
}

/** Parses `sha256sum` output: `<hash>  <file>` per line. */
export function parseChecksums(text: string): Map<string, string> {
  const sums = new Map<string, string>();
  for (const line of text.split('\n')) {
    const match = /^([0-9a-f]{64})\s+\*?(.+)$/i.exec(line.trim());
    if (match?.[1] && match[2]) sums.set(match[2], match[1].toLowerCase());
  }
  return sums;
}
