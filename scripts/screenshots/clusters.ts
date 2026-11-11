/**
 * Two demo Clusters for the Kubernetes screenshots: `eks-prod` (signing in
 * through the AWS CLI) and a local `kind-dev`. Everything is invented: no
 * real accounts, addresses, keys or tokens.
 */
import type {
  ClusterNodeDetail,
  ClusterNodeView,
  ClusterServiceDetail,
  ClusterServiceView,
  ClusterSummary,
  ClusterView,
  ConfigMapView,
  EventView,
  IngressDetail,
  IngressView,
  KubeContexts,
  LocalClusters,
  NamespaceView,
  PodDetail,
  PodView,
  SecretView,
  WorkloadDetail,
  WorkloadKind,
  WorkloadView,
} from '@/shared/api';
import { NOW } from './data';

const MIN = 60;
const HOUR = 60 * MIN;
const DAY = 24 * HOUR;
const iso = (ago: number) => new Date((NOW - ago) * 1000).toISOString();

export const EKS = 'cluster-eks-prod';
export const KIND = 'cluster-kind-dev';
const EKS_SERVER = 'https://4F2D9C1E7B.gr7.eu-west-1.eks.amazonaws.com';

export const clusters: ClusterView[] = [
  {
    cluster: {
      id: EKS,
      name: 'eks-prod',
      kind: 'kubernetes',
      connection: {
        kind: 'kubernetes',
        server: EKS_SERVER,
        tlsServerName: null,
        insecureSkipTlsVerify: false,
        proxyUrl: null,
        namespace: 'shop',
        auth: { method: 'exec', command: 'aws' },
        source: { context: 'arn:aws:eks:eu-west-1:000000000000:cluster/prod', pasted: false },
      },
      enabled: true,
      createdAt: iso(20 * DAY),
    },
    status: {
      state: 'connected',
      serverVersion: 'v1.33.4-eks-3a8b2c1',
      platform: 'linux/amd64',
      region: null,
    },
  },
  {
    cluster: {
      id: KIND,
      name: 'kind-dev',
      kind: 'kubernetes',
      connection: {
        kind: 'kubernetes',
        server: 'https://127.0.0.1:41235',
        tlsServerName: null,
        insecureSkipTlsVerify: false,
        proxyUrl: null,
        namespace: null,
        auth: { method: 'clientCertificate' },
        source: { context: 'kind-dev', pasted: false },
      },
      enabled: true,
      createdAt: iso(3 * DAY),
    },
    status: { state: 'connected', serverVersion: 'v1.34.0', platform: 'linux/arm64', region: null },
  },
];

export const namespaces: NamespaceView[] = [
  'default',
  'shop',
  'monitoring',
  'ingress-nginx',
  'kube-system',
].map((name, i) => ({ name, phase: 'Active', created: iso((40 - i) * DAY), labels: {} }));

const workload = (
  kind: WorkloadKind,
  name: string,
  ready: number,
  desired: number,
  images: string[],
  overrides: Partial<WorkloadView> = {},
): WorkloadView => ({
  kind,
  namespace: 'shop',
  name,
  ready,
  desired,
  upToDate: kind === 'job' || kind === 'cronJob' ? null : desired,
  health: ready >= desired ? 'ready' : 'progressing',
  images,
  created: iso(12 * DAY),
  schedule: null,
  lastSchedule: null,
  owner: null,
  ...overrides,
});

