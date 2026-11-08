/**
 * Query keys of the Swarms and of what each one holds. Every key of a Swarm
 * starts with `swarmKeys.scope(swarmId)`, so one invalidation refreshes all
 * of it, whichever manager Host answers.
 */
export const swarmKeys = {
  list: ['swarms'] as const,
  scope: (swarmId: string) => ['swarm', swarmId] as const,
  summary: (swarmId: string) => ['swarm', swarmId, 'summary'] as const,
  nodes: (swarmId: string) => ['swarm', swarmId, 'nodes'] as const,
  services: (swarmId: string) => ['swarm', swarmId, 'services'] as const,
  tasks: (swarmId: string) => ['swarm', swarmId, 'tasks'] as const,
  secrets: (swarmId: string) => ['swarm', swarmId, 'secrets'] as const,
  configs: (swarmId: string) => ['swarm', swarmId, 'configs'] as const,
  stacks: (swarmId: string) => ['swarm', swarmId, 'stacks'] as const,
};
