import { useCallback, useMemo } from 'react';
import { useSelection } from '@/core/selection';
import type { ClusterView } from '@/shared/api';
import { useClusters } from '../api/queries';

export interface ClusterSelection {
  /** The selected Cluster, `null` when a Host or nothing is selected. */
  clusterId: string | null;
  cluster: ClusterView | null;
  select: (clusterId: string) => void;
}

/** The Cluster the Cluster pages show, and a way to pick another. */
export function useClusterSelection(): ClusterSelection {
  const { selection, select: choose } = useSelection();
  const clusters = useClusters();
  const clusterId = selection?.kind === 'cluster' ? selection.id : null;
  const select = useCallback((id: string) => choose({ kind: 'cluster', id }), [choose]);
  const list = clusters.data;
  return useMemo(
    () => ({
      clusterId,
      cluster: list?.find((c) => c.cluster.id === clusterId) ?? null,
      select,
    }),
    [clusterId, list, select],
  );
}
