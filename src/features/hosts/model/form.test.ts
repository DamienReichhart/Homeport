import { emptyBastion, emptyForm, formFromHost, moveBastion, toHostInput } from './form';

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
        bastions: [],
      },
      credentials: null,
      bastionKeys: [],
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
      engine: 'docker',
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
        bastions: [],
      },
    });
    expect(form).toMatchObject({ kind: 'ssh', sshHost: 'h', sshPort: '22', sshAuth: 'agent' });
    expect(toHostInput(form, true).connection).toMatchObject({
      socketPath: '/run/user/1000/docker.sock',
    });
  });

  it('sends bastions in order, and only the keys typed for them', () => {
    const input = toHostInput(
      {
        ...emptyForm('ssh'),
        sshAuth: 'agent',
        bastions: [
          { ...emptyBastion(), host: ' jump ', user: 'me', port: '2222' },
          { ...emptyBastion(), host: 'inner', user: 'me', auth: 'imported', privateKey: 'KEY' },
          { ...emptyBastion(), host: 'kept', user: 'me', auth: 'imported', storedPublicKey: 'pk' },
        ],
      },
      true,
    );
    expect(input.connection.kind === 'ssh' && input.connection.bastions).toEqual([
      { host: 'jump', port: 2222, user: 'me', auth: 'sameAsHost', publicKey: null, hostKey: null },
      { host: 'inner', port: 22, user: 'me', auth: 'imported', publicKey: null, hostKey: null },
      { host: 'kept', port: 22, user: 'me', auth: 'imported', publicKey: null, hostKey: null },
    ]);
    expect(input.bastionKeys).toEqual([{ index: 1, privateKey: 'KEY', passphrase: null }]);
  });

  it('reads an existing host’s bastions, remembering which have a stored key', () => {
    const form = formFromHost({
      id: '1',
      name: 'box',
      engine: 'docker',
      enabled: true,
      hostKey: null,
      createdAt: '',
      connection: {
        kind: 'ssh',
        host: 'h',
        port: 22,
        user: 'u',
        socketPath: '/var/run/docker.sock',
        auth: 'generated',
        publicKey: null,
        bastions: [
          {
            host: 'jump',
            port: 22,
            user: 'me',
            auth: 'imported',
            publicKey: 'ssh-ed25519 JUMP',
            hostKey: 'ssh-ed25519 SERVER',
          },
        ],
      },
    });
    expect(form.bastions).toEqual([
      {
        ...emptyBastion(),
        host: 'jump',
        user: 'me',
        auth: 'imported',
        storedPublicKey: 'ssh-ed25519 JUMP',
      },
    ]);
  });

  it('moves a bastion up or down, never out of the list', () => {
    const [a, b, c] = ['a', 'b', 'c'].map((host) => ({ ...emptyBastion(), host }));
    const list = [a!, b!, c!];
    expect(moveBastion(list, 2, -1).map((x) => x.host)).toEqual(['a', 'c', 'b']);
    expect(moveBastion(list, 0, 1).map((x) => x.host)).toEqual(['b', 'a', 'c']);
    expect(moveBastion(list, 0, -1)).toBe(list);
    expect(moveBastion(list, 2, 1)).toBe(list);
  });
});
