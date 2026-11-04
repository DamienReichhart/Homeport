/**
 * Query keys of the Hosts and of the Engine resources each one has. Every key
 * of a Host starts with `hostKeys.scope(hostId)`, so one invalidation
 * refreshes all of it.
 */
export const hostKeys = {
  list: ['hosts'] as const,
  localEngines: ['local-engines'] as const,
  scope: (hostId: string) => ['host', hostId] as const,
  containers: (hostId: string) => ['host', hostId, 'containers'] as const,
  images: (hostId: string) => ['host', hostId, 'images'] as const,
  volumes: (hostId: string) => ['host', hostId, 'volumes'] as const,
  networks: (hostId: string) => ['host', hostId, 'networks'] as const,
  stacks: (hostId: string) => ['host', hostId, 'stacks'] as const,
  system: (hostId: string) => ['host', hostId, 'system'] as const,
  events: (hostId: string) => ['host', hostId, 'events'] as const,
};
