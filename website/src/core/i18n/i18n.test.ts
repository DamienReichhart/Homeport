import { describe, expect, it } from 'bun:test';
import { readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { LOCALES, ROOT_LOCALE } from '@/core/site';
import { localeStaticPaths, pageLocale, strings, t, TABLES } from './index';
import en from './strings/en';

const fr = LOCALES.find((l) => l.code === 'fr')!;
const folder = (path: string) => fileURLToPath(new URL(path, import.meta.url));
const AREAS = readdirSync(folder('./strings/en')).filter((file) => file !== 'index.ts');

describe('strings', () => {
  it('has a table for every language, and none for a language the site lacks', () => {
    expect(Object.keys(TABLES).sort()).toEqual(LOCALES.map((l) => l.code).sort());
    const folders = readdirSync(folder('./strings')).sort();
    expect(folders).toEqual(LOCALES.map((l) => l.code || 'en').sort());
  });

  it("gives every language exactly English's keys", () => {
    const keys = Object.keys(en).sort();
    for (const locale of LOCALES) {
      expect(Object.keys(strings(locale)).sort(), locale.lang).toEqual(keys);
    }
  });

  it("splits every language into English's files, with the same keys in each", async () => {
    for (const locale of LOCALES.filter((l) => l.code)) {
      expect(readdirSync(folder(`./strings/${locale.code}`)).sort()).toEqual(
        [...AREAS, 'index.ts'].sort(),
      );
      for (const area of AREAS) {
        const english = (await import(`./strings/en/${area}`)).default;
        const translated = (await import(`./strings/${locale.code}/${area}`)).default;
        expect(Object.keys(translated).sort(), `${locale.code}/${area}`).toEqual(
          Object.keys(english).sort(),
        );
      }
    }
  });

  it('leaves no value empty', () => {
    for (const locale of LOCALES) {
      for (const [key, value] of Object.entries(strings(locale))) {
        expect(value.trim(), `${locale.lang} ${key}`).not.toBe('');
      }
    }
  });

  it('keeps the placeholders of the English text', () => {
    const names = (text: string) => [...text.matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort();
    for (const locale of LOCALES) {
      for (const [key, value] of Object.entries(strings(locale))) {
        expect(names(value), `${locale.lang} ${key}`).toEqual(names(en[key as keyof typeof en]));
      }
    }
  });
});

describe('t', () => {
  it('reads a string and fills its placeholders', () => {
    expect(t(ROOT_LOCALE, 'home.hero.title')).toBe('Every Docker Engine, on one board.');
    expect(t(ROOT_LOCALE, 'home.hero.release', { version: '0.4.0' })).toBe('Homeport 0.4.0 is out');
    expect(t(fr, 'download.meta.version', { version: '1.2.0' })).toContain('1.2.0');
  });

  it('falls back to English for a language without a table', () => {
    const klingon = { code: 'tlh', lang: 'tlh', label: 'tlhIngan Hol', dir: 'ltr' } as const;
    expect(strings(klingon)).toBe(strings(ROOT_LOCALE));
  });
});

describe('pages', () => {
  it('reads the language of a page from its URL, after the base path', () => {
    const base = import.meta.env.BASE_URL;
    import.meta.env.BASE_URL = '/Homeport/';
    try {
      expect(pageLocale(new URL('https://example.com/Homeport/fr/download/'))).toBe(fr);
      expect(pageLocale(new URL('https://example.com/Homeport/download/'))).toBe(ROOT_LOCALE);
    } finally {
      import.meta.env.BASE_URL = base;
    }
  });

  it('builds a page in every language but English', () => {
    const paths = localeStaticPaths();
    expect(paths.map((p) => p.params.locale)).toEqual(
      LOCALES.filter((l) => l.code).map((l) => l.code),
    );
    expect(paths.find((p) => p.params.locale === 'fr')?.props.locale).toBe(fr);
  });
});
