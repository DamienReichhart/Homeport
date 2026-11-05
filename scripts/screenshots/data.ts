/**
 * Demo data for the website screenshots: three Hosts, a dozen containers in a
 * few Stacks, and the images, volumes, networks and events that go with them.
 * Everything here is invented; keep it free of real names, keys and addresses.
 */
import type {
  Container,
  DiskUsage,
  EngineEvent,
  EngineInfo,
  HostView,
  Image,
  ManagedStack,
  NetworkView,
  Stack,
  VolumeView,
} from '@/shared/api';

export const NOW = Math.floor(Date.now() / 1000);
const MIN = 60;
const HOUR = 60 * MIN;
const DAY = 24 * HOUR;
const MB = 1024 * 1024;
const GB = 1024 * MB;

/** A stable 64-character hex id derived from a name, like an Engine id. */
export function hexId(seed: string): string {
  let h = 2166136261;
  let out = '';
  while (out.length < 64) {
    for (const ch of seed + out.length) h = Math.imul(h ^ ch.charCodeAt(0), 16777619) >>> 0;
    out += h.toString(16).padStart(8, '0');
  }
  return out.slice(0, 64);
}

/** A small seeded PRNG, so every capture shows the same numbers. */
export function random(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s / 2 ** 32;
  };
}

export const PROD = 'host-prod-eu-1';
export const LOCAL = 'host-this-mac';
export const STAGING = 'host-staging';

const DEMO_KEY =
  'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHqkV0m3cDemoKeyForScreenshotsOnlyX7f2Lq9rTz homeport@prod-eu-1';

export const hosts: HostView[] = [
  {
    host: {
      id: PROD,
      name: 'prod-eu-1',
      connection: {
        kind: 'ssh',
        host: 'prod-eu-1.example.com',
        port: 22,
        user: 'deploy',
        socketPath: '/var/run/docker.sock',
        auth: 'generated',
        publicKey: DEMO_KEY,
        bastions: [],
      },
      enabled: true,
      hostKey: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDemoServerKeyNotReal',
      createdAt: new Date((NOW - 40 * DAY) * 1000).toISOString(),
    },
    status: {
      state: 'connected',
      engineVersion: '28.5.1',
      apiVersion: '1.51',
      os: 'linux',
      arch: 'x86_64',
    },
  },
  {
    host: {
      id: LOCAL,
      name: 'This Mac',
      connection: { kind: 'local', socketPath: '/var/run/docker.sock' },
      enabled: true,
      hostKey: null,
      createdAt: new Date((NOW - 60 * DAY) * 1000).toISOString(),
    },
    status: {
      state: 'connected',
      engineVersion: '28.5.1',
      apiVersion: '1.51',
      os: 'linux',
      arch: 'aarch64',
    },
  },
  {
    host: {
      id: STAGING,
      name: 'staging',
      connection: { kind: 'tls', address: 'staging.example.com:2376' },
      enabled: true,
      hostKey: null,
      createdAt: new Date((NOW - 30 * DAY) * 1000).toISOString(),
    },
    status: {
      state: 'connected',
      engineVersion: '27.3.1',
      apiVersion: '1.47',
      os: 'linux',
      arch: 'x86_64',
    },
  },
];

export const sshSetup = {
  publicKey: DEMO_KEY,
  fingerprint: 'SHA256:q3Vd8cXnR2mJ0k1yWf5tZp6oLhA9sBe4uGiC7vNwKxE',
  installCommand: `mkdir -p ~/.ssh && echo '${DEMO_KEY}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys`,
  dockerGroupCommand: 'sudo usermod -aG docker deploy',
};

interface Spec {
  name: string;
  image: string;
  stack?: string;
  service?: string;
  running: boolean;
  status: string;
  health?: string;
  ports?: [number | null, number][];
  age: number;
  cpu?: number;
  memory?: number;
  command: string;
  networks: string[];
}

