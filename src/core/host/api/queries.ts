import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { hostKeys } from './keys';

export const hostQueries = {
  list: () => queryOptions({ queryKey: hostKeys.list, queryFn: () => commands.hostsList() }),
  /** Engines answering on this machine's sockets, Hosts or not. */
  localEngines: () =>
    queryOptions({ queryKey: hostKeys.localEngines, queryFn: () => commands.hostsDetectLocal() }),
};

/**
 * The Engine resources of a Host. Several features read them, and Engine
 * events make them stale (see `useEngineEventUpdates`).
 */
export const resourceQueries = {
  containers: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.containers(hostId),
      queryFn: () => commands.containersList(hostId),
    }),
  images: (hostId: string) =>
    queryOptions({ queryKey: hostKeys.images(hostId), queryFn: () => commands.imagesList(hostId) }),
  volumes: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.volumes(hostId),
      queryFn: () => commands.volumesList(hostId),
    }),
  networks: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.networks(hostId),
      queryFn: () => commands.networksList(hostId),
    }),
  stacks: (hostId: string) =>
    queryOptions({ queryKey: hostKeys.stacks(hostId), queryFn: () => commands.stacksList(hostId) }),
  systemInfo: (hostId: string) =>
    queryOptions({
      queryKey: [...hostKeys.system(hostId), 'info'],
      queryFn: () => commands.systemInfo(hostId),
    }),
  diskUsage: (hostId: string) =>
    queryOptions({
      queryKey: [...hostKeys.system(hostId), 'df'],
      queryFn: () => commands.systemDiskUsage(hostId),
    }),
  events: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.events(hostId),
      queryFn: () => commands.hostsEvents(hostId),
    }),
};

export const useHosts = () => useQuery(hostQueries.list());

export const useContainers = (hostId: string) => useQuery(resourceQueries.containers(hostId));
