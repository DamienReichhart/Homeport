import { Fragment, useEffect, useSyncExternalStore, type ReactNode } from 'react';
import { RawIntlProvider } from 'react-intl';
import { Direction } from 'radix-ui';
import { direction, intl, onLocaleChange } from '@/shared/i18n';

/**
 * Hands the app's formatter to react-intl, sets the page's language and
 * direction, and redraws everything when the language changes: the tree is
 * keyed by locale, so text computed once (table columns, memoised labels)
 * is computed again. Queries and the route live above it and survive.
 */
export function I18nProvider({ children }: { children: ReactNode }) {
  const formatter = useSyncExternalStore(onLocaleChange, intl);
  const dir = direction(formatter.locale);
  useEffect(() => {
    document.documentElement.lang = formatter.locale;
    document.documentElement.dir = dir;
  }, [formatter.locale, dir]);
  return (
    <RawIntlProvider value={formatter}>
      <Direction.Provider dir={dir}>
        <Fragment key={formatter.locale}>{children}</Fragment>
      </Direction.Provider>
    </RawIntlProvider>
  );
}
