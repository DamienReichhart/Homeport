import { emptyRunForm, podOf, runFormErrors, toRunSpec, type RunForm } from './runForm';
import { emptyAdvanced } from './runAdvanced';

const unset = {
  cpus: null,
  cpuShares: null,
  memory: null,
  memoryReservation: null,
  memorySwap: null,
  labels: [],
  capAdd: [],
  capDrop: [],
  privileged: false,
  devices: [],
  gpus: null,
  healthcheck: null,
  hostname: null,
  domainname: null,
  dns: [],
  dnsSearch: [],
  extraHosts: [],
  log: null,
  basedOn: null,
};

const withAdvanced = (patch: (a: RunForm['advanced']) => void): RunForm => {
  const advanced = emptyAdvanced();
  patch(advanced);
  return { ...emptyRunForm('nginx'), advanced };
};

describe('run form', () => {
  it('leaves ports and the network to the pod a container runs in', () => {
    const form: RunForm = {
      ...emptyRunForm('nginx'),
      ports: [{ hostPort: '8080', containerPort: '80', protocol: 'tcp' }],
      network: 'web',
      pod: 'shop',
    };
    expect(podOf(form)).toBe('shop');
    expect(toRunSpec(form)).toMatchObject({ ports: [], network: null });
    expect(podOf({ ...form, pod: ' ' })).toBeNull();
    expect(toRunSpec({ ...form, pod: '' })).toMatchObject({ network: 'web' });
  });

  it('turns form rows into a run request', () => {
    const spec = toRunSpec({
      ...emptyRunForm(' nginx '),
      name: 'web',
      env: 'A=1\n\n# comment\nB=two words\n',
      ports: [
        { hostPort: '8080', containerPort: '80', protocol: 'tcp' },
        { hostPort: '', containerPort: '53', protocol: 'udp' },
        { hostPort: '', containerPort: '', protocol: 'tcp' },
      ],
      mounts: [
        { source: '/srv', target: '/data', readOnly: true },
        { source: '', target: '', readOnly: false },
      ],
      restart: 'unless-stopped',
    });
    expect(spec).toEqual({
      image: 'nginx',
      name: 'web',
      command: null,
      env: ['A=1', 'B=two words'],
      ports: [
        { hostIp: null, hostPort: 8080, containerPort: 80, protocol: 'tcp' },
        { hostIp: null, hostPort: null, containerPort: 53, protocol: 'udp' },
      ],
      mounts: [{ source: '/srv', target: '/data', readOnly: true }],
      network: null,
      restart: 'unless-stopped',
      autoRemove: false,
      ...unset,
    });
  });

  it('sends limits and labels in bytes and CPUs', () => {
    const form = withAdvanced((a) => {
      a.resources = {
        cpus: '0.5',
        cpuShares: '512',
        memory: '512m',
        memoryReservation: '256m',
        memorySwap: '-1',
      };
      a.labels = 'traefik.enable=true\n\nteam=web';
    });
    expect(runFormErrors(form)).toEqual({});
    expect(toRunSpec(form)).toMatchObject({
      cpus: 0.5,
      cpuShares: 512,
      memory: 512 * 1024 ** 2,
      memoryReservation: 256 * 1024 ** 2,
      memorySwap: -1,
      labels: ['traefik.enable=true', 'team=web'],
    });
  });

  it('sends capabilities, devices and GPUs', () => {
    const form = withAdvanced((a) => {
      a.security = {
        capAdd: ['NET_ADMIN'],
        capDrop: ['ALL'],
        privileged: false,
        devices: [
          { hostPath: '/dev/dri', containerPath: '', permissions: '' },
          { hostPath: '', containerPath: '', permissions: 'rwm' },
        ],
        gpus: { mode: 'ids', count: '1', ids: '0, GPU-abc' },
      };
    });
    expect(toRunSpec(form)).toMatchObject({
      capAdd: ['NET_ADMIN'],
      capDrop: ['ALL'],
      devices: [{ hostPath: '/dev/dri', containerPath: null, permissions: 'rwm' }],
      gpus: { kind: 'ids', ids: ['0', 'GPU-abc'] },
    });
  });

  it('sends a healthcheck, network identity and logging', () => {
    const form = withAdvanced((a) => {
      a.healthcheck = {
        mode: 'custom',
        command: 'curl -f localhost',
        interval: '30s',
        timeout: '5s',
        retries: '3',
        startPeriod: '1m30s',
      };
      a.identity = {
        hostname: 'web',
        domainname: '',
        dns: '1.1.1.1\n9.9.9.9',
        dnsSearch: 'home.arpa',
        extraHosts: 'nas:192.168.1.10',
      };
      a.logging = { driver: 'json-file', options: 'max-size=10m\nmax-file=3' };
    });
    expect(toRunSpec(form)).toMatchObject({
      healthcheck: {
        command: 'curl -f localhost',
        intervalS: 30,
        timeoutS: 5,
        retries: 3,
        startPeriodS: 90,
        disabled: false,
      },
      hostname: 'web',
      domainname: null,
      dns: ['1.1.1.1', '9.9.9.9'],
      dnsSearch: ['home.arpa'],
      extraHosts: ['nas:192.168.1.10'],
      log: { driver: 'json-file', options: ['max-size=10m', 'max-file=3'] },
    });
  });

  it("turns off the image's healthcheck", () => {
    const form = withAdvanced((a) => {
      a.healthcheck.mode = 'disabled';
    });
    expect(toRunSpec(form).healthcheck).toMatchObject({ disabled: true });
  });

  it('explains values it cannot read', () => {
    const form = withAdvanced((a) => {
      a.resources.cpus = 'half';
      a.resources.memory = '512x';
      a.resources.memorySwap = 'lots';
      a.security.gpus = { mode: 'count', count: '0', ids: '' };
      a.healthcheck = { ...a.healthcheck, mode: 'custom', interval: '30 sec', retries: '1.5' };
    });
    expect(Object.keys(runFormErrors(form)).sort()).toEqual(
      ['cpus', 'gpuCount', 'interval', 'memory', 'memorySwap', 'retries'].sort(),
    );
  });

  it('ignores the durations of a healthcheck left to the image', () => {
    const form = withAdvanced((a) => {
      a.healthcheck.interval = 'nonsense';
    });
    expect(runFormErrors(form)).toEqual({});
  });
});
