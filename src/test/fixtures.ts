import type { Container, FileEntry, HostView, StatsSample } from '@/shared/api';

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
  get_capabilities: () => ({
    keychain: true,
    compose: {
      state: 'unavailable',
      problem: {
        code: 'compose_missing',
        message: 'The docker compose CLI isn’t installed on this computer.',
        hint: null,
        detail: null,
      },
    },
  }),
  hosts_list: () => [connectedHost],
  registries_list: () => [],
  images_freshness: () => [],
  stream_cancel: () => null,
  updates_check: () => ({ currentVersion: '0.3.0', available: null, method: { kind: 'inApp' } }),
};

/** An Engine inspect document for a running container with a port, a volume and a label. */
export function inspectDocument(name = 'web') {
  return {
    Id: `${name}-0123456789abcdef`,
    Name: `/${name}`,
    Created: '2026-01-02T03:04:05Z',
    Path: 'nginx',
    Args: ['-g', 'daemon off;'],
    RestartCount: 0,
    State: {
      Status: 'running',
      Running: true,
      StartedAt: '2026-01-02T03:04:06Z',
      FinishedAt: '0001-01-01T00:00:00Z',
      ExitCode: 0,
      OOMKilled: false,
    },
    Config: {
      Hostname: '0123456789ab',
      User: '',
      Env: ['PATH=/usr/bin', 'MODE=production'],
      Cmd: ['nginx', '-g', 'daemon off;'],
      Entrypoint: null,
      Image: 'nginx:1.27',
      WorkingDir: '',
      Labels: { 'org.example.team': 'web' },
      Tty: false,
    },
    HostConfig: {
      RestartPolicy: { Name: 'unless-stopped', MaximumRetryCount: 0 },
      NetworkMode: 'bridge',
      Memory: 0,
      NanoCpus: 0,
      AutoRemove: false,
      Privileged: false,
    },
    Mounts: [
      {
        Type: 'volume',
        Name: 'web-data',
        Source: '/var/lib/docker/volumes/web-data/_data',
        Destination: '/data',
        Mode: 'z',
        RW: true,
      },
    ],
    NetworkSettings: {
      Ports: { '80/tcp': [{ HostIp: '0.0.0.0', HostPort: '8080' }] },
      Networks: {
        bridge: { IPAddress: '172.17.0.2', Gateway: '172.17.0.1', MacAddress: '', Aliases: null },
      },
    },
  };
}

/** A directory listing with a folder, a file and a symlink. */
export function fileEntries(dir = '/'): FileEntry[] {
  const at = (name: string) => (dir === '/' ? `/${name}` : `${dir}/${name}`);
  return [
    {
      name: 'etc',
      path: at('etc'),
      kind: 'directory',
      size: 0,
      modified: 1_700_000_000,
      mode: 'drwxr-xr-x',
    },
    {
      name: 'hello.txt',
      path: at('hello.txt'),
      kind: 'file',
      size: 2048,
      modified: 1_700_000_000,
      mode: '-rw-r--r--',
    },
    {
      name: 'lib',
      path: at('lib'),
      kind: 'symlink',
      size: 7,
      modified: 1_700_000_000,
      mode: 'lrwxrwxrwx',
    },
  ];
}

/** A stats sample; counters grow with `n` so rates are positive. */
export function statsSample(n = 1, overrides: Partial<StatsSample> = {}): StatsSample {
  return {
    cpuPercent: 12.5,
    memoryUsage: 64 * 1024 * 1024,
    memoryLimit: 1024 * 1024 * 1024,
    netRx: 1000 * n,
    netTx: 500 * n,
    blockRead: 0,
    blockWrite: 0,
    pids: 3,
    ...overrides,
  };
}