function container(host: string, s: Spec): Container {
  const labels: Record<string, string> = {};
  if (s.stack && s.service) {
    labels['com.docker.compose.project'] = s.stack;
    labels['com.docker.compose.service'] = s.service;
  }
  return {
    id: hexId(`${host}/${s.name}`),
    name: s.name,
    image: s.image,
    imageId: `sha256:${hexId(s.image)}`,
    command: s.command,
    created: NOW - s.age,
    state: s.running ? 'running' : 'exited',
    status: s.status,
    health: s.health ?? null,
    ports: (s.ports ?? []).map(([publicPort, privatePort]) => ({
      ip: publicPort ? '0.0.0.0' : null,
      privatePort,
      publicPort,
      protocol: 'tcp',
    })),
    labels,
    stack: s.stack ?? null,
    service: s.service ?? null,
    networks: s.networks,
  };
}

const API = 'ghcr.io/acme/shop-api:2.4.1';

const prodSpecs: Spec[] = [
  {
    name: 'shop-web-1',
    image: 'nginx:1.27-alpine',
    stack: 'shop',
    service: 'web',
    running: true,
    status: 'Up 6 days',
    ports: [
      [443, 443],
      [80, 80],
    ],
    age: 6 * DAY,
    cpu: 0.4,
    memory: 18 * MB,
    command: "/docker-entrypoint.sh nginx -g 'daemon off;'",
    networks: ['shop_default'],
  },
  {
    name: 'shop-api-1',
    image: API,
    stack: 'shop',
    service: 'api',
    running: true,
    status: 'Up 6 days (healthy)',
    health: 'healthy',
    ports: [[null, 8080]],
    age: 6 * DAY,
    cpu: 12.6,
    memory: 412 * MB,
    command: 'node dist/server.js',
    networks: ['shop_default'],
  },
  {
    name: 'shop-api-2',
    image: API,
    stack: 'shop',
    service: 'api',
    running: true,
    status: 'Up 6 days (healthy)',
    health: 'healthy',
    ports: [[null, 8080]],
    age: 6 * DAY,
    cpu: 9.8,
    memory: 398 * MB,
    command: 'node dist/server.js',
    networks: ['shop_default'],
  },
  {
    name: 'shop-worker-1',
    image: API,
    stack: 'shop',
    service: 'worker',
    running: true,
    status: 'Up 6 days',
    age: 6 * DAY,
    cpu: 3.1,
    memory: 236 * MB,
    command: 'node dist/worker.js',
    networks: ['shop_default'],
  },
  {
    name: 'shop-db-1',
    image: 'postgres:16.4-alpine',
    stack: 'shop',
    service: 'db',
    running: true,
    status: 'Up 6 days (healthy)',
    health: 'healthy',
    ports: [[null, 5432]],
    age: 6 * DAY,
    cpu: 4.7,
    memory: 1.12 * GB,
    command: 'docker-entrypoint.sh postgres',
    networks: ['shop_default'],
  },
  {
    name: 'shop-cache-1',
    image: 'redis:7.4-alpine',
    stack: 'shop',
    service: 'cache',
    running: true,
    status: 'Up 6 days (healthy)',
    health: 'healthy',
    ports: [[null, 6379]],
    age: 6 * DAY,
    cpu: 0.9,
    memory: 64 * MB,
    command: 'docker-entrypoint.sh redis-server',
    networks: ['shop_default'],
  },
  {
    name: 'monitoring-grafana-1',
    image: 'grafana/grafana:11.3.0',
    stack: 'monitoring',
    service: 'grafana',
    running: true,
    status: 'Up 2 weeks',
    ports: [[3000, 3000]],
    age: 15 * DAY,
    cpu: 1.2,
    memory: 142 * MB,
    command: '/run.sh',
    networks: ['monitoring_default'],
  },
  {
    name: 'monitoring-prometheus-1',
    image: 'prom/prometheus:v2.55.0',
    stack: 'monitoring',
    service: 'prometheus',
    running: true,
    status: 'Up 2 weeks',
    ports: [[9090, 9090]],
    age: 15 * DAY,
    cpu: 2.4,
    memory: 520 * MB,
    command: '/bin/prometheus --config.file=/etc/prometheus/prometheus.yml',
    networks: ['monitoring_default'],
  },
  {
    name: 'monitoring-node-exporter-1',
    image: 'prom/node-exporter:v1.8.2',
    stack: 'monitoring',
    service: 'node-exporter',
    running: true,
    status: 'Up 2 weeks',
    ports: [[null, 9100]],
    age: 15 * DAY,
    cpu: 0.3,
    memory: 21 * MB,
    command: '/bin/node_exporter --path.rootfs=/host',
    networks: ['monitoring_default'],
  },
  {
    name: 'website-ghost-1',
    image: 'ghost:5.96-alpine',
    stack: 'website',
    service: 'ghost',
    running: false,
    status: 'Exited (0) 2 days ago',
    ports: [[null, 2368]],
    age: 21 * DAY,
    command: 'docker-entrypoint.sh node current/index.js',
    networks: ['website_default'],
  },
  {
    name: 'website-mysql-1',
    image: 'mysql:8.4',
    stack: 'website',
    service: 'mysql',
    running: false,
    status: 'Exited (0) 2 days ago',
    ports: [
      [null, 3306],
      [null, 33060],
    ],
    age: 21 * DAY,
    command: 'docker-entrypoint.sh mysqld',
    networks: ['website_default'],
  },
  {
    name: 'uptime-kuma',
    image: 'louislam/uptime-kuma:1.23.15',
    running: true,
    status: 'Up 4 days (healthy)',
    health: 'healthy',
    ports: [[3001, 3001]],
    age: 9 * DAY,
    cpu: 1.6,
    memory: 118 * MB,
    command: '/usr/bin/dumb-init -- extra/entrypoint.sh node server/server.js',
    networks: ['bridge'],
  },
  {
    name: 'volume-backup',
    image: 'offen/docker-volume-backup:v2.43.0',
    running: true,
    status: 'Up 6 days',
    age: 6 * DAY,
    cpu: 0.0,
    memory: 9 * MB,
    command: '/root/entrypoint.sh',
    networks: ['bridge'],
  },
];

