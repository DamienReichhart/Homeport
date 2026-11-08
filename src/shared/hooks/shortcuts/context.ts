import { createContext, useContext } from 'react';
import type { ShortcutRegistry } from './registry';

export const ShortcutsContext = createContext<ShortcutRegistry | null>(null);

/** The app's shortcuts; `null` outside a `ShortcutsProvider` (a component rendered alone). */
export function useRegistry(): ShortcutRegistry | null {
  return useContext(ShortcutsContext);
}
