/**
 * Answers the frontend's IPC calls with the demo data in `data.ts`, the way
 * the Rust backend would, including the streams behind logs, stats and
 * terminals. Unknown commands resolve to `null` and are logged.
 */
import { mockIPC, mockWindows } from '@tauri-apps/api/mocks';
import type {
  ContainerUsage,
  EngineInfo,
  ExecEvent,
  LogEvent,
  LogLine,
  LogSource,
  Settings,
  StatsSample,
  Theme,
} from '@/shared/api';
import {
  LOCAL,
  NOW,
  PROD,
  SHOP_ENV,
  containersOf,
  diskUsage,
  engineInfo,
  events,
  findContainer,
  hosts,
  images,
  managedStacks,
  networks,
  random,
  sshSetup,
  stacksOf,
  swarm,
  swarmServices,
  swarmSummary,
  usageOf,
  volumes,
} from './data';
import { clusterHandlers, clusterSettings, swarmHandlers } from './orchestrators';

interface Internals {
  runCallback: (id: number, data: unknown) => void;
}

const sent = new Map<number, number>();

/** Delivers a message to a frontend `Channel`, as the backend would. */
function send(channel: unknown, message: unknown) {
  const { id } = channel as { id: number };
  const index = sent.get(id) ?? 0;
  sent.set(id, index + 1);
  const internals = (window as unknown as { __TAURI_INTERNALS__: Internals }).__TAURI_INTERNALS__;
  internals.runCallback(id, { index, message });
}

let nextStream = 1;
const timers = new Map<number, ReturnType<typeof setInterval>>();

/** Starts a stream: `tick` runs now and then every `every` ms until cancelled. */
function stream(tick: (n: number) => boolean | void, every?: number): number {
  const id = nextStream++;
  let n = 0;
  setTimeout(() => {
    if (tick(n++) === false || !every) return;
    timers.set(
      id,
      setInterval(() => {
        if (tick(n++) === false) clearInterval(timers.get(id));
      }, every),
    );
  }, 0);
  return id;
}

const ESC = '\x1b[';
const dim = (s: string) => `${ESC}2m${s}${ESC}22m`;
const colour = (code: number, s: string) => `${ESC}${code}m${s}${ESC}39m`;

const ROUTES: [string, string, number][] = [
  ['GET', '/api/products?page=1', 200],
  ['GET', '/api/products?page=2', 200],
  ['GET', '/api/products/4812', 200],
  ['GET', '/api/cart', 200],
  ['POST', '/api/cart/items', 201],
  ['GET', '/api/categories', 200],
  ['POST', '/api/checkout', 201],
  ['GET', '/api/orders/18342', 200],
  ['GET', '/api/products/9921', 404],
  ['GET', '/healthz', 200],
];

function clock(t: number): string {
  const d = new Date(t * 1000);
  const pad = (n: number, w = 2) => String(n).padStart(w, '0');
  return `${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}.${pad(d.getMilliseconds(), 3)}`;
}

/** Plausible structured output of the demo shop's API containers. */
function apiLogs(count: number, seed: number): string[] {
  const rnd = random(seed);
  const lines: string[] = [];
  let t = NOW - count * 1.7;
  for (let i = 0; i < count; i++) {
    t += 0.4 + rnd() * 2.6;
    const time = dim(`[${clock(t + rnd())}]`);
    const roll = rnd();
    if (roll < 0.06) {
      lines.push(
        `${time} ${colour(33, 'WARN ')} slow query ${dim('table=')}orders ${dim('ms=')}${Math.round(300 + rnd() * 400)}`,
      );
    } else if (roll < 0.12) {
      lines.push(
        `${time} ${colour(36, 'DEBUG')} cache ${rnd() < 0.7 ? 'hit' : 'miss'} ${dim('key=')}catalog:page:${1 + Math.floor(rnd() * 9)}`,
      );
    } else if (roll < 0.15) {
      lines.push(
        `${time} ${colour(32, 'INFO ')} order placed ${dim('id=')}${18300 + Math.floor(rnd() * 60)} ${dim('total=')}€${(20 + rnd() * 180).toFixed(2)}`,
      );
    } else {
      const [method, path, code] = ROUTES[Math.floor(rnd() * ROUTES.length)] ?? ROUTES[0]!;
      const status = code >= 400 ? colour(33, String(code)) : String(code);
      lines.push(
        `${time} ${colour(32, 'INFO ')} ${method.padEnd(4)} ${path} ${status} ${dim(`${Math.round(3 + rnd() * 40)}ms`)}`,
      );
    }
  }
  return lines;
}

