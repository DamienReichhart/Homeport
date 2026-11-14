import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { RunSpec } from '@/shared/api';
import { normaliseCapability } from './capabilities';
import { copyName } from './duplicate';
import { fromRunSpec } from './fromRunSpec';
import { emptyAdvanced, type AdvancedForm } from './runAdvanced';
import { emptyRunForm, podOf, runFormErrors, toRunSpec, type RunForm } from './runForm';

/**
 * React's Run requests for a table of forms, kept as
 * `app/tests/fixtures/run-specs.json`: the iced app's Run dialog is held to
 * sending the same request for the same form (`app/src/ui/domain/containers/
 * run/tests/fixture.rs`). This test fails when React's output drifts from
 * the fixture; `UPDATE_FIXTURES=1` writes it again.
 *
 * A case is filled in one of three ways (`via`):
 * - `run`: the Run dialog's `form`, as typed;
 * - `duplicate`: a copy of the container `from` was read back as, under the
 *   name `copyName` gives among `taken`;
 * - `edit`: Edit's form for the container `name`, read back as `from`.
 *
 * `typedNotAdded` is a capability left in its field when Run was pressed:
 * React's field added it as it lost focus.
 */

/** From the repository's root, where the tests run. */
const FIXTURE = resolve('app/tests/fixtures/run-specs.json');

type Patch = Partial<Omit<RunForm, 'advanced'>> & { advanced?: Partial<AdvancedForm> };

interface Case {
  name: string;
  via: 'run' | 'duplicate' | 'edit';
  /** What the user typed (`run`). */
  form?: RunForm;
  /** The container's settings as `containers_run_spec` read them. */
  from?: RunSpec;
  /** `duplicate`: the names taken; `edit`: the container's name. */
  taken?: string[];
  container?: string;
  typedNotAdded?: { capAdd?: string; capDrop?: string };
  /** React's request; `null` when the form can't be sent as typed. */
  spec: RunSpec | null;
  /** The pod it runs in (`podOf`). */
  pod: string | null;
}

const form = (patch: Patch): RunForm => {
  const empty = emptyRunForm('nginx:1.27');
  return { ...empty, ...patch, advanced: { ...emptyAdvanced(), ...patch.advanced } };
};

const MIB = 1024 ** 2;

/** A container's settings, read back, with something in each section. */
const full: RunSpec = {
  image: 'nginx:1.27',
  name: null,
  command: "nginx -c '/etc/nginx/alt conf'",
  env: ['MODE=prod', 'URL=postgres://app:pw@db/app?ssl=1'],
  ports: [
    { hostIp: null, hostPort: 8080, containerPort: 80, protocol: 'tcp' },
    { hostIp: null, hostPort: null, containerPort: 53, protocol: 'udp' },
  ],
  mounts: [
    { source: 'site', target: '/srv', readOnly: true },
    { source: '/etc/nginx/conf.d', target: '/etc/nginx/conf.d', readOnly: false },
  ],
  network: 'proxy',
  restart: 'unless-stopped',
  autoRemove: false,
  cpus: 1.5,
  cpuShares: 512,
  memory: 512 * MIB,
  memoryReservation: 256 * MIB,
  memorySwap: -1,
  labels: ['team=web', 'traefik.enable=true'],
  capAdd: ['NET_ADMIN'],
  capDrop: ['ALL'],
  privileged: false,
  devices: [{ hostPath: '/dev/ttyUSB0', containerPath: '/dev/zigbee', permissions: 'rw' }],
  gpus: { kind: 'count', count: 2 },
  healthcheck: {
    command: 'curl -f localhost',
    intervalS: 90,
    timeoutS: 5,
    retries: 3,
    startPeriodS: null,
    disabled: false,
  },
  hostname: 'web',
  domainname: null,
  dns: ['1.1.1.1'],
  dnsSearch: ['home.arpa'],
  extraHosts: ['nas:192.168.1.10'],
  log: { driver: 'local', options: ['max-size=10m'] },
  basedOn: 'c1',
};

