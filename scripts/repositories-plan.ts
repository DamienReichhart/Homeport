/**
 * Decides what the package repositories and the update manifest on the website
 * contain, from Homeport's GitHub releases (ADR 0003). Pure functions only;
 * `repositories.ts` downloads the files and builds the repositories from them.
 */
import { describeAsset } from './release-assets-plan';

/** How many stable releases the APT and RPM repositories keep. */
export const KEEP_RELEASES = 5;

/** The first release built for the repositories: signed RPMs, updater files, a package that registers the repository. */
export const FIRST_REPOSITORY_VERSION = '0.3.0';

export interface Asset {
  name: string;
  url: string;
}

export interface Release {
  tag: string;
  version: string;
  publishedAt: string;
  body: string;
  assets: Asset[];
}

/** The fields of a GitHub API release that the repositories need. */
export interface GitHubRelease {
  tag_name: string;
  body: string | null;
  draft: boolean;
  prerelease: boolean;
  published_at: string | null;
  assets: { name: string; browser_download_url: string }[];
}

const STABLE = /^v?(\d+)\.(\d+)\.(\d+)$/;

function parts(version: string): [number, number, number] | undefined {
  const match = STABLE.exec(version);
  return match ? [Number(match[1]), Number(match[2]), Number(match[3])] : undefined;
}

/** Orders two stable `x.y.z` versions. */
export function compareVersions(a: string, b: string): number {
  const [pa, pb] = [parts(a), parts(b)];
  if (!pa || !pb) throw new Error(`Not a stable version: ${pa ? b : a}`);
  return pa[0] - pb[0] || pa[1] - pb[1] || pa[2] - pb[2];
}

/**
 * The releases the repositories carry: published, stable, built for the
 * repositories, newest first, at most `keep` of them.
 */
export function selectReleases(
  releases: readonly GitHubRelease[],
  { keep = KEEP_RELEASES, since = FIRST_REPOSITORY_VERSION } = {},
): Release[] {
  return releases
    .filter((r) => !r.draft && !r.prerelease && parts(r.tag_name))
    .filter((r) => compareVersions(r.tag_name, since) >= 0)
    .sort((a, b) => compareVersions(b.tag_name, a.tag_name))
    .slice(0, keep)
    .map((r) => ({
      tag: r.tag_name,
      version: r.tag_name.replace(/^v/, ''),
      publishedAt: r.published_at ?? '',
      body: r.body ?? '',
      assets: r.assets.map((a) => ({ name: a.name, url: a.browser_download_url })),
    }));
}

/**
 * The `.deb` and `.rpm` files of a release, by extension alone: the
 * repositories keep releases from before and after their files were renamed.
 */
export function packagesOf(release: Release): { deb: Asset[]; rpm: Asset[] } {
  return {
    deb: release.assets.filter((a) => a.name.endsWith('.deb')),
    rpm: release.assets.filter((a) => a.name.endsWith('.rpm')),
  };
}

const DEB_ARCH = { x64: 'amd64', arm64: 'arm64' } as const;

/**
 * The name a release's `.deb` takes in the APT repository's pool:
 * `Homeport_<version>_<amd64|arm64>.deb`, whichever way the release names it.
 * `apt-ftparchive --arch <arch>` indexes only the files named `*_<arch>.deb`,
 * so a `.deb` kept under its release name (`Homeport-1.0.0-linux-x64.deb`)
 * would be left out of the repository without an error.
 */
export function aptPoolName(release: Release, asset: Asset): string {
  const kind = describeAsset(asset.name);
  return kind?.kind === 'deb'
    ? `Homeport_${release.version}_${DEB_ARCH[kind.arch]}.deb`
    : asset.name;
}

const UPDATER_OS = { windows: 'windows', macos: 'darwin', linux: 'linux' } as const;
const UPDATER_ARCH = { x64: 'x86_64', arm64: 'aarch64' } as const;

/**
 * The updater targets (`{os}-{arch}` and `{os}-{arch}-{installer}`) a release
 * file serves, whichever way the release names its files (`describeAsset`):
 * installed copies of every version read this manifest. `{os}-{arch}` is the
 * fallback for any other install type: on Linux that's the AppImage, which a
 * .deb or .rpm install only uses to learn that a new version exists, never to
 * install it.
 */
export function updaterTargets(file: string): string[] | undefined {
  const asset = describeAsset(file);
  if (!asset) return undefined;
  const target = `${UPDATER_OS[asset.os]}-${UPDATER_ARCH[asset.arch]}`;
  switch (asset.kind) {
    case 'appimage':
    case 'nsis':
    case 'app':
      return [target, `${target}-${asset.kind}`];
    case 'msi':
      return [`${target}-msi`];
    default:
      return undefined; // the package managers' files and the .dmg, which the updater never installs
  }
}

export interface UpdateManifest {
  version: string;
  notes: string;
  pub_date: string;
  platforms: Record<string, { url: string; signature: string }>;
}

/**
 * The "What's changed" part of the release notes, without the download table
 * that follows it (see `releaseNotes` in `release-plan.ts`).
 */
export function updateNotes(body: string): string {
  const start = body.indexOf("## What's changed");
  const text = start >= 0 ? body.slice(start + "## What's changed".length) : body;
  const end = text.indexOf('\n## ');
  return (end >= 0 ? text.slice(0, end) : text).trim();
}

/**
 * The Tauri updater manifest (`updates/latest.json`) for `release`.
 * `signatures` maps a file name to the contents of its `.sig` file. Files
 * without one are left out; `undefined` when no file has one.
 */
export function updateManifest(
  release: Release,
  signatures: ReadonlyMap<string, string>,
): UpdateManifest | undefined {
  const platforms: UpdateManifest['platforms'] = {};
  for (const asset of release.assets) {
    const signature = signatures.get(asset.name)?.trim();
    const targets = updaterTargets(asset.name);
    if (!signature || !targets) continue;
    for (const target of targets) platforms[target] = { url: asset.url, signature };
  }
  if (!Object.keys(platforms).length) return undefined;
  return {
    version: release.version,
    notes: updateNotes(release.body),
    pub_date: release.publishedAt,
    platforms: Object.fromEntries(Object.entries(platforms).sort(([a], [b]) => a.localeCompare(b))),
  };
}
