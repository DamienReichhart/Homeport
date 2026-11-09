import type { Cluster, ClusterKind, ClusterView, KubernetesConnection } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** The name of each kind of Cluster, as the Cluster pages show it. */
export const CLUSTER_KIND_LABELS: Record<ClusterKind, MessageKey> = {
  kubernetes: 'core.cluster.kind.kubernetes',
  nomad: 'core.cluster.kind.nomad',
};

/** The address of a Cluster's API: a Kubernetes API server's URL, a Nomad agent's. */
export function clusterAddress(cluster: Cluster): string {
  const c = cluster.connection;
  return (c.kind === 'kubernetes' ? c.server : c.address) ?? '';
}

/** How a Kubernetes Cluster is reached; `null` for another kind. */
export function kubernetesConnection(cluster: Cluster): KubernetesConnection | null {
  return cluster.connection.kind === 'kubernetes' ? cluster.connection : null;
}

/** Whether the Cluster of `view` is of `kind`; `false` without one. */
export function isClusterKind(view: ClusterView | null | undefined, kind: ClusterKind): boolean {
  return view?.cluster.kind === kind;
}