export const workloads: Record<WorkloadKind, WorkloadView[]> = {
  deployment: [
    workload('deployment', 'api', 2, 3, ['ghcr.io/acme/shop-api:2.5.0'], { upToDate: 1 }),
    workload('deployment', 'checkout', 2, 2, ['ghcr.io/acme/checkout:1.9.3']),
    workload('deployment', 'web', 3, 3, ['nginx:1.27-alpine']),
    workload('deployment', 'worker', 4, 4, ['ghcr.io/acme/shop-worker:2.5.0']),
    workload('deployment', 'search', 0, 1, ['ghcr.io/acme/search:0.8.1'], {
      health: 'degraded',
    }),
  ],
  statefulSet: [
    workload('statefulSet', 'postgres', 3, 3, ['postgres:17']),
    workload('statefulSet', 'redis', 1, 1, ['redis:7.4-alpine']),
  ],
  daemonSet: [
    workload('daemonSet', 'node-exporter', 4, 5, ['prom/node-exporter:v1.8.2'], {
      namespace: 'monitoring',
      upToDate: 5,
    }),
  ],
  job: [
    workload('job', 'migrate-2-5-0', 1, 1, ['ghcr.io/acme/shop-api:2.5.0'], {
      created: iso(50 * MIN),
    }),
  ],
  cronJob: [
    workload('cronJob', 'nightly-report', 0, 0, ['ghcr.io/acme/reports:1.2.0'], {
      schedule: '0 3 * * *',
      lastSchedule: iso(9 * HOUR),
    }),
    workload('cronJob', 'db-backup', 0, 0, ['ghcr.io/acme/pg-backup:3.1'], {
      schedule: '*/30 * * * *',
      lastSchedule: iso(12 * MIN),
    }),
  ],
};

const NODES = ['ip-10-0-1-23', 'ip-10-0-1-87', 'ip-10-0-2-14', 'ip-10-0-2-56', 'ip-10-0-3-31'];

const pod = (name: string, node: number, overrides: Partial<PodView> = {}): PodView => ({
  namespace: 'shop',
  name,
  phase: 'Running',
  status: 'Running',
  health: 'running',
  ready: 1,
  containers: 1,
  restarts: 0,
  node: NODES[node] ?? null,
  ip: `10.0.${1 + (node % 3)}.${100 + name.length * 3 + node}`,
  created: iso((node + 1) * 7 * HOUR),
  // The ReplicaSet is the Pod's name without its random suffix.
  owner: { kind: 'ReplicaSet', name: name.split('-').slice(0, -1).join('-') },
  images: [],
  cpu: { request: 250, limit: 1000 },
  memory: { request: 256 * 1024 ** 2, limit: 512 * 1024 ** 2 },
  ...overrides,
});

export const pods: PodView[] = [
  pod('api-7c9d5f8b6-2kqzx', 0, {
    ready: 2,
    containers: 2,
    images: ['ghcr.io/acme/shop-api:2.5.0', 'envoyproxy/envoy:v1.31'],
  }),
  pod('api-7c9d5f8b6-9wlmt', 1, { ready: 2, containers: 2 }),
  pod('api-6b8f4d7c5-xr4np', 3, {
    status: 'CrashLoopBackOff',
    health: 'crashing',
    ready: 1,
    containers: 2,
    restarts: 7,
    created: iso(25 * MIN),
  }),
  pod('checkout-5f6c8d9b7-hq2vz', 2),
  pod('checkout-5f6c8d9b7-n8kfd', 4),
  pod('web-6d4c8b9f5-4tzpl', 0),
  pod('web-6d4c8b9f5-b7xqm', 2),
  pod('web-6d4c8b9f5-rk9ws', 4),
  pod('worker-8b5d7c6f9-2mhxc', 1, { restarts: 1 }),
  pod('worker-8b5d7c6f9-7qpnr', 1),
  pod('worker-8b5d7c6f9-dk5vw', 3),
  pod('worker-8b5d7c6f9-lz8sg', 4),
  pod('search-5c7b9d8f6-q9vht', 2, { status: 'ImagePullBackOff', health: 'crashing', ready: 0 }),
  pod('postgres-0', 0, { owner: { kind: 'StatefulSet', name: 'postgres' } }),
  pod('postgres-1', 2, { owner: { kind: 'StatefulSet', name: 'postgres' } }),
  pod('postgres-2', 4, { owner: { kind: 'StatefulSet', name: 'postgres' } }),
  pod('redis-0', 3, { owner: { kind: 'StatefulSet', name: 'redis' } }),
  pod('migrate-2-5-0-tq7cz', 1, {
    phase: 'Succeeded',
    status: 'Completed',
    health: 'succeeded',
    ready: 0,
    owner: { kind: 'Job', name: 'migrate-2-5-0' },
  }),
];

