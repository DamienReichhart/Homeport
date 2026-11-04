import { useEffect, type ReactNode } from 'react';
import { RawIntlProvider } from 'react-intl';
import { intl } from '@/shared/i18n';

/** Hands the app's formatter to react-intl and sets the page's language. */
export function I18nProvider({ children }: { children: ReactNode }) {
  const formatter = intl();
  useEffect(() => {
    document.documentElement.lang = formatter.locale;
  }, [formatter.locale]);
  return <RawIntlProvider value={formatter}>{children}</RawIntlProvider>;
}
