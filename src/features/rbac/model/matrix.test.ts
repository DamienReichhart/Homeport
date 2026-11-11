import type { ResourceGrant } from '@/shared/api';
import { matrixOf, resourceName, STANDARD_VERBS, urlRows } from './matrix';
import { binds, rolePath, subjectOf, subjectPath } from './paths';

const grant = (
  apiGroup: string,
  resource: string,
  verbs: string[],
  resourceNames: string[] = [],
): ResourceGrant => ({ apiGroup, resource, verbs, resourceNames, bindings: [0] });

describe('the permission matrix', () => {
  it('has a column per standard verb, then the other verbs granted', () => {
    const m = matrixOf([
      grant('rbac.authorization.k8s.io', 'roles', ['bind', 'escalate', 'get']),
      grant('', 'pods', ['get']),
    ]);
    expect(m.columns).toEqual([...STANDARD_VERBS, 'bind', 'escalate']);
    const roles = m.rows.find((r) => r.resource === 'roles.rbac.authorization.k8s.io');
    expect(roles?.cells[m.columns.indexOf('bind')]).toBe('yes');
    expect(roles?.cells[m.columns.indexOf('list')]).toBeNull();
  });

  it('marks every cell of a row granting `*`, without adding a `*` column', () => {
    const m = matrixOf([grant('*', '*', ['*'])]);
    expect(m.columns).toEqual([...STANDARD_VERBS]);
    expect(m.rows[0]?.resource).toBe('*.*');
    expect(m.rows[0]?.every).toBe(true);
    expect(m.rows[0]?.cells.every((c) => c === 'all')).toBe(true);
  });

  it('puts the core group first and wildcards last', () => {
    const m = matrixOf([
      grant('*', '*', ['get']),
      grant('apps', '*', ['get']),
      grant('apps', 'deployments', ['get']),
      grant('', 'pods/log', ['get']),
      grant('', 'configmaps', ['get'], ['app']),
    ]);
    expect(m.rows.map((r) => r.resource)).toEqual([
      'configmaps',
      'pods/log',
      'deployments.apps',
      '*.apps',
      '*.*',
    ]);
    expect(m.rows[0]?.names).toEqual(['app']);
  });

  it('keeps a row per set of names, each with its own id', () => {
    const m = matrixOf([
      grant('', 'configmaps', ['get'], ['a']),
      grant('', 'configmaps', ['update'], ['b']),
    ]);
    expect(new Set(m.rows.map((r) => r.id)).size).toBe(2);
  });

  it('names resources as kubectl does', () => {
    expect(resourceName({ apiGroup: '', resource: 'secrets' })).toBe('secrets');
    expect(resourceName({ apiGroup: 'apps', resource: 'deployments/scale' })).toBe(
      'deployments.apps/scale',
    );
  });

  it('sorts non-resource URLs', () => {
    const rows = urlRows([
      { url: '/version', verbs: ['get'], bindings: [] },
      { url: '/healthz', verbs: ['get'], bindings: [] },
    ]);
    expect(rows.map((r) => r.url)).toEqual(['/healthz', '/version']);
  });
});

describe('RBAC paths', () => {
  it('give each subject a page and read it back', () => {
    const sa = { kind: 'ServiceAccount' as const, name: 'ci', namespace: 'prod' };
    expect(subjectPath(sa)).toBe('/rbac/serviceaccounts/prod/ci');
    expect(subjectOf('serviceaccounts', 'ci', 'prod')).toEqual(sa);
    const group = { kind: 'Group' as const, name: 'system:masters', namespace: null };
    expect(subjectPath(group)).toBe('/rbac/groups/system%3Amasters');
    expect(subjectOf('groups', 'system:masters')).toEqual(group);
    expect(subjectOf('robots', 'x')).toBeNull();
  });

  it('tell which bindings bind a role', () => {
    const binding = {
      kind: 'RoleBinding' as const,
      name: 'b',
      namespace: 'prod',
      roleKind: 'ClusterRole' as const,
      roleName: 'edit',
      roleFound: true,
      subjects: [],
      created: null,
    };
    expect(binds(binding, { kind: 'ClusterRole', namespace: null, name: 'edit' })).toBe(true);
    expect(binds(binding, { kind: 'Role', namespace: 'prod', name: 'edit' })).toBe(false);
    expect(rolePath({ kind: 'Role', namespace: 'prod', name: 'reader' })).toBe(
      '/rbac/roles/prod/reader',
    );
  });
});
