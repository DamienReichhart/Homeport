/**
 * The site's languages. English is the root (`/`, `/docs/…`); every other
 * language lives under its own prefix (`/fr/`, `/fr/docs/…`). The app's
 * languages are the same set (`src/shared/i18n/` at the repository root).
 * Adding one: an entry here, its strings in `src/app/i18n/strings/`, and its
 * docs under `src/content/docs/<code>/` (pages without a translation fall
 * back to English).
 */
export interface Locale {
  /** The URL prefix and Starlight locale key; empty for English, the root. */
  code: string;
  /** The BCP 47 tag for `lang`, `hreflang` and date formatting. */
  lang: string;
  /** The language's own name for itself. */
  label: string;
  dir: 'ltr' | 'rtl';
}

export const ROOT_LOCALE: Locale = { code: '', lang: 'en-GB', label: 'English', dir: 'ltr' };

export const LOCALES: readonly Locale[] = [
  ROOT_LOCALE,
  { code: 'fr', lang: 'fr', label: 'Français', dir: 'ltr' },
  { code: 'es', lang: 'es', label: 'Español', dir: 'ltr' },
  { code: 'de', lang: 'de', label: 'Deutsch', dir: 'ltr' },
  { code: 'it', lang: 'it', label: 'Italiano', dir: 'ltr' },
  { code: 'pt-br', lang: 'pt-BR', label: 'Português (Brasil)', dir: 'ltr' },
  { code: 'ru', lang: 'ru', label: 'Русский', dir: 'ltr' },
  { code: 'zh-cn', lang: 'zh-CN', label: '简体中文', dir: 'ltr' },
  { code: 'ja', lang: 'ja', label: '日本語', dir: 'ltr' },
  { code: 'ko', lang: 'ko', label: '한국어', dir: 'ltr' },
  { code: 'hi', lang: 'hi', label: 'हिन्दी', dir: 'ltr' },
  { code: 'ar', lang: 'ar', label: 'العربية', dir: 'rtl' },
];

/** The languages with a URL prefix: every one but English. */
export const PREFIXED_LOCALES = LOCALES.filter((l) => l.code !== '');

/** The language a path (without the base path) is in: `/fr/download/` → French. */
export function localeOf(path: string): Locale {
  const first = path.replace(/^\/+/, '').split('/')[0] ?? '';
  return PREFIXED_LOCALES.find((l) => l.code === first) ?? ROOT_LOCALE;
}

/** `path` without its language prefix: `/fr/download/` → `/download/`. */
export function unlocalized(path: string): string {
  const locale = localeOf(path);
  if (!locale.code) return path.startsWith('/') ? path : `/${path}`;
  const rest = path.replace(/^\/+/, '').slice(locale.code.length);
  return rest.startsWith('/') ? rest : `/${rest}`;
}

/** `path` (an English one, or another language's) in `locale`: `/download/` → `/fr/download/`. */
export function localePath(path: string, locale: Locale): string {
  const plain = unlocalized(path);
  return locale.code ? `/${locale.code}${plain}` : plain;
}
