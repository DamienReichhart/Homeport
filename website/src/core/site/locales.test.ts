import { describe, expect, it } from 'bun:test';
import { LOCALES, localeOf, localePath, unlocalized } from './locales';

const fr = LOCALES.find((l) => l.code === 'fr')!;
const ptBr = LOCALES.find((l) => l.code === 'pt-br')!;
const en = LOCALES[0]!;

describe('locales', () => {
  it('reads the language from the path', () => {
    expect(localeOf('/fr/download/').lang).toBe('fr');
    expect(localeOf('/pt-br/docs/').lang).toBe('pt-BR');
    expect(localeOf('/download/')).toBe(en);
    expect(localeOf('/')).toBe(en);
    expect(localeOf('/frame/')).toBe(en);
  });

  it('moves a path between languages', () => {
    expect(unlocalized('/fr/docs/hosts/')).toBe('/docs/hosts/');
    expect(unlocalized('/fr/')).toBe('/');
    expect(localePath('/docs/hosts/', fr)).toBe('/fr/docs/hosts/');
    expect(localePath('/fr/changelog/', ptBr)).toBe('/pt-br/changelog/');
    expect(localePath('/fr/changelog/', en)).toBe('/changelog/');
    expect(localePath('/', fr)).toBe('/fr/');
  });
});
