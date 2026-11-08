import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands } from '@/shared/api';

/** Pods' keys sit under the Pod kind, so the Pods' watch refreshes them. */
export const podQueries = {
  list: (
    clusterId: string,
    namespace: string | null,
    filter: { selector?: string; node?: string } = {},
  ) =>
    queryOptions({
      queryKey: [
        ...clusterKeys.kind(clusterId, 'pod'),
        'list',
        namespace,
        filter.selector ?? null,
        filter.node ?? null,
      ],
      queryFn: () =>
        commands.clustersPodsList(
          clusterId,
          namespace,
          filter.selector ?? null,
          filter.node ?? null,
        ),
      retry: false,
    }),
  detail: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'pod'), 'detail', namespace, name],
      queryFn: () => commands.clustersPodGet(clusterId, namespace, name),
      retry: false,
    }),
};
