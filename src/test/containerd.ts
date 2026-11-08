import type { ContainerdContainer, HostView } from '@/shared/api';
import { connectedHost } from './fixtures';

/** A local containerd Host, connected. */
export const containerdHost: HostView = {
  host: {
    ...connectedHost.host,
    id: 'c1',
    name: 'k3s',
    engine: 'containerd',
    connection: { kind: 'local', socketPath: '/run/containerd/containerd.sock' },
  },
  status: {
    state: 'connected',
    engineVersion: '2.1.4',
    apiVersion: 'b7a25261',
    os: 'linux',
    arch: 'amd64',
    podmanVersion: null,
    containerdVersion: '2.1.4',
    swarm: null,
  },
};

/** A containerd container with a running task. */
export const containerdContainer = (
  id: string,
  over: Partial<ContainerdContainer> = {},
): ContainerdContainer => ({
  id,
  name: id,
  image: 'docker.io/library/alpine:3.22',
  labels: {},
  runtime: 'io.containerd.runc.v2',
  snapshotter: 'overlayfs',
  snapshotKey: id,
  createdAt: '2026-11-08T10:00:00Z',
  createdBy: 'other',
  pod: null,
  sandbox: false,
  state: 'running',
  pid: 42,
  exitStatus: null,
  exitedAt: null,
  ...over,
});
