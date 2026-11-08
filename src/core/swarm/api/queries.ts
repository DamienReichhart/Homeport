import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { swarmKeys } from './keys';

export const swarmQueries = {
  /** Every Swarm the Hosts belong to, once each. */
  list: () => queryOptions({ queryKey: swarmKeys.list, queryFn: () => commands.swarmsList() }),
  /** What a connected manager says about the Swarm. */
  summary: (swarmId: string) =>
    queryOptions({
      queryKey: swarmKeys.summary(swarmId),
      queryFn: () => commands.swarmInspect(swarmId),
      retry: false,
    }),
};

export const useSwarms = () => useQuery(swarmQueries.list());
