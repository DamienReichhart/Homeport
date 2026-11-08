import type { HostView, IncusImage, IncusInstance, IncusProject, RemoteImage } from '@/shared/api';
import { connectedHost } from './fixtures';

/** A local Incus Host, connected. */
export const incusHost: HostView = {
  host: {
    ...connectedHost.host,
    id: 'i1',
    name: 'lab',
    engine: 'incus',
    connection: { kind: 'local', socketPath: '/var/lib/incus/unix.socket' },
  },
  status: {
    state: 'connected',
    engineVersion: '6.0.4',
    apiVersion: '1.0',
    os: 'Debian GNU/Linux 12',
    arch: 'x86_64',
    podmanVersion: null,
    containerdVersion: null,
    incusVersion: '6.0.4',
    swarm: null,
  },
};

/** An Incus project holding `instances` Instances. */
export const incusProject = (name: string, instances = 0, images = 0): IncusProject => ({
  name,
  description: '',
  instances,
  images,
});

/** A running container Instance. */
export const incusInstance = (name: string, over: Partial<IncusInstance> = {}): IncusInstance => ({
  name,
  type: 'container',
  status: 'running',
  description: '',
  image: 'Alpine 3.21 amd64 (20261108_13:00)',
  architecture: 'x86_64',
  profiles: ['default'],
  addresses: ['10.42.71.166'],
  cpuUsageNs: 270_905_168,
  memoryBytes: 2_785_280,
  processes: 9,
  createdAt: '2026-11-08T22:35:36Z',
  lastUsedAt: '2026-11-08T22:35:36Z',
  snapshots: 0,
  ephemeral: false,
  location: 'none',
  ...over,
});

/** An image kept on an Incus Host. */
export const incusImage = (fingerprint: string, over: Partial<IncusImage> = {}): IncusImage => ({
  fingerprint,
  aliases: [],
  description: 'Alpine 3.21 amd64 (20261108_13:00)',
  type: 'container',
  architecture: 'x86_64',
  size: 3_367_792,
  os: 'Alpine',
  release: '3.21',
  variant: 'default',
  uploadedAt: '2026-11-08T22:35:36Z',
  lastUsedAt: null,
  autoUpdate: true,
  cached: false,
  public: false,
  source: { server: 'https://images.linuxcontainers.org', alias: 'alpine/3.21' },
  ...over,
});

/** An image a remote server publishes. */
export const remoteImage = (alias: string, over: Partial<RemoteImage> = {}): RemoteImage => ({
  alias,
  aliases: [alias],
  os: 'Debian',
  release: '12',
  releaseTitle: 'bookworm',
  variant: 'default',
  arch: 'amd64',
  container: true,
  virtualMachine: true,
  serial: '20261108_05:24',
  containerSize: 100_000_000,
  virtualMachineSize: 400_000_000,
  ...over,
});
