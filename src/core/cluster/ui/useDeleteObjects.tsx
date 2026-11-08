import type { ReactNode } from 'react';
import type { ClusterView } from '@/shared/api';
import { useConfirmedAction } from '@/shared/ui';
import { useDeleteObjects as useDeleteMutation } from '../api/actions';
import { deleteConfirmation, type DeleteTarget } from '../model/deletion';
import { t } from '@/shared/i18n';

/**
 * `const [dialog, remove] = useDeleteObjects(cluster)`: render `dialog`,
 * call `remove(targets, onDeleted?)` to ask, naming each object, its
 * Namespace and the Cluster, and delete them once confirmed.
 */
export function useDeleteObjects(
  cluster: ClusterView,
): [ReactNode, (targets: DeleteTarget[], onDeleted?: () => void) => void] {
  const [dialog, run] = useConfirmedAction();
  const remove = useDeleteMutation(cluster.cluster.id);
  return [
    dialog,
    (targets, onDeleted) => {
      if (targets.length === 0) return;
      const { title, description, consequences } = deleteConfirmation(
        cluster.cluster.name,
        targets,
      );
      run({
        title,
        description: (
          <div className="grid gap-2">
            <p className="break-words">{description}</p>
            {consequences.map((c) => (
              <p key={c}>{c}</p>
            ))}
          </div>
        ),
        confirmLabel: t('core.delete.confirm'),
        destructive: true,
        action: async () => {
          await remove.mutateAsync(targets);
          onDeleted?.();
        },
      });
    },
  ];
}
