/**
 * Reads Homeport's releases from the GitHub API at build time, so the download
 * and changelog pages always list the files of the latest published release.
 *
 * The Pages workflow rebuilds the site after every release. Set GITHUB_TOKEN to
 * avoid the anonymous rate limit, and RELEASES_REQUIRED=1 to fail the build
 * (rather than deploy a page without download links) when the API can't be read.
 */
import { REPO } from '@/core/site';

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

interface GitHubAsset {
  name: string;
  size: number;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  name: string | null;
  html_url: string;
  published_at: string | null;
  prerelease: boolean;
  draft: boolean;
  body: string | null;
  assets: GitHubAsset[];
}

/** How each installer format is named by Tauri's bundler, and what it's for. */
export const FORMATS: Record<Format, { label: string; ext: string; hint: string }> = {
  nsis: { label: 'Installer', ext: '.exe', hint: 'Recommended. Installs for the current user.' },
  msi: { label: 'MSI package', ext: '.msi', hint: 'For managed and silent installs.' },
  dmg: { label: 'Disk image', ext: '.dmg', hint: 'Drag Homeport into Applications.' },
  deb: { label: 'Debian package', ext: '.deb', hint: 'Debian, Ubuntu, Linux Mint, Pop!_OS.' },
  rpm: { label: 'RPM package', ext: '.rpm', hint: 'Fedora, RHEL, Rocky, openSUSE.' },
  appimage: { label: 'AppImage', ext: '.AppImage', hint: 'Any distribution, no install step.' },
};

export const OS_LABELS: Record<Os, string> = {
  windows: 'Windows',
  macos: 'macOS',
  linux: 'Linux',
};

export function archLabel(os: Os, arch: Arch): string {
  if (os === 'macos') return arch === 'arm64' ? 'Apple silicon' : 'Intel';
  return arch === 'arm64' ? 'ARM64' : 'x64';
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

const ORDER: Format[] = ['nsis', 'msi', 'dmg', 'deb', 'rpm', 'appimage'];

function toRelease(raw: GitHubRelease): Release {
  const installers: Installer[] = [];
  for (const asset of raw.assets) {
    const kind = classify(asset.name);
    if (kind) {
      installers.push({
        name: asset.name,
        url: asset.browser_download_url,
        size: asset.size,
        ...kind,
      });
    }
  }
  installers.sort(
    (a, b) => ORDER.indexOf(a.format) - ORDER.indexOf(b.format) || a.arch.localeCompare(b.arch),
  );
  return {
    tag: raw.tag_name,
    version: raw.tag_name.replace(/^v/, ''),
    name: raw.name || raw.tag_name,
    url: raw.html_url,
    publishedAt: raw.published_at ?? '',
    prerelease: raw.prerelease,
    body: raw.body ?? '',
    installers,
    checksumsUrl: raw.assets.find((a) => a.name === 'SHA256SUMS.txt')?.browser_download_url,
  };
}

async function request(url: string): Promise<Response> {
  const headers: Record<string, string> = {
    accept: 'application/vnd.github+json',
    'user-agent': 'homeport-website-build',
  };
  const token = process.env.GITHUB_TOKEN;
  if (token && url.startsWith('https://api.github.com/')) headers.authorization = `Bearer ${token}`;
  const response = await fetch(url, { headers });
  if (!response.ok) throw new Error(`GET ${url}: ${response.status} ${response.statusText}`);
  return response;
}

async function load(): Promise<Release[]> {
  try {
    const response = await request(`https://api.github.com/repos/${REPO}/releases?per_page=50`);
    const raw = (await response.json()) as GitHubRelease[];
    const releases = raw.filter((r) => !r.draft).map(toRelease);

    // Attach each installer's checksum from the latest release's SHA256SUMS.txt.
    const latest = pickLatest(releases);
    if (latest?.checksumsUrl) {
      const sums = parseChecksums(await (await request(latest.checksumsUrl)).text());
      for (const installer of latest.installers) installer.sha256 = sums.get(installer.name);
    }
    return releases;
  } catch (error) {
    if (process.env.RELEASES_REQUIRED) throw error;
    console.warn(`[releases] Could not read releases, building without them: ${String(error)}`);
    return [];
  }
}

function pickLatest(releases: Release[]): Release | undefined {
  return releases.find((r) => !r.prerelease && r.installers.length > 0);
}

let cache: Promise<Release[]> | undefined;

/** Every published release, newest first. Fetched once per build. */
export function getReleases(): Promise<Release[]> {
  cache ??= load();
  return cache;
}

/** The newest stable release that has installers. */
export async function getLatestRelease(): Promise<Release | undefined> {
  return pickLatest(await getReleases());
}

export function formatSize(bytes: number): string {
  const mb = bytes / 1024 / 1024;
  return `${mb >= 10 ? Math.round(mb) : mb.toFixed(1)} MB`;
}

export function formatDate(iso: string): string {
  if (!iso) return '';
  return new Intl.DateTimeFormat('en-GB', { dateStyle: 'long', timeZone: 'UTC' }).format(
    new Date(iso),
  );
}
