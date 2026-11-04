import { registryOf } from '@/features/registries';

/**
 * The Docker Hub search term a typed reference stands for, or `null` when
 * there's nothing to search: too short, a tag or digest already given, or
 * another registry named.
 */
export function searchTerm(reference: string): string | null {
  const term = reference.trim();
  if (term.length < 2 || /[:@\s]/.test(term)) return null;
  if (registryOf(term) !== 'docker.io' || term.startsWith('docker.io/')) return null;
  return term;
}

/** `library/postgres` and `postgres` are the same repository; show the short name. */
export const shortRepository = (name: string) => name.replace(/^library\//, '');
