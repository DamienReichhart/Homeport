/**
 * Decides what the package repositories and the update manifest on the website
 * contain, from Homeport's GitHub releases (ADR 0003). Pure functions only;
 * `repositories.ts` downloads the files and builds the repositories from them.
 */

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

/** The `.deb` and `.rpm` files of a release. */
export function packagesOf(release: Release): { deb: Asset[]; rpm: Asset[] } {
  return {
    deb: release.assets.filter((a) => a.name.endsWith('.deb')),
    rpm: release.assets.filter((a) => a.name.endsWith('.rpm')),
  };
}

/**
 * The updater targets (`{os}-{arch}` and `{os}-{arch}-{installer}`) a release
 * file serves. `{os}-{arch}` is the fallback for any other install type: on
 * Linux that's the AppImage, which a .deb or .rpm install only uses to learn
 * that a new version exists, never to install it.
 */
export function updaterTargets(file: string): string[] | undefined {
  const rules: [RegExp, string, string][] = [
    [/_amd64\.AppImage$/, 'linux-x86_64', 'appimage'],
    [/_aarch64\.AppImage$/, 'linux-aarch64', 'appimage'],
    [/_x64-setup\.exe$/, 'windows-x86_64', 'nsis'],
    [/_arm64-setup\.exe$/, 'windows-aarch64', 'nsis'],
    [/_aarch64\.app\.tar\.gz$/, 'darwin-aarch64', 'app'],
    [/_x64\.app\.tar\.gz$/, 'darwin-x86_64', 'app'],
  ];
  for (const [pattern, target, installer] of rules) {
    if (pattern.test(file)) return [target, `${target}-${installer}`];
  }
  if (/_x64_[\w-]+\.msi$/.test(file)) return ['windows-x86_64-msi'];
  return undefined;
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
