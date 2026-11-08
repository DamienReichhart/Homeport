import { clusterView } from '@/test/clusters';
import type { ClusterView } from '@/shared/api';
import { chosenNamespace } from './namespace';

const cluster = clusterView('kind-dev') as unknown as ClusterView;

describe('chosenNamespace', () => {
  it('starts at the context’s namespace', () => {
    expect(chosenNamespace(cluster, {})).toBe('default');
    const shop = {
      ...cluster,
      cluster: {
        ...cluster.cluster,
        connection: { ...cluster.cluster.connection, namespace: 'shop' },
      },
    };
    expect(chosenNamespace(shop, undefined)).toBe('shop');
  });

  it('keeps the one remembered for the cluster, or every namespace', () => {
    expect(chosenNamespace(cluster, { c1: 'kube-system' })).toBe('kube-system');
    expect(chosenNamespace(cluster, { c1: '' })).toBeNull();
    expect(chosenNamespace(cluster, { other: 'x' })).toBe('default');
  });
});
