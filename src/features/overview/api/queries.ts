import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

export const overviewQueries = {
  /**
   * A Cluster at a glance, counted by the backend; asked again every 30 s
   * while the Overview is open and visible.
   */
  clusterSummary: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.scope(clusterId), 'summary', namespace],
      queryFn: () => commands.clustersSummary(clusterId, namespace),
      refetchInterval: 30_000,
      retry: false,
    }),
};
