import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const snapshotQueries = {
  /** One snapshotter's snapshots in `namespace` (the default with `null`); snapshot events refresh them. */
  list: (hostId: string, namespace: string, snapshotter: string | null) =>
    queryOptions({
      queryKey: [...hostKeys.containerdSnapshots(hostId, namespace), snapshotter ?? ''],
      queryFn: () => commands.containerdSnapshots(hostId, namespace, snapshotter),
    }),
};
