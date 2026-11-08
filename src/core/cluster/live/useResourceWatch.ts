import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import type { ResourceKind } from '@/shared/api';
import { clusterKeys } from '../api/keys';
import { watchResources } from '../api/resources';

/**
 * While mounted, watches the resources of `kind` in `namespace` (every one
 * with `null`) and refreshes every query of that kind on the Cluster when
 * they change, instead of polling.
 */
export function useResourceWatch(clusterId: string, kind: ResourceKind, namespace: string | null) {
  const client = useQueryClient();
  useEffect(
    () =>
      watchResources(
        clusterId,
        kind,
        namespace,
        () => void client.invalidateQueries({ queryKey: clusterKeys.kind(clusterId, kind) }),
      ),
    [client, clusterId, kind, namespace],
  );
}
