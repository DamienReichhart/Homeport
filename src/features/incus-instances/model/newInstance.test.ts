import { describe, expect, it } from 'vitest';
import { emptyNewInstance, problem, toSpec, validName, validSize } from './newInstance';

const server = 'https://images.linuxcontainers.org';

describe('new instance form', () => {
  it('accepts hostname labels as names', () => {
    expect(validName('web-1')).toBe(true);
    expect(validName('1web')).toBe(false);
    expect(validName('web-')).toBe(false);
    expect(validName('my web')).toBe(false);
  });

  it('accepts sizes Incus reads', () => {
    expect(validSize('2GiB')).toBe(true);
    expect(validSize('512 MiB')).toBe(true);
    expect(validSize('')).toBe(true);
    expect(validSize('lots')).toBe(false);
  });

  it('says what’s missing first', () => {
    const form = emptyNewInstance(server);
    expect(problem(form)).toBe('name');
    expect(problem({ ...form, name: 'web' })).toBe('image');
    expect(problem({ ...form, name: 'web', alias: 'debian/12', cpu: '0' })).toBe('cpu');
    expect(problem({ ...form, name: 'web', alias: 'debian/12', memory: 'x' })).toBe('memory');
    expect(problem({ ...form, name: 'web', alias: 'debian/12' })).toBeNull();
  });

  it('turns the form into a spec, from a remote alias or a local image', () => {
    const form = {
      ...emptyNewInstance(server),
      name: ' web ',
      alias: 'debian/12',
      cpu: '2',
      memory: '2 GiB',
    };
    expect(toSpec(form)).toEqual({
      name: 'web',
      type: 'container',
      source: { kind: 'remote', server, alias: 'debian/12' },
      profiles: ['default'],
      cpu: 2,
      memory: '2GiB',
      disk: null,
      start: true,
    });
    expect(toSpec({ ...form, origin: 'local', fingerprint: 'abc' }).source).toEqual({
      kind: 'local',
      fingerprint: 'abc',
    });
  });
});