const localSpecs: Spec[] = [
  {
    name: 'dev-postgres-1',
    image: 'postgres:16.4-alpine',
    stack: 'dev',
    service: 'postgres',
    running: true,
    status: 'Up 3 hours',
    ports: [[5432, 5432]],
    age: 3 * HOUR,
    cpu: 0.6,
    memory: 48 * MB,
    command: 'docker-entrypoint.sh postgres',
    networks: ['dev_default'],
  },
  {
    name: 'dev-mailpit-1',
    image: 'axllent/mailpit:v1.21',
    stack: 'dev',
    service: 'mailpit',
    running: true,
    status: 'Up 3 hours (healthy)',
    health: 'healthy',
    ports: [
      [8025, 8025],
      [1025, 1025],
    ],
    age: 3 * HOUR,
    cpu: 0.1,
    memory: 14 * MB,
    command: '/mailpit',
    networks: ['dev_default'],
  },
  {
    name: 'dev-minio-1',
    image: 'minio/minio:RELEASE.2024-10-13T13-34-11Z',
    stack: 'dev',
    service: 'minio',
    running: true,
    status: 'Up 3 hours',
    ports: [
      [9000, 9000],
      [9001, 9001],
    ],
    age: 3 * HOUR,
    cpu: 0.3,
    memory: 96 * MB,
    command: 'minio server /data --console-address :9001',
    networks: ['dev_default'],
  },
  {
    name: 'node-sandbox',
    image: 'node:22-alpine',
    running: false,
    status: 'Exited (130) 5 hours ago',
    age: 26 * HOUR,
    command: 'node',
    networks: ['bridge'],
  },
];

const stagingSpecs: Spec[] = prodSpecs
  .filter((s) => s.stack === 'shop')
  .map((s) => ({
    ...s,
    status: s.status.replace('6 days', '5 hours'),
    age: 5 * HOUR,
    cpu: (s.cpu ?? 0) / 3,
    memory: (s.memory ?? 0) * 0.6,
  }));

const SPECS: Record<string, Spec[]> = {
  [PROD]: prodSpecs,
  [LOCAL]: localSpecs,
  [STAGING]: stagingSpecs,
};

export function containersOf(hostId: string): Container[] {
  return (SPECS[hostId] ?? []).map((s) => container(hostId, s));
}

/** Live CPU and memory for every running container on a Host. */
export function usageOf(hostId: string, jitter = 0) {
  const rnd = random(jitter + 7);
  return (SPECS[hostId] ?? [])
    .filter((s) => s.running)
    .map((s) => ({
      id: hexId(`${hostId}/${s.name}`),
      cpuPercent: Math.max(0, (s.cpu ?? 0) * (0.9 + rnd() * 0.2)),
      memoryUsage: s.memory ?? 0,
      memoryLimit: 32 * GB,
    }));
}

