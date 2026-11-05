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
  /**
   * The healthcheck and its last results, refetched when the status in the
   * list changes and, while shown, once per check interval.
   */
  health: (hostId: string, containerId: string, status: string | null) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'health', status],
      queryFn: () => commands.containersHealth(hostId, containerId),
      refetchInterval: (query) => (query.state.data?.config.intervalS ?? 30) * 1000,
    }),
  files: (hostId: string, containerId: string, path: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'files', path],
      queryFn: () => commands.filesList(hostId, containerId, path),
    }),
  /** The Run dialog's form for this container, for Duplicate and Edit. */
  runSpec: (hostId: string, containerId: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'run-spec'],
      queryFn: () => commands.containersRunSpec(hostId, containerId),
      staleTime: 0,
      gcTime: 0,
    }),
  /** What recreating this container would lose, for Edit's confirmation. */
  editLosses: (hostId: string, containerId: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'edit-losses'],
      queryFn: () => commands.containersEditLosses(hostId, containerId),
      staleTime: 0,
      gcTime: 0,
    }),
  processes: (hostId: string, containerId: string) =>
    queryOptions({
      queryKey: [...containerKey(hostId, containerId), 'processes'],
      queryFn: () => commands.containersProcesses(hostId, containerId),
    }),
};
