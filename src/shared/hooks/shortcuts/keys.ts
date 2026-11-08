/**
 * Keyboard shortcuts as written in code, `Mod+K`, `Alt+ArrowUp`, `?`: parsing,
 * matching key presses, and the keys to show. `Mod` is ⌘ on macOS and Ctrl
 * elsewhere. Pure, so it's tested without a DOM.
 */

export interface Combo {
  /** ⌘ on macOS, Ctrl elsewhere. */
  mod: boolean;
  alt: boolean;
  shift: boolean;
  /** `KeyboardEvent.key`, lower case for letters. */
  key: string;
}

/** The parts of a key press matching needs. */
export interface KeyPress {
  key: string;
  metaKey: boolean;
  ctrlKey: boolean;
  altKey: boolean;
  shiftKey: boolean;
}

/** Where a key press happened, which decides what may fire there. */
export type KeyTarget = 'terminal' | 'text' | 'dialog' | 'page';

export function parseKeys(spec: string): Combo {
  // `Mod++` would be ambiguous; no shortcut uses `+` as its key.
  const parts = spec.split('+');
  const key = parts.pop() ?? '';
  const mods = new Set(parts.map((p) => p.toLowerCase()));
  return {
    mod: mods.has('mod'),
    alt: mods.has('alt'),
    shift: mods.has('shift'),
    key: key.length === 1 ? key.toLowerCase() : key,
  };
}

/** Whether `press` is the shortcut `combo`. */
export function matches(combo: Combo, press: KeyPress, mac: boolean): boolean {
  const mod = mac ? press.metaKey : press.ctrlKey;
  const other = mac ? press.ctrlKey : press.metaKey;
  if (mod !== combo.mod || other || press.altKey !== combo.alt) return false;
  const key = press.key.length === 1 ? press.key.toLowerCase() : press.key;
  if (key !== combo.key) return false;
  // Letters and named keys care about Shift; `?` or `/` are typed with
  // whatever the layout needs.
  const cares = /^[a-z]$/.test(combo.key) || combo.key.length > 1;
  return !cares || press.shiftKey === combo.shift;
}

/**
 * Whether `combo` may fire at `target`. Nothing fires in the terminal,
 * which needs every key. Combinations with Mod fire everywhere else; plain
 * keys (and Alt combinations, which editors use) only outside text fields
 * and dialogs, so typing `n` or `/` types it.
 */
export function firesAt(combo: Combo, target: KeyTarget): boolean {
  if (target === 'terminal') return false;
  if (combo.mod) return true;
  return target === 'page';
}

/** Where `element` is, for {@link firesAt}. */
export function targetOf(element: EventTarget | null): KeyTarget {
  if (!(element instanceof Element)) return 'page';
  if (element.closest('.xterm')) return 'terminal';
  if (
    element.closest(
      'input, textarea, select, [contenteditable=""], [contenteditable="true"], .cm-editor',
    )
  ) {
    return 'text';
  }
  if (element.closest('[role="dialog"], [role="alertdialog"], [role="menu"], [role="listbox"]')) {
    return 'dialog';
  }
  return 'page';
}

const NAMED: Record<string, [mac: string, other: string]> = {
  ArrowUp: ['↑', '↑'],
  ArrowDown: ['↓', '↓'],
  ArrowLeft: ['←', '←'],
  ArrowRight: ['→', '→'],
  Enter: ['↵', 'Enter'],
  Escape: ['Esc', 'Esc'],
};

/** The keys to show for `spec`, in order: `['⌘', 'K']` or `['Ctrl', 'K']`. */
export function formatKeys(spec: string, mac: boolean): string[] {
  const combo = parseKeys(spec);
  const keys: string[] = [];
  if (combo.mod) keys.push(mac ? '⌘' : 'Ctrl');
  if (combo.alt) keys.push(mac ? '⌥' : 'Alt');
  if (combo.shift) keys.push(mac ? '⇧' : 'Shift');
  const named = NAMED[combo.key];
  keys.push(named ? named[mac ? 0 : 1] : combo.key.toUpperCase());
  return keys;
}

/** Whether this is a Mac, where Mod is ⌘. */
export function isMac(): boolean {
  if (typeof navigator === 'undefined') return false;
  return /mac/i.test(navigator.platform || navigator.userAgent);
}
