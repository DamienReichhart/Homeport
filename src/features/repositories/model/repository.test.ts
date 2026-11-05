import { describe, expect, it } from 'vitest';
import {
  authsFor,
  emptyRepository,
  missingField,
  repositoryParts,
  toDraft,
  transportOf,
  withUrl,
} from './repository';

describe('transportOf', () => {
  it('accepts the URLs the backend accepts', () => {
    expect(transportOf('git@github.com:owner/repo.git')).toBe('ssh');
    expect(transportOf('ssh://git@gitlab.example.com:2222/group/repo.git')).toBe('ssh');
    expect(transportOf('https://codeberg.org/owner/repo')).toBe('https');
  });

  it('refuses the rest', () => {
    for (const url of [
      '',
      'http://github.com/o/r.git',
      'file:///srv/repo.git',
      '/srv/repo.git',
      'C:\\repos\\app',
      'https://github.com',
      'git@github.com:owner/my repo.git',
      '-oProxyCommand=x:repo',
      'ssh://-oProxyCommand=x/repo',
    ]) {
      expect(transportOf(url), url).toBeNull();
    }
  });
});

describe('the add form', () => {
  it('offers a deploy key over SSH and a token or nothing over HTTPS', () => {
    expect(authsFor('git@github.com:o/r.git')).toEqual(['sshKey']);
    expect(authsFor('https://github.com/o/r.git')).toEqual(['httpsToken', 'none']);
    expect(withUrl(emptyRepository, 'https://gitlab.com/o/r.git').auth).toBe('httpsToken');
    expect(withUrl({ ...emptyRepository, auth: 'none' }, 'https://x.org/o/r').auth).toBe('none');
  });

  it('says what is missing', () => {
    expect(missingField(emptyRepository)).toBe('Enter the repository’s URL.');
    expect(missingField({ ...emptyRepository, url: 'nope' })).toMatch(/SSH or HTTPS/);
    const https = withUrl(emptyRepository, 'https://gitlab.com/o/r.git');
    expect(missingField(https)).toBe('Enter the user name the token belongs to.');
    expect(missingField({ ...https, username: 'bot' })).toBe('Enter the access token.');
    expect(missingField({ ...https, username: 'bot', token: 't' })).toBeNull();
    expect(missingField(withUrl(emptyRepository, 'git@github.com:o/r.git'))).toBeNull();
  });

  it('sends a token only with token access', () => {
    const form = {
      url: ' git@github.com:o/r.git ',
      auth: 'sshKey' as const,
      username: 'a',
      token: 'b',
    };
    expect(toDraft(form)).toEqual({
      url: 'git@github.com:o/r.git',
      auth: 'sshKey',
      username: null,
      token: null,
    });
  });
});

describe('repositoryParts', () => {
  it('names the server and the repository', () => {
    expect(repositoryParts('git@github.com:owner/repo.git')).toEqual({
      host: 'github.com',
      path: 'owner/repo',
    });
    expect(repositoryParts('ssh://git@host:2222/group/sub/repo.git')).toEqual({
      host: 'host',
      path: 'group/sub/repo',
    });
    expect(repositoryParts('https://codeberg.org/owner/repo/')).toEqual({
      host: 'codeberg.org',
      path: 'owner/repo',
    });
  });
});
