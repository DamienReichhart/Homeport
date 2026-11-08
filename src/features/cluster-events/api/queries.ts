import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

export const clusterEventQueries = {
  list: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'event'), 'list', namespace],
      queryFn: () => commands.clustersEventsList(clusterId, namespace),
      retry: false,
    }),
};
