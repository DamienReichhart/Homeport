import { createContext, useContext } from 'react';

export const ShortcutsHelpContext = createContext<(() => void) | null>(null);

/** Opens the keyboard shortcuts help sheet. */
export function useOpenShortcutsHelp(): () => void {
  return useContext(ShortcutsHelpContext) ?? noop;
}

function noop() {}
