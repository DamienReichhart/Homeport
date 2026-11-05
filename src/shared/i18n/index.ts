// The app's text (ADR 0011). Import from "@/shared/i18n" only.
import { createIntl, createIntlCache, type IntlShape } from 'react-intl';
import type { ReactNode } from 'react';
import { en, uiMessages } from './en';

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

const cache = createIntlCache();
const current: IntlShape = createIntl(
  {
    locale: DEFAULT_LOCALE,
    defaultLocale: DEFAULT_LOCALE,
    messages: en,
    // A key missing from a later language falls back to English; that's expected.
    onError: (error) => {
      if (error.code !== 'MISSING_TRANSLATION') console.error(error);
    },
  },
  cache,
);

/** The formatter the app uses; the provider hands the same one to react-intl. */
export const intl = (): IntlShape => current;

/** The active locale, such as `en`. */
export const locale = () => current.locale;

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
