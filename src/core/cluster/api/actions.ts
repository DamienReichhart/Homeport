import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import type { DeleteTarget } from '../model/deletion';
import { clusterKeys } from './keys';
import { t } from '@/shared/i18n';

/** Deletes objects of a Cluster one after the other; the first failure stops it. */
export function useDeleteObjects(clusterId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: async (targets: DeleteTarget[]) => {
      for (const target of targets) {
        try {
          if ('kind' in target.type)
            await commands.clustersDelete(
              clusterId,
              target.type.kind,
              target.namespace,
              target.name,
              null,
            );
          else
            await commands.clustersApiDelete(
              clusterId,
              target.type.api,
              target.namespace,
              target.name,
            );
        } catch (e) {
          toastError(e, t('core.delete.failed', { name: target.name }));
          throw e;
        }
      }
    },
    onSuccess: (_, targets) => {
      toast.success(t('core.delete.done', { count: targets.length, name: targets[0]?.name ?? '' }));
    },
    onSettled: (_, __, targets) =>
      Promise.all(
        targets.map((target) =>
          client.invalidateQueries({
            queryKey:
              'kind' in target.type
                ? clusterKeys.kind(clusterId, target.type.kind)
                : clusterKeys.api(clusterId, target.type.api),
          }),
        ),
      ),
  });
}
