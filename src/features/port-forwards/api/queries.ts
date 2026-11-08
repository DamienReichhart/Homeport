import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

/** Forwards live in the backend's memory, across Clusters. */
export const forwardKeys = {
  all: ['port-forwards'] as const,
};

export const forwardQueries = {
  /** Every active forward; polled while shown, for its connections. */
  list: () =>
    queryOptions({
      queryKey: [...forwardKeys.all, 'list'],
      queryFn: () => commands.clustersForwardList(),
      refetchInterval: 2_000,
    }),
  /** Whether `port` is free on this computer, to propose it. */
  free: (port: number) =>
    queryOptions({
      queryKey: [...forwardKeys.all, 'free', port],
      queryFn: () => commands.clustersForwardPortFree(port),
      staleTime: 0,
      gcTime: 0,
    }),
};
