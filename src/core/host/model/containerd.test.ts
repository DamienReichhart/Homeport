import type { ContainerdNamespace } from '@/shared/api';
import { hostKeys } from '../api/keys';
import { chosenContainerdNamespace, preferredNamespace } from './containerd';

const ns = (name: string, containers = 0, images = 0): ContainerdNamespace => ({
  name,
  labels: {},
  containers,
  images,
});

describe('the namespace a containerd Host shows', () => {
  it('is default, unless only another namespace holds something', () => {
    expect(preferredNamespace([])).toBe('default');
    expect(preferredNamespace([ns('default')])).toBe('default');
    expect(preferredNamespace([ns('default'), ns('k8s.io', 12, 30)])).toBe('k8s.io');
    expect(preferredNamespace([ns('default', 1), ns('k8s.io', 12, 30)])).toBe('default');
    expect(preferredNamespace([ns('buildkit', 0, 2), ns('k8s.io', 12)])).toBe('default');
  });

  it('is the one remembered for the Host once picked', () => {
    const list = [ns('default'), ns('k8s.io', 3)];
    expect(chosenContainerdNamespace('h1', { h1: 'moby' }, list)).toBe('moby');
    expect(chosenContainerdNamespace('h2', { h1: 'moby' }, list)).toBe('k8s.io');
    expect(chosenContainerdNamespace('h2', undefined, undefined)).toBeNull();
  });

  it('scopes the pages’ keys by namespace, under one key per kind', () => {
    const one = hostKeys.containerdContainers('h1', 'k8s.io');
    const all = hostKeys.containerdContainers('h1');
    expect(one.slice(0, all.length)).toEqual(all);
    expect(one).not.toEqual(hostKeys.containerdContainers('h1', 'default'));
    expect(all.slice(0, 2)).toEqual(hostKeys.scope('h1'));
  });
});
