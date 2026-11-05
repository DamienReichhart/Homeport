import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

/** Repositories are app-wide, not a Host's, so their keys sit outside `hostKeys`. */
export const repositoryKeys = {
  all: ['repositories'] as const,
  list: ['repositories', 'list'] as const,
  refs: (id: string) => ['repositories', 'refs', id] as const,
};

export const repositoryQueries = {
  /** Every stored repository, without its key or token. */
  list: () =>
    queryOptions({ queryKey: repositoryKeys.list, queryFn: () => commands.repositoriesList() }),
  /** A repository's branches and tags, read from its server ("Test access"). */
  refs: (id: string | null) =>
    queryOptions({
      queryKey: repositoryKeys.refs(id ?? ''),
      queryFn: () => commands.repositoriesTest(id ?? ''),
      enabled: id !== null,
      retry: false,
      staleTime: 60_000,
    }),
};
