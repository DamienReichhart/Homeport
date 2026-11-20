import { useMemo, useState, type ReactNode } from 'react';
import { PaletteContext } from './paletteContext';

/** Holds whether the command palette is open, so the sidebar's search button can open it. */
export function PaletteProvider({ children }: { children: ReactNode }) {
  const [open, setOpen] = useState(false);
  const value = useMemo(() => ({ open, setOpen }), [open]);
  return <PaletteContext.Provider value={value}>{children}</PaletteContext.Provider>;
}
