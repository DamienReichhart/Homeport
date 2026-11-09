import { useSyncExternalStore } from 'react';

const QUERY = '(prefers-reduced-motion: reduce)';

const media = () =>
  typeof window === 'undefined' || !window.matchMedia ? null : window.matchMedia(QUERY);

function subscribe(listener: () => void) {
  const list = media();
  list?.addEventListener('change', listener);
  return () => list?.removeEventListener('change', listener);
}

/** Whether the OS asks for reduced motion now; follows changes while mounted. */
export const prefersReducedMotion = (): boolean => media()?.matches ?? false;

/**
 * Whether the OS asks for reduced motion, for what CSS can't reach (the
 * terminal's cursor). Re-renders when the setting changes.
 */
export function usePrefersReducedMotion(): boolean {
  return useSyncExternalStore(subscribe, prefersReducedMotion, () => false);
}
