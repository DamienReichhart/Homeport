/**
 * The marketing pages' text in every language of `LOCALES` (`@/core/site`).
 * English (`strings/en/`) is the source; each other language has a folder of
 * the same files, typed so that a missing or extra key fails `astro check`.
 */
import { type Locale, localeOf, PREFIXED_LOCALES } from '@/core/site';
import { unbased } from '@/shared/lib/href';
import { fill, type TemplateValues } from '@/shared/lib/template';
import ar from './strings/ar';
import de from './strings/de';
import en from './strings/en';
import es from './strings/es';
import fr from './strings/fr';
import hi from './strings/hi';
import it from './strings/it';
import ja from './strings/ja';
import ko from './strings/ko';
import ptBr from './strings/pt-br';
import ru from './strings/ru';
import zhCn from './strings/zh-cn';
import type { StringKey, Strings } from './types';

export type { StringKey, Strings, Translation } from './types';

/** Each language's strings, by its code in `LOCALES`. */
export const TABLES: Record<string, Strings> = {
  '': en,
  fr,
  es,
  de,
  it,
  'pt-br': ptBr,
  ru,
  'zh-cn': zhCn,
  ja,
  ko,
  hi,
  ar,
};

/** A language's strings; English for a language without a table. */
export function strings(locale: Locale): Strings {
  return TABLES[locale.code] ?? en;
}

/** One string in `locale`, its `{name}` placeholders filled from `values`. */
export function t(locale: Locale, key: StringKey, values?: TemplateValues): string {
  return fill(strings(locale)[key], values);
}

/** The language of the page at `url`, from its path after the base. */
export function pageLocale(url: URL): Locale {
  return localeOf(unbased(url.pathname));
}

/** `getStaticPaths()` of a page in `pages/[locale]/`: one page per language but English. */
export function localeStaticPaths(): { params: { locale: string }; props: { locale: Locale } }[] {
  return PREFIXED_LOCALES.map((locale) => ({ params: { locale: locale.code }, props: { locale } }));
}
