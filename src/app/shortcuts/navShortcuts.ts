import { navEntries } from '../router/features';

/** Mod+1…9 open the sidebar's entries in order. */
export const NAV_SHORTCUTS = navEntries.slice(0, 9).map((entry, i) => ({
  entry,
  keys: `Mod+${i + 1}`,
}));
