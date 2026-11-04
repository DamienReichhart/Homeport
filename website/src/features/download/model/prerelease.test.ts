import { describe, expect, test } from 'bun:test';
import type { Release } from '@/core/release';
import { newerPrerelease } from './prerelease';

const release = (tag: string, publishedAt: string, prerelease: boolean, files = 1): Release => ({
  tag,
  version: tag,
  name: tag,
  url: '',
  publishedAt,
  prerelease,
  body: '',
  installers: Array.from({ length: files }, () => ({
    name: 'x.dmg',
    url: '',
    size: 1,
    os: 'macos' as const,
    arch: 'arm64' as const,
    format: 'dmg' as const,
  })),
});

const stable = release('v1.0.0', '2026-10-01T00:00:00Z', false);

describe('newerPrerelease', () => {
  test('offers a prerelease published after the latest release', () => {
    const beta = release('v1.1.0-beta.1', '2026-11-01T00:00:00Z', true);
    expect(newerPrerelease([beta, stable], stable)).toBe(beta);
  });

  test('ignores older prereleases and those without installers', () => {
    const old = release('v1.0.0-rc.1', '2026-09-01T00:00:00Z', true);
    const empty = release('v1.1.0-beta.1', '2026-11-01T00:00:00Z', true, 0);
    expect(newerPrerelease([empty, stable, old], stable)).toBeUndefined();
  });

  test('offers any prerelease with installers when nothing is released yet', () => {
    const beta = release('v0.1.0-beta.1', '2026-01-01T00:00:00Z', true);
    expect(newerPrerelease([beta], undefined)).toBe(beta);
  });
});
