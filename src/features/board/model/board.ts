import type { Container, ContainerUsage } from '@/shared/api';

/** One container on the board, with the Host it runs on. */
export interface BoardRow {
  /** Unique across Hosts: `<hostId>/<containerId>`. */
  key: string;
  hostId: string;
  hostName: string;
  container: Container;
}

export type StateFilter = 'all' | 'running' | 'stopped' | 'unhealthy';

export interface BoardFilter {
  /** The Hosts shown; empty for every Host. */
  hosts: string[];
  state: StateFilter;
  /** One Stack's containers; `null` for any. */
  stack: string | null;
  query: string;
}

export type SortKey = 'host' | 'name' | 'image' | 'status' | 'stack' | 'cpu' | 'memory' | 'created';

export interface BoardSort {
  key: SortKey;
  desc: boolean;
}

export const rowKey = (hostId: string, containerId: string) => `${hostId}/${containerId}`;

/** Every Host's containers as rows. */
export function boardRows(
  hosts: { hostId: string; hostName: string; containers: Container[] }[],
): BoardRow[] {
  return hosts.flatMap(({ hostId, hostName, containers }) =>
    containers.map((container) => ({
      key: rowKey(hostId, container.id),
      hostId,
      hostName,
      container,
    })),
  );
}

const STATES: StateFilter[] = ['all', 'running', 'stopped', 'unhealthy'];

/** The filter kept in the board's URL: `?host=a&host=b&state=running&stack=web&q=api`. */
export function readFilter(params: URLSearchParams): BoardFilter {
  const state = params.get('state') as StateFilter | null;
  return {
    hosts: params.getAll('host'),
    state: state && STATES.includes(state) ? state : 'all',
    stack: params.get('stack') || null,
    query: params.get('q') ?? '',
  };
}

export function writeFilter(filter: BoardFilter): URLSearchParams {
  const params = new URLSearchParams();
  for (const host of filter.hosts) params.append('host', host);
  if (filter.state !== 'all') params.set('state', filter.state);
  if (filter.stack) params.set('stack', filter.stack);
  if (filter.query) params.set('q', filter.query);
  return params;
}

function matchesState(container: Container, state: StateFilter): boolean {
  switch (state) {
    case 'all':
      return true;
    case 'running':
      return container.state === 'running';
    case 'stopped':
      return container.state !== 'running';
    case 'unhealthy':
      return container.health === 'unhealthy';
  }
}

/** The rows `filter` keeps; the search matches the name, short ID, image, Stack and Host. */
export function filterRows(rows: BoardRow[], filter: BoardFilter): BoardRow[] {
  const q = filter.query.trim().toLowerCase();
  return rows.filter(({ hostId, hostName, container: c }) => {
    if (filter.hosts.length && !filter.hosts.includes(hostId)) return false;
    if (!matchesState(c, filter.state)) return false;
    if (filter.stack && c.stack !== filter.stack) return false;
    if (!q) return true;
    return [c.name, c.id.slice(0, 12), c.image, c.stack ?? '', hostName].some((text) =>
      text.toLowerCase().includes(q),
    );
  });
}

/** The Stacks the rows belong to, sorted, for the Stack filter. */
export function stacksOf(rows: BoardRow[]): string[] {
  return [...new Set(rows.flatMap((r) => (r.container.stack ? [r.container.stack] : [])))].sort();
}

/** `rows` sorted by `sort`; ties keep the Host, then the name, order. */
export function sortRows(
  rows: BoardRow[],
  sort: BoardSort,
  usage: (row: BoardRow) => ContainerUsage | undefined,
): BoardRow[] {
  const value = (row: BoardRow): string | number => {
    const c = row.container;
    switch (sort.key) {
      case 'host':
        return row.hostName.toLowerCase();
      case 'name':
        return c.name.toLowerCase();
      case 'image':
        return c.image.toLowerCase();
      case 'status':
        return c.state;
      case 'stack':
        return c.stack ?? '';
      case 'cpu':
        return usage(row)?.cpuPercent ?? -1;
      case 'memory':
        return usage(row)?.memoryUsage ?? -1;
      case 'created':
        return c.created;
    }
  };
  const tie = (a: BoardRow, b: BoardRow) =>
    a.hostName.localeCompare(b.hostName) || a.container.name.localeCompare(b.container.name);
  return [...rows].sort((a, b) => {
    const [x, y] = [value(a), value(b)];
    const order = x < y ? -1 : x > y ? 1 : 0;
    return (sort.desc ? -order : order) || tie(a, b);
  });
}

/** The selected rows' container ids, by Host, for one request per Host. */
export function idsByHost(rows: BoardRow[]): Map<string, string[]> {
  const byHost = new Map<string, string[]>();
  for (const row of rows) {
    byHost.set(row.hostId, [...(byHost.get(row.hostId) ?? []), row.container.id]);
  }
  return byHost;
}
