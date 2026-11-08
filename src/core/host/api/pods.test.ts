import type { PodmanPod } from '@/shared/api';
import { podsByContainer } from './pods';

const pod = (id: string, containers: string[]): PodmanPod => ({
  id,
  name: `${id}-name`,
  status: 'running',
  created: '',
  infraId: null,
  labels: {},
  networks: [],
  containers: containers.map((c) => ({ id: c, name: c, state: 'running', infra: false })),
});

describe('podsByContainer', () => {
  it('finds the pod of each container', () => {
    const index = podsByContainer([pod('p1', ['a', 'b']), pod('p2', ['c'])]);
    expect(index.get('b')).toEqual({ id: 'p1', name: 'p1-name' });
    expect(index.get('c')?.name).toBe('p2-name');
    expect(index.get('z')).toBeUndefined();
  });
});
