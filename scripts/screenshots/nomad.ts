/**
 * A demo Nomad Cluster for the Nomad screenshots: `nomad-prod`, with a few
 * jobs in its `shop` namespace and three client nodes. Everything is
 * invented: no real addresses, tokens or certificates.
 */
import type {
  ClusterView,
  NomadAllocationView,
  NomadGroupCounts,
  NomadJobDetail,
  NomadJobSource,
  NomadJobView,
  NomadNamespaces,
  NomadNodeView,
  NomadPlan,
} from '@/shared/api';
import { NOW } from './data';

const MIN = 60;
const HOUR = 60 * MIN;
const DAY = 24 * HOUR;
const iso = (ago: number) => new Date((NOW - ago) * 1000).toISOString();

export const NOMAD = 'cluster-nomad-prod';

export const nomadCluster: ClusterView = {
  cluster: {
    id: NOMAD,
    name: 'nomad-prod',
    kind: 'nomad',
    connection: {
      kind: 'nomad',
      address: 'https://nomad.acme.internal:4646',
      region: null,
      tlsServerName: 'server.global.nomad',
      aclToken: true,
      customCa: true,
      clientCertificate: false,
    },
    enabled: true,
    createdAt: iso(12 * DAY),
  },
  status: { state: 'connected', serverVersion: '1.10.3', platform: null, region: 'global' },
};

const counts = (
  name: string,
  running: number,
  desired: number | null,
  more: Partial<NomadGroupCounts> = {},
): NomadGroupCounts => ({
  name,
  desired,
  running,
  starting: 0,
  queued: 0,
  failed: 0,
  lost: 0,
  complete: 0,
  ...more,
});

const job = (
  name: string,
  jobType: string,
  groups: NomadGroupCounts[],
  ago: number,
  more: Partial<NomadJobView> = {},
): NomadJobView => ({
  id: name,
  name,
  namespace: 'shop',
  jobType,
  status: 'running',
  statusDescription: null,
  stopped: false,
  priority: 50,
  periodic: false,
  parameterized: false,
  parentId: null,
  submitted: iso(ago),
  groups,
  ...more,
});

export const nomadJobs: NomadJobView[] = [
  job('shop-api', 'service', [counts('api', 3, 3)], 2 * HOUR),
  job('shop-worker', 'service', [counts('worker', 1, 2, { starting: 1 })], 6 * MIN),
  job('storefront', 'service', [counts('web', 2, 2), counts('cache', 1, 1)], 3 * DAY),
  job('nightly-backup', 'batch', [counts('backup', 0, 1, { complete: 14 })], 9 * DAY, {
    periodic: true,
  }),
  job('node-exporter', 'system', [counts('exporter', 3, null)], 20 * DAY),
  job('traefik', 'service', [counts('ingress', 2, 2)], 11 * DAY),
];

const allocation = (
  index: number,
  node: string,
  status: string,
  ago: number,
  version = 7,
): NomadAllocationView => ({
  id: `7c${index}e41a9-3b2d-4f0a-9c61-2a5b8d${index}f04e1`,
  name: `shop-api.api[${index}]`,
  namespace: 'shop',
  jobId: 'shop-api',
  taskGroup: 'api',
  nodeId: `5e8${index}a7c2-11d4-4b9e-8f3a-6c0d2b9e7a10`,
  nodeName: node,
  clientStatus: status,
  desiredStatus: status === 'complete' ? 'stop' : 'run',
  jobVersion: version,
  created: iso(ago),
  modified: iso(ago - 30),
  tasks: [
    {
      name: 'server',
      state: status === 'complete' ? 'dead' : 'running',
      failed: false,
      restarts: 0,
      started: iso(ago - 20),
    },
  ],
});

export const shopApi: NomadJobDetail = {
  job: nomadJobs[0]!,
  version: 7,
  datacenters: ['eu-west-1a', 'eu-west-1b'],
  nodePool: 'default',
  groups: [
    {
      counts: counts('api', 3, 3),
      scaling: { min: 2, max: 8, enabled: false },
      tasks: [{ name: 'server', driver: 'docker' }],
    },
  ],
  allocations: [
    allocation(2, 'client-eu-1b', 'running', 2 * HOUR),
    allocation(1, 'client-eu-1a', 'running', 2 * HOUR + 40),
    allocation(0, 'client-eu-1c', 'running', 2 * HOUR + 80),
    allocation(3, 'client-eu-1a', 'complete', DAY, 6),
  ],
};

export const shopApiSource: NomadJobSource = {
  format: 'hcl2',
  submitted: true,
  source: `job "shop-api" {
  namespace   = "shop"
  datacenters = ["eu-west-1a", "eu-west-1b"]

  group "api" {
    count = 4

    scaling {
      min = 2
      max = 8
    }

    network {
      port "http" { to = 8080 }
    }

    task "server" {
      driver = "docker"

      config {
        image = "ghcr.io/acme/shop-api:2.6.0"
        ports = ["http"]
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }
  }
}
`,
};

export const shopApiPlan: NomadPlan = {
  jobId: 'shop-api',
  namespace: 'shop',
  format: 'hcl2',
  job: { ID: 'shop-api' },
  jobModifyIndex: 18_204,
  newJob: false,
  change: 'Edited',
  groups: [
    {
      name: 'api',
      change: 'Edited',
      place: 1,
      stop: 0,
      inPlace: 0,
      destructive: 3,
      migrate: 0,
      canary: 0,
      ignore: 0,
      fields: [
        { path: 'Count', change: 'Edited', old: '3', new: '4' },
        {
          path: 'Task "server" › Config › image',
          change: 'Edited',
          old: 'ghcr.io/acme/shop-api:2.5.0',
          new: 'ghcr.io/acme/shop-api:2.6.0',
        },
      ],
      failure: null,
    },
  ],
  warnings: null,
  placement: null,
};

export const nomadNamespaces: NomadNamespaces = {
  listable: true,
  namespaces: [
    { name: 'default', description: null },
    { name: 'billing', description: null },
    { name: 'shop', description: 'The web shop' },
  ],
};

const node = (name: string, running: number, more: Partial<NomadNodeView> = {}): NomadNodeView => ({
  id: `5e8${name.length}a7c2-11d4-4b9e-8f3a-6c0d2b9e7a1${name.at(-1)}`,
  name,
  datacenter: `eu-west-1${name.at(-1)}`,
  nodePool: 'default',
  nodeClass: null,
  address: `10.20.${name.length}.1${name.charCodeAt(name.length - 1) % 10}`,
  version: '1.10.3',
  status: 'ready',
  statusDescription: null,
  eligible: true,
  draining: false,
  running,
  ...more,
});

export const nomadNodes: NomadNodeView[] = [
  node('client-eu-1a', 5),
  node('client-eu-1b', 4),
  node('client-eu-1c', 1, { draining: true, eligible: false }),
];