export function workloadDetail(kind: WorkloadKind, name: string): WorkloadDetail {
  const view = workloads[kind].find((w) => w.name === name) ?? workloads.deployment[0]!;
  return {
    view,
    labels: { app: view.name, 'app.kubernetes.io/part-of': 'shop', team: 'payments' },
    annotations: { 'deployment.kubernetes.io/revision': '14' },
    selector: `app=${view.name}`,
    conditions: [
      {
        type: 'Available',
        status: view.ready > 0 ? 'True' : 'False',
        reason: view.ready > 0 ? 'MinimumReplicasAvailable' : 'MinimumReplicasUnavailable',
        message: 'Deployment has minimum availability.',
        lastTransition: iso(3 * DAY),
      },
      {
        type: 'Progressing',
        status: 'True',
        reason: 'ReplicaSetUpdated',
        message: `ReplicaSet "${view.name}-6b8f4d7c5" is progressing.`,
        lastTransition: iso(25 * MIN),
      },
    ],
    containers: [
      { name: view.name, image: view.images[0] ?? '', ports: ['8080/TCP', '9090/TCP'] },
      { name: 'envoy', image: 'envoyproxy/envoy:v1.31', ports: ['15001/TCP'] },
    ],
    strategy: 'RollingUpdate',
  };
}

export function podDetail(name: string): PodDetail {
  const view = pods.find((p) => p.name === name) ?? pods[2]!;
  return {
    view,
    labels: { app: name.split('-')[0] ?? '', 'pod-template-hash': '6b8f4d7c5' },
    annotations: {},
    conditions: [
      {
        type: 'PodScheduled',
        status: 'True',
        reason: null,
        message: null,
        lastTransition: iso(25 * MIN),
      },
      {
        type: 'Initialized',
        status: 'True',
        reason: null,
        message: null,
        lastTransition: iso(24 * MIN),
      },
      {
        type: 'ContainersReady',
        status: 'False',
        reason: 'ContainersNotReady',
        message: 'containers with unready status: [api]',
        lastTransition: iso(3 * MIN),
      },
      {
        type: 'Ready',
        status: 'False',
        reason: 'ContainersNotReady',
        message: 'containers with unready status: [api]',
        lastTransition: iso(3 * MIN),
      },
    ],
    containers: [
      {
        name: 'migrate',
        image: 'ghcr.io/acme/shop-api:2.5.0',
        init: true,
        state: 'terminated',
        reason: 'Completed',
        message: null,
        started: iso(24 * MIN),
        exitCode: 0,
        ready: false,
        restarts: 0,
        lastReason: null,
        ports: [],
        cpu: { request: null, limit: null },
        memory: { request: null, limit: null },
      },
      {
        name: 'api',
        image: 'ghcr.io/acme/shop-api:2.5.0',
        init: false,
        state: 'waiting',
        reason: 'CrashLoopBackOff',
        message: 'back-off 5m0s restarting failed container=api',
        started: null,
        exitCode: null,
        ready: false,
        restarts: 7,
        lastReason: 'Error',
        ports: [],
        cpu: { request: null, limit: null },
        memory: { request: null, limit: null },
      },
      {
        name: 'envoy',
        image: 'envoyproxy/envoy:v1.31',
        init: false,
        state: 'running',
        reason: null,
        message: null,
        started: iso(24 * MIN),
        exitCode: null,
        ready: true,
        restarts: 0,
        lastReason: null,
        ports: [],
        cpu: { request: null, limit: null },
        memory: { request: null, limit: null },
      },
    ],
    hostIp: '10.0.2.56',
    qosClass: 'Burstable',
    serviceAccount: 'shop-api',
  };
}

const service = (
  name: string,
  type: string,
  ports: string[],
  external: string[] = [],
): ClusterServiceView => ({
  namespace: 'shop',
  name,
  type,
  clusterIp: `172.20.${name.length * 7}.${name.length * 13}`,
  external,
  ports,
  selector: `app=${name}`,
  created: iso(30 * DAY),
});

