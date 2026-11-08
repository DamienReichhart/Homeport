import { container, network } from '@/test/fixtures';
import { hitPath, search, type HostResources } from './search';

const hosts: HostResources[] = [
  {
    hostId: 'a',
    hostName: 'laptop',
    containers: [container('postgres-dev', null), container('web', 'shop')],
    volumes: [
      {
        name: 'pgdata',
        driver: 'local',
        mountpoint: '',
        createdAt: null,
        labels: {},
        stack: null,
        usedBy: [],
      },
    ],
  },
  {
    hostId: 'b',
    hostName: 'prod-server',
    containers: [container('shop-postgres-1', 'shop', 'exited')],
    images: [
      {
        id: 'sha256:abcdef0123456789',
        tags: ['postgres:16'],
        digests: [],
        created: 0,
        size: 0,
        containers: 1,
        labels: {},
      },
    ],
    networks: [network('backend')],
  },
];

describe('resource search', () => {
  it('finds a container on whichever host runs it, prefix matches first', () => {
    const hits = search(hosts, 'postgres').filter((h) => h.kind === 'container');
    expect(hits.map((h) => [h.label, h.hostName, h.state])).toEqual([
      ['postgres-dev', 'laptop', 'running'],
      ['shop-postgres-1', 'prod-server', 'stopped'],
    ]);
  });

  it('searches every kind, ignoring case', () => {
    const kinds = search(hosts, 'POSTGRES').map((h) => h.kind);
    expect(new Set(kinds)).toEqual(new Set(['container', 'image']));
    expect(search(hosts, 'pgd').map((h) => h.label)).toEqual(['pgdata']);
    expect(search(hosts, 'back').map((h) => [h.kind, h.hostName])).toEqual([
      ['network', 'prod-server'],
    ]);
  });

  it('matches short IDs and images', () => {
    expect(search(hosts, 'abcdef01').map((h) => h.label)).toEqual(['postgres:16']);
    // web's image is web:latest.
    expect(search(hosts, 'web:lat').map((h) => h.label)).toEqual(['web']);
  });

  it('needs two characters', () => {
    expect(search(hosts, 'p')).toEqual([]);
    expect(search(hosts, ' p ')).toEqual([]);
  });

  it('opens a container or Stack, and lists the rest filtered to it', () => {
    const [hit] = search(hosts, 'postgres-dev');
    expect(hitPath(hit!)).toBe('/containers/postgres-dev-0123456789abcdef/logs');
    const [volume] = search(hosts, 'pgdata');
    expect(hitPath(volume!)).toBe('/volumes?q=pgdata');
  });
});
