import { useEffect, useRef, useSyncExternalStore } from 'react';
import { useRegistry } from './context';
import type { Shortcut, ShortcutOptions } from './registry';

/**
 * Runs `handler` on `keys` while the calling component is mounted, and
 * lists it in the help sheet under `group`.
 */
export function useShortcut({ keys, label, group, handler, enabled = true }: ShortcutOptions) {
  const registry = useRegistry();
  const latest = useRef(handler);
  useEffect(() => {
    latest.current = handler;
  });
  useEffect(
    () =>
      registry?.register({
        keys,
        label,
        group,
        enabled,
        handler: (event) => latest.current(event),
      }),
    [registry, keys, label, group, enabled],
  );
}

const none = (): Shortcut[] => [];
const noop = () => () => {};

/** The shortcuts active now, for the help sheet and the palette. */
export function useShortcutList(): Shortcut[] {
  const registry = useRegistry();
  return useSyncExternalStore(registry?.subscribe ?? noop, registry?.list ?? none);
}
