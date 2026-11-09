// Fixtures of Clusters and what runs on them, for the Cluster pages' tests.
import { baseCommands } from './fixtures';

/** A Cluster connected to its API server, as `clusters_list` returns it. */
export function clusterView(name: string, overrides: Record<string, unknown> = {}) {
  return {
    cluster: {
      id: 'c1',
      name,
      kind: 'kubernetes',
      connection: {
        kind: 'kubernetes',
        server: 'https://127.0.0.1:6443',
        tlsServerName: null,
        insecureSkipTlsVerify: false,
        proxyUrl: null,
        namespace: 'default',
        auth: { method: 'clientCertificate' },
        source: { context: name, pasted: false },
      },
      enabled: true,
      createdAt: '2026-11-01T10:00:00Z',
    },
    status: { state: 'connected', serverVersion: 'v1.33.1', platform: 'linux/arm64', region: null },
    ...overrides,
  };
}

/** A Nomad Cluster connected to its agent, as `clusters_list` returns it. */
export function nomadClusterView(name: string, overrides: Record<string, unknown> = {}) {
  return {
    cluster: {
      id: 'n1',
      name,
      kind: 'nomad',
      connection: {
        kind: 'nomad',
        address: 'http://127.0.0.1:4646',
        region: null,
        tlsServerName: null,
        aclToken: true,
        customCa: false,
        clientCertificate: false,
      },
      enabled: true,
      createdAt: '2026-11-01T10:00:00Z',
    },
    status: { state: 'connected', serverVersion: '1.10.3', platform: null, region: 'global' },
    ...overrides,
  };
}

/** The settings with the Nomad Cluster `n1` selected. */
export const nomadSelected = () => ({
  ...baseCommands.get_settings(),
  lastSelection: { kind: 'cluster', id: 'n1' },
});

/** The settings with `cluster` selected. */
export const clusterSelected = () => ({
  ...baseCommands.get_settings(),
  lastSelection: { kind: 'cluster', id: 'c1' },
});

/** A Deployment as `clusters_workloads_list` returns it. */
export function workloadView(name: string, overrides: Record<string, unknown> = {}) {
  return {
    kind: 'deployment',
    namespace: 'default',
    name,
    ready: 3,
    desired: 3,
    upToDate: 3,
    health: 'ready',
    images: ['nginx:1.27'],
    created: '2026-11-01T10:00:00Z',
    schedule: null,
    lastSchedule: null,
    owner: null,
    ...overrides,
  };
}

/** A running Pod as `clusters_pods_list` returns it. */
export function podView(name: string, overrides: Record<string, unknown> = {}) {
  return {
    namespace: 'default',
    name,
    phase: 'Running',
    status: 'Running',
    health: 'running',
    ready: 1,
    containers: 1,
    restarts: 0,
    node: 'node-a',
    ip: '10.1.0.4',
    created: '2026-11-01T10:00:00Z',
    owner: null,
    images: ['nginx'],
    ...overrides,
  };
}

/** The commands every page of a selected, connected Cluster needs. */
export const clusterCommands = {
  get_settings: clusterSelected,
  clusters_list: () => [clusterView('kind-dev')],
  clusters_namespaces: () => [
    { name: 'default', phase: 'Active', created: null, labels: {} },
    { name: 'shop', phase: 'Active', created: null, labels: {} },
  ],
  clusters_watch_subscribe: () => 1,
  stream_cancel: () => null,
};