const inputs: Omit<Case, 'spec' | 'pod'>[] = [
  { name: 'an image alone', via: 'run', form: form({}) },
  {
    name: 'environment values holding = and empty ones',
    via: 'run',
    form: form({
      name: 'web',
      command: "sh -c 'echo $A'",
      env: 'A=b=c\nEMPTY=\n\n# a comment\nURL=postgres://u:p@h/db?x=1\n  SPACED = kept  \n',
    }),
  },
  {
    name: 'ports over tcp and udp, with and without a host port',
    via: 'run',
    form: form({
      ports: [
        { hostPort: '8080', containerPort: '80', protocol: 'tcp' },
        { hostPort: '', containerPort: '53', protocol: 'udp' },
        { hostPort: '5353', containerPort: '5353', protocol: 'udp' },
        { hostPort: '', containerPort: '', protocol: 'tcp' },
      ],
    }),
  },
  {
    name: 'a port range and an IPv6 address are no ports',
    via: 'run',
    form: form({
      ports: [
        { hostPort: '8000-8010', containerPort: '8000-8010', protocol: 'tcp' },
        { hostPort: '[::1]:8080', containerPort: '80', protocol: 'tcp' },
      ],
    }),
  },
  {
    name: 'named volumes and bind mounts, read-only or not',
    via: 'run',
    form: form({
      mounts: [
        { source: 'site', target: '/srv', readOnly: true },
        { source: ' /srv/data ', target: '/data', readOnly: false },
        { source: '', target: '', readOnly: false },
      ],
    }),
  },
  {
    name: 'a network, a restart policy',
    via: 'run',
    form: form({ network: 'proxy', restart: 'on-failure' }),
  },
  {
    name: 'removed when it exits',
    via: 'run',
    form: form({ restart: 'no', autoRemove: true }),
  },
  {
    name: 'resources and labels',
    via: 'run',
    form: form({
      advanced: {
        resources: {
          cpus: '0.5',
          cpuShares: '512',
          memory: '1.5g',
          memoryReservation: '256MiB',
          memorySwap: '-1',
        },
        labels: 'traefik.enable=true\n\ntraefik.http.routers.web.rule=Host(`web.home.arpa`)',
      },
    }),
  },
  {
    name: 'security, devices and a number of GPUs',
    via: 'run',
    form: form({
      advanced: {
        security: {
          capAdd: ['NET_ADMIN', 'SYS_MODULE'],
          capDrop: ['ALL'],
          privileged: true,
          devices: [
            { hostPath: '/dev/dri', containerPath: '', permissions: '' },
            { hostPath: '/dev/ttyUSB0', containerPath: '/dev/zigbee', permissions: 'rw' },
            { hostPath: '', containerPath: '', permissions: 'rwm' },
          ],
          gpus: { mode: 'count', count: '2', ids: '' },
        },
      },
    }),
  },
  {
    name: 'chosen GPUs, a healthcheck, the network identity and logging',
    via: 'run',
    form: form({
      advanced: {
        security: {
          ...emptyAdvanced().security,
          gpus: { mode: 'ids', count: '1', ids: '0, GPU-abc' },
        },
        healthcheck: {
          mode: 'custom',
          command: ' curl -f localhost ',
          interval: '30s',
          timeout: '5',
          retries: '3',
          startPeriod: '1m30s',
        },
        identity: {
          hostname: 'web',
          domainname: 'home.arpa',
          dns: '1.1.1.1\n9.9.9.9',
          dnsSearch: 'home.arpa',
          extraHosts: 'nas:192.168.1.10\n# the NAS\nhost.docker.internal:host-gateway',
        },
        logging: { driver: 'json-file', options: 'max-size=10m\nmax-file=3' },
      },
    }),
  },
  {
    name: "the image's healthcheck turned off",
    via: 'run',
    form: form({
      advanced: { healthcheck: { ...emptyAdvanced().healthcheck, mode: 'disabled' } },
    }),
  },
  {
    name: 'run into a pod, whose ports and network it takes',
    via: 'run',
    form: form({
      ports: [{ hostPort: '8080', containerPort: '80', protocol: 'tcp' }],
      network: 'proxy',
      pod: 'shop',
    }),
  },
  {
    name: 'a capability typed and not added',
    via: 'run',
    form: form({
      advanced: { security: { ...emptyAdvanced().security, capAdd: ['NET_ADMIN'] } },
    }),
    typedNotAdded: { capDrop: 'all' },
  },
  {
    name: 'a copy, with what each section holds',
    via: 'duplicate',
    from: full,
    taken: ['web', 'web-copy'],
    container: 'web',
  },
  { name: "Edit's form, read back", via: 'edit', from: full, container: 'web' },
];

/** The form React sent for `input`, as it was when Run or Apply was pressed. */
function sent(input: Omit<Case, 'spec' | 'pod'>): RunForm {
  if (input.via === 'run') {
    const typed = input.form ?? form({});
    const security = typed.advanced.security;
    const added = (list: string[], value?: string) =>
      value ? [...list, normaliseCapability(value)] : list;
    // The field lost focus as Run was pressed (`TokenInput`'s `onBlur`).
    return {
      ...typed,
      advanced: {
        ...typed.advanced,
        security: {
          ...security,
          capAdd: added(security.capAdd, input.typedNotAdded?.capAdd),
          capDrop: added(security.capDrop, input.typedNotAdded?.capDrop),
        },
      },
    };
  }
  const original = fromRunSpec(input.from ?? full);
  return input.via === 'duplicate'
    ? { ...original, name: copyName(input.container ?? '', input.taken ?? []) }
    : { ...original, name: input.container ?? '' };
}

/** Whether every port of `form` is a number, as a JSON request could hold it. */
const portsAreNumbers = (f: RunForm) =>
  toRunSpec(f).ports.every(
    (p) =>
      Number.isInteger(p.containerPort) && (p.hostPort === null || Number.isInteger(p.hostPort)),
  );

const cases: Case[] = inputs.map((input) => {
  const f = sent(input);
  const readable = Object.keys(runFormErrors(f)).length === 0 && portsAreNumbers(f);
  return {
    ...input,
    // A port that is no number went to Tauri as `null`, which refused it.
    spec: readable ? toRunSpec(f) : null,
    pod: podOf(f),
  };
});

describe('the Run requests React sent, kept for the iced app', () => {
  it('are the fixture', () => {
    const json = `${JSON.stringify(cases, null, 2)}\n`;
    if (process.env.UPDATE_FIXTURES) writeFileSync(FIXTURE, json);
    expect(readFileSync(FIXTURE, 'utf8')).toBe(json);
  });
});
