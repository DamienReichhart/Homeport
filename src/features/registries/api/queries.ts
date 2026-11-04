import { infiniteQueryOptions, queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

/** Logins are app-wide, not a Host's, so their keys sit outside `hostKeys`. */
export const registryKeys = {
  all: ['registries'] as const,
  list: ['registries', 'list'] as const,
  loginFor: (reference: string) => ['registries', 'login-for', reference] as const,
  hubTags: (repository: string, filter: string) =>
    ['registries', 'hub-tags', repository, filter] as const,
};

export const registryQueries = {
  /** Every stored login, without its secret. */
  list: () =>
    queryOptions({ queryKey: registryKeys.list, queryFn: () => commands.registriesList() }),
  /** The login an image reference would be pulled or pushed with; `null` for anonymous. */
  loginFor: (reference: string) =>
    queryOptions({
      queryKey: registryKeys.loginFor(reference),
      queryFn: () => commands.registriesLoginFor(reference),
      enabled: reference.trim() !== '',
    }),
};

/** A Docker Hub repository's tags, newest first, a page at a time. */
export const hubTagsQuery = (repository: string, filter: string) =>
  infiniteQueryOptions({
    queryKey: registryKeys.hubTags(repository, filter),
    queryFn: ({ pageParam }) => commands.registriesHubTags(repository, pageParam, filter),
    initialPageParam: 1,
    getNextPageParam: (page) => page.nextPage ?? undefined,
    staleTime: 5 * 60_000,
    retry: false,
  });
