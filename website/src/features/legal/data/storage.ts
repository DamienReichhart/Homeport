/**
 * Everything the site stores in the visitor's browser, as the cookie policy
 * lists it. There are no cookies. Keep this in step with the code: check in
 * DevTools (Application → Storage) after adding a script or a Starlight feature
 * that remembers something. A test checks the docs' synced tabs against it.
 */
export interface StoredItem {
  name: string;
  storage: 'Local storage' | 'Session storage';
  purpose: string;
  set: string;
  duration: string;
}

const UNTIL_CLEARED = 'Until you clear this site’s data';

export const STORED_ITEMS: StoredItem[] = [
  {
    name: 'starlight-theme',
    storage: 'Local storage',
    purpose:
      'Remembers the theme you chose (light or dark). Empty means “follow the system”, which the documentation writes the first time you open it.',
    set: 'When you open the documentation, or switch theme on any page',
    duration: UNTIL_CLEARED,
  },
  {
    name: 'starlight-synced-tabs__os',
    storage: 'Local storage',
    purpose:
      'Remembers the operating system tab you picked in the documentation, so other pages open on the same one.',
    set: 'When you click such a tab',
    duration: UNTIL_CLEARED,
  },
  {
    name: 'starlight-synced-tabs__linux-format',
    storage: 'Local storage',
    purpose: 'The same, for the Linux package format tabs (.deb, .rpm, AppImage…).',
    set: 'When you click such a tab',
    duration: UNTIL_CLEARED,
  },
  {
    name: 'sl-sidebar-state',
    storage: 'Session storage',
    purpose:
      'Keeps the documentation sidebar’s open sections and scroll position as you move between pages.',
    set: 'When you browse the documentation',
    duration: 'Until you close the tab',
  },
];

/** The prefix Starlight's `<Tabs syncKey>` stores the picked tab under. */
export const SYNCED_TABS_PREFIX = 'starlight-synced-tabs__';
