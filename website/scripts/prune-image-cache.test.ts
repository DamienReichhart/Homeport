import { describe, expect, test } from 'bun:test';
import { unused } from './prune-image-cache';

describe('prune-image-cache', () => {
  test('keeps what the build used, with its metadata, and drops the rest', () => {
    const built = new Set(['shot.AAAA_x1.avif', 'shot.AAAA_x2.webp', 'remote.BBBB_y.png']);
    const cached = [
      'shot.AAAA_x1.avif',
      'shot.AAAA_x2.webp',
      // The same screenshot before its content changed: another source hash.
      'shot.ZZZZ_x1.avif',
      'remote.BBBB_y.png',
      'remote.BBBB_y.png.json',
      'gone.CCCC_z.png.json',
    ];
    expect(unused(cached, built)).toEqual(['shot.ZZZZ_x1.avif', 'gone.CCCC_z.png.json']);
  });
});
