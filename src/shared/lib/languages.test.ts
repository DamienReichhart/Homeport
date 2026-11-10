import { FALLBACK_LANGUAGE, keepValidLanguages, validLanguages } from './languages';

/** A navigator reporting `languages`, as WebKitGTK does under `LANG=C.UTF-8`. */
const reporting = (languages: string[]) =>
  ({ language: languages[0] ?? '', languages }) as unknown as Navigator;

describe('the OS languages', () => {
  it('keep only tags Intl accepts', () => {
    expect(validLanguages(['C', 'fr-FR', 'en_US', 'en'])).toEqual(['fr-FR', 'en']);
  });

  it('fall back to English when the locale is C, so Intl never throws', () => {
    const nav = reporting(['C']);
    keepValidLanguages(nav);
    expect(nav.language).toBe(FALLBACK_LANGUAGE);
    expect(nav.languages).toEqual([FALLBACK_LANGUAGE]);
    expect(() => new Intl.NumberFormat(nav.language)).not.toThrow();
  });

  it('drop the invalid ones and keep the order of the rest', () => {
    const nav = reporting(['C', 'de-AT', 'de']);
    keepValidLanguages(nav);
    expect(nav.languages).toEqual(['de-AT', 'de']);
    expect(nav.language).toBe('de-AT');
  });

  it('leave valid languages as the webview reports them', () => {
    const nav = reporting(['pt-BR', 'en']);
    keepValidLanguages(nav);
    expect(nav.languages).toEqual(['pt-BR', 'en']);
    expect(Object.getOwnPropertyDescriptor(nav, 'language')?.get).toBeUndefined();
  });
});