function serviceLogs(service: string, count: number, seed: number): string[] {
  const rnd = random(seed);
  switch (service) {
    case 'api':
    case 'worker':
      return apiLogs(count, seed);
    case 'web':
      return Array.from({ length: count }, () => {
        const [method, path, code] = ROUTES[Math.floor(rnd() * ROUTES.length)] ?? ROUTES[0]!;
        return `203.0.113.${Math.floor(rnd() * 254)} - - "${method} ${path} HTTP/2.0" ${code} ${Math.floor(200 + rnd() * 9000)}`;
      });
    case 'db':
      return Array.from(
        { length: count },
        (_, i) =>
          `${dim('LOG:')}  checkpoint ${i % 2 ? 'complete' : 'starting'}: ${i % 2 ? `wrote ${Math.floor(rnd() * 900)} buffers` : 'time'}`,
      );
    case 'cache':
      return Array.from(
        { length: count },
        (_, i) =>
          `1:M ${dim(`03 Nov 2026 ${clock(NOW - (count - i) * 9)}`)} * ${1 + Math.floor(rnd() * 99)} changes in 300 seconds. Saving...`,
      );
    default:
      return apiLogs(count, seed);
  }
}

function logLines(hostId: string, sources: LogSource[]): LogLine[] {
  const all = containersOf(hostId);
  if (sources.length === 1) {
    const c = all.find((x) => x.id === sources[0]?.containerId);
    return serviceLogs(c?.service ?? 'api', 120, 11).map((text) => ({
      source: null,
      stream: 'stdout',
      text,
    }));
  }
  // Interleave several containers like `docker compose logs`.
  const per = sources.map((s, i) => {
    const c = all.find((x) => x.id === s.containerId);
    return serviceLogs(c?.service ?? 'api', 30, 100 + i).map((text) => ({
      source: s.label,
      stream: 'stdout',
      text,
    }));
  });
  // Pick sources in proportion to what they have left, so they stay mixed.
  const rnd = random(5);
  const out: LogLine[] = [];
  let left = per.reduce((sum, p) => sum + p.length, 0);
  while (left > 0) {
    let pick = rnd() * left;
    const list = per.find((p) => (pick -= p.length) < 0) ?? per.find((p) => p.length);
    const line = list?.shift();
    if (line) out.push(line);
    left--;
  }
  return out;
}

const PSQL_SESSION = [
  '\x1b[1;32mroot@4b1f0c2d9e7a\x1b[0m:/# psql -U shop -d shop\r\n',
  'psql (16.4)\r\nType "help" for help.\r\n\r\n',
  'shop=# \\dt\r\n',
  '            List of relations\r\n',
  ' Schema |    Name     | Type  | Owner\r\n',
  '--------+-------------+-------+-------\r\n',
  ' public | carts       | table | shop\r\n',
  ' public | categories  | table | shop\r\n',
  ' public | customers   | table | shop\r\n',
  ' public | order_items | table | shop\r\n',
  ' public | orders      | table | shop\r\n',
  ' public | products    | table | shop\r\n',
  '(6 rows)\r\n\r\n',
  'shop=# SELECT status, count(*) FROM orders GROUP BY status ORDER BY 2 DESC;\r\n',
  '  status   | count\r\n',
  '-----------+-------\r\n',
  ' delivered | 16904\r\n',
  ' shipped   |   978\r\n',
  ' paid      |   402\r\n',
  ' cancelled |    58\r\n',
  '(4 rows)\r\n\r\n',
  'shop=# ',
];

