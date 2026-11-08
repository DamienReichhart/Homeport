import { queryOptions } from '@tanstack/react-query';
import { swarmKeys } from '@/core/swarm';
import { commands } from '@/shared/api';

export const swarmFeatureQueries = {
  nodes: (swarmId: string) =>
    queryOptions({
      queryKey: swarmKeys.nodes(swarmId),
      queryFn: () => commands.swarmNodesList(swarmId),
    }),
  nodeInspect: (swarmId: string, id: string) =>
    queryOptions({
      queryKey: [...swarmKeys.nodes(swarmId), id],
      queryFn: () => commands.swarmNodesInspect(swarmId, id),
    }),
  services: (swarmId: string) =>
    queryOptions({
      queryKey: swarmKeys.services(swarmId),
      queryFn: () => commands.swarmServicesList(swarmId),
    }),
  serviceInspect: (swarmId: string, id: string) =>
    queryOptions({
      queryKey: [...swarmKeys.services(swarmId), id, 'inspect'],
      queryFn: () => commands.swarmServicesInspect(swarmId, id),
    }),
};
