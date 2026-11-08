import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const containerdImageQueries = {
  /** The images of `namespace`; image and content events refresh them. */
  list: (hostId: string, namespace: string) =>
    queryOptions({
      queryKey: hostKeys.containerdImages(hostId, namespace),
      queryFn: () => commands.containerdImages(hostId, namespace),
    }),
};
