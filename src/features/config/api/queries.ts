import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands, type RevealedValue } from '@/shared/api';

export const configQueries = {
  configMaps: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'configMap'), 'list', namespace],
      queryFn: () => commands.clustersConfigMapsList(clusterId, namespace),
      retry: false,
    }),
  configMap: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'configMap'), 'detail', namespace, name],
      queryFn: () => commands.clustersConfigMapGet(clusterId, namespace, name),
      retry: false,
    }),
  /** Secrets' keys and sizes; values are never part of a query. */
  secrets: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'secret'), 'list', namespace],
      queryFn: () => commands.clustersSecretsList(clusterId, namespace),
      retry: false,
    }),
  secret: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'secret'), 'detail', namespace, name],
      queryFn: () => commands.clustersSecretGet(clusterId, namespace, name),
      retry: false,
    }),
};

/**
 * One value of a Secret, read when the user reveals it. Deliberately not a
 * query or a mutation: neither cache may keep it.
 */
export const revealSecretValue = (
  clusterId: string,
  namespace: string,
  name: string,
  key: string,
): Promise<RevealedValue> => commands.clustersSecretReveal(clusterId, namespace, name, key);
