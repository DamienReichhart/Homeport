import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const podQueries = {
  /** One pod with its ports and shared namespaces; under the list, so events refresh it. */
  inspect: (hostId: string, id: string) =>
    queryOptions({
      queryKey: [...hostKeys.pods(hostId), 'inspect', id],
      queryFn: () => commands.podsInspect(hostId, id),
    }),
};