export function findContainer(id: string): { hostId: string; container: Container } | null {
  for (const hostId of Object.keys(SPECS)) {
    const found = containersOf(hostId).find((c) => c.id === id || c.id.startsWith(id));
    if (found) return { hostId, container: found };
  }
  return null;
}

export const engineInfo: Record<string, EngineInfo> = {
  [PROD]: {
    name: 'prod-eu-1',
    serverVersion: '28.5.1',
    operatingSystem: 'Ubuntu 24.04.1 LTS',
    kernelVersion: '6.8.0-48-generic',
    architecture: 'x86_64',
    cpus: 8,
    memory: 32 * GB,
    rootDir: '/var/lib/docker',
    storageDriver: 'overlay2',
    containers: 13,
    running: 11,
    paused: 0,
    stopped: 2,
    images: 13,
  },
  [LOCAL]: {
    name: 'docker-desktop',
    serverVersion: '28.5.1',
    operatingSystem: 'Docker Desktop',
    kernelVersion: '6.10.14-linuxkit',
    architecture: 'aarch64',
    cpus: 10,
    memory: 16 * GB,
    rootDir: '/var/lib/docker',
    storageDriver: 'overlayfs',
    containers: 4,
    running: 3,
    paused: 0,
    stopped: 1,
    images: 9,
  },
  [STAGING]: {
    name: 'staging-1',
    serverVersion: '27.3.1',
    operatingSystem: 'Debian GNU/Linux 12 (bookworm)',
    kernelVersion: '6.1.0-26-amd64',
    architecture: 'x86_64',
    cpus: 4,
    memory: 8 * GB,
    rootDir: '/var/lib/docker',
    storageDriver: 'overlay2',
    containers: 6,
    running: 6,
    paused: 0,
    stopped: 0,
    images: 7,
  },
};

export const diskUsage: DiskUsage = {
  images: { total: 13, active: 11, size: 3.94 * GB, reclaimable: 472 * MB },
  containers: { total: 13, active: 11, size: 182 * MB, reclaimable: 3.4 * MB },
  volumes: { total: 9, active: 7, size: 6.81 * GB, reclaimable: 1.18 * GB },
  buildCache: { total: 24, active: 0, size: 1.06 * GB, reclaimable: 1.06 * GB },
  partial: false,
};

const imageSpecs: [string[], number, number, number][] = [
  // tags, size, age, containers
  [['ghcr.io/acme/shop-api:2.4.1', 'ghcr.io/acme/shop-api:latest'], 238 * MB, 6 * DAY, 3],
  [['ghcr.io/acme/shop-api:2.4.0'], 236 * MB, 13 * DAY, 0],
  [['nginx:1.27-alpine'], 47.9 * MB, 24 * DAY, 1],
  [['postgres:16.4-alpine'], 262 * MB, 32 * DAY, 1],
  [['redis:7.4-alpine'], 45.2 * MB, 28 * DAY, 1],
  [['grafana/grafana:11.3.0'], 476 * MB, 18 * DAY, 1],
  [['prom/prometheus:v2.55.0'], 288 * MB, 19 * DAY, 1],
  [['prom/node-exporter:v1.8.2'], 23.1 * MB, 60 * DAY, 1],
  [['ghost:5.96-alpine'], 579 * MB, 22 * DAY, 1],
  [['mysql:8.4'], 584 * MB, 35 * DAY, 1],
  [['louislam/uptime-kuma:1.23.15'], 440 * MB, 70 * DAY, 1],
  [['offen/docker-volume-backup:v2.43.0'], 33.4 * MB, 40 * DAY, 1],
  [[], 236 * MB, 20 * DAY, 0],
];

export const images: Image[] = imageSpecs.map(([tags, size, age, containers], i) => ({
  id: `sha256:${hexId(tags[0] ?? `dangling-${i}`)}`,
  tags,
  digests: tags[0]
    ? [`${tags[0].replace(/:[^:/]+$/, '')}@sha256:${hexId(`digest-${tags[0]}`)}`]
    : [],
  created: NOW - age,
  size,
  containers,
  labels: {},
}));

