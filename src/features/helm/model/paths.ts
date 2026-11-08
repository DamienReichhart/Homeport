import { kindPath } from '@/core/cluster';
import type { ApiResourceInfo, ReleaseResource } from '@/shared/api';

const part = encodeURIComponent;

/** The API groups of the kinds Homeport has its own pages for. */
const BUILT_IN_GROUPS = ['', 'apps', 'batch', 'networking.k8s.io'];
/** Of those, the kinds that aren't in a Namespace. */
const CLUSTER_SCOPED = ['Namespace', 'Node'];

/** The page of a release. */
export function releasePath(namespace: string, name: string, tab?: string): string {
  return `/helm/${part(namespace)}/${part(name)}${tab ? `/${tab}` : ''}`;
}

/** The group and version of an `apiVersion` (`apps/v1`, `v1`). */
function groupVersion(apiVersion: string): [string, string] {
  const slash = apiVersion.indexOf('/');
  return slash < 0 ? ['', apiVersion] : [apiVersion.slice(0, slash), apiVersion.slice(slash + 1)];
}

/**
 * The page of an object a release created: its own page for the kinds
 * Homeport has one for, else its page under API resources, found in the
 * Cluster's discovered `types`; `null` when the type isn't served. A
 * namespaced object without a Namespace is in the release's.
 */
export function resourcePath(
  resource: ReleaseResource,
  releaseNamespace: string,
  types: ApiResourceInfo[] | undefined,
): string | null {
  const [group, version] = groupVersion(resource.apiVersion);
  const type = types?.find(
    (t) => t.api.group === group && t.api.version === version && t.api.kind === resource.kind,
  );
  const builtIn = BUILT_IN_GROUPS.includes(group);
  const namespaced = type
    ? type.api.namespaced
    : resource.namespace !== null || (builtIn && !CLUSTER_SCOPED.includes(resource.kind));
  const namespace = namespaced ? (resource.namespace ?? releaseNamespace) : null;
  const own = builtIn ? kindPath(resource.kind, namespace, resource.name) : null;
  if (own) return own;
  if (!type) return null;
  return `/api-resources/${part(group === '' ? 'core' : group)}/${part(version)}/${part(type.api.plural)}/${part(namespace ?? '-')}/${part(resource.name)}`;
}
