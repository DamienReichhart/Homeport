import { useQueries, useQuery } from '@tanstack/react-query';
import { chosenNamespace, useClusters } from '@/core/cluster';
import { settingsQuery } from '@/core/settings';
import type { WorkloadView } from '@/shared/api';
import { overviewQueries } from './queries';

export interface ClusterUnready {
  clusterId: string;
  clusterName: string;
  workloads: WorkloadView[];
}

/**
 * The workloads that aren't ready on every connected Kubernetes Cluster, in
 * the Namespace each one shows, from the same summary as its Board tile.
 */
export function useClusterUnready(): ClusterUnready[] {
  const clusters = useClusters();
  const settings = useQuery(settingsQuery());
  const connected = (clusters.data ?? []).filter(
    (c) => c.status.state === 'connected' && c.cluster.kind === 'kubernetes',
  );
  const summaries = useQueries({
    queries: connected.map((c) =>
      overviewQueries.clusterSummary(
        c.cluster.id,
        chosenNamespace(c, settings.data?.clusterNamespaces),
      ),
    ),
  });
  return connected.flatMap((c, i) => {
    const unready = summaries[i]?.data?.unready;
    return unready?.state === 'read' && unready.value.length > 0
      ? [{ clusterId: c.cluster.id, clusterName: c.cluster.name, workloads: unready.value }]
      : [];
  });
}