/** The label Compose puts on everything it creates for a Stack. */
function stackLabels(stack: string | null): Record<string, string> {
  return stack ? { 'com.docker.compose.project': stack } : {};
}

const volumeSpecs: [string, string | null, string[], number][] = [
  ['shop_pgdata', 'shop', ['shop-db-1'], 6 * DAY],
  ['shop_redis', 'shop', ['shop-cache-1'], 6 * DAY],
  ['shop_certs', 'shop', ['shop-web-1'], 6 * DAY],
  ['monitoring_grafana', 'monitoring', ['monitoring-grafana-1'], 15 * DAY],
  ['monitoring_prometheus', 'monitoring', ['monitoring-prometheus-1'], 15 * DAY],
  ['website_content', 'website', ['website-ghost-1'], 21 * DAY],
  ['website_mysql', 'website', ['website-mysql-1'], 21 * DAY],
  ['uptime-kuma', null, ['uptime-kuma'], 9 * DAY],
  [hexId('anonymous-volume'), null, [], 30 * DAY],
];

export const volumes: VolumeView[] = volumeSpecs.map(([name, stack, usedBy, age]) => ({
  name,
  driver: 'local',
  mountpoint: `/var/lib/docker/volumes/${name}/_data`,
  createdAt: new Date((NOW - age) * 1000).toISOString(),
  labels: stackLabels(stack),
  stack,
  usedBy,
}));

function network(
  name: string,
  driver: string,
  subnet: string | null,
  containers: string[],
  stack: string | null = null,
  builtIn = false,
): NetworkView {
  return {
    id: hexId(`network/${name}`),
    name,
    driver,
    scope: 'local',
    internal: false,
    attachable: false,
    subnets: subnet ? [subnet] : [],
    created: new Date((NOW - 40 * DAY) * 1000).toISOString(),
    labels: stackLabels(stack),
    stack,
    builtIn,
    containers,
  };
}

export const networks: NetworkView[] = [
  network('bridge', 'bridge', '172.17.0.0/16', ['uptime-kuma', 'volume-backup'], null, true),
  network('host', 'host', null, [], null, true),
  network('none', 'null', null, [], null, true),
  network(
    'shop_default',
    'bridge',
    '172.18.0.0/16',
    ['shop-web-1', 'shop-api-1', 'shop-api-2', 'shop-worker-1', 'shop-db-1', 'shop-cache-1'],
    'shop',
  ),
  network(
    'monitoring_default',
    'bridge',
    '172.19.0.0/16',
    ['monitoring-grafana-1', 'monitoring-prometheus-1', 'monitoring-node-exporter-1'],
    'monitoring',
  ),
  network('website_default', 'bridge', '172.20.0.0/16', [], 'website'),
];

export const SHOP_COMPOSE = `name: shop

services:
  web:
    image: nginx:1.27-alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - certs:/etc/nginx/certs:ro
    depends_on:
      - api
    restart: unless-stopped

  api:
    image: ghcr.io/acme/shop-api:\${SHOP_TAG}
    deploy:
      replicas: 2
    environment:
      DATABASE_URL: postgres://shop:\${POSTGRES_PASSWORD}@db:5432/shop
      REDIS_URL: redis://cache:6379
    healthcheck:
      test: ["CMD", "wget", "-qO-", "http://localhost:8080/healthz"]
      interval: 10s
    restart: unless-stopped

  worker:
    image: ghcr.io/acme/shop-api:\${SHOP_TAG}
    command: ["node", "dist/worker.js"]
    environment:
      DATABASE_URL: postgres://shop:\${POSTGRES_PASSWORD}@db:5432/shop
    restart: unless-stopped

  db:
    image: postgres:16.4-alpine
    environment:
      POSTGRES_USER: shop
      POSTGRES_PASSWORD: \${POSTGRES_PASSWORD}
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD", "pg_isready", "-U", "shop"]
    restart: unless-stopped

  cache:
    image: redis:7.4-alpine
    volumes:
      - redis:/data
    restart: unless-stopped

volumes:
  pgdata:
  redis:
  certs:
`;

export const SHOP_ENV = `SHOP_TAG=2.4.1
POSTGRES_PASSWORD=s3cret-demo
`;

