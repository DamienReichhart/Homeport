import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const incusStorageQueries = {
  /** Every storage pool with how full it is; storage events refresh them. */
  pools: (hostId: string, project: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'storage', project), 'pools'],
      queryFn: () => commands.incusStoragePools(hostId, project),
    }),
  /** A pool's volumes in `project`, with what uses each. */
  volumes: (hostId: string, project: string, pool: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'storage', project), 'volumes', pool],
      queryFn: () => commands.incusStorageVolumes(hostId, project, pool),
    }),
};
