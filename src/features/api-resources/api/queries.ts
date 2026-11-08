import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands, subscribe, type ApiType } from '@/shared/api';

/** Discovery and the objects of each type, under the Cluster's scope. */
export const apiQueries = {
  types: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.scope(clusterId), 'api-types', namespace],
      queryFn: () => commands.clustersApiResources(clusterId, namespace),
      staleTime: 5 * 60_000,
      retry: false,
    }),
  list: (clusterId: string, api: ApiType, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.api(clusterId, api), 'list', namespace],
      queryFn: () => commands.clustersApiList(clusterId, api, namespace),
      retry: false,
    }),
  object: (clusterId: string, api: ApiType, namespace: string | null, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.api(clusterId, api), 'object', namespace, name],
      queryFn: () => commands.clustersApiGet(clusterId, api, namespace, name),
      retry: false,
    }),
};

/** Watches the objects of `api`; returns the canceller. */
export function watchApi(
  clusterId: string,
  api: ApiType,
  namespace: string | null,
  onChange: () => void,
): () => void {
  return subscribe(
    (channel) => commands.clustersApiWatchSubscribe(clusterId, api, namespace, channel),
    onChange,
  );
}

/** Where CRDs live: a change there can add or remove types. */
export const CRDS: ApiType = {
  group: 'apiextensions.k8s.io',
  version: 'v1',
  kind: 'CustomResourceDefinition',
  plural: 'customresourcedefinitions',
  namespaced: false,
};
