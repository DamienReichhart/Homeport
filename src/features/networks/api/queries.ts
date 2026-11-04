import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const networkQueries = {
  inspect: (hostId: string, networkId: string) =>
    queryOptions({
      queryKey: [...hostKeys.networks(hostId), networkId],
      queryFn: () => commands.networksInspect(hostId, networkId),
    }),
};