const WEBSITE_COMPOSE = `services:
  ghost:
    image: ghost:5.96-alpine
    environment:
      url: https://www.example.com
      database__client: mysql
      database__connection__host: mysql
    volumes:
      - content:/var/lib/ghost/content
  mysql:
    image: mysql:8.4
    volumes:
      - mysql:/var/lib/mysql

volumes:
  content:
  mysql:
`;

const ANALYTICS_COMPOSE = `services:
  plausible:
    image: ghcr.io/plausible/community-edition:v2.1
    ports:
      - "8000:8000"
`;

export const managedStacks: ManagedStack[] = [
  ['stack-shop', 'shop', SHOP_COMPOSE, 6 * DAY],
  ['stack-website', 'website', WEBSITE_COMPOSE, 21 * DAY],
  ['stack-analytics', 'analytics', ANALYTICS_COMPOSE, 1 * HOUR],
].map(([id, name, compose, age]) => ({
  id: id as string,
  hostId: PROD,
  name: name as string,
  compose: compose as string,
  hasEnv: name === 'shop',
  createdAt: new Date((NOW - (age as number)) * 1000).toISOString(),
  updatedAt: new Date((NOW - (age as number)) * 1000).toISOString(),
}));

export function stacksOf(hostId: string): Stack[] {
  const all = containersOf(hostId);
  const names = [...new Set(all.flatMap((c) => (c.stack ? [c.stack] : [])))];
  const stacks: Stack[] = names.map((name) => {
    const members = all.filter((c) => c.stack === name);
    const services = [...new Set(members.map((c) => c.service ?? ''))].map((service) => {
      const of = members.filter((c) => c.service === service);
      return {
        name: service,
        containerIds: of.map((c) => c.id),
        running: of.filter((c) => c.state === 'running').length,
      };
    });
    const running = members.filter((c) => c.state === 'running').length;
    const managed = hostId === PROD ? managedStacks.find((m) => m.name === name) : undefined;
    return {
      name,
      kind: managed ? 'managed' : 'external',
      id: managed?.id ?? null,
      state: running === 0 ? 'stopped' : running === members.length ? 'running' : 'partial',
      services,
      running,
      total: members.length,
      configFiles: managed ? null : `/opt/${name}/compose.yaml`,
      workingDir: null,
      problem: null,
    };
  });
  if (hostId === PROD) {
    stacks.push({
      name: 'analytics',
      kind: 'managed',
      id: 'stack-analytics',
      state: 'notDeployed',
      services: [],
      running: 0,
      total: 0,
      configFiles: null,
      workingDir: null,
      problem: null,
    });
  }
  return stacks.sort((a, b) => a.name.localeCompare(b.name));
}

const eventSpecs: [number, string, string, string][] = [
  [52 * MIN, 'image', 'pull', 'ghcr.io/acme/shop-api:2.4.1'],
  [51 * MIN, 'container', 'create', 'shop-api-2'],
  [51 * MIN, 'network', 'connect', 'shop_default'],
  [51 * MIN, 'container', 'start', 'shop-api-2'],
  [50 * MIN, 'container', 'health_status: healthy', 'shop-api-2'],
  [38 * MIN, 'container', 'exec_start: psql -U shop', 'shop-db-1'],
  [31 * MIN, 'volume', 'mount', 'shop_pgdata'],
  [24 * MIN, 'container', 'kill', 'volume-backup'],
  [24 * MIN, 'container', 'restart', 'volume-backup'],
  [12 * MIN, 'container', 'exec_die', 'shop-db-1'],
  [6 * MIN, 'container', 'health_status: healthy', 'shop-api-1'],
  [3 * MIN, 'image', 'tag', 'ghcr.io/acme/shop-api:latest'],
  [2 * MIN, 'container', 'top', 'shop-worker-1'],
  [40, 'container', 'health_status: healthy', 'uptime-kuma'],
];

export const events: EngineEvent[] = eventSpecs.map(([ago, kind, action, name]) => ({
  time: NOW - ago,
  kind,
  action,
  actorId: hexId(`actor/${name}`),
  attributes: {
    name,
    ...stackLabels(
      kind === 'container' && /^(shop|monitoring|website)-/.test(name)
        ? (name.split('-')[0] ?? null)
        : null,
    ),
  },
}));
