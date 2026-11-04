/** The GitHub API's view of a release, and how it becomes a Release. */
import { classify, type Format, type Installer, type Release } from './model';

interface GitHubAsset {
  name: string;
  size: number;
  browser_download_url: string;
}

export interface GitHubRelease {
  tag_name: string;
  name: string | null;
  html_url: string;
  published_at: string | null;
  prerelease: boolean;
  draft: boolean;
  body: string | null;
  assets: GitHubAsset[];
}

const ORDER: Format[] = ['nsis', 'msi', 'dmg', 'deb', 'rpm', 'appimage'];

export function toRelease(raw: GitHubRelease): Release {
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
