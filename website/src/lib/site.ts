/** Facts about the project that several pages repeat. */
export const REPO = 'DamienReichhart/Homeport';
export const REPO_URL = `https://github.com/${REPO}`;
export const RELEASES_URL = `${REPO_URL}/releases`;
export const ISSUES_URL = `${REPO_URL}/issues`;
export const TAGLINE = 'Every Docker Engine, docked in one place.';
export const DESCRIPTION =
  'Homeport is a free, open-source desktop app for managing Docker Engines anywhere — this machine’s, or remote ones over SSH, TLS or TCP — for Windows, macOS and Linux.';

/** Prefixes a site path with the configured base, e.g. `/docs/` → `/Homeport/docs/`. */
export function href(path: string): string {
  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  return `${base}${path.startsWith('/') ? path : `/${path}`}`;
}
