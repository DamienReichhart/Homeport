import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const containerdContainerQueries = {
  /** The containers of `namespace`, each with its task; task events refresh them. */
  list: (hostId: string, namespace: string) =>
    queryOptions({
      queryKey: hostKeys.containerdContainers(hostId, namespace),
      queryFn: () => commands.containerdContainers(hostId, namespace),
    }),
  /** One container with its spec; under the list, so the same events refresh it. */
  inspect: (hostId: string, namespace: string, id: string) =>
    queryOptions({
      queryKey: [...hostKeys.containerdContainers(hostId, namespace), 'inspect', id],
      queryFn: () => commands.containerdContainerInspect(hostId, namespace, id),
    }),
  /** Where a container's output was kept, if anywhere. */
  logsSource: (hostId: string, namespace: string, id: string) =>
    queryOptions({
      queryKey: [...hostKeys.containerdContainers(hostId, namespace), 'logs-source', id],
      queryFn: () => commands.containerdLogsSource(hostId, namespace, id),
    }),
};
