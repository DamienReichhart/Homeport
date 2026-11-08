import { useMutation, useQueryClient } from '@tanstack/react-query';
import { commands, type ClusterView, type LocalCluster } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { clusterKeys } from './keys';
import { t } from '@/shared/i18n';

/** Connects a Cluster again at once, instead of waiting for its next retry. */
export function useReconnectCluster() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => commands.clustersReconnect(id),
    onSuccess: () => client.invalidateQueries({ queryKey: clusterKeys.list }),
    onError: (e) => toastError(e, t('core.cluster.reconnectFailed')),
  });
}

/** Adds a local cluster in one click, from this machine's kubeconfig. */
export function useAddLocalCluster(onAdded?: (view: ClusterView) => void) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (local: LocalCluster) =>
      commands.clustersImport([{ context: local.context, name: local.suggestedName }], null),
    onSuccess: async ([view]) => {
      await client.invalidateQueries({ queryKey: clusterKeys.list });
      await client.invalidateQueries({ queryKey: clusterKeys.localClusters });
      if (view) onAdded?.(view);
    },
    onError: (e) => toastError(e, t('core.localClusters.addFailed')),
  });
}
