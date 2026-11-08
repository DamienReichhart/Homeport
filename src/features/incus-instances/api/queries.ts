import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const incusInstanceQueries = {
  /** The Instances of `project`, with their state; lifecycle events refresh them. */
  list: (hostId: string, project: string) =>
    queryOptions({
      queryKey: hostKeys.incus(hostId, 'instances', project),
      queryFn: () => commands.incusInstances(hostId, project),
    }),
  /** One Instance with its configuration; under the list, so the same events refresh it. */
  detail: (hostId: string, project: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'instances', project), 'detail', name],
      queryFn: () => commands.incusInstance(hostId, project, name),
    }),
  /** An Instance's snapshots, oldest first; snapshot events refresh them. */
  snapshots: (hostId: string, project: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'instances', project), 'snapshots', name],
      queryFn: () => commands.incusSnapshots(hostId, project, name),
    }),
  /** What the Instance printed on its console before attaching, if Incus kept it. */
  consoleLog: (hostId: string, project: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'instances', project), 'console-log', name],
      queryFn: () => commands.incusConsoleLog(hostId, project, name),
      staleTime: Infinity,
    }),
};