const SHELL_SESSION = [
  '\x1b[1;32m/app\x1b[0m $ ls\r\n',
  '\x1b[1;34mdist\x1b[0m  \x1b[1;34mnode_modules\x1b[0m  package.json  \x1b[1;34mpublic\x1b[0m\r\n',
  '\x1b[1;32m/app\x1b[0m $ node --version\r\nv22.11.0\r\n',
  '\x1b[1;32m/app\x1b[0m $ wget -qO- localhost:8080/healthz\r\n',
  '{"status":"ok","db":"up","cache":"up","uptime":518402}\r\n',
  '\x1b[1;32m/app\x1b[0m $ ',
];

function inspectOf(id: string) {
  const found = findContainer(id);
  if (!found) return {};
  const c = found.container;
  return {
    Id: c.id,
    Name: `/${c.name}`,
    Created: new Date(c.created * 1000).toISOString(),
    Path: c.command.split(' ')[0],
    Args: c.command.split(' ').slice(1),
    RestartCount: 0,
    State: {
      Status: c.state,
      Running: c.state === 'running',
      StartedAt: new Date(c.created * 1000).toISOString(),
      FinishedAt: '0001-01-01T00:00:00Z',
      ExitCode: 0,
      OOMKilled: false,
      ...(c.health
        ? {
            Health: {
              Status: c.health,
              FailingStreak: 0,
              Log: [
                {
                  Start: new Date((NOW - 10) * 1000).toISOString(),
                  ExitCode: 0,
                  Output: '{"status":"ok"}',
                },
              ],
            },
          }
        : {}),
    },
    Config: {
      Hostname: c.id.slice(0, 12),
      User: '',
      Env: ['NODE_ENV=production', 'PORT=8080', 'PATH=/usr/local/bin:/usr/bin:/bin'],
      Cmd: c.command.split(' '),
      Entrypoint: null,
      Image: c.image,
      WorkingDir: '/app',
      Labels: c.labels,
      Tty: false,
    },
    HostConfig: {
      RestartPolicy: { Name: 'unless-stopped', MaximumRetryCount: 0 },
      NetworkMode: c.networks[0] ?? 'bridge',
      Memory: 0,
      NanoCpus: 0,
      AutoRemove: false,
      Privileged: false,
    },
    Mounts: [],
    NetworkSettings: {
      Ports: Object.fromEntries(
        c.ports.map((p) => [
          `${p.privatePort}/${p.protocol}`,
          p.publicPort ? [{ HostIp: '0.0.0.0', HostPort: String(p.publicPort) }] : null,
        ]),
      ),
      Networks: Object.fromEntries(
        c.networks.map((n, i) => [
          n,
          {
            IPAddress: `172.18.0.${i + 4}`,
            Gateway: '172.18.0.1',
            MacAddress: '02:42:ac:12:00:04',
            Aliases: c.service ? [c.service] : null,
          },
        ]),
      ),
    },
  };
}

/**
 * Resource samples that wander like a busy API process. Byte counters are
 * cumulative, as the Engine reports them, so each stream keeps running totals.
 */
