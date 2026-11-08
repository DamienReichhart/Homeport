import { kindPath, objectPath } from './paths';

describe('object paths', () => {
  it('leads to each kind’s page', () => {
    expect(objectPath('deployment', 'shop', 'web')).toBe('/workloads/deployments/shop/web');
    expect(objectPath('pod', 'kube system', 'a/b')).toBe('/pods/kube%20system/a%2Fb');
    expect(objectPath('configMap', 'shop', 'cfg')).toBe('/configmaps/shop/cfg');
    expect(objectPath('node', null, 'node-a')).toBe('/nodes/node-a');
    expect(objectPath('event', 'shop', 'x')).toBeNull();
  });

  it('reads the kinds Events name', () => {
    expect(kindPath('StatefulSet', 'shop', 'db')).toBe('/workloads/statefulsets/shop/db');
    expect(kindPath('Ingress', 'shop', 'web')).toBe('/ingresses/shop/web');
    expect(kindPath('ReplicaSet', 'shop', 'web-5d')).toBeNull();
  });
});
