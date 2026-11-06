import { useEffect, useState, type ReactNode } from 'react';
import { ShortcutsContext } from './context';
import { ShortcutRegistry } from './registry';

/** Listens for the keyboard shortcuts its children register. */
export function ShortcutsProvider({ children }: { children: ReactNode }) {
  const [registry] = useState(() => new ShortcutRegistry());
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => void registry.dispatch(event);
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [registry]);
  return <ShortcutsContext.Provider value={registry}>{children}</ShortcutsContext.Provider>;
}
