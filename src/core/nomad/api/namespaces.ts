import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { nomadKeys } from './keys';

/** The namespaces of a Nomad Cluster the token can list. */
export const nomadNamespacesQuery = (clusterId: string) =>
  queryOptions({
    queryKey: nomadKeys.namespaces(clusterId),
    queryFn: () => commands.clustersNomadNamespaces(clusterId),
    staleTime: 60_000,
  });
