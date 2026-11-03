import { emptyRunForm, toRunSpec } from './runForm';

describe('run form', () => {
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
    });
  });
});