export const services: ClusterServiceView[] = [
  service('api', 'ClusterIP', ['8080/TCP', '9090/TCP']),
  service('checkout', 'ClusterIP', ['8080/TCP']),
  service(
    'web',
    'LoadBalancer',
    ['80:31080/TCP', '443:31443/TCP'],
    ['a1b2c3-0000.elb.eu-west-1.amazonaws.com'],
  ),
  service('postgres', 'ClusterIP', ['5432/TCP']),
  service('redis', 'ClusterIP', ['6379/TCP']),
  { ...service('postgres-headless', 'ClusterIP', ['5432/TCP']), clusterIp: null },
];

export function serviceDetail(name: string): ClusterServiceDetail {
  const view = services.find((s) => s.name === name) ?? services[2]!;
  return {
    view,
    labels: { app: view.name },
    annotations: {},
    ports: [
      { name: 'http', port: 80, targetPort: 'http', nodePort: 31080, protocol: 'TCP' },
      { name: 'https', port: 443, targetPort: '8443', nodePort: 31443, protocol: 'TCP' },
    ],
    sessionAffinity: 'None',
  };
}

export const ingresses: IngressView[] = [
  {
    namespace: 'shop',
    name: 'storefront',
    class: 'nginx',
    hosts: ['shop.example.com', 'www.shop.example.com'],
    addresses: ['a1b2c3-0000.elb.eu-west-1.amazonaws.com'],
    tls: true,
    created: iso(30 * DAY),
  },
  {
    namespace: 'shop',
    name: 'admin',
    class: 'nginx',
    hosts: ['admin.shop.example.com'],
    addresses: ['a1b2c3-0000.elb.eu-west-1.amazonaws.com'],
    tls: true,
    created: iso(14 * DAY),
  },
];

export const ingressDetail: IngressDetail = {
  view: ingresses[0]!,
  labels: {},
  annotations: { 'cert-manager.io/cluster-issuer': 'letsencrypt' },
  paths: [
    {
      host: 'shop.example.com',
      path: '/api',
      pathType: 'Prefix',
      service: 'api',
      port: '8080',
      resource: null,
    },
    {
      host: 'shop.example.com',
      path: '/checkout',
      pathType: 'Prefix',
      service: 'checkout',
      port: '8080',
      resource: null,
    },
    {
      host: 'shop.example.com',
      path: '/',
      pathType: 'Prefix',
      service: 'web',
      port: 'http',
      resource: null,
    },
    {
      host: 'www.shop.example.com',
      path: '/',
      pathType: 'Prefix',
      service: 'web',
      port: 'http',
      resource: null,
    },
  ],
  tls: [{ hosts: ['shop.example.com', 'www.shop.example.com'], secret: 'storefront-tls' }],
};

export const configMaps: ConfigMapView[] = [
  {
    namespace: 'shop',
    name: 'api-config',
    keys: [
      { key: 'LOG_LEVEL', size: 4, binary: false },
      { key: 'settings.yaml', size: 812, binary: false },
    ],
    created: iso(9 * DAY),
  },
  {
    namespace: 'shop',
    name: 'nginx-conf',
    keys: [{ key: 'default.conf', size: 1430, binary: false }],
    created: iso(30 * DAY),
  },
  {
    namespace: 'shop',
    name: 'kube-root-ca.crt',
    keys: [{ key: 'ca.crt', size: 1066, binary: false }],
    created: iso(40 * DAY),
  },
];

export const secrets: SecretView[] = [
  {
    namespace: 'shop',
    name: 'db-credentials',
    type: 'Opaque',
    keys: [
      { key: 'password', size: 24, binary: false },
      { key: 'username', size: 4, binary: false },
    ],
    created: iso(40 * DAY),
  },
  {
    namespace: 'shop',
    name: 'storefront-tls',
    type: 'kubernetes.io/tls',
    keys: [
      { key: 'tls.crt', size: 3912, binary: false },
      { key: 'tls.key', size: 1704, binary: false },
    ],
    created: iso(20 * DAY),
  },
  {
    namespace: 'shop',
    name: 'ghcr-pull',
    type: 'kubernetes.io/dockerconfigjson',
    keys: [{ key: '.dockerconfigjson', size: 182, binary: false }],
    created: iso(40 * DAY),
  },
];

