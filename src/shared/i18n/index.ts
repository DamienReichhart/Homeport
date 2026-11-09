// The app's text (ADR 0011). Import from "@/shared/i18n" only.
import { createIntl, createIntlCache, type IntlShape } from 'react-intl';
import type { ReactNode } from 'react';
import { en, uiMessages } from './en';
import type { LocaleMeta } from './types';

export type { LocaleMeta, Translation } from './types';

/** A key of the UI catalogue, typed from the English messages. */
export type MessageKey = keyof typeof uiMessages;

/** Values for a message's `{arguments}`. */
export type MessageValues = Record<string, string | number | null | undefined>;

/** Values for a message with tags, such as `<strong>…</strong>`. */
export type RichValues = Record<
  string,
  string | number | ReactNode | ((chunks: ReactNode[]) => ReactNode)
>;

export const DEFAULT_LOCALE = 'en';

/** The Settings value that follows the OS's language. */
export const SYSTEM_LANGUAGE = 'system';

// Every folder with a `meta.ts` is a language: adding one needs no code (ADR 0011).
const metas = import.meta.glob<LocaleMeta>('./*/meta.ts', { eager: true, import: 'default' });
// Its namespace files load when the language is chosen, so English users don't pay for them.
const namespaceFiles = import.meta.glob<Record<string, string>>(
  ['./*/*.ts', './*/*.json', '!./en/**', '!./*/meta.ts', '!./*/index.ts'],
  { import: 'default' },
);

/** A language the app has a catalogue for. */
export interface Locale extends LocaleMeta {
  /** Its tag, as stored in Settings: `en`, `fr`, `pt-BR`. */
  tag: string;
}

/** The app's languages: English first, then by tag. */
export const LOCALES: readonly Locale[] = Object.entries(metas)
  .map(([path, meta]) => ({ ...meta, tag: path.split('/')[1] ?? '' }))
  .sort((a, b) =>
    a.tag === DEFAULT_LOCALE ? -1 : b.tag === DEFAULT_LOCALE ? 1 : a.tag.localeCompare(b.tag),
  );

const base = (tag: string) => tag.toLowerCase().split('-')[0] ?? '';
/** Chinese written in Traditional characters isn't Simplified Chinese's to show. */
const traditional = (tag: string) => /^zh-(hant|tw|hk|mo)\b/i.test(tag);

/** The supported language for `tag`: exact (`pt-BR`), then by base (`fr-CA` → `fr`). */
function match(tag: string): Locale | undefined {
  const exact = LOCALES.find((l) => l.tag.toLowerCase() === tag.toLowerCase());
  if (exact) return exact;
  if (traditional(tag)) return undefined;
  return LOCALES.find((l) => base(l.tag) === base(tag));
}

/**
 * The language to show for the Settings value `language`: the chosen one, or
 * for `system` (or one no catalogue exists for) the first of the OS's
 * preferred languages the app has, else English.
 */
export function resolveLocale(
  language: string,
  preferred: readonly string[] = typeof navigator === 'undefined' ? [] : navigator.languages,
): string {
  if (language !== SYSTEM_LANGUAGE) {
    const chosen = match(language);
    if (chosen) return chosen.tag;
  }
  for (const tag of preferred) {
    const found = match(tag);
    if (found) return found.tag;
  }
  return DEFAULT_LOCALE;
}

const cache = createIntlCache();
const create = (locale: string, messages: Record<string, string>) =>
  createIntl(
    {
      locale,
      defaultLocale: DEFAULT_LOCALE,
      messages,
      // A key missing from a later language falls back to English; that's expected.
      onError: (error) => {
        if (error.code !== 'MISSING_TRANSLATION') console.error(error);
      },
    },
    cache,
  );

let current: IntlShape = create(DEFAULT_LOCALE, en);
/** The active language's own messages, without the English fallback. */
let translated: Record<string, string> = {};
const listeners = new Set<() => void>();

/** The formatter the app uses; the provider hands the same one to react-intl. */
export const intl = (): IntlShape => current;

/** The active locale, such as `en`. */
export const locale = () => current.locale;

/** The active language's writing direction. */
export const direction = (tag = current.locale): 'ltr' | 'rtl' =>
  LOCALES.find((l) => l.tag === tag)?.dir ?? 'ltr';

/**
 * The locale dates and numbers are formatted in: the OS's own variant of the
 * app's language when it prefers one (`en-GB`, `fr-CA`), else the language.
 */
export function formattingLocale(
  preferred: readonly string[] = typeof navigator === 'undefined' ? [] : navigator.languages,
): string {
  return preferred.find((tag) => base(tag) === base(current.locale)) ?? current.locale;
}

/** Calls `listener` whenever the language changes; returns the unsubscribe. */
export function onLocaleChange(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** Loads `tag`'s catalogue (English for an unknown one) and makes it the app's language. */
export async function setLocale(tag: string): Promise<void> {
  const target = match(tag)?.tag ?? DEFAULT_LOCALE;
  if (target === current.locale) return;
  const own: Record<string, string> = {};
  if (target !== DEFAULT_LOCALE) {
    const loads = Object.entries(namespaceFiles)
      .filter(([path]) => path.split('/')[1] === target)
      .map(([, load]) => load());
    for (const messages of await Promise.all(loads)) Object.assign(own, messages);
  }
  translated = own;
  current = create(target, { ...en, ...own });
  for (const listener of listeners) listener();
}

/**
 * The active language's templates for the text Rust renders for the OS
 * (`native.*`, and the `errors.*` a notification quotes); empty for English.
 */
export function nativeTemplates(): Record<string, string> {
  return Object.fromEntries(
    Object.entries(translated).filter(([key]) => /^(native|errors)\./.test(key)),
  );
}

/** A message as text, with its arguments filled in. */
export function t(key: MessageKey, values?: MessageValues): string {
  return current.formatMessage({ id: key }, values ?? undefined);
}

/**
 * A message with tags rendered by `values`, such as
 * `tRich('volumes.remove.warning', { strong: (c) => <strong>{c}</strong> })`.
 */
export function tRich(key: MessageKey, values: RichValues): ReactNode {
  return current.formatMessage({ id: key }, values);
}

/** Whether the catalogue has `key`; error keys come from the backend unchecked. */
export const hasMessage = (key: string): boolean => key in current.messages;

/** A message by a key known only at runtime (an error from the backend). */
export function formatKey(key: string, values?: MessageValues): string {
  return current.formatMessage({ id: key }, values ?? undefined);
}

/** Every key of the UI catalogue, for the checks. */
export const uiKeys = (): string[] => Object.keys(uiMessages);
