import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import type { ContainerInspect } from '../model/types';

const containerKey = (hostId: string, containerId: string) =>
  [...hostKeys.containers(hostId), containerId] as const;

export const containerQueries = {
  /** The inspect document, refetched when the container's state changes. */
  inspect: (hostId: string, containerId: string, state?: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'inspect', state],
      queryFn: () =>
        commands.containersInspect(
          hostId,
          containerId,
        ) as Promise<unknown> as Promise<ContainerInspect>,
    }),
  files: (hostId: string, containerId: string, path: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'files', path],
      queryFn: () => commands.filesList(hostId, containerId, path),
    }),
  processes: (hostId: string, containerId: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'processes'],
      queryFn: () => commands.containersProcesses(hostId, containerId),
    }),
};
