import { screen, waitFor } from '@testing-library/react';
import { PROD } from '../../scripts/screenshots/data';
import { installMocks } from '../../scripts/screenshots/mocks';
import { SHOTS } from '../../scripts/screenshots/shots';
import { DEFAULT_LOCALE, intl, setLocale } from '@/shared/i18n';
import { en } from '@/shared/i18n/en';
import { renderApp } from '@/test/render';

/**
 * With the app in French, no text on the main pages is still an English
 * message from the catalogue. Names, ids, commands and Engine output aren't
 * catalogue messages, so they never match.
 */
const english = new Set(
  Object.values(en)
    .filter((m) => !m.includes('{') && /[a-z]{3}/.test(m))
    .map((m) => m.trim()),
);

/** Every piece of text on screen, and every accessible name. */
function texts(): string[] {
  const out: string[] = [];
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const text = node.textContent?.trim();
    if (text) out.push(text);
  }
  for (const el of document.querySelectorAll('[aria-label],[title],[placeholder]')) {
    for (const attr of ['aria-label', 'title', 'placeholder']) {
      const value = el.getAttribute(attr)?.trim();
      if (value) out.push(value);
    }
  }
  return out;
}

/** The French catalogue's own words, without the English fallback. */
const french = new Set(
  Object.values(
    import.meta.glob<Record<string, string>>('../shared/i18n/fr/*.ts', {
      eager: true,
      import: 'default',
    }),
  ).flatMap((messages) => Object.values(messages)),
);

describe('the app in French', () => {
  beforeAll(() => setLocale('fr'));
  afterAll(() => setLocale(DEFAULT_LOCALE));
  afterEach(() => window.history.replaceState(null, '', '/'));

  const logs = SHOTS.find((s) => s.name === 'container-logs')?.route ?? '/containers';
  it.each(['/hosts', '/containers', logs, '/stacks', '/settings'])(
    '%s shows no English',
    async (route) => {
      window.history.replaceState(null, '', `/?route=${encodeURIComponent(route)}`);
      installMocks({ theme: 'light', hostId: PROD });
      renderApp(route);
      await screen.findByRole('main');
      await waitFor(
        () =>
          expect(
            screen.queryByText(intl().formatMessage({ id: 'common.loading' })),
          ).not.toBeInTheDocument(),
        {
          timeout: 4000,
        },
      );
      // Words French shares with English (« Stack », « Images ») aren't English left behind.
      const leftovers = texts().filter((text) => english.has(text) && !french.has(text));
      expect([...new Set(leftovers)]).toEqual([]);
      expect(document.documentElement.lang).toBe('fr');
    },
    20_000,
  );
});
