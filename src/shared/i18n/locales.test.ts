import { DEFAULT_LOCALE, LOCALES, locale, resolveLocale, setLocale, t } from '.';

describe('the app’s languages', () => {
  afterEach(() => setLocale(DEFAULT_LOCALE));

  it('lists every catalogue folder, English first, each named in its own words', () => {
    expect(LOCALES[0]).toEqual({ tag: 'en', name: 'English', dir: 'ltr' });
    expect(LOCALES.map((l) => l.tag)).toEqual(expect.arrayContaining(['fr', 'ja', 'pt-BR']));
    expect(LOCALES.find((l) => l.tag === 'ar')?.dir).toBe('rtl');
  });

  it('follows the OS for "system", by region or by language, else English', () => {
    expect(resolveLocale('system', ['fr-CA', 'en'])).toBe('fr');
    expect(resolveLocale('system', ['pt-br'])).toBe('pt-BR');
    expect(resolveLocale('system', ['pt-PT'])).toBe('pt-BR');
    expect(resolveLocale('system', ['zh-Hans-CN'])).toBe('zh-CN');
    expect(resolveLocale('system', ['zh-TW', 'ja'])).toBe('ja');
    expect(resolveLocale('system', ['sw', 'tlh'])).toBe('en');
    expect(resolveLocale('system', [])).toBe('en');
  });

  it('keeps a chosen language, and reads an unknown one as the OS’s', () => {
    expect(resolveLocale('de', ['fr'])).toBe('de');
    expect(resolveLocale('xx', ['it-IT'])).toBe('it');
  });

  it('switches the catalogue without a reload, falling back to English for nothing', async () => {
    expect(t('common.cancel')).toBe('Cancel');
    await setLocale('fr');
    expect(locale()).toBe('fr');
    expect(t('common.cancel')).not.toBe('Cancel');
    await setLocale('en');
    expect(t('common.cancel')).toBe('Cancel');
  });
});
