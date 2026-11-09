import { describe, expect, it } from 'bun:test';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { LEGAL_DOCUMENTS } from '@/features/legal';

/*
 * Every marketing page is in every language: English at `pages/<page>.astro`,
 * the others through one route, `pages/[locale]/<page>.astro`, whose
 * getStaticPaths() lists every language (core/i18n's localeStaticPaths). The
 * legal documents are in English only.
 */
const pages = fileURLToPath(new URL('../../pages/', import.meta.url));
const routes = (folder: string) =>
  readdirSync(folder).filter((file) => file.endsWith('.astro') && !file.startsWith('['));

const englishOnly = new Set(
  LEGAL_DOCUMENTS.map((document) => `${document.path.replaceAll('/', '')}.astro`),
);
const english = routes(pages).filter((file) => !englishOnly.has(file));
const localized = routes(`${pages}[locale]`);

describe('routes', () => {
  it('has the marketing pages', () => {
    expect(english.sort()).toEqual(['changelog.astro', 'download.astro', 'index.astro']);
  });

  it('gives every English marketing page its route in the other languages', () => {
    for (const file of english) {
      expect(existsSync(`${pages}[locale]/${file}`), `[locale]/${file} is missing`).toBe(true);
    }
  });

  it('translates no page that has no English one, nor the legal documents', () => {
    for (const file of localized) {
      expect(english, `[locale]/${file} has no English page`).toContain(file);
    }
  });

  it('builds each translated page in every language', () => {
    for (const file of localized) {
      const source = readFileSync(`${pages}[locale]/${file}`, 'utf8');
      expect(source, file).toContain('export const getStaticPaths = localeStaticPaths;');
    }
  });
});
