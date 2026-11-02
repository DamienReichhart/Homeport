import { emptyForm, formFromHost, toHostInput } from './form';

describe('host form', () => {
  it('builds an SSH host with a generated key', () => {
    const input = toHostInput(
      { ...emptyForm('ssh'), name: ' prod ', sshHost: 'srv', sshUser: 'deploy', sshPort: '2222' },
      false,
    );
    expect(input).toEqual({
      name: 'prod',
      connection: {
        kind: 'ssh',
        host: 'srv',
        port: 2222,
        user: 'deploy',
        socketPath: '/var/run/docker.sock',
        auth: 'generated',
        publicKey: null,
      },
      credentials: null,
    });
  });

  it('sends imported keys with an optional passphrase', () => {
    const input = toHostInput(
      { ...emptyForm('ssh'), sshAuth: 'imported', privateKey: 'KEY', passphrase: '' },
      false,
    );
    expect(input.credentials).toEqual({ kind: 'sshKey', privateKey: 'KEY', passphrase: null });
  });

  it('keeps stored TLS certificates when editing leaves them blank', () => {
    const form = { ...emptyForm('tls'), name: 'tls', address: 'h:2376' };
    expect(toHostInput(form, true).credentials).toBeNull();
    expect(toHostInput(form, false).credentials).toEqual({
      kind: 'tls',
      caPem: '',
      certPem: '',
      keyPem: '',
    });
  });

  it('round-trips an existing host into the form', () => {
    const form = formFromHost({
      id: '1',
      name: 'box',
      enabled: true,
      hostKey: null,
      createdAt: '',
      connection: {
        kind: 'ssh',
        host: 'h',
        port: 22,
        user: 'u',
        socketPath: '/run/user/1000/docker.sock',
        auth: 'agent',
        publicKey: null,
      },
    });
    expect(form).toMatchObject({ kind: 'ssh', sshHost: 'h', sshPort: '22', sshAuth: 'agent' });
    expect(toHostInput(form, true).connection).toMatchObject({
      socketPath: '/run/user/1000/docker.sock',
    });
  });
});
