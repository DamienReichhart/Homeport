import { createContext, useContext } from 'react';
import type { HostView } from '@/shared/api';

export interface HostSelection {
  hostId: string | null;
  host: HostView | null;
  select: (hostId: string) => void;
}

export const HostSelectionContext = createContext<HostSelection | null>(null);

/** The Host the resource pages show, and a way to pick another. */
export function useHostSelection(): HostSelection {
  const value = useContext(HostSelectionContext);
  if (!value) throw new Error('useHostSelection needs a HostProvider');
  return value;
}
