import { describeStatus } from './status';

describe('describeStatus', () => {
  it('summarises a connected host with its engine version', () => {
    expect(
      describeStatus({
        state: 'connected',
        engineVersion: '29.1',
        apiVersion: '1.52',
        os: 'linux',
        arch: 'amd64',
      }),
    ).toEqual({ label: 'Connected', tone: 'success', detail: 'Engine 29.1' });
  });

  it('explains why a host is unreachable and when it retries', () => {
    const s = describeStatus({
      state: 'unreachable',
      error: {
        code: 'connection_refused',
        key: '',
        params: {},
        message: 'connection refused',
        hint: null,
        detail: null,
      },
      retryInSecs: 8,
    });
    expect(s.tone).toBe('danger');
    expect(s.detail).toBe('connection refused — retrying in 8s');
  });

  it('asks the user to check an untrusted server key', () => {
    const s = describeStatus({
      state: 'hostKeyUntrusted',
      key: 'ssh-ed25519 AAAA',
      fingerprint: 'SHA256:abc',
      bastion: null,
    });
    expect(s.tone).toBe('warning');
    expect(s.detail).toContain('SHA256:abc');
  });
});
