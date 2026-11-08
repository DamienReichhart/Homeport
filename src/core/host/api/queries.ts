import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { EVENT_PAGE } from '../model/events';
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
  /**
   * Each volume's size, `null` when its driver doesn't report one. Slow on a
   * large Host, so it's apart from the list and kept a minute; volume events
   * and the Volumes page's refresh ask again.
   */
  volumeSizes: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.volumeSizes(hostId),
      queryFn: () => commands.volumesSizes(hostId),
      staleTime: 60_000,
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
  /**
   * Whether a newer image is published for each container's image and each
   * tag. The backend caches registry answers, so this is cheap to refetch.
   */
  freshness: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.freshness(hostId),
      queryFn: () => commands.imagesFreshness(hostId, false),
      staleTime: 5 * 60_000,
      retry: false,
    }),
  /** A Podman Host's pods; container events make them stale. */
  pods: (hostId: string) =>
    queryOptions({ queryKey: hostKeys.pods(hostId), queryFn: () => commands.podsList(hostId) }),
  /** The latest page of a Host's kept events; live updates append to it. */
  events: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.events(hostId),
      queryFn: () => commands.hostsEvents(hostId, EVENT_PAGE, null),
    }),
};

export const useHosts = () => useQuery(hostQueries.list());

export const useContainers = (hostId: string) => useQuery(resourceQueries.containers(hostId));
