import {
  describeLogin,
  emptyLogin,
  loginFormOf,
  missingField,
  registryLabel,
  registryOf,
  toDraft,
} from './login';

const stored = {
  id: 'r1',
  address: 'ghcr.io',
  username: 'alice',
  createdAt: '',
  updatedAt: '',
};

describe('registry login form', () => {
  it('asks for every field of a new login', () => {
    expect(missingField(emptyLogin, false)).toMatch(/address/);
    expect(missingField({ ...emptyLogin, address: 'ghcr.io' }, false)).toMatch(/user name/);
    expect(missingField({ address: 'ghcr.io', username: 'a', secret: '' }, false)).toMatch(
      /password/,
    );
    expect(missingField({ address: 'ghcr.io', username: 'a', secret: 'x' }, false)).toBeNull();
  });

  it('keeps the stored secret when editing without a new one', () => {
    const form = loginFormOf(stored);
    expect(form.secret).toBe('');
    expect(missingField(form, true)).toBeNull();
    expect(toDraft(form)).toEqual({ address: 'ghcr.io', username: 'alice', secret: null });
    expect(toDraft({ ...form, secret: ' tok ' }).secret).toBe(' tok ');
  });

  it('trims the address and user name', () => {
    expect(toDraft({ address: ' ghcr.io ', username: ' a ', secret: 'x' })).toEqual({
      address: 'ghcr.io',
      username: 'a',
      secret: 'x',
    });
  });

  it('says which login an image uses', () => {
    expect(describeLogin(stored)).toBe('as alice on ghcr.io');
    expect(describeLogin(null)).toBe('anonymously');
    expect(registryLabel('docker.io')).toBe('Docker Hub (docker.io)');
    expect(registryLabel('ghcr.io')).toBe('ghcr.io');
  });
});

describe('registry of a reference', () => {
  it('follows Docker’s rule', () => {
    expect(registryOf('nginx')).toBe('docker.io');
    expect(registryOf('bitnami/redis:7')).toBe('docker.io');
    expect(registryOf('ghcr.io/org/app:1')).toBe('ghcr.io');
    expect(registryOf('localhost:5000/app')).toBe('localhost:5000');
    expect(registryOf('index.docker.io/library/nginx')).toBe('docker.io');
  });
});
