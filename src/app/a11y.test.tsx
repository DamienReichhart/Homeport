import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { PROD } from '../../scripts/screenshots/data';
import { installMocks } from '../../scripts/screenshots/mocks';
import { SHOTS } from '../../scripts/screenshots/shots';
import { navEntriesOf } from '@/core/feature';
import { accessibilityProblems } from '@/test/axe';
import { renderApp } from '@/test/render';
import { intl } from '@/shared/i18n';
import { features } from './router/features';

/**
 * Every page the website photographs, and every sidebar page, on the demo
 * data, checked with axe (WCAG 2.2 A and AA, without colour contrast). A new
 * page is checked once it has a sidebar entry or a screenshot.
 */
interface Page {
  route: string;
  demo?: 'clusters' | 'nomad';
  steps?: string[];
  /** The steps open a dialog, which must then be on screen. */
  dialog?: boolean;
}

const pages = new Map<string, Page>();
const add = (page: Page) => pages.set(JSON.stringify(page), page);
for (const shot of SHOTS) add({ route: shot.route, demo: shot.demo, steps: shot.steps });
// The main dialogs, opened from their page's button.
for (const [route, button] of [
  ['/images', 'Pull image'],
  ['/containers', 'Run container'],
  ['/volumes', 'Create volume'],
  ['/networks', 'Create network'],
  ['/settings', 'Add login'],
  ['/settings', 'Add repository'],
  ['/settings', 'Back up…'],
]) {
  add({ route: route ?? '/', steps: [`click:${button}`], dialog: true });
}
for (const entry of navEntriesOf(features)) {
  add({ route: entry.path });
  add({ route: entry.path, demo: 'clusters' });
}

/** Clicks the control whose text is `text` (`@key`: that key's text), as the screenshot steps do. */
async function click(text: string) {
  const label = text.startsWith('@') ? intl().formatMessage({ id: text.slice(1) }) : text;
  const el = [
    ...document.querySelectorAll<HTMLElement>(
      'button, [role="tab"], [role="menuitem"], [role="option"], a',
    ),
  ].find((e) => e.textContent?.trim() === label);
  if (el) await userEvent.click(el);
}

async function fill(placeholder: string, value: string) {
  const el = screen.queryByPlaceholderText(placeholder);
  if (el) await userEvent.type(el, value);
}

describe('every page, checked with axe', () => {
  afterEach(() => window.history.replaceState(null, '', '/'));

  it.each(
    [...pages.values()].map((p) => [
      `${p.route}${p.demo ? ` (${p.demo})` : ''}${p.steps ? ` ${p.steps.join(', ')}` : ''}`,
      p,
    ]),
  )(
    '%s',
    async (_, page) => {
      const query = new URLSearchParams({ route: page.route });
      if (page.demo) query.set('demo', page.demo);
      window.history.replaceState(null, '', `/?${query}`);
      installMocks({ theme: 'light', hostId: PROD });
      renderApp(page.route);
      await screen.findByRole('main');
      await waitFor(() => expect(screen.queryByText('Loading…')).not.toBeInTheDocument(), {
        timeout: 4000,
      });
      for (const step of page.steps ?? []) {
        const [kind, ...rest] = step.split(':');
        const arg = rest.join(':');
        if (kind === 'click') await click(arg ?? '');
        else if (kind === 'fill') {
          const [placeholder = '', ...value] = arg.split('=');
          await fill(placeholder, value.join('='));
        }
      }
      // A page that threw shows the crash page, which would pass for the wrong reason.
      expect(screen.queryByText('Something went wrong')).not.toBeInTheDocument();
      if (page.dialog) expect(await screen.findByRole('dialog')).toBeInTheDocument();
      expect(await accessibilityProblems()).toEqual([]);
    },
    20_000,
  );
});