function statsSamples(): (n: number) => StatsSample {
  const total = { netRx: 2e9, netTx: 5e9, blockRead: 3e8, blockWrite: 9e8 };
  return (n) => {
    const rnd = random(n + 3);
    const burst = Math.exp(-(((n % 45) - 36) ** 2) / 8);
    const wave = Math.sin(n / 9) * 3 + Math.sin(n / 3.7) * 1.5;
    total.netRx += 150_000 + Math.sin(n / 7) * 40_000 + rnd() * 30_000 + burst * 300_000;
    total.netTx += 420_000 + Math.sin(n / 7) * 90_000 + rnd() * 60_000 + burst * 900_000;
    total.blockRead += rnd() < 0.08 ? 400_000 * rnd() : 0;
    total.blockWrite += 30_000 + rnd() * 30_000 + burst * 250_000;
    return {
      cpuPercent: Math.max(0.5, 11 + wave + burst * 16 + rnd() * 2.5),
      memoryUsage: (380 + n * 0.25 + Math.sin(n / 13) * 8 + rnd() * 3) * 1024 * 1024,
      memoryLimit: 2 * 1024 * 1024 * 1024,
      ...total,
      pids: 23,
    };
  };
}

export interface MockOptions {
  theme: Theme;
  hostId: string;
  /** The app's language, as Settings stores it. */
  language?: string;
}

/** The page being photographed. */
const routeOf = () => new URLSearchParams(window.location.search).get('route') ?? '/';