export const nodes: ClusterNodeView[] = NODES.map((name, i) => ({
  name,
  ready: i !== 3,
  unschedulable: i === 3,
  roles: [],
  version: 'v1.33.4-eks-3a8b2c1',
  os: 'linux',
  arch: i === 4 ? 'arm64' : 'amd64',
  internalIp: `10.0.${1 + Math.floor(i / 2)}.${[23, 87, 14, 56, 31][i]}`,
  cpu: i === 4 ? '7910m' : '3920m',
  memory: i === 4 ? '31482856Ki' : '15335132Ki',
  cpuMillicores: i === 4 ? 7910 : 3920,
  memoryBytes: (i === 4 ? 31482856 : 15335132) * 1024,
  taints: i === 3 ? 2 : 0,
  created: iso((30 - i * 4) * DAY),
}));

export function nodeDetail(name: string): ClusterNodeDetail {
  const view = nodes.find((n) => n.name === name) ?? nodes[0]!;
  return {
    view,
    labels: {
      'node.kubernetes.io/instance-type': 'm7i.xlarge',
      'topology.kubernetes.io/zone': 'eu-west-1a',
    },
    annotations: {},
    conditions: [
      {
        type: 'MemoryPressure',
        status: 'False',
        reason: 'KubeletHasSufficientMemory',
        message: null,
        lastTransition: iso(9 * DAY),
      },
      {
        type: 'DiskPressure',
        status: 'False',
        reason: 'KubeletHasNoDiskPressure',
        message: null,
        lastTransition: iso(9 * DAY),
      },
      {
        type: 'Ready',
        status: 'True',
        reason: 'KubeletReady',
        message: 'kubelet is posting ready status',
        lastTransition: iso(9 * DAY),
      },
    ],
    capacity: { cpu: '4', memory: '16073436Ki', pods: '58', 'ephemeral-storage': '83873772Ki' },
    allocatable: {
      cpu: view.cpu ?? '',
      memory: view.memory ?? '',
      pods: '58',
      'ephemeral-storage': '76224326324',
    },
    taints: [],
    osImage: 'Amazon Linux 2023.6.20250303',
    kernel: '6.1.129-138.220.amzn2023.x86_64',
    containerRuntime: 'containerd://1.7.25',
    addresses: [`InternalIP ${view.internalIp}`, `Hostname ${name}.eu-west-1.compute.internal`],
  };
}

const event = (
  type: string,
  reason: string,
  kind: string,
  name: string,
  message: string,
  ago: number,
  count = 1,
): EventView => ({
  namespace: 'shop',
  name: `${name}.${reason}.${ago}`,
  type,
  reason,
  message,
  object: { kind, namespace: kind === 'Node' ? null : 'shop', name },
  count,
  firstSeen: iso(ago + count * 60),
  lastSeen: iso(ago),
  source: kind === 'Pod' ? 'kubelet' : 'deployment-controller',
});

export const events: EventView[] = [
  event(
    'Warning',
    'BackOff',
    'Pod',
    'api-6b8f4d7c5-xr4np',
    'Back-off restarting failed container api in pod api-6b8f4d7c5-xr4np',
    40,
    23,
  ),
  event(
    'Warning',
    'Failed',
    'Pod',
    'search-5c7b9d8f6-q9vht',
    'Failed to pull image "ghcr.io/acme/search:0.8.1": not found',
    2 * MIN,
    9,
  ),
  event(
    'Warning',
    'NodeNotReady',
    'Node',
    'ip-10-0-2-56',
    'Node ip-10-0-2-56 status is now: NodeNotReady',
    6 * MIN,
  ),
  event(
    'Normal',
    'ScalingReplicaSet',
    'Deployment',
    'api',
    'Scaled up replica set api-6b8f4d7c5 to 1',
    25 * MIN,
  ),
  event(
    'Normal',
    'SuccessfulCreate',
    'Job',
    'migrate-2-5-0',
    'Created pod: migrate-2-5-0-tq7cz',
    50 * MIN,
  ),
  event('Normal', 'Completed', 'Job', 'migrate-2-5-0', 'Job completed', 48 * MIN),
  event(
    'Normal',
    'Pulled',
    'Pod',
    'web-6d4c8b9f5-4tzpl',
    'Container image "nginx:1.27-alpine" already present on machine',
    3 * HOUR,
  ),
];

