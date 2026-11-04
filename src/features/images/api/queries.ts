import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const imageQueries = {
  /** Docker Hub repositories matching `term`, searched from the Host. */
  search: (hostId: string, term: string) =>
    queryOptions({
      queryKey: [...hostKeys.images(hostId), 'search', term],
      queryFn: () => commands.imagesSearch(hostId, term),
      enabled: term.length >= 2,
      staleTime: 5 * 60_000,
      retry: false,
    }),
  /** The layers an image was built from. */
  history: (hostId: string, imageId: string) =>
    queryOptions({
      queryKey: [...hostKeys.images(hostId), imageId, 'history'],
      queryFn: () => commands.imagesHistory(hostId, imageId),
    }),
  inspect: (hostId: string, imageId: string) =>
    queryOptions({
      queryKey: [...hostKeys.images(hostId), imageId, 'inspect'],
      queryFn: () => commands.imagesInspect(hostId, imageId),
    }),
};
