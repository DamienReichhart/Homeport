import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

/** Homeport's license and the notices of the packages it bundles; fixed per build. */
export const licensesQuery = () =>
  queryOptions({
    queryKey: ['licenses'],
    queryFn: () => commands.getLicenses(),
    staleTime: Infinity,
  });
