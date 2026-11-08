import { connectedHost, hostView } from '@/test/fixtures';
import type { HostView } from '@/shared/api';
import { podmanAtLeast, podmanVersion, supports, versionAtLeast } from './engine';

const podman = (version: string): HostView => ({
  ...connectedHost,
  status: { ...connectedHost.status, podmanVersion: version } as HostView['status'],
});

describe('supports', () => {
  it('gives a Docker Engine the Docker pages and not Podman’s', () => {
    expect(supports(connectedHost, 'docker')).toBe(true);
    expect(supports(connectedHost, 'podman')).toBe(false);
  });

  it('gives a Podman Engine both', () => {
    expect(supports(podman('5.8.1'), 'docker')).toBe(true);
    expect(supports(podman('5.8.1'), 'podman')).toBe(true);
  });

  it('knows Podman only once connected', () => {
    const connecting = hostView('lab', { status: { state: 'connecting' } });
    expect(supports(connecting, 'docker')).toBe(true);
    expect(supports(connecting, 'podman')).toBe(false);
    expect(podmanVersion(connecting)).toBeNull();
  });

  it('gives another kind of Engine neither, and nothing without a Host', () => {
    const incus: HostView = { ...connectedHost, host: { ...connectedHost.host, engine: 'incus' } };
    expect(supports(incus, 'docker')).toBe(false);
    expect(supports(incus, 'podman')).toBe(false);
    expect(supports(null, 'docker')).toBe(false);
  });
});

describe('versions', () => {
  it('compares their numbers in order', () => {
    expect(versionAtLeast('5.8.0', '5.8')).toBe(true);
    expect(versionAtLeast('5.10.1', '5.8')).toBe(true);
    expect(versionAtLeast('6.0.0-dev', '5.8')).toBe(true);
    expect(versionAtLeast('5.7.2', '5.8')).toBe(false);
    expect(versionAtLeast('4.9', '5.7')).toBe(false);
    expect(versionAtLeast('', '5.7')).toBe(false);
  });

  it('reads the Podman version of a Host', () => {
    expect(podmanAtLeast(podman('5.8.1'), '5.8')).toBe(true);
    expect(podmanAtLeast(podman('5.6.0'), '5.7')).toBe(false);
    expect(podmanAtLeast(connectedHost, '1.0')).toBe(false);
  });
});
