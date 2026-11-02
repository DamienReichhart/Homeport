import { useSyncExternalStore } from 'react';

export type Theme = 'system' | 'light' | 'dark';
export type ResolvedTheme = 'light' | 'dark';

let current: Theme = 'system';
const listeners = new Set<() => void>();

const darkQuery = () =>
  typeof window !== 'undefined' && typeof window.matchMedia === 'function'
    ? window.matchMedia('(prefers-color-scheme: dark)')
    : undefined;

function resolve(theme: Theme): ResolvedTheme {
  if (theme !== 'system') return theme;
  return darkQuery()?.matches ? 'dark' : 'light';
}

function render() {
  const root = document.documentElement;
  root.dataset.theme = current;
  root.classList.toggle('dark', resolve(current) === 'dark');
  listeners.forEach((l) => l());
}

darkQuery()?.addEventListener('change', () => current === 'system' && render());

/** Applies the theme to the document root; Tailwind's `dark:` variant reacts to `.dark`. */
export function applyTheme(theme: Theme): void {
  current = theme;
  render();
}

function subscribe(listener: () => void) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** The theme actually shown, with `system` resolved against the OS preference. */
export function useResolvedTheme(): ResolvedTheme {
  return useSyncExternalStore(subscribe, () => resolve(current));
}
