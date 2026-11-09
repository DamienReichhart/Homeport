/** Facts about the project that several pages repeat. */
export const REPO = 'DamienReichhart/Homeport';
export const REPO_URL = `https://github.com/${REPO}`;
export const RELEASES_URL = `${REPO_URL}/releases`;
export const ISSUES_URL = `${REPO_URL}/issues`;
export const TAGLINE = 'Every Docker Engine, on one board.';
export const DESCRIPTION =
  'Homeport is a free, open-source desktop app for managing Docker Engines anywhere — this machine’s, or remote ones over SSH, TLS or TCP — for Windows, macOS and Linux.';
export {
  LOCALES,
  PREFIXED_LOCALES,
  ROOT_LOCALE,
  localeOf,
  localePath,
  unlocalized,
} from './locales';
export type { Locale } from './locales';
