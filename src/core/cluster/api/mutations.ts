import { useMutation, useQueryClient } from '@tanstack/react-query';
import { commands } from '@/shared/api';
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
