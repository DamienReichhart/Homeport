import { createContext, useContext } from 'react';

export interface PaletteState {
  open: boolean;
  setOpen: (open: boolean) => void;
}

export const PaletteContext = createContext<PaletteState | null>(null);

/** Whether the command palette is open, and a way to open it from anywhere in the shell. */
export function usePalette(): PaletteState {
  const value = useContext(PaletteContext);
  if (!value) throw new Error('usePalette needs a PaletteProvider');
  return value;
}
