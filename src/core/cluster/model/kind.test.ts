import type { Cluster, ClusterView } from '@/shared/api';
import { clusterView } from '@/test/clusters';
import { clusterAddress, isClusterKind, kubernetesConnection } from './kind';

const kubernetes = clusterView('kind-dev') as unknown as ClusterView;
const nomad = {
  ...kubernetes,
  cluster: {
    ...kubernetes.cluster,
    kind: 'nomad',
    connection: { kind: 'nomad', address: 'http://127.0.0.1:4646' },
  } as Cluster,
};

describe('the kind of a Cluster', () => {
  it('reads a Kubernetes Cluster’s connection', () => {
    expect(clusterAddress(kubernetes.cluster)).toBe('https://127.0.0.1:6443');
    expect(kubernetesConnection(kubernetes.cluster)?.namespace).toBe('default');
    expect(isClusterKind(kubernetes, 'kubernetes')).toBe(true);
  });

  it('reads a Nomad Cluster’s address, and has no Kubernetes connection', () => {
    expect(clusterAddress(nomad.cluster)).toBe('http://127.0.0.1:4646');
    expect(kubernetesConnection(nomad.cluster)).toBeNull();
    expect(isClusterKind(nomad, 'kubernetes')).toBe(false);
    expect(isClusterKind(null, 'nomad')).toBe(false);
  });
});
