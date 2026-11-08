import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

export const networkQueries = {
  services: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'service'), 'list', namespace],
      queryFn: () => commands.clustersServicesList(clusterId, namespace),
      retry: false,
    }),
  service: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'service'), 'detail', namespace, name],
      queryFn: () => commands.clustersServiceGet(clusterId, namespace, name),
      retry: false,
    }),
  ingresses: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'ingress'), 'list', namespace],
      queryFn: () => commands.clustersIngressesList(clusterId, namespace),
      retry: false,
    }),
  ingress: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'ingress'), 'detail', namespace, name],
      queryFn: () => commands.clustersIngressGet(clusterId, namespace, name),
      retry: false,
    }),
};
