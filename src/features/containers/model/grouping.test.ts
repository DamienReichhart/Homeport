import type { Container } from '@/shared/api';
import { groupByStack, matches, portLabels } from './grouping';

const c = (
  name: string,
  stack: string | null,
  state = 'running',
  extra: Partial<Container> = {},
): Container => ({
  id: `${name}-id`,
  name,
  image: `${name}:latest`,
  imageId: 'sha256:x',
  command: '',
  created: 0,
  state,
  status: '',
  health: null,
  ports: [],
  labels: {},
  stack,
  service: stack ? (name.split('-')[1] ?? name) : null,
  networks: [],
  ...extra,
});

describe('container grouping', () => {
  const all = [
    c('shop-web-1', 'shop'),
    c('shop-db-1', 'shop', 'exited'),
    c('blog-app-1', 'blog'),
    c('redis', null),
  ];

  it('nests containers under their stack, sorted', () => {
    const { groups, standalone } = groupByStack(all);
    expect(groups.map((g) => [g.stack, g.containers.map((x) => x.name), g.running])).toEqual([
      ['blog', ['blog-app-1'], 1],
      ['shop', ['shop-db-1', 'shop-web-1'], 1],
    ]);
    expect(standalone.map((x) => x.name)).toEqual(['redis']);
  });

  it('nests a swarm stack’s task containers under it, apart from a Compose project of that name', () => {
    const { groups, standalone } = groupByStack([
      c('shop_web.1.abc', null, 'running', { labels: { 'com.docker.stack.namespace': 'shop' } }),
      c('shop-web-1', 'shop'),
      c('lonely', null),
    ]);
    expect(groups.map((g) => [g.stack, g.swarm, g.containers.length])).toEqual([
      ['shop', false, 1],
      ['shop', true, 1],
    ]);
    expect(standalone.map((x) => x.name)).toEqual(['lonely']);
  });

  it('filters by search and running state', () => {
    expect(groupByStack(all, 'shop').groups.map((g) => g.stack)).toEqual(['shop']);
    const running = groupByStack(all, '', true);
    expect(running.groups.find((g) => g.stack === 'shop')?.containers.map((x) => x.name)).toEqual([
      'shop-web-1',
    ]);
  });

  it('matches by image, service and id prefix', () => {
    const web = c('shop-web-1', 'shop');
    expect(matches(web, 'web-1:lat')).toBe(true);
    expect(matches(web, 'shop-web-1-i')).toBe(true);
    expect(matches(web, 'nope')).toBe(false);
  });

  it('labels published ports before internal ones', () => {
    const x = c('x', null, 'running', {
      ports: [
        { ip: null, privatePort: 6379, publicPort: null, protocol: 'tcp' },
        { ip: '0.0.0.0', privatePort: 80, publicPort: 8080, protocol: 'tcp' },
      ],
    });
    expect(portLabels(x).map((p) => p.label)).toEqual(['8080:80/tcp', '6379/tcp']);
  });
});
