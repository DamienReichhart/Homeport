import { container } from '@/test/fixtures';
import {
  boardRows,
  filterRows,
  idsByHost,
  readFilter,
  sortRows,
  stacksOf,
  writeFilter,
  type BoardFilter,
} from './board';

const rows = boardRows([
  {
    hostId: 'a',
    hostName: 'laptop',
    containers: [container('web', 'shop'), container('db', 'shop', 'exited')],
  },
  {
    hostId: 'b',
    hostName: 'prod-server',
    containers: [
      { ...container('api', null), health: 'unhealthy' },
      container('cache', 'shop', 'running'),
    ],
  },
]);
const all: BoardFilter = { hosts: [], state: 'all', stack: null, query: '' };
const names = (list: typeof rows) => list.map((r) => `${r.hostName}:${r.container.name}`);

describe('board', () => {
  it('lists every host’s containers, each with its host', () => {
    expect(names(rows)).toEqual([
      'laptop:web',
      'laptop:db',
      'prod-server:api',
      'prod-server:cache',
    ]);
    expect(new Set(rows.map((r) => r.key)).size).toBe(4);
  });

  it('filters by host, state, Stack and search', () => {
    expect(names(filterRows(rows, { ...all, hosts: ['b'] }))).toEqual([
      'prod-server:api',
      'prod-server:cache',
    ]);
    expect(names(filterRows(rows, { ...all, state: 'stopped' }))).toEqual(['laptop:db']);
    expect(names(filterRows(rows, { ...all, state: 'unhealthy' }))).toEqual(['prod-server:api']);
    expect(names(filterRows(rows, { ...all, stack: 'shop', state: 'running' }))).toEqual([
      'laptop:web',
      'prod-server:cache',
    ]);
    // The search matches the host's name too.
    expect(names(filterRows(rows, { ...all, query: 'PROD' }))).toHaveLength(2);
    expect(names(filterRows(rows, { ...all, query: 'cache' }))).toEqual(['prod-server:cache']);
  });

  it('keeps the filter in the URL', () => {
    const filter: BoardFilter = { hosts: ['a', 'b'], state: 'running', stack: 'shop', query: 'w' };
    expect(writeFilter(filter).toString()).toBe('host=a&host=b&state=running&stack=shop&q=w');
    expect(readFilter(writeFilter(filter))).toEqual(filter);
    expect(readFilter(new URLSearchParams('state=bogus'))).toEqual(all);
  });

  it('sorts by any column, ties by host then name', () => {
    const byName = sortRows(rows, { key: 'name', desc: false }, () => undefined);
    expect(names(byName)).toEqual([
      'prod-server:api',
      'prod-server:cache',
      'laptop:db',
      'laptop:web',
    ]);
    const byCpu = sortRows(rows, { key: 'cpu', desc: true }, (r) =>
      r.container.name === 'db' ? undefined : ({ cpuPercent: r.container.name.length } as never),
    );
    expect(names(byCpu)[0]).toBe('prod-server:cache');
    expect(names(byCpu).at(-1)).toBe('laptop:db');
    expect(names(sortRows(rows, { key: 'host', desc: true }, () => undefined))[0]).toBe(
      'prod-server:api',
    );
  });

  it('groups a selection by host, and lists the Stacks', () => {
    const byHost = idsByHost([rows[0]!, rows[2]!, rows[3]!]);
    expect([...byHost.keys()]).toEqual(['a', 'b']);
    expect(byHost.get('b')).toHaveLength(2);
    expect(stacksOf(rows)).toEqual(['shop']);
  });
});
