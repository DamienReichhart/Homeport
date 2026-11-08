import type { ResourceKind } from '@/shared/api';

/**
 * Query keys of the Clusters and of what each one holds. Every key of a
 * Cluster starts with `clusterKeys.scope(clusterId)`, so one invalidation
 * refreshes all of it; every list of one kind of resource starts with
 * `clusterKeys.kind(clusterId, kind)`, so its watch refreshes it.
 */
export const clusterKeys = {
  list: ['clusters'] as const,
  localClusters: ['local-clusters'] as const,
  scope: (clusterId: string) => ['cluster', clusterId] as const,
  kind: (clusterId: string, kind: ResourceKind) => ['cluster', clusterId, kind] as const,
  /** An object's Events, refreshed with the Events' watch. */
  objectEvents: (clusterId: string, kind: ResourceKind, namespace: string | null, name: string) =>
    ['cluster', clusterId, 'event', 'of', kind, namespace, name] as const,
  yaml: (clusterId: string, kind: ResourceKind, namespace: string | null, name: string) =>
    ['cluster', clusterId, kind, 'yaml', namespace, name] as const,
};
