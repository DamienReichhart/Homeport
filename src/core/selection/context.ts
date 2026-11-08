import { createContext, useContext } from 'react';
import type { Selection } from './model/selection';

export interface SelectionValue {
  /** The Host or Cluster the resource pages show, after falling back. */
  selection: Selection | null;
  select: (selection: Selection) => void;
}

export const SelectionContext = createContext<SelectionValue | null>(null);

/** The Host or Cluster the pages show, and a way to pick another. */
export function useSelection(): SelectionValue {
  const value = useContext(SelectionContext);
  if (!value) throw new Error('useSelection needs a SelectionProvider');
  return value;
}
