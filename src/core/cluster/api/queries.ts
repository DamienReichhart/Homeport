import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { clusterKeys } from './keys';

export const clusterQueries = {
  list: () => queryOptions({ queryKey: clusterKeys.list, queryFn: () => commands.clustersList() }),
  /** Clusters running on this machine that its kubeconfig reaches, each probed. */
  localClusters: () =>
    queryOptions({
      queryKey: clusterKeys.localClusters,
      queryFn: () => commands.clustersDetectLocal(),
      retry: false,
    }),
};

export const useClusters = () => useQuery(clusterQueries.list());
