/**
 * Query keys of the Clusters and of what each one holds. Every key of a
 * Cluster starts with `clusterKeys.scope(clusterId)`, so one invalidation
 * refreshes all of it.
 */
export const clusterKeys = {
  list: ['clusters'] as const,
  scope: (clusterId: string) => ['cluster', clusterId] as const,
};
