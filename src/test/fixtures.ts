import type { Container, HostView } from '@/shared/api';

export const connectedHost: HostView = {
  host: {
    id: 'h1',
    name: 'local',
    enabled: true,
    hostKey: null,
    createdAt: '',
    connection: { kind: 'local', socketPath: '/var/run/docker.sock' },
  },
  status: {
    state: 'connected',
    engineVersion: '29.1',
    apiVersion: '1.52',
    os: 'linux',
    arch: 'amd64',
  },
};

export function container(name: string, stack: string | null, state = 'running'): Container {
  return {
    id: `${name}-0123456789abcdef`,
    name,
    image: `${name.split('-')[1] ?? name}:latest`,
    imageId: 'sha256:abc',
    command: '',
    created: Math.floor(Date.now() / 1000) - 60,
    state,
    status: state === 'running' ? 'Up 1 minute' : 'Exited (0)',
    health: null,
    ports: [],
    labels: {},
    stack,
    service: stack ? (name.split('-')[1] ?? null) : null,
    networks: [],
  };
}

/** Commands every rendered app needs, regardless of the page. */
export const baseCommands = {
  get_settings: () => ({ theme: 'system', lastHostId: 'h1', checkForUpdates: true }),
  update_settings: ({ settings }: Record<string, unknown>) => settings,
  get_capabilities: () => ({ keychain: true, compose: null, composeProblem: null }),
  hosts_list: () => [connectedHost],
  stream_cancel: () => null,
  updates_check: () => ({ currentVersion: '0.3.0', available: null, method: { kind: 'inApp' } }),
};
