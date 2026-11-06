import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

/**
 * Keys outside `hostKeys.volumes`, which container events invalidate: the
 * helper's image and browse listings shouldn't refetch whenever a container
 * starts.
 */
export const volumeKeys = {
  helperImage: (hostId: string) => [...hostKeys.scope(hostId), 'volume-helper-image'] as const,
  browse: (hostId: string, session: string) =>
    [...hostKeys.scope(hostId), 'volume-browse', session] as const,
};

export const volumeQueries = {
  inspect: (hostId: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.volumes(hostId), name],
      queryFn: () => commands.volumesInspect(hostId, name),
    }),
  /** The helper container's image (ADR 0015), and whether the Host has it. */
  helperImage: (hostId: string) =>
    queryOptions({
      queryKey: volumeKeys.helperImage(hostId),
      queryFn: () => commands.volumesHelperImage(hostId),
      staleTime: 0,
    }),
  /** The running containers using a volume, asked afresh each time. */
  runningUsers: (hostId: string, volume: string) =>
    queryOptions({
      queryKey: [...hostKeys.containers(hostId), 'using-volume', volume],
      queryFn: () => commands.volumesRunningUsers(hostId, volume),
      staleTime: 0,
      gcTime: 0,
    }),
  /** A folder of a browsed volume; paths are relative to the volume. */
  browse: (hostId: string, session: string, path: string) =>
    queryOptions({
      queryKey: [...volumeKeys.browse(hostId, session), path],
      queryFn: () => commands.volumesBrowseList(hostId, session, path),
    }),
};
