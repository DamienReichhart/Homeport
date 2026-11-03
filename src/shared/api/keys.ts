/**
 * TanStack Query keys, shared so live Engine events can invalidate the
 * queries of any feature.
 */
export const keys = {
  capabilities: ['capabilities'] as const,
  appInfo: ['app-info'] as const,
  licenses: ['licenses'] as const,
  localEngines: ['local-engines'] as const,
  settings: ['settings'] as const,
  hosts: ['hosts'] as const,
  sshSetup: (hostId: string) => ['hosts', hostId, 'ssh-setup'] as const,
  host: (hostId: string) => ['host', hostId] as const,
  containers: (hostId: string) => ['host', hostId, 'containers'] as const,
  container: (hostId: string, id: string) => ['host', hostId, 'containers', id] as const,
  images: (hostId: string) => ['host', hostId, 'images'] as const,
  volumes: (hostId: string) => ['host', hostId, 'volumes'] as const,
  networks: (hostId: string) => ['host', hostId, 'networks'] as const,
  stacks: (hostId: string) => ['host', hostId, 'stacks'] as const,
  system: (hostId: string) => ['host', hostId, 'system'] as const,
  events: (hostId: string) => ['host', hostId, 'events'] as const,
  files: (hostId: string, containerId: string, path: string) =>
    ['host', hostId, 'containers', containerId, 'files', path] as const,
  processes: (hostId: string, containerId: string) =>
    ['host', hostId, 'containers', containerId, 'processes'] as const,
  managedStack: (id: string) => ['managed-stack', id] as const,
};
