import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

/** helm has no watch: open lists ask again this often. */
export const REFRESH_MS = 30_000;

/** Keys of what helm says about a Cluster, under the Cluster's scope. */
export const helmKeys = {
  /** helm's own chart repositories on this machine, the same for every Cluster. */
  repositories: ['helm-repositories'] as const,
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
  repositories: () =>
    queryOptions({
      queryKey: [...helmKeys.repositories, 'list'],
      queryFn: () => commands.helmReposList(),
      retry: false,
    }),
  /** Charts in the repositories matching `query`, latest versions only or every one. */
  charts: (query: string, versions: boolean) =>
    queryOptions({
      queryKey: [...helmKeys.repositories, 'charts', query, versions],
      queryFn: () => commands.helmChartsSearch(query, versions),
      staleTime: 60_000,
      retry: false,
    }),
  /** A chart's default values, at `version` or its latest; fixed per version. */
  chartValues: (chart: string, version: string | null) =>
    queryOptions({
      queryKey: [...helmKeys.repositories, 'values', chart, version],
      queryFn: () => commands.helmChartValues(chart, version),
      staleTime: version ? Infinity : 60_000,
      retry: false,
    }),
  /** Every version of a repository chart, newest first. */
  chartVersions: (chart: string) =>
    queryOptions({
      queryKey: [...helmKeys.repositories, 'versions', chart],
      queryFn: () => commands.helmChartVersions(chart),
      staleTime: 60_000,
      retry: false,
    }),
  /** The repository charts a release of `chart` at `version` can be upgraded from. */
  chartCandidates: (chart: string, version: string) =>
    queryOptions({
      queryKey: [...helmKeys.repositories, 'candidates', chart, version],
      queryFn: () => commands.helmChartCandidates(chart, version),
      staleTime: 60_000,
      retry: false,
    }),
};