export function installMocks({ theme, hostId, language = 'en' }: MockOptions) {
  let settings: Settings = {
    theme,
    language,
    lastHostId: hostId,
    checkForUpdates: true,
    ...clusterSettings(),
  };
  mockWindows('main');

  const handlers: Record<string, (args: Record<string, unknown>) => unknown> = {
    get_app_info: () => ({ name: 'Homeport', version: '0.2.0', os: 'macos', arch: 'aarch64' }),
    get_capabilities: () => ({
      keychain: true,
      tray: true,
      compose: {
        state: 'available',
        cli: { program: 'docker', prefix: ['compose'], version: '2.31.0' },
      },
      stackDeploy: {
        state: 'available',
        cli: { program: 'docker', prefix: [], version: '28.5.1' },
      },
      git: { state: 'available', cli: { program: 'git', version: '2.47.1' } },
      helm: { state: 'available', cli: { program: 'helm', version: 'v3.16.2' } },
    }),
    get_licenses: () => ({ app: 'GNU AFFERO GENERAL PUBLIC LICENSE', third_party: null }),
    // Up to date, so no update bar shows in the screenshots.
    updates_check: () => ({ currentVersion: '0.2.0', available: null, method: { kind: 'inApp' } }),
    get_settings: () => settings,
    update_settings: (a) => {
      settings = a.settings as Settings;
      return settings;
    },
    hosts_list: () => hosts,
    ...swarmHandlers,
    ...clusterHandlers(),
    // The demo Swarm shows only where it's photographed, so other shots stay as they were.
    swarms_list: () => (routeOf().startsWith('/swarms') ? [swarm] : []),
    swarm_inspect: () => swarmSummary,
    swarm_services_list: () => swarmServices,
    registries_list: () => [
      { id: 'r1', address: 'docker.io', username: 'acme', createdAt: '', updatedAt: '' },
      { id: 'r2', address: 'ghcr.io', username: 'acme-ci', createdAt: '', updatedAt: '' },
    ],
    registries_login_for: () => null,
    // Nothing outdated, so no newer-image marks show in the screenshots.
    images_freshness: () => [],
    hosts_detect_local: () => [
      {
        label: 'Docker Desktop',
        socketPath: '/var/run/docker.sock',
        engine: 'docker',
        engineVersion: '28.5.1',
      },
      {
        label: 'Colima',
        socketPath: '/Users/demo/.colima/default/docker.sock',
        engine: 'docker',
        engineVersion: '27.4.0',
      },
    ],
    hosts_ssh_setup: () => sshSetup,
    hosts_events: (a) => (a.id === PROD ? events : events.slice(-4)),
    hosts_reconnect: (a) => hosts.find((h) => h.host.id === a.id),
    containers_list: (a) => containersOf(a.hostId as string),
    containers_inspect: (a) => inspectOf(a.id as string),
    containers_processes: () => ({
      titles: ['UID', 'PID', 'PPID', 'C', 'STIME', 'TTY', 'TIME', 'CMD'],
      processes: [
        ['node', '2841', '2820', '3', '09:12', '?', '00:41:07', 'node dist/server.js'],
        ['node', '2903', '2841', '0', '09:12', '?', '00:00:02', '/usr/local/bin/node --worker'],
        ['node', '2904', '2841', '0', '09:12', '?', '00:00:02', '/usr/local/bin/node --worker'],
      ],
    }),
    files_list: (a) => {
      const now = NOW - 3600;
      const dirs = ['app', 'bin', 'etc', 'home', 'lib', 'tmp', 'usr', 'var'];
      return a.path === '/'
        ? dirs.map((name) => ({
            name,
            path: `/${name}`,
            kind: 'directory',
            size: 4096,
            modified: now,
            mode: 'drwxr-xr-x',
          }))
        : [];
    },
    stream_cancel: (a) => {
      clearInterval(timers.get(a.id as number));
      return null;
    },
    logs_subscribe: (a) => {
      const lines = logLines(a.hostId as string, a.sources as LogSource[]);
      return stream(() => {
        send(a.onEvent, { type: 'lines', lines } satisfies LogEvent);
      });
    },
    stats_subscribe: (a) => {
      const sample = statsSamples();
      return stream((n) => {
        send(a.onSample, sample(n));
        return n < 170;
      }, 40);
    },
    usage_subscribe: (a) =>
      stream((n) => {
        send(a.onUsage, usageOf(a.hostId as string, n) satisfies ContainerUsage[]);
      }, 2000),
    exec_start: (a) => {
      const found = findContainer(a.containerId as string);
      const script = found?.container.service === 'db' ? PSQL_SESSION : SHELL_SESSION;
      return stream((n) => {
        const chunk = script[n];
        if (chunk === undefined) return false;
        send(a.onEvent, { type: 'output', data: chunk } satisfies ExecEvent);
      }, 15);
    },
    exec_input: () => null,
    exec_resize: () => null,
    exec_close: () => null,
    images_list: (a) => (a.hostId === PROD ? images : images.slice(2, 8)),
    images_history: () => [],
    images_inspect: () => ({}),
    volumes_list: (a) => (a.hostId === PROD ? volumes : []),
    volumes_sizes: (a) =>
      Object.fromEntries(
        (a.hostId === PROD ? volumes : []).map((v, i) => [v.name, (i + 1) * 187 * 1024 * 1024]),
      ),
    networks_list: (a) => (a.hostId === PROD ? networks : networks.slice(0, 3)),
    system_info: (a) => engineInfo[a.hostId as string] ?? (engineInfo[LOCAL] as EngineInfo),
    system_disk_usage: () => diskUsage,
    stacks_list: (a) => stacksOf(a.hostId as string),
    stacks_get: (a) => managedStacks.find((m) => m.id === a.id),
    stacks_get_env: (a) => (a.id === 'stack-shop' ? SHOP_ENV : ''),
    stacks_check: () => ({ errors: [], warnings: [], profiles: [] }),
    stacks_compose_subscribe: (a) => {
      const lines = [
        ' Container shop-db-1  Running',
        ' Container shop-cache-1  Running',
        ' Container shop-api-1  Recreate',
        ' Container shop-api-2  Recreate',
        ' Container shop-api-1  Recreated',
        ' Container shop-api-2  Recreated',
        ' Container shop-api-1  Started',
        ' Container shop-api-2  Started',
      ];
      return stream((n) => {
        if (n < lines.length) send(a.onEvent, { type: 'line', text: lines[n] });
        else {
          send(a.onEvent, { type: 'exit', success: true, code: 0, error: null });
          return false;
        }
      }, 60);
    },
  };

  mockIPC(
    (cmd, args) => {
      const handler = handlers[cmd];
      if (!handler) {
        console.warn(`[screenshots] no mock for ${cmd}`, args);
        return null;
      }
      return handler((args ?? {}) as Record<string, unknown>);
    },
    { shouldMockEvents: true },
  );
}
