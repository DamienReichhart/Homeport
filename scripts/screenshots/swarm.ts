/**
 * The demo Swarm's nodes, tasks, stacks, secrets and configs, for the
 * `/swarms` screenshots. Invented like the rest: no real names, keys or tokens.
 */
import type { JoinInfo, Stack, SwarmNode, SwarmObject, SwarmTask, TaskFilter } from '@/shared/api';
import { hexId, NOW, PROD, STAGING, swarmServices } from './data';

const HOUR = 3600;
const DAY = 24 * HOUR;
const iso = (ago: number) => new Date((NOW - ago) * 1000).toISOString();
const short = (seed: string) => hexId(seed).slice(0, 25);

const node = (
  hostname: string,
  role: 'manager' | 'worker',
  overrides: Partial<SwarmNode> = {},
): SwarmNode => ({
  id: short(`node/${hostname}`),
  version: 120,
  hostname,
  role,
  availability: 'active',
  state: 'ready',
  message: null,
  managerStatus: role === 'manager' ? 'reachable' : null,
  engineVersion: '28.5.1',
  address: null,
  os: 'linux',
  arch: 'x86_64',
  labels: {},
  created: iso(90 * DAY),
  hostId: null,
  ...overrides,
});

export const swarmNodes: SwarmNode[] = [
  node('prod-eu-1', 'manager', {
    managerStatus: 'leader',
    address: '10.20.0.11',
    hostId: PROD,
    labels: { zone: 'eu-west-1a' },
  }),
  node('staging', 'manager', { address: '10.20.0.12', hostId: STAGING, engineVersion: '27.3.1' }),
  node('prod-eu-3', 'manager', { address: '10.20.0.13', labels: { zone: 'eu-west-1c' } }),
  node('worker-eu-1', 'worker', { address: '10.20.1.21', labels: { tier: 'app' } }),
  node('worker-eu-2', 'worker', {
    address: '10.20.1.22',
    availability: 'drain',
    labels: { tier: 'app' },
  }),
];

const placement = ['prod-eu-1', 'worker-eu-1', 'prod-eu-3', 'staging', 'worker-eu-1'];

/** Each service's tasks, one per replica (or node, for a global one). */
export const swarmTasks: SwarmTask[] = swarmServices.flatMap((service) => {
  const count = service.replicas ?? swarmNodes.length;
  return Array.from({ length: count }, (_, i) => {
    const hostname =
      service.replicas === null
        ? (swarmNodes[i]?.hostname ?? 'prod-eu-1')
        : (placement[(i + service.name.length) % placement.length] ?? 'prod-eu-1');
    const nodeOf = swarmNodes.find((n) => n.hostname === hostname);
    // shop_api is rolling out: its fourth replica can't pull the new image yet.
    const failing = service.name === 'shop_api' && i === 3;
    return {
      id: short(`task/${service.name}/${i}`),
      serviceId: service.id,
      serviceName: service.name,
      slot: service.replicas === null ? null : i + 1,
      nodeId: nodeOf?.id ?? null,
      nodeHostname: hostname,
      desiredState: 'running',
      state: failing ? 'preparing' : 'running',
      since: iso(failing ? 40 : (i + 2) * HOUR),
      error: null,
      message: failing ? 'preparing' : 'started',
      image: service.image,
      containerId: failing ? null : hexId(`task-container/${service.name}/${i}`),
      exitCode: null,
      hostId: nodeOf?.hostId ?? null,
    };
  });
});

export function tasksOf(filter: TaskFilter | undefined): SwarmTask[] {
  return swarmTasks.filter(
    (t) =>
      (!filter?.serviceId || t.serviceId === filter.serviceId) &&
      (!filter?.nodeId || t.nodeId === filter.nodeId),
  );
}

const servicesOf = (stack: string) =>
  swarmServices
    .filter((s) => s.stack === stack)
    .map((s) => ({
      name: s.name.replace(`${stack}_`, ''),
      containerIds: [],
      running: s.running,
      serviceId: s.id,
      desired: s.desired,
    }));

export const swarmStacks: Stack[] = [
  ['shop', 'managed', 'stack-swarm-shop'],
  ['monitoring', 'external', null],
].map(([name, kind, id]) => {
  const services = servicesOf(name as string);
  const running = services.reduce((n, s) => n + s.running, 0);
  const total = services.reduce((n, s) => n + (s.desired ?? 0), 0);
  return {
    name: name as string,
    kind: kind as Stack['kind'],
    id: id as string | null,
    state: running === total ? 'running' : 'partial',
    services,
    running,
    total,
    configFiles: null,
    workingDir: null,
    problem: null,
    updateAvailable: false,
    registryAuth: kind === 'managed',
  };
});

const object = (name: string, services: string[], ago: number): SwarmObject => ({
  id: short(`object/${name}`),
  name,
  created: iso(ago),
  updated: iso(ago),
  labels: name.startsWith('shop_') ? { 'com.docker.stack.namespace': 'shop' } : {},
  services,
});

export const swarmSecrets: SwarmObject[] = [
  object('shop_db_password', ['shop_api', 'shop_db'], 30 * DAY),
  object('shop_stripe_key', ['shop_api'], 12 * DAY),
  object('traefik_acme_token', ['traefik'], 60 * DAY),
];

export const swarmConfigs: SwarmObject[] = [
  object('shop_nginx_conf', ['shop_web'], 6 * DAY),
  object('traefik_yml', ['traefik'], 60 * DAY),
  object('node_exporter_web', [], 45 * DAY),
];

export const joinInfo: JoinInfo = {
  workerToken: 'SWMTKN-1-demo0worker0token0for0screenshots0only-0000000000000000demo',
  managerToken: 'SWMTKN-1-demo0manager0token0for0screenshots0only-000000000000000demo',
  managerAddrs: ['10.20.0.11:2377', '10.20.0.12:2377', '10.20.0.13:2377'],
};

/** What `docker service inspect` would say about a demo service. */
export function serviceInspect(id: string) {
  const s = swarmServices.find((x) => x.id === id);
  return {
    ID: id,
    Version: { Index: s?.version ?? 1 },
    Spec: {
      Name: s?.name,
      Labels: s?.labels,
      TaskTemplate: { ContainerSpec: { Image: s?.image } },
      Mode: s?.replicas === null ? { Global: {} } : { Replicated: { Replicas: s?.replicas } },
      EndpointSpec: {
        Ports: s?.ports.map((p) => ({
          TargetPort: p.target,
          PublishedPort: p.published,
          Protocol: p.protocol,
        })),
      },
    },
    UpdateStatus: s?.updateState ? { State: s.updateState } : undefined,
  };
}
