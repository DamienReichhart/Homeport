import type { RunSpec } from '@/shared/api';
import { copyName, sharedHostPorts } from './duplicate';
import { fromRunSpec, hasAdvanced } from './fromRunSpec';
import { emptyRunForm, toRunSpec } from './runForm';

const full: RunSpec = {
  image: 'nginx:1.27',
  name: null,
  command: "nginx -c '/etc/nginx/alt conf'",
  env: ['MODE=prod'],
  ports: [{ hostIp: null, hostPort: 8080, containerPort: 80, protocol: 'tcp' }],
  mounts: [{ source: 'site', target: '/srv', readOnly: true }],
  network: 'proxy',
  restart: 'unless-stopped',
  autoRemove: false,
  cpus: 1.5,
  cpuShares: 512,
  memory: 512 * 1024 ** 2,
  memoryReservation: 256 * 1024 ** 2,
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
  basedOn: 'abc123',
};

describe('a run form read back from a container', () => {
  it('sends the same request it was read from', () => {
    expect(toRunSpec(fromRunSpec(full))).toEqual(full);
  });

  it('shows sizes and durations as they are typed', () => {
    const form = fromRunSpec(full);
    expect(form.advanced.resources.memory).toBe('512m');
    expect(form.advanced.healthcheck.interval).toBe('1m30s');
    expect(form.advanced.security.gpus).toMatchObject({ mode: 'count', count: '2' });
    expect(hasAdvanced(form)).toBe(true);
  });

  it('leaves the advanced area closed when nothing in it is set', () => {
    expect(hasAdvanced(emptyRunForm('nginx'))).toBe(false);
  });
});

describe('duplicating', () => {
  it('suggests a free copy name', () => {
    expect(copyName('web', ['web'])).toBe('web-copy');
    expect(copyName('web', ['web', 'web-copy', 'web-copy-2'])).toBe('web-copy-3');
  });

  it('finds the host ports the original already publishes', () => {
    const original = fromRunSpec(full);
    const copy = {
      ...original,
      ports: [
        { hostPort: '8080', containerPort: '80', protocol: 'tcp' },
        { hostPort: '8080', containerPort: '81', protocol: 'udp' },
        { hostPort: '', containerPort: '443', protocol: 'tcp' },
      ],
    };
    expect(sharedHostPorts(copy, original)).toEqual(['8080']);
  });
});
