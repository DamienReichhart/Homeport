import type { ApiResourceInfo, ReleaseResource } from '@/shared/api';
import { releasePath, resourcePath } from './paths';

const type = (group: string, version: string, kind: string, plural: string, namespaced: boolean) =>
  ({
    api: { group, version, kind, plural, namespaced },
    shortNames: [],
    verbs: ['list', 'get'],
    categories: [],
    listable: true,
  }) as ApiResourceInfo;

const types = [
  type('', 'v1', 'Service', 'services', true),
  type('', 'v1', 'ServiceAccount', 'serviceaccounts', true),
  type('apps', 'v1', 'Deployment', 'deployments', true),
  type('rbac.authorization.k8s.io', 'v1', 'ClusterRole', 'clusterroles', false),
];

const resource = (
  apiVersion: string,
  kind: string,
  name: string,
  namespace: string | null = null,
) => ({ apiVersion, kind, name, namespace }) as ReleaseResource;

describe('release paths', () => {
  it('names a release’s page and tab', () => {
    expect(releasePath('shop', 'web')).toBe('/helm/shop/web');
    expect(releasePath('shop', 'web', 'history')).toBe('/helm/shop/web/history');
  });

  it('opens kinds Homeport has a page for, in the release’s Namespace', () => {
    expect(resourcePath(resource('v1', 'Service', 'web'), 'shop', types)).toBe(
      '/services/shop/web',
    );
    expect(resourcePath(resource('apps/v1', 'Deployment', 'web', 'other'), 'shop', types)).toBe(
      '/workloads/deployments/other/web',
    );
  });

  it('opens other kinds under API resources', () => {
    expect(resourcePath(resource('v1', 'ServiceAccount', 'web'), 'shop', types)).toBe(
      '/api-resources/core/v1/serviceaccounts/shop/web',
    );
    expect(
      resourcePath(
        resource('rbac.authorization.k8s.io/v1', 'ClusterRole', 'reader'),
        'shop',
        types,
      ),
    ).toBe('/api-resources/rbac.authorization.k8s.io/v1/clusterroles/-/reader');
  });

  it('links nothing for a type the Cluster doesn’t serve', () => {
    expect(resourcePath(resource('example.com/v1', 'Widget', 'w'), 'shop', types)).toBeNull();
  });
});
