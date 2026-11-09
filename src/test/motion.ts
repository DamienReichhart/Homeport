/** Stands in for the OS's reduced-motion setting; `set` changes it live. */
export function mockReducedMotion(reduce: boolean) {
  const original = window.matchMedia;
  const listeners = new Set<() => void>();
  let matches = reduce;
  window.matchMedia = (query: string) =>
    ({
      get matches() {
        return query.includes('prefers-reduced-motion') ? matches : false;
      },
      media: query,
      onchange: null,
      addEventListener: (_: string, l: () => void) => listeners.add(l),
      removeEventListener: (_: string, l: () => void) => listeners.delete(l),
      addListener: () => {},
      removeListener: () => {},
      dispatchEvent: () => false,
    }) as unknown as MediaQueryList;
  return {
    set(next: boolean) {
      matches = next;
      for (const l of listeners) l();
    },
    restore() {
      window.matchMedia = original;
    },
  };
}
