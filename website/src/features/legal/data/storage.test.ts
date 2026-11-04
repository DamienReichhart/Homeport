import { expect, test } from 'bun:test';
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { STORED_ITEMS, SYNCED_TABS_PREFIX } from './storage';

const docs = fileURLToPath(new URL('../../../content/docs', import.meta.url));

function syncKeys(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) return syncKeys(path);
    if (!/\.mdx?$/.test(entry.name)) return [];
    return [...readFileSync(path, 'utf8').matchAll(/syncKey="([^"]+)"/g)].map((m) => m[1]!);
  });
}

test('the cookie policy lists every synced tab group the docs store', () => {
  const listed = STORED_ITEMS.map((item) => item.name);
  for (const key of new Set(syncKeys(docs))) {
    expect(listed).toContain(`${SYNCED_TABS_PREFIX}${key}`);
  }
});

test('lists each item once', () => {
  const names = STORED_ITEMS.map((item) => item.name);
  expect(new Set(names).size).toBe(names.length);
});
