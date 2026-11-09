// Fixtures of a Nomad Cluster and what runs on it, for the Nomad pages' tests.
import { nomadClusterView, nomadSelected } from './clusters';

/** The commands every page of a selected, connected Nomad Cluster needs. */
export const nomadCommands = {
  get_settings: nomadSelected,
  clusters_list: () => [nomadClusterView('nomad-dev')],
  clusters_nomad_watch_subscribe: () => 1,
  clusters_nomad_namespaces: () => ({ listable: false, namespaces: [] }),
  stream_cancel: () => null,
};

/** A task group's counts. */
export const groupCounts = (name: string, running: number, desired: number | null) => ({
  name,
  desired,
  running,
  starting: 0,
  queued: 0,
  failed: 0,
  lost: 0,
  complete: 0,
});

/** A running service job as `clusters_nomad_jobs` lists it. */
export function nomadJob(name: string, overrides: Record<string, unknown> = {}) {
  return {
    id: name,
    name,
    namespace: 'default',
    jobType: 'service',
    status: 'running',
    statusDescription: null,
    stopped: false,
    priority: 50,
    periodic: false,
    parameterized: false,
    parentId: null,
    submitted: '2026-11-01T10:00:00Z',
    groups: [groupCounts('api', 2, 3)],
    ...overrides,
  };
}

/** An allocation as the job and node pages list it. */
export function nomadAllocation(name: string, overrides: Record<string, unknown> = {}) {
  return {
    id: `a1b2c3d4-0000-0000-0000-00000000000${name.length}`,
    name,
    namespace: 'default',
    jobId: 'web',
    taskGroup: 'api',
    nodeId: 'f00dbabe-0000-0000-0000-000000000001',
    nodeName: 'client-1',
    clientStatus: 'running',
    desiredStatus: 'run',
    jobVersion: 3,
    created: '2026-11-01T10:00:00Z',
    modified: '2026-11-01T10:01:00Z',
    tasks: [{ name: 'server', state: 'running', failed: false, restarts: 1, started: null }],
    ...overrides,
  };
}

/** The detail of the job `web`, as `clusters_nomad_job` returns it. */
export function nomadJobDetail(overrides: Record<string, unknown> = {}) {
  return {
    job: nomadJob('web'),
    version: 3,
    datacenters: ['dc1'],
    nodePool: 'default',
    groups: [
      {
        counts: groupCounts('api', 2, 3),
        scaling: { min: 1, max: 5, enabled: false },
        tasks: [{ name: 'server', driver: 'docker' }],
      },
    ],
    allocations: [nomadAllocation('web.api[0]')],
    ...overrides,
  };
}

/** A ready, eligible client node as `clusters_nomad_nodes` lists it. */
export function nomadNode(name: string, overrides: Record<string, unknown> = {}) {
  return {
    id: `f00d000${name.length}-0000-0000-0000-000000000000`,
    name,
    datacenter: 'dc1',
    nodePool: 'default',
    nodeClass: null,
    address: '10.0.0.11',
    version: '1.10.3',
    status: 'ready',
    statusDescription: null,
    eligible: true,
    draining: false,
    running: 2,
    ...overrides,
  };
}
