import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

export const clustersQueries = {
  /** The contexts of this machine's kubeconfig, read afresh each time the page opens. */
  localContexts: () =>
    queryOptions({
      queryKey: [...clusterKeys.list, 'kubeconfig'],
      queryFn: () => commands.clustersKubeconfigContexts(null),
      staleTime: 0,
      gcTime: 0,
      retry: false,
    }),
};
