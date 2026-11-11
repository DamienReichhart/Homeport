import type { BindingView, RoleKind, RoleView, Subject, SubjectKind } from '@/shared/api';

const part = encodeURIComponent;

/** The RBAC page's tabs, in order. */
export const RBAC_TABS = ['roles', 'bindings', 'subjects', 'access'] as const;
export type RbacTab = (typeof RBAC_TABS)[number];

/** The page of a Role or ClusterRole. */
export function rolePath(role: Pick<RoleView, 'kind' | 'namespace' | 'name'>): string {
  return role.kind === 'ClusterRole' || !role.namespace
    ? `/rbac/clusterroles/${part(role.name)}`
    : `/rbac/roles/${part(role.namespace)}/${part(role.name)}`;
}

/** The page of the role a binding binds. */
export function boundRolePath(binding: BindingView): string {
  return rolePath({
    kind: binding.roleKind,
    namespace: binding.roleKind === 'Role' ? binding.namespace : null,
    name: binding.roleName,
  });
}

const SEGMENTS: Record<SubjectKind, string> = {
  User: 'users',
  Group: 'groups',
  ServiceAccount: 'serviceaccounts',
};

/** The page of a User, Group or ServiceAccount. */
export function subjectPath(subject: Subject): string {
  const base = `/rbac/${SEGMENTS[subject.kind]}`;
  return subject.kind === 'ServiceAccount' && subject.namespace
    ? `${base}/${part(subject.namespace)}/${part(subject.name)}`
    : `${base}/${part(subject.name)}`;
}

/** The subject a page's path names: its kind's segment, then its Namespace and name. */
export function subjectOf(segment: string, name: string, namespace?: string): Subject | null {
  const kind = (Object.keys(SEGMENTS) as SubjectKind[]).find((k) => SEGMENTS[k] === segment);
  if (!kind || !name) return null;
  return {
    kind,
    name,
    namespace: kind === 'ServiceAccount' ? (namespace ?? null) : null,
  };
}

/** A subject as kubectl and the bindings write it: `prod/ci` for a ServiceAccount. */
export function subjectName(subject: Subject): string {
  return subject.namespace ? `${subject.namespace}/${subject.name}` : subject.name;
}

/** Whether `binding` binds `role`. */
export function binds(binding: BindingView, role: Pick<RoleView, 'kind' | 'namespace' | 'name'>) {
  return (
    binding.roleKind === role.kind &&
    binding.roleName === role.name &&
    (role.kind === 'ClusterRole' || binding.namespace === role.namespace)
  );
}

/** The role a page's path names. */
export function roleOf(kind: RoleKind, name: string, namespace?: string) {
  return { kind, name, namespace: kind === 'Role' ? (namespace ?? null) : null };
}
