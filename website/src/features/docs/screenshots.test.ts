import { describe, expect, test } from 'bun:test';
import { existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { PREFIXED_LOCALES } from '@/core/site';

/**
 * Every language shows the app in its own language: each shot English has
 * exists in every language's folder (`bun run screenshots` at the repo root
 * captures them all), so no page falls back to an English screenshot.
 */
const dir = join(import.meta.dir, '..', '..', 'assets', 'screenshots');
const shots = (folder: string) =>
  existsSync(folder) ? readdirSync(folder).filter((f) => f.endsWith('.png')) : [];

describe('screenshots', () => {
  const english = shots(dir);

  test('English has some', () => expect(english.length).toBeGreaterThan(0));

  for (const locale of PREFIXED_LOCALES) {
    test(`${locale.label} has every shot English has`, () => {
      const own = new Set(shots(join(dir, locale.code)));
      expect(english.filter((f) => !own.has(f))).toEqual([]);
    });
  }
});
