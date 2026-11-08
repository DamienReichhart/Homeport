import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

/** helm has no watch: open lists ask again this often. */
export const REFRESH_MS = 30_000;

/** Keys of what helm says about a Cluster, under the Cluster's scope. */
export const helmKeys = {
  all: (clusterId: string) => [...clusterKeys.scope(clusterId), 'helm'] as const,
  release: (clusterId: string, namespace: string, name: string) =>
    [...helmKeys.all(clusterId), 'release', namespace, name] as const,
};

export const helmQueries = {
  releases: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...helmKeys.all(clusterId), 'releases', namespace],
      queryFn: () => commands.clustersHelmReleases(clusterId, namespace),
      refetchInterval: REFRESH_MS,
      retry: false,
    }),
  history: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...helmKeys.release(clusterId, namespace, name), 'history'],
      queryFn: () => commands.clustersHelmHistory(clusterId, namespace, name),
      refetchInterval: REFRESH_MS,
      retry: false,
    }),
  details: (clusterId: string, namespace: string, name: string, revision: number) =>
    queryOptions({
      queryKey: [...helmKeys.release(clusterId, namespace, name), 'details', revision],
      queryFn: () => commands.clustersHelmReleaseDetails(clusterId, namespace, name, revision),
      // A revision never changes once written.
      staleTime: Infinity,
      retry: false,
    }),
  /** The types the Cluster serves, as the API resources pages discover them (same key). */
  apiTypes: (clusterId: string) =>
    queryOptions({
      queryKey: [...clusterKeys.scope(clusterId), 'api-types', null],
      queryFn: () => commands.clustersApiResources(clusterId, null),
      staleTime: 5 * 60_000,
      retry: false,
    }),
};
