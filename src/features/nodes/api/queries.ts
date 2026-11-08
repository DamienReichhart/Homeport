import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

export const nodeQueries = {
  list: (clusterId: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'node'), 'list'],
      queryFn: () => commands.clustersNodesList(clusterId),
      retry: false,
    }),
  detail: (clusterId: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'node'), 'detail', name],
      queryFn: () => commands.clustersNodeGet(clusterId, name),
      retry: false,
    }),
};
