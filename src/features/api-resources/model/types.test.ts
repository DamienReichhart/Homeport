import { describe, expect, it } from 'vitest';
import type { ApiResourceInfo } from '@/shared/api';
import { byGroup, findType, matches, notListable, objectPagePath, typePath } from './types';

const info = (
  group: string,
  kind: string,
  plural: string,
  shortNames: string[] = [],
  extra: Partial<ApiResourceInfo> = {},
): ApiResourceInfo => ({
  api: { group, version: 'v1', kind, plural, namespaced: true },
  shortNames,
  verbs: ['get', 'list', 'watch', 'delete', 'patch'],
  categories: [],
  listable: null,
  ...extra,
});

const pvc = info('', 'PersistentVolumeClaim', 'persistentvolumeclaims', ['pvc']);
const cert = info('cert-manager.io', 'Certificate', 'certificates', ['cert', 'certs']);

describe('API types', () => {
  it('are found by a short name, their kind, plural or group', () => {
    expect(matches(pvc, 'pvc')).toBe(true);
    expect(matches(pvc, 'PersistentVolume')).toBe(true);
    expect(matches(cert, 'cert-manager')).toBe(true);
    expect(matches(cert, 'certificates.cert')).toBe(true);
    expect(matches(pvc, 'cert')).toBe(false);
    expect(matches(cert, '')).toBe(true);
  });

  it('are grouped by API group, the core group first', () => {
    const groups = byGroup([pvc, cert, info('', 'Pod', 'pods')]);
    expect(groups.map((g) => g.group)).toEqual(['', 'cert-manager.io']);
    expect(groups[0]?.types).toHaveLength(2);
  });

  it('have pages that name them by group, version and plural', () => {
    expect(typePath(pvc.api)).toBe('/api-resources/core/v1/persistentvolumeclaims');
    expect(objectPagePath(cert.api, 'shop', 'web-tls')).toBe(
      '/api-resources/cert-manager.io/v1/certificates/shop/web-tls',
    );
    expect(objectPagePath({ ...cert.api, namespaced: false }, null, 'x')).toBe(
      '/api-resources/cert-manager.io/v1/certificates/-/x',
    );
    expect(findType([pvc, cert], 'core', 'v1', 'persistentvolumeclaims')).toBe(pvc);
  });

  it('say why they can’t be listed', () => {
    expect(notListable(pvc)).toBeNull();
    expect(notListable({ ...pvc, verbs: ['create'] })).toBe('verb');
    expect(notListable({ ...pvc, listable: false })).toBe('forbidden');
  });
});
