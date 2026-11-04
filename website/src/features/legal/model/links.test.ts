import { afterEach, expect, test } from 'bun:test';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { LEGAL_DOCUMENTS } from '../data/documents';
import { legalLinks } from './links';

const base = import.meta.env.BASE_URL;

afterEach(() => {
  import.meta.env.BASE_URL = base;
});

test('every legal document has a route', () => {
  for (const document of LEGAL_DOCUMENTS) {
    const route = new URL(
      `../../../pages${document.path.replace(/\/$/, '')}.astro`,
      import.meta.url,
    );
    expect(existsSync(fileURLToPath(route)), `${document.path} has no page`).toBe(true);
  }
});

test('links to the documents under the base path', () => {
  import.meta.env.BASE_URL = '/Homeport/';
  expect(legalLinks()[0]).toEqual({ label: 'Legal notice', url: '/Homeport/legal-notice/' });
});
