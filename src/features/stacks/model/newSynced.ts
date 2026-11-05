import type { RefKind, RemoteRefs } from '@/shared/api';

/** A Stack name Compose takes, from a repository's name: `My App.git` → `my-app`. */
export function nameFromRepository(path: string): string {
  const last = path.split('/').pop() ?? '';
  return last
    .replace(/\.git$/, '')
    .toLowerCase()
    .replace(/[^a-z0-9_-]+/g, '-')
    .replace(/^[-_]+|-+$/g, '');
}

/** The refs to offer for `kind`, the default branch first. */
export function refChoices(refs: RemoteRefs | undefined, kind: RefKind): string[] {
  if (!refs) return [];
  if (kind === 'tag') return [...refs.tags].reverse();
  const first = refs.defaultBranch;
  return first && refs.branches.includes(first)
    ? [first, ...refs.branches.filter((b) => b !== first)]
    : refs.branches;
}

/** A short commit hash, as git shows it. */
export const shortCommit = (commit: string | null | undefined) => commit?.slice(0, 7) ?? '';
