import { advertiseFromConnection, confirmsLeave, leaveRisk } from './membership';

describe('Swarm membership', () => {
  it('advertises the address Homeport reaches a host at, when it is an IP', () => {
    expect(advertiseFromConnection({ kind: 'tcp', address: '192.168.1.10:2375' })).toBe(
      '192.168.1.10',
    );
    expect(advertiseFromConnection({ kind: 'tls', address: 'nas.home:2376' })).toBe('');
    expect(advertiseFromConnection({ kind: 'local', socketPath: '/var/run/docker.sock' })).toBe('');
    expect(
      advertiseFromConnection({
        kind: 'ssh',
        host: '10.0.0.5',
        port: 22,
        user: 'me',
        socketPath: '/var/run/docker.sock',
        auth: 'agent',
        publicKey: null,
        bastions: [],
      }),
    ).toBe('10.0.0.5');
  });

  it('guards the last manager', () => {
    expect(leaveRisk('worker', 1)).toBe('worker');
    expect(leaveRisk('manager', 3)).toBe('manager');
    expect(leaveRisk('manager', 1)).toBe('lastManager');
    expect(confirmsLeave(' prod ', 'prod')).toBe(true);
    expect(confirmsLeave('pro', 'prod')).toBe(false);
  });
});
