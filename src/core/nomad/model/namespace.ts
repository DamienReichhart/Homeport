import type { NomadNamespaces } from '@/shared/api';

/** What Nomad's API calls every namespace the token can read. */
export const ALL_NOMAD_NAMESPACES = '*';

/**
 * The namespace the Jobs page asks Nomad for: `null` (the token's default)
 * while the namespaces are unknown or can't be listed, `*` for all, the
 * one chosen, or `default` when the chosen one is gone.
 */
export function effectiveNamespace(
  listing: NomadNamespaces | undefined,
  choice: string | null,
): string | null {
  if (!listing?.listable) return null;
  if (choice === null) return ALL_NOMAD_NAMESPACES;
  return listing.namespaces.some((n) => n.name === choice) ? choice : 'default';
}
