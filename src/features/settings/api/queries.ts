import { queryOptions } from '@tanstack/react-query';
import { appQueries, commands } from '@/shared/api';

/** Homeport's license and the notices of the packages it bundles; fixed per build. */
export const licensesQuery = () =>
  queryOptions({
    queryKey: ['licenses'],
    queryFn: () => commands.getLicenses(),
    staleTime: Infinity,
  });

/**
 * What this machine offers, checked again each time Settings opens, so a
 * compose CLI installed since launch shows up without a restart.
 */
export const machineQuery = () => queryOptions({ ...appQueries.capabilities(), staleTime: 0 });
