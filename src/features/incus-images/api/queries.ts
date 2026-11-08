import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const incusImageQueries = {
  /** The images of `project`, newest first; image events refresh them. */
  list: (hostId: string, project: string) =>
    queryOptions({
      queryKey: hostKeys.incus(hostId, 'images', project),
      queryFn: () => commands.incusImages(hostId, project),
    }),
  /** What a remote image server publishes for the Host's architecture. */
  remote: (hostId: string, server: string) =>
    queryOptions({
      queryKey: ['incus-remote-images', hostId, server],
      queryFn: () => commands.incusRemoteImages(hostId, server),
      staleTime: 10 * 60 * 1000,
      retry: false,
    }),
};
