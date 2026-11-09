import type { ClusterView } from '@/shared/api';
import { kubernetesConnection } from './kind';

/** All Namespaces, as the settings store the choice. */
export const ALL_NAMESPACES = '';

/**
 * The Namespace a Cluster's pages show: the one remembered for it, else its
 * context's, else `default`. `null` means every Namespace.
 */
export function chosenNamespace(
  cluster: ClusterView,
  remembered: Partial<Record<string, string>> | undefined,
): string | null {
  const value = remembered?.[cluster.cluster.id];
  if (value === ALL_NAMESPACES) return null;
  return value ?? kubernetesConnection(cluster.cluster)?.namespace ?? 'default';
}
