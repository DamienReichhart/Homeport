import { describe, expect, test } from 'bun:test';
import { type GitHubRelease, toRelease } from './github';

const asset = (name: string, size = 1) => ({
  name,
  size,
  browser_download_url: `https://example.com/${name}`,
});

const raw = (overrides: Partial<GitHubRelease> = {}): GitHubRelease => ({
  tag_name: 'v0.4.0',
  name: 'Homeport v0.4.0',
  html_url: 'https://github.com/DamienReichhart/Homeport/releases/tag/v0.4.0',
  published_at: '2026-11-03T12:00:00Z',
  prerelease: false,
  draft: false,
  body: 'Notes',
  assets: [],
  ...overrides,
});

describe('toRelease', () => {
  test('keeps the installers, in format order then by architecture', () => {
    const release = toRelease(
      raw({
        assets: [
          asset('Homeport_0.4.0_amd64.AppImage'),
          asset('SHA256SUMS.txt'),
          asset('Homeport_0.4.0_arm64-setup.exe'),
          asset('Homeport_0.4.0_aarch64.dmg'),
          asset('latest.json'),
          asset('Homeport_0.4.0_x64-setup.exe'),
          asset('Homeport_0.4.0_x64_en-US.msi'),
        ],
      }),
    );
    expect(release.installers.map((i) => i.name)).toEqual([
      'Homeport_0.4.0_arm64-setup.exe',
      'Homeport_0.4.0_x64-setup.exe',
      'Homeport_0.4.0_x64_en-US.msi',
      'Homeport_0.4.0_aarch64.dmg',
      'Homeport_0.4.0_amd64.AppImage',
    ]);
    expect(release.checksumsUrl).toBe('https://example.com/SHA256SUMS.txt');
  });

  test('strips the v from the version and copies the release details', () => {
    expect(toRelease(raw())).toMatchObject({
      tag: 'v0.4.0',
      version: '0.4.0',
      name: 'Homeport v0.4.0',
      publishedAt: '2026-11-03T12:00:00Z',
      prerelease: false,
      body: 'Notes',
      installers: [],
      checksumsUrl: undefined,
    });
  });

  test('falls back on the tag for a missing name, and on empty text', () => {
    expect(toRelease(raw({ name: null, published_at: null, body: null }))).toMatchObject({
      name: 'v0.4.0',
      publishedAt: '',
      body: '',
    });
  });
});
