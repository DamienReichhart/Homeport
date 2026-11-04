/**
 * Reads Homeport's releases from the GitHub API at build time, so the download
 * and changelog pages always list the files of the latest published release.
 *
 * The Pages workflow rebuilds the site after every release. Set GITHUB_TOKEN to
 * avoid the anonymous rate limit, and RELEASES_REQUIRED=1 to fail the build
 * (rather than deploy a page without download links) when the API can't be read.
 */
import { REPO } from '@/core/site';
import { type GitHubRelease, toRelease } from './github';
import { parseChecksums, type Release } from './model';

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

/** Reads every published release from GitHub, with the latest's checksums. Not cached. */
export async function loadReleases(): Promise<Release[]> {
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
  cache ??= loadReleases();
  return cache;
}

/** The newest stable release that has installers. */
export async function getLatestRelease(): Promise<Release | undefined> {
  return pickLatest(await getReleases());
}
