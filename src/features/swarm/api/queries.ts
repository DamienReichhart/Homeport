import { queryOptions } from '@tanstack/react-query';
import { swarmKeys } from '@/core/swarm';
import { commands, type TaskFilter } from '@/shared/api';

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
  tasks: (swarmId: string, filter: TaskFilter) =>
    queryOptions({
      queryKey: [...swarmKeys.tasks(swarmId), filter],
      queryFn: () => commands.swarmTasksList(swarmId, filter),
    }),
  /** A Swarm's secrets or configs. */
  objects: (swarmId: string, kind: 'secret' | 'config') =>
    queryOptions({
      queryKey: kind === 'secret' ? swarmKeys.secrets(swarmId) : swarmKeys.configs(swarmId),
      queryFn: () =>
        kind === 'secret' ? commands.swarmSecretsList(swarmId) : commands.swarmConfigsList(swarmId),
    }),
  configContent: (swarmId: string, id: string) =>
    queryOptions({
      queryKey: [...swarmKeys.configs(swarmId), id, 'content'],
      queryFn: () => commands.swarmConfigsInspect(swarmId, id),
    }),
  /** The Swarm's overlay networks, for a service to attach to. */
  networks: (swarmId: string) =>
    queryOptions({
      queryKey: [...swarmKeys.scope(swarmId), 'networks'],
      queryFn: () => commands.swarmNetworksList(swarmId),
    }),
  /** Join tokens: never kept once nothing shows them. */
  joinInfo: (swarmId: string) =>
    queryOptions({
      queryKey: [...swarmKeys.scope(swarmId), 'join-info'],
      queryFn: () => commands.swarmJoinInfo(swarmId),
      gcTime: 0,
      staleTime: 0,
    }),
};
