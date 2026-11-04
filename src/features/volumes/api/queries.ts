import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const volumeQueries = {
  inspect: (hostId: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.volumes(hostId), name],
      queryFn: () => commands.volumesInspect(hostId, name),
    }),
};
