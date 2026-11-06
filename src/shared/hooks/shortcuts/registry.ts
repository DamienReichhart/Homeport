import { firesAt, isMac, matches, parseKeys, targetOf, type Combo } from './keys';

export interface ShortcutOptions {
  /** As in `Mod+K`, `Alt+ArrowUp`, `n`, `?` (see `parseKeys`). */
  keys: string;
  /** What it does, as the help sheet lists it. */
  label: string;
  /** The help sheet's heading it's listed under. */
  group: string;
  handler: (event: KeyboardEvent) => void;
  /** Off for now, e.g. while its page has nothing to act on. Defaults to on. */
  enabled?: boolean;
}

/** A registered shortcut, as the help sheet and the palette read it. */
export interface Shortcut {
  keys: string;
  label: string;
  group: string;
}

interface Entry extends ShortcutOptions {
  id: number;
  combo: Combo;
}

/**
 * Every shortcut on screen. One key listener dispatches to the most recently
 * registered match, so a page's shortcut wins over the app's for the same keys.
 */
export class ShortcutRegistry {
  private entries: Entry[] = [];
  private next = 0;
  private listeners = new Set<() => void>();
  private snapshot: Shortcut[] = [];
  private readonly mac = isMac();

  register(options: ShortcutOptions): () => void {
    const entry: Entry = { ...options, id: this.next++, combo: parseKeys(options.keys) };
    this.entries.push(entry);
    this.changed();
    return () => {
      this.entries = this.entries.filter((e) => e.id !== entry.id);
      this.changed();
    };
  }

  /** Handles `event` with the shortcut it's for; whether one ran. */
  dispatch(event: KeyboardEvent): boolean {
    if (event.defaultPrevented || event.isComposing) return false;
    const target = targetOf(event.target);
    for (let i = this.entries.length - 1; i >= 0; i--) {
      const entry = this.entries[i];
      if (!entry || entry.enabled === false) continue;
      if (!matches(entry.combo, event, this.mac) || !firesAt(entry.combo, target)) continue;
      event.preventDefault();
      entry.handler(event);
      return true;
    }
    return false;
  }

  /** The active shortcuts, once each, in the order they were registered. */
  list = (): Shortcut[] => this.snapshot;

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  private changed() {
    const seen = new Set<string>();
    this.snapshot = this.entries
      .filter((e) => e.enabled !== false)
      .filter((e) => {
        const key = `${e.keys}\n${e.label}`;
        if (seen.has(key)) return false;
        seen.add(key);
        return true;
      })
      .map(({ keys, label, group }) => ({ keys, label, group }));
    for (const listener of this.listeners) listener();
  }
}
