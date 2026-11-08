import type { HostView, IncusProject } from '@/shared/api';
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
