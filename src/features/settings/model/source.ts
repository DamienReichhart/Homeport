/** Where Homeport's source code is published. */
export const REPOSITORY = 'https://github.com/DamienReichhart/Homeport';

/**
 * The source code of exactly the running version, as the AGPL asks every
 * copy to offer: its release tag, `v0.12.0` for 0.12.0.
 */
export function sourceUrl(version: string): string {
  return `${REPOSITORY}/tree/v${version}`;
}
