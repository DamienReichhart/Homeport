import { afterEach, beforeEach, expect, test } from 'bun:test';
import type { Release } from '@/core/release';
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
  expect(softwareApplication(site, undefined)).toMatchObject({
    '@type': 'SoftwareApplication',
    url: 'https://damienreichhart.github.io/Homeport/',
    downloadUrl: 'https://damienreichhart.github.io/Homeport/download/',
  });
});

test('gives the version only when a release is out', () => {
  expect(softwareApplication(site, undefined)).not.toHaveProperty('softwareVersion');
  const release = { version: '0.4.0' } as Release;
  expect(softwareApplication(site, release)).toHaveProperty('softwareVersion', '0.4.0');
});
