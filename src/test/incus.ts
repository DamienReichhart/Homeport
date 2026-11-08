import type { HostView, IncusInstance, IncusProject } from '@/shared/api';
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