export const summary: ClusterSummary = {
  nodes: { state: 'read', value: { ready: 4, total: 5, notReady: ['ip-10-0-2-56'] } },
  pods: {
    state: 'read',
    value: {
      total: pods.length,
      running: 15,
      pending: 0,
      succeeded: 1,
      failed: 0,
      crashing: 2,
      other: 0,
    },
  },
  unready: {
    state: 'read',
    value: [...workloads.deployment, ...workloads.daemonSet].filter((w) => w.ready < w.desired),
  },
  warnings: { state: 'read', value: events.filter((e) => e.type === 'Warning') },
};

/** kind-dev at a glance: one Node, a few Pods. */
export const kindSummary: ClusterSummary = {
  nodes: { state: 'read', value: { ready: 1, total: 1, notReady: [] } },
  pods: {
    state: 'read',
    value: { total: 11, running: 11, pending: 0, succeeded: 0, failed: 0, crashing: 0, other: 0 },
  },
  unready: { state: 'read', value: [] },
  warnings: { state: 'read', value: [] },
};

export const localClusters: LocalClusters = {
  found: [
    {
      tool: 'kind',
      context: 'kind-dev',
      suggestedName: 'kind-dev',
      server: 'https://127.0.0.1:41235',
      serverVersion: 'v1.34.0',
      added: { id: KIND, name: 'kind-dev' },
    },
    {
      tool: 'dockerDesktop',
      context: 'docker-desktop',
      suggestedName: 'Docker Desktop',
      server: 'https://kubernetes.docker.internal:6443',
      serverVersion: 'v1.34.1',
      added: null,
    },
    {
      tool: 'orbstack',
      context: 'orbstack',
      suggestedName: 'OrbStack',
      server: 'https://127.0.0.1:26443',
      serverVersion: null,
      added: null,
    },
  ],
  missing: [
    { tool: 'k3d', name: 'edge', command: 'k3d kubeconfig merge edge --kubeconfig-merge-default' },
  ],
};

export const kubeContexts: KubeContexts = {
  paths: ['/Users/demo/.kube/config'],
  contexts: [
    {
      name: 'arn:aws:eks:eu-west-1:000000000000:cluster/prod',
      server: EKS_SERVER,
      user: 'eks-prod',
      namespace: 'shop',
      auth: { method: 'exec', command: 'aws' },
      current: false,
      added: { id: EKS, name: 'eks-prod' },
      problem: null,
    },
    {
      name: 'gke_acme-staging_europe-west1_staging',
      server: 'https://203.0.113.40',
      user: 'gke-staging',
      namespace: null,
      auth: { method: 'exec', command: 'gke-gcloud-auth-plugin' },
      current: true,
      added: null,
      problem: null,
    },
    {
      name: 'aks-analytics',
      server: 'https://aks-analytics-dns-0000.hcp.westeurope.azmk8s.io:443',
      user: 'clusterUser_analytics',
      namespace: 'reports',
      auth: { method: 'exec', command: 'kubelogin' },
      current: false,
      added: null,
      problem: null,
    },
    {
      name: 'kind-dev',
      server: 'https://127.0.0.1:41235',
      user: 'kind-dev',
      namespace: null,
      auth: { method: 'clientCertificate' },
      current: false,
      added: { id: KIND, name: 'kind-dev' },
      problem: null,
    },
    {
      name: 'old-gke',
      server: 'https://198.51.100.12',
      user: 'gcp-user',
      namespace: null,
      auth: null,
      current: false,
      added: null,
      problem: {
        code: 'kubeconfig_auth_provider_unsupported',
        key: 'errors.clusters.kubeconfig.authProviderUnsupported',
        params: { provider: 'gcp' },
        message:
          'Its user signs in with the gcp auth provider, which Kubernetes no longer supports.',
        hint: null,
        detail: null,
      },
    },
  ],
};
