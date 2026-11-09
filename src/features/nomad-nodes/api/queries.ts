import { queryOptions } from '@tanstack/react-query';
import { nomadKeys } from '@/core/nomad';
import { commands } from '@/shared/api';

export const nomadNodeQueries = {
  list: (clusterId: string) =>
    queryOptions({
      queryKey: nomadKeys.nodes(clusterId),
      queryFn: () => commands.clustersNomadNodes(clusterId),
    }),
  detail: (clusterId: string, nodeId: string) =>
    queryOptions({
      queryKey: nomadKeys.node(clusterId, nodeId),
      queryFn: () => commands.clustersNomadNode(clusterId, nodeId),
    }),
};
