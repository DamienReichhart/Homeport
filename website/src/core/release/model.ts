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

/** Recognises a Tauri bundle file name, e.g. `Homeport_0.2.0_aarch64.dmg`. */
export function classify(name: string): Pick<Installer, 'os' | 'arch' | 'format'> | undefined {
  const arm = /(aarch64|arm64)/i.test(name);
  const arch: Arch = arm ? 'arm64' : 'x64';
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
