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

  test('offers the installers named by system and processor, never the updater’s files', () => {
    const names = [
      'Homeport-1.0.0-linux-arm64.AppImage',
      'Homeport-1.0.0-linux-arm64.AppImage.sig',
      'Homeport-1.0.0-linux-arm64.deb',
      'Homeport-1.0.0-linux-arm64.deb.sig',
      'Homeport-1.0.0-linux-arm64.rpm',
      'Homeport-1.0.0-linux-arm64.rpm.sig',
      'Homeport-1.0.0-linux-x64.AppImage',
      'Homeport-1.0.0-linux-x64.AppImage.sig',
      'Homeport-1.0.0-linux-x64.deb',
      'Homeport-1.0.0-linux-x64.deb.sig',
      'Homeport-1.0.0-linux-x64.rpm',
      'Homeport-1.0.0-linux-x64.rpm.sig',
      'Homeport-1.0.0-macos-arm64-update.app.tar.gz',
      'Homeport-1.0.0-macos-arm64-update.app.tar.gz.sig',
      'Homeport-1.0.0-macos-arm64.dmg',
      'Homeport-1.0.0-macos-x64-update.app.tar.gz',
      'Homeport-1.0.0-macos-x64-update.app.tar.gz.sig',
      'Homeport-1.0.0-macos-x64.dmg',
      'Homeport-1.0.0-windows-arm64-setup.exe',
      'Homeport-1.0.0-windows-arm64-setup.exe.sig',
      'Homeport-1.0.0-windows-x64-setup.exe',
      'Homeport-1.0.0-windows-x64-setup.exe.sig',
      'Homeport-1.0.0-windows-x64.msi',
      'Homeport-1.0.0-windows-x64.msi.sig',
      'SHA256SUMS.txt',
    ];
    const release = toRelease(raw({ tag_name: 'v1.0.0', assets: names.map((n) => asset(n)) }));
    expect(release.installers.map((i) => [i.name, i.os, i.arch])).toEqual([
      ['Homeport-1.0.0-windows-arm64-setup.exe', 'windows', 'arm64'],
      ['Homeport-1.0.0-windows-x64-setup.exe', 'windows', 'x64'],
      ['Homeport-1.0.0-windows-x64.msi', 'windows', 'x64'],
      ['Homeport-1.0.0-macos-arm64.dmg', 'macos', 'arm64'],
      ['Homeport-1.0.0-macos-x64.dmg', 'macos', 'x64'],
      ['Homeport-1.0.0-linux-arm64.deb', 'linux', 'arm64'],
      ['Homeport-1.0.0-linux-x64.deb', 'linux', 'x64'],
      ['Homeport-1.0.0-linux-arm64.rpm', 'linux', 'arm64'],
      ['Homeport-1.0.0-linux-x64.rpm', 'linux', 'x64'],
      ['Homeport-1.0.0-linux-arm64.AppImage', 'linux', 'arm64'],
      ['Homeport-1.0.0-linux-x64.AppImage', 'linux', 'x64'],
    ]);
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
