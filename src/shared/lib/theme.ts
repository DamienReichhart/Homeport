export type Theme = 'system' | 'light' | 'dark';

/** Applies the theme to the document root; CSS reacts to `data-theme`. */
export function applyTheme(theme: Theme): void {
  document.documentElement.dataset.theme = theme;
}
