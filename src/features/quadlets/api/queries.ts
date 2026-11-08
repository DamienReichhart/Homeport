import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const quadletQueries = {
  /** A Podman Host's Quadlet units; container events make them stale. */
  list: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.quadlets(hostId),
      queryFn: () => commands.quadletsList(hostId),
    }),
  /** A unit's file as Podman reads it on the Host. */
  file: (hostId: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.quadlets(hostId), 'file', name],
      queryFn: () => commands.quadletsFile(hostId, name),
    }),
};
