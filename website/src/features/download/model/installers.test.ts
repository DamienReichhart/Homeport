import { describe, expect, test } from 'bun:test';
import type { Arch, Format, Installer, Os, Release } from '@/core/release';
import { formatsFor } from './installers';

const file = (os: Os, arch: Arch, format: Format): Installer => ({
  name: `${os}-${arch}.${format}`,
  url: '',
  size: 1,
  os,
  arch,
  format,
});

const release = (installers: Installer[]): Release => ({
  tag: 'v1.0.0',
  version: '1.0.0',
  name: '',
  url: '',
  publishedAt: '',
  prerelease: false,
  body: '',
  installers,
});

const names = (rows: ReturnType<typeof formatsFor>) =>
  rows.map(({ format, files }) => [format, files.map((f) => f.arch)]);

describe('formatsFor', () => {
  test('groups one OS’s files by format, x64 first', () => {
    const r = release([
      file('windows', 'arm64', 'nsis'),
      file('windows', 'x64', 'nsis'),
      file('windows', 'x64', 'msi'),
      file('linux', 'x64', 'deb'),
    ]);
    expect(names(formatsFor(r, 'windows'))).toEqual([
      ['nsis', ['x64', 'arm64']],
      ['msi', ['x64']],
    ]);
  });

  test('puts Apple silicon first on macOS', () => {
    const r = release([file('macos', 'x64', 'dmg'), file('macos', 'arm64', 'dmg')]);
    expect(names(formatsFor(r, 'macos'))).toEqual([['dmg', ['arm64', 'x64']]]);
  });

  test('has no rows without a release or without files for the OS', () => {
    expect(formatsFor(undefined, 'linux')).toEqual([]);
    expect(formatsFor(release([file('macos', 'arm64', 'dmg')]), 'linux')).toEqual([]);
  });
});
