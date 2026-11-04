import { queryOptions, useQuery } from '@tanstack/react-query';
import { commands } from './invoke';

/** Queries about this copy of Homeport and the machine it runs on, not about Hosts. */
export const appQueries = {
  info: () => queryOptions({ queryKey: ['app-info'], queryFn: () => commands.getAppInfo() }),
  /** What this machine offers (an OS keychain, the compose CLI…). Fixed for the session. */
  capabilities: () =>
    queryOptions({
      queryKey: ['capabilities'],
      queryFn: () => commands.getCapabilities(),
      staleTime: Infinity,
    }),
};

export const useCapabilities = () => useQuery(appQueries.capabilities());
