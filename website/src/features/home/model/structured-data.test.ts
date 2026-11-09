import { afterEach, beforeEach, expect, test } from 'bun:test';
import type { Release } from '@/core/release';
import { LOCALES, ROOT_LOCALE } from '@/core/site';
import { softwareApplication } from './structured-data';

const base = import.meta.env.BASE_URL;
const site = new URL('https://damienreichhart.github.io');

beforeEach(() => {
  import.meta.env.BASE_URL = '/Homeport/';
});

afterEach(() => {
  import.meta.env.BASE_URL = base;
});

test('describes Homeport with absolute URLs under the base path', () => {
  expect(softwareApplication(site, undefined, ROOT_LOCALE)).toMatchObject({
    '@type': 'SoftwareApplication',
    url: 'https://damienreichhart.github.io/Homeport/',
    downloadUrl: 'https://damienreichhart.github.io/Homeport/download/',
  });
});

test("points at the language's own pages", () => {
  const fr = LOCALES.find((l) => l.code === 'fr')!;
  expect(softwareApplication(site, undefined, fr)).toMatchObject({
    url: 'https://damienreichhart.github.io/Homeport/fr/',
    downloadUrl: 'https://damienreichhart.github.io/Homeport/fr/download/',
  });
});

test('gives the version only when a release is out', () => {
  expect(softwareApplication(site, undefined, ROOT_LOCALE)).not.toHaveProperty('softwareVersion');
  const release = { version: '0.4.0' } as Release;
  expect(softwareApplication(site, release, ROOT_LOCALE)).toHaveProperty(
    'softwareVersion',
    '0.4.0',
  );
});
