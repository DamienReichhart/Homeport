import type { ResourceGrant, UrlGrant } from '@/shared/api';

/** The verbs Kubernetes' own resources take, in the order kubectl lists them. */
export const STANDARD_VERBS = [
  'get',
  'list',
  'watch',
  'create',
  'update',
  'patch',
  'delete',
  'deletecollection',
] as const;

/** RBAC's wildcard: every verb, API group, resource or URL. */
export const ALL = '*';

/** What a cell of the matrix says: allowed by name, by `*`, or not. */
export type Cell = 'yes' | 'all' | null;

export interface MatrixRow {
  /** Unique within the matrix. */
  id: string;
  /** As kubectl names it: `pods`, `deployments.apps`, `pods/log`, `*.*`. */
  resource: string;
  /** Only on these objects, when not empty. */
  names: string[];
  /** Every verb, through `*`. */
  every: boolean;
  /** One per column. */
  cells: Cell[];
  /** The bindings granting it, by index. */
  bindings: number[];
}

export interface Matrix {
  /** The standard verbs, then any other verb a row grants (`bind`, `escalate`…). */
  columns: string[];
  rows: MatrixRow[];
}

/**
 * A resource as `kubectl auth can-i --list` names it: its plural, its group
 * unless it is the core one, then its subresource (`deployments.apps/scale`).
 */
export function resourceName(grant: Pick<ResourceGrant, 'apiGroup' | 'resource'>): string {
  if (!grant.apiGroup) return grant.resource;
  const [plural, sub] = grant.resource.split('/', 2);
  return sub === undefined ? `${plural}.${grant.apiGroup}` : `${plural}.${grant.apiGroup}/${sub}`;
}

/** The core group first, `*` last; within a group, `*` resources last. */
const rank = (value: string) => (value === '' ? 0 : value === ALL ? 2 : 1);
function compare(a: ResourceGrant, b: ResourceGrant): number {
  return (
    rank(a.apiGroup) - rank(b.apiGroup) ||
    a.apiGroup.localeCompare(b.apiGroup) ||
    rank(a.resource.split('/')[0] ?? '') - rank(b.resource.split('/')[0] ?? '') ||
    a.resource.localeCompare(b.resource) ||
    a.resourceNames.join(',').localeCompare(b.resourceNames.join(','))
  );
}

/**
 * Grants as a resource × verb matrix: a row per resource (and set of
 * names), a column per verb any of them grants, the standard ones first
 * and always shown.
 */
export function matrixOf(grants: ResourceGrant[]): Matrix {
  const standard: readonly string[] = STANDARD_VERBS;
  const extra = new Set<string>();
  for (const g of grants)
    for (const v of g.verbs) if (v !== ALL && !standard.includes(v)) extra.add(v);
  const columns = [...standard, ...[...extra].sort()];
  const rows = [...grants].sort(compare).map((g) => {
    const every = g.verbs.includes(ALL);
    return {
      id: `${g.apiGroup}|${g.resource}|${g.resourceNames.join(',')}`,
      resource: resourceName(g),
      names: g.resourceNames,
      every,
      cells: columns.map((verb): Cell => (every ? 'all' : g.verbs.includes(verb) ? 'yes' : null)),
      bindings: g.bindings,
    };
  });
  return { columns, rows };
}

/** Non-resource URLs, sorted, with their verbs in kubectl's order. */
export function urlRows(urls: UrlGrant[]): UrlGrant[] {
  return [...urls].sort((a, b) => a.url.localeCompare(b.url));
}
