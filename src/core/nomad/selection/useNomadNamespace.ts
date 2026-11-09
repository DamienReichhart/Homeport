import { useQuery } from '@tanstack/react-query';
import { useNamespace } from '@/core/cluster';
import type { ClusterView, NomadNamespaceView } from '@/shared/api';
import { nomadNamespacesQuery } from '../api/namespaces';
import { effectiveNamespace } from '../model/namespace';

export interface NomadNamespaceSelection {
  /** What to ask Nomad for: a name, `*` for all, `null` for the token's default. */
  namespace: string | null;
  /** The picker's value: a name, or `null` for all. */
  choice: string | null;
  choose: (choice: string | null) => void;
  namespaces: NomadNamespaceView[];
  /** Whether there is anything to pick: the token can list namespaces. */
  listable: boolean;
  /** Whether the namespaces are known, so `namespace` won't change on its own. */
  ready: boolean;
}

/**
 * The namespace a Nomad Cluster's Jobs page shows, remembered per Cluster
 * as a Kubernetes Cluster's is, with the namespaces to pick from.
 */
export function useNomadNamespace(cluster: ClusterView): NomadNamespaceSelection {
  const [choice, choose] = useNamespace(cluster);
  const listing = useQuery(nomadNamespacesQuery(cluster.cluster.id));
  return {
    namespace: effectiveNamespace(listing.data, choice),
    choice,
    choose,
    namespaces: listing.data?.namespaces ?? [],
    listable: listing.data?.listable ?? false,
    ready: !listing.isPending,
  };
}
