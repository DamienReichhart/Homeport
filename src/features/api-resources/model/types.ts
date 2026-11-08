import type { ApiResourceInfo, ApiType } from '@/shared/api';

/** The URL segment of an API group; the core group has no name. */
export const groupSegment = (group: string) => (group === '' ? 'core' : group);
export const groupOf = (segment: string) => (segment === 'core' ? '' : segment);

const part = encodeURIComponent;

/** The page listing the objects of a type. */
export function typePath(api: ApiType): string {
  return `/api-resources/${part(groupSegment(api.group))}/${part(api.version)}/${part(api.plural)}`;
}

/** The page of one object; `-` stands for no namespace. */
export function objectPagePath(api: ApiType, namespace: string | null, name: string): string {
  return `${typePath(api)}/${part(namespace ?? '-')}/${part(name)}`;
}

/** `deployments.apps`, or `pods`, as kubectl names a type. */
export function qualifiedName(api: ApiType): string {
  return api.group ? `${api.plural}.${api.group}` : api.plural;
}

/** Whether a type answers a search: its kind, plural, group or a short name (`pvc`). */
export function matches(info: ApiResourceInfo, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  const { api } = info;
  return (
    info.shortNames.some((s) => s.toLowerCase() === q) ||
    api.kind.toLowerCase().includes(q) ||
    api.plural.toLowerCase().includes(q) ||
    qualifiedName(api).toLowerCase().includes(q) ||
    api.group.toLowerCase().includes(q)
  );
}

/** Types by API group, the core group first, in the order discovery gave. */
export function byGroup(types: ApiResourceInfo[]): { group: string; types: ApiResourceInfo[] }[] {
  const groups = new Map<string, ApiResourceInfo[]>();
  for (const info of types) {
    const list = groups.get(info.api.group) ?? [];
    list.push(info);
    groups.set(info.api.group, list);
  }
  return [...groups].map(([group, list]) => ({ group, types: list }));
}

/** The type at a page's URL, among those discovered. */
export function findType(
  types: ApiResourceInfo[],
  group: string,
  version: string,
  plural: string,
): ApiResourceInfo | undefined {
  return types.find(
    (t) => t.api.group === groupOf(group) && t.api.version === version && t.api.plural === plural,
  );
}

/** Why a type can't be listed, or `null` when it can (or the cluster can't say). */
export function notListable(info: ApiResourceInfo): 'verb' | 'forbidden' | null {
  if (!info.verbs.includes('list')) return 'verb';
  if (info.listable === false) return 'forbidden';
  return null;
}
