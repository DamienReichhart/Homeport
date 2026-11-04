import { indexFreshness } from '@/core/host';
import type { Container, Stack } from '@/shared/api';
import { newerServicesLabel, serviceFreshness, stackFreshness } from './freshness';

const c = (id: string, image: string): Container =>
  ({ id, image, imageId: `sha256:${id}` }) as Container;

const stack = {
  name: 'shop',
  services: [
    { name: 'web', containerIds: ['w1', 'w2'] },
    { name: 'db', containerIds: ['d1'] },
    { name: 'build', containerIds: ['b1'] },
  ],
} as Stack;
const containers = new Map(
  [c('w1', 'nginx:1.27'), c('w2', 'nginx:1.27'), c('d1', 'postgres:17'), c('b1', 'shop:dev')].map(
    (x) => [x.id, x],
  ),
);

describe('Stack freshness', () => {
  it('names the services whose image moved on', () => {
    const index = indexFreshness([
      { reference: 'nginx:1.27', imageId: 'sha256:w1', freshness: 'upToDate' },
      { reference: 'nginx:1.27', imageId: 'sha256:w2', freshness: 'newer' },
      { reference: 'postgres:17', imageId: 'sha256:d1', freshness: 'upToDate' },
      { reference: 'shop:dev', imageId: 'sha256:b1', freshness: 'unknown' },
    ]);
    const services = serviceFreshness(stack, containers, index);
    expect(services.map((s) => [s.service, s.freshness])).toEqual([
      ['web', 'newer'],
      ['db', 'upToDate'],
      ['build', 'unknown'],
    ]);
    const result = stackFreshness(services);
    expect(result.freshness).toBe('newer');
    expect(newerServicesLabel(result.newer)).toBe('Newer images for web');
  });

  it('shows nothing for a Stack of local builds', () => {
    const index = indexFreshness([]);
    expect(stackFreshness(serviceFreshness(stack, containers, index)).freshness).toBe('unknown');
  });
});
