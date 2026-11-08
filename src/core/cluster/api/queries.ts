import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { clusterKeys } from './keys';

export const clusterQueries = {
  list: () => queryOptions({ queryKey: clusterKeys.list, queryFn: () => commands.clustersList() }),
};

export const useClusters = () => useQuery(clusterQueries.list());
