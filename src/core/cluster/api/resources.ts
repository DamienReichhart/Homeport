import { queryOptions } from '@tanstack/react-query';
import { commands, subscribe, type ResourceKind } from '@/shared/api';
import { clusterKeys } from './keys';

/** What several Cluster pages read: Namespaces, an object's Events and YAML. */
export const clusterResourceQueries = {
  namespaces: (clusterId: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'namespace'), 'list'],
      queryFn: () => commands.clustersNamespaces(clusterId),
      retry: false,
    }),
  objectEvents: (clusterId: string, kind: ResourceKind, namespace: string | null, name: string) =>
    queryOptions({
      queryKey: clusterKeys.objectEvents(clusterId, kind, namespace, name),
      queryFn: () => commands.clustersObjectEvents(clusterId, kind, namespace, name),
      retry: false,
    }),
  yaml: (clusterId: string, kind: ResourceKind, namespace: string | null, name: string) =>
    queryOptions({
      queryKey: clusterKeys.yaml(clusterId, kind, namespace, name),
      queryFn: () => commands.clustersResourceYaml(clusterId, kind, namespace, name),
    }),
};

/** Watches the resources of `kind` in `namespace` (every one with `null`); returns its canceller. */
export function watchResources(
  clusterId: string,
  kind: ResourceKind,
  namespace: string | null,
  onChange: () => void,
): () => void {
  return subscribe(
    (channel) => commands.clustersWatchSubscribe(clusterId, kind, namespace, channel),
    onChange,
  );
}
